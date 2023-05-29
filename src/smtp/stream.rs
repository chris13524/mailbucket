use super::DeliverMail;
use futures::io::AsyncWriteExt;
use futures_lite::future::FutureExt;
use futures_util::io::Cursor;
use log::{debug, trace};
use pin_project::pin_project;
use samotop_core::common::*;
use samotop_delivery::MailDataStream;

#[pin_project(project=StreamProj)]
pub struct Stream {
    id: String,
    closed: bool,
    buf: Cursor<Vec<u8>>,
    deliver_mail: DeliverMail,
}

impl Stream {
    pub fn new(id: String, deliver_mail: DeliverMail) -> Self {
        Self {
            id,
            closed: false,
            buf: Cursor::new(Vec::new()),
            deliver_mail,
        }
    }
}

impl std::fmt::Debug for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stream")
            .field("id", &self.id)
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
        debug!(
            "poll_write: Writing data for {}: {} bytes.",
            self.id,
            buf.len()
        );
        self.project()
            .buf
            .write_all(buf)
            .poll(cx)
            .map(|x| x.map(|_| buf.len()))
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        trace!("poll_flush: Flushing data for {}.", self.id);
        self.project().buf.flush().poll(cx)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        trace!("poll_close");
        ready!(self.as_mut().poll_flush(cx))?;

        let mail = String::from_utf8(self.buf.get_ref().clone()).unwrap();
        let deliver_mail = self.as_mut().project().deliver_mail;
        deliver_mail(&mail);

        *self.project().closed = true;
        Poll::Ready(Ok(()))
    }
}

impl MailDataStream for Stream {
    fn is_done(&self) -> bool {
        self.closed
    }
}
