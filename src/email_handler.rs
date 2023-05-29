use crate::smtp::Email;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct EmailHandler {
    emails: Arc<Mutex<HashMap<String, Email>>>,
}

impl EmailHandler {
    pub fn new(emails: Arc<Mutex<HashMap<String, Email>>>) -> Self {
        EmailHandler { emails }
    }

    pub async fn deliver_email(self, email: Email) {
        let mut emails = self.emails.lock().await;
        emails.insert(email.envelope.to()[0].to_string(), email);
    }
}
