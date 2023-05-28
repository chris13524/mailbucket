use crate::dispatch::DispatchMail;
use crate::transport::MailTransport;
use samotop_core::{common::*, mail::*};

#[derive(Debug)]
pub struct MailHandler {
}

impl MailHandler {
    pub fn new() -> Result<MailHandler> {
        Ok(MailHandler { })
    }
}

impl<T: AcceptsDispatch> MailSetup<T> for MailHandler
where
    DispatchMail<MailTransport>: MailDispatch,
{
    fn setup(self, config: &mut T) {
        let transport = MailTransport::new();
        config.add_last_dispatch(DispatchMail::new(transport))
    }
}
