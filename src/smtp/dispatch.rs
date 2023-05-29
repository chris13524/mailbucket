use log::{error, trace};
use samotop_core::{common::*, mail::*, smtp::SmtpSession};
use samotop_delivery::prelude::{EmailAddress, Envelope, Transport};
use std::fmt;

#[derive(Debug)]
pub struct DispatchMail<T> {
    transport: T,
}

impl<T> DispatchMail<T> {
    pub fn new(transport: T) -> Self
    where
        T: fmt::Debug,
    {
        Self { transport }
    }
}

impl<T> MailDispatch for DispatchMail<T>
where
    T: Transport + Send + Sync,
    T::DataStream: Sync + Send + 'static,
    T::Error: std::error::Error + Sync + Send + 'static,
{
    fn open_mail_body<'a, 's, 'f>(
        &'a self,
        session: &'s mut SmtpSession,
    ) -> S1Fut<'f, DispatchResult>
    where
        'a: 'f,
        's: 'f,
    {
        Box::pin(async move {
            match deliver_mail(&self.transport, &mut session.transaction).await {
                Err(e) => {
                    error!("Failed to start mail: {:?}", e);
                    Err(DispatchError::Temporary)
                }
                Ok(()) => Ok(()),
            }
        })
    }
}

async fn deliver_mail<T>(transport: &T, transaction: &mut Transaction) -> Result<()>
where
    T: Transport + Send + Sync,
    T::DataStream: Sync + Send + 'static,
    T::Error: std::error::Error + Sync + Send + 'static,
{
    let sender = transaction
        .mail
        .as_ref()
        .map(|mail| EmailAddress::new(mail.sender().address()))
        .transpose()?;
    let recipients: std::result::Result<Vec<_>, _> = transaction
        .rcpts
        .iter()
        .map(|rcpt| EmailAddress::new(rcpt.address.address()))
        .collect();

    let envelope =
        Envelope::new(sender, recipients?, transaction.id.clone()).map_err(Error::from)?;
    trace!("Starting downstream mail transaction.");
    let stream = transport.send_stream(envelope).await?;
    transaction.sink = Some(Box::pin(stream));

    Ok(())
}
