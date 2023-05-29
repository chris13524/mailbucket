use crate::server::{smtp_server::Email, EmailHandler};
use futures_lite::FutureExt;
use futures_util::io::Cursor;
use log::{debug, trace};
use pin_project::pin_project;
use samotop_core::common::*;
use samotop_delivery::{types::Envelope, MailDataStream};

enum State {
    Writing {
        email_handler: EmailHandler,
        envelope: Envelope,
        buf: Cursor<Vec<u8>>,
    },
    Closing {
        email_handler_future: Pin<Box<dyn Future<Output = ()> + Send + Sync>>,
    },
    Closed,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Writing {
                email_handler,
                envelope,
                buf,
            } => f
                .debug_struct("State::Writing")
                .field("email_handler", email_handler)
                .field("envelope", envelope)
                .field("buf", buf)
                .finish(),
            State::Closing { .. } => write!(f, "State::Closing(..)"),
            State::Closed => write!(f, "State::Closed"),
        }
    }
}

#[pin_project(project=StreamProj)]
#[derive(Debug)]
pub struct Stream {
    state: State,
}

impl Stream {
    pub fn new(envelope: Envelope, email_handler: EmailHandler) -> Self {
        Self {
            state: State::Writing {
                buf: Cursor::new(Vec::new()),
                envelope,
                email_handler,
            },
        }
    }
}

impl io::Write for Stream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        debug!("poll_write");
        let state = self.project().state;
        if let State::Writing { buf: cursor, .. } = state {
            Pin::new(cursor).poll_write(cx, buf)
        } else {
            panic!("Can only poll_write when in State::Writing but was in {state:?}");
        }
    }

    fn poll_flush<'a>(self: Pin<&'a mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        trace!("poll_flush");
        let state = self.project().state;
        match state {
            State::Writing { buf, .. } => Pin::new(buf).poll_flush(cx),
            State::Closing { .. } | State::Closed => Poll::Ready(Ok(())),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        trace!("poll_close");
        ready!(self.as_mut().poll_flush(cx))?;

        let state = self.project().state;
        match state {
            State::Writing {
                email_handler,
                envelope,
                buf,
            } => {
                let body = String::from_utf8(buf.get_ref().clone()).unwrap();
                let mut email_handler_future =
                    Box::pin(email_handler.clone().deliver_email(Email {
                        envelope: envelope.clone(), // TODO can we move envelope instead of cloning it?
                        body,
                    }));
                let result = email_handler_future.poll(cx).map(|_| Ok(()));
                *state = State::Closing {
                    email_handler_future,
                };
                result
            }
            State::Closing {
                email_handler_future,
            } => {
                let result = email_handler_future.poll(cx).map(|_| Ok(()));
                *state = State::Closed;
                result
            }
            State::Closed => Poll::Ready(Ok(())),
        }
    }
}

impl MailDataStream for Stream
where
    Self: fmt::Debug + io::Write,
{
    fn is_done(&self) -> bool {
        matches!(self.state, State::Closed)
    }
}
