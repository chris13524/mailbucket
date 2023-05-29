mod dispatch;
mod handler;
mod stream;

use self::handler::MailHandler;
use crate::server::EmailHandler;
use samotop::smtp::SmtpParser;
use samotop_core::{
    mail::{Builder, DebugService},
    server::TcpServer,
    smtp::Esmtp,
};
use samotop_delivery::types::Envelope;
use serde::Serialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Debug)]
pub struct Email {
    pub envelope: Envelope,
    pub body: String,
}

pub async fn run(bind_addrs: &str, email_handler: EmailHandler) -> Result<()> {
    let service = Builder
        + DebugService::default()
        + Esmtp.with(SmtpParser)
        + MailHandler::new(email_handler);

    TcpServer::on(bind_addrs).serve(service.build()).await
}
