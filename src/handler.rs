use crate::transport::MailTransport;
use crate::{dispatch::DispatchMail, DeliverMail};
use samotop_core::{common::*, mail::*};

pub struct MailHandler {
    deliver_mail: DeliverMail,
}

impl fmt::Debug for MailHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MailHandler").finish()
    }
}

impl MailHandler {
    pub fn new(delivered_mail: DeliverMail) -> Result<MailHandler> {
        Ok(MailHandler { deliver_mail: delivered_mail })
    }
}

impl<T: AcceptsDispatch> MailSetup<T> for MailHandler
where
    DispatchMail<MailTransport>: MailDispatch,
{
    fn setup(self, config: &mut T) {
        let transport = MailTransport::new(self.deliver_mail);
        config.add_last_dispatch(DispatchMail::new(transport))
    }
}
