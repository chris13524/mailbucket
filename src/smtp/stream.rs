use super::DeliverMail;
use crate::smtp::Email;
use futures_util::io::Cursor;
use log::{debug, trace};
use pin_project::pin_project;
use samotop_core::common::*;
use samotop_delivery::{types::Envelope, MailDataStream};

#[pin_project(project=StreamProj)]
pub struct Stream {
    closed: bool,
    buf: Cursor<Vec<u8>>,
    envelope: Envelope,
    deliver_mail: DeliverMail,
}

impl Stream {
    pub fn new(envelope: Envelope, deliver_mail: DeliverMail) -> Self {
        Self {
            closed: false,
            buf: Cursor::new(Vec::new()),
            envelope,
            deliver_mail,
        }
    }
}

impl std::fmt::Debug for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stream")
            .field("closed", &self.closed)
            .finish()
    }
}

impl io::Write for Stream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        debug!("poll_write");
        Pin::new(self.project().buf).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        trace!("poll_flush");
        Pin::new(self.project().buf).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        trace!("poll_close");
        ready!(self.as_mut().poll_flush(cx))?;

        let body = String::from_utf8(self.buf.get_ref().clone()).unwrap();
        let deliver_mail = self.as_mut().project().deliver_mail;
        deliver_mail(Email {
            envelope: self.as_mut().envelope.clone(),
            body,
        });

        *self.project().closed = true;
        Poll::Ready(Ok(()))
    }
}

impl MailDataStream for Stream {
    fn is_done(&self) -> bool {
        self.closed
    }
}
