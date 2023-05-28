use crate::stream::Stream;
use samotop_core::mail::DispatchError;
use samotop_delivery::{types::Envelope, SyncFuture, Transport};

#[derive(Debug)]
pub struct MailTransport {}

impl MailTransport {
    pub fn new() -> MailTransport {
        MailTransport {}
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
        let id = envelope.message_id().to_owned();

        let mut headers = String::new();
        if let Some(sender) = envelope.from() {
            headers += format!("X-Samotop-From: {}\r\n", sender).as_str();
        }
        for rcpt in envelope.to() {
            headers += format!("X-Samotop-To: {}\r\n", rcpt).as_str();
        }

        Box::pin(async move { Ok(Stream::new(id)) })
    }
}
