use super::{stream::Stream, DeliverMail};
use samotop_core::mail::DispatchError;
use samotop_delivery::{types::Envelope, SyncFuture, Transport};

pub struct MailTransport {
    deliver_mail: DeliverMail,
}

impl MailTransport {
    pub fn new(delivered_mail: DeliverMail) -> MailTransport {
        MailTransport {
            deliver_mail: delivered_mail,
        }
    }
}

impl std::fmt::Debug for MailTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MailTransport").finish()
    }
}

impl Transport for MailTransport {
    type DataStream = Stream;
    type Error = DispatchError; // TODO remove dep on DispatchError
    fn send_stream<'s, 'a>(
        &'s self,
        envelope: Envelope,
    ) -> SyncFuture<'a, Result<Stream, Self::Error>>
    where
        's: 'a,
    {
        Box::pin(async move { Ok(Stream::new(envelope, self.deliver_mail)) })
    }
}
