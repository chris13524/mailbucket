use super::dispatch::DispatchMail;
use crate::email_handler::EmailHandler;
use samotop_core::{common::*, mail::*};

pub struct MailHandler {
    email_handler: EmailHandler,
}

impl fmt::Debug for MailHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MailHandler").finish()
    }
}

impl MailHandler {
    pub fn new(email_handler: EmailHandler) -> Self {
        MailHandler { email_handler }
    }
}

impl<T: AcceptsDispatch> MailSetup<T> for MailHandler {
    fn setup(self, config: &mut T) {
        config.add_last_dispatch(DispatchMail::new(self.email_handler))
    }
}
