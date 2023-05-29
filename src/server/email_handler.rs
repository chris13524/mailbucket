use super::smtp_server::Email;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub type Emails = Arc<Mutex<HashMap<String, Email>>>;

#[derive(Debug, Clone)]
pub struct EmailHandler {
    emails: Emails,
}

impl EmailHandler {
    pub fn new(emails: Emails) -> Self {
        EmailHandler { emails }
    }

    pub async fn deliver_email(self, email: Email) {
        let mut emails = self.emails.lock().await;
        emails.insert(email.envelope.to()[0].to_string(), email);
    }
}
