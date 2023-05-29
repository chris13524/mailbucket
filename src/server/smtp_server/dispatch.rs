use crate::server::{smtp_server::stream::Stream, EmailHandler};
use log::{error, trace};
use samotop_core::{common::*, mail::*, smtp::SmtpSession};
use samotop_delivery::prelude::{EmailAddress, Envelope};

#[derive(Debug)]
pub struct DispatchMail {
    email_handler: EmailHandler,
}

impl DispatchMail {
    pub fn new(email_handler: EmailHandler) -> Self {
        Self { email_handler }
    }
}

impl MailDispatch for DispatchMail {
    fn open_mail_body<'a, 's, 'f>(
        &'a self,
        session: &'s mut SmtpSession,
    ) -> S1Fut<'f, DispatchResult>
    where
        'a: 'f,
        's: 'f,
    {
        Box::pin(async move {
            match email_handler(&self.email_handler, &mut session.transaction).await {
                Err(e) => {
                    error!("Failed to start mail: {:?}", e);
                    Err(DispatchError::Temporary)
                }
                Ok(()) => Ok(()),
            }
        })
    }
}

async fn email_handler(email_handler: &EmailHandler, transaction: &mut Transaction) -> Result<()> {
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
    transaction.sink = Some(Box::pin(Stream::new(envelope, email_handler.clone())));

    Ok(())
}
