mod dispatch;
mod handler;
mod stream;

use self::handler::MailHandler;
use samotop::smtp::SmtpParser;
use samotop_core::{
    mail::{Builder, DebugService},
    server::TcpServer,
    smtp::Esmtp,
};
use samotop_delivery::types::Envelope;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct Email {
    pub envelope: Envelope,
    pub body: String,
}

pub type DeliverMail = fn(email: Email) -> ();

pub async fn smtp_server(bind_addrs: &str, deliver_mail: DeliverMail) -> Result<()> {
    let service = Builder
        + DebugService::default()
        + Esmtp.with(SmtpParser)
        + MailHandler::new(deliver_mail)?;

    TcpServer::on(bind_addrs).serve(service.build()).await
}
