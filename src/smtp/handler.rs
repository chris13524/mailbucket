use super::dispatch::DispatchMail;
use super::DeliverMail;
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
        Ok(MailHandler {
            deliver_mail: delivered_mail,
        })
    }
}

impl<T: AcceptsDispatch> MailSetup<T> for MailHandler {
    fn setup(self, config: &mut T) {
        config.add_last_dispatch(DispatchMail::new(self.deliver_mail))
    }
}
