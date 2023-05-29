mod args;
mod dispatch;
mod handler;
mod stream;
mod transport;

use args::Args;
use clap::Parser;
use handler::MailHandler;
use samotop::mail::{Builder, DebugService};
use samotop::server::TcpServer;
use samotop::smtp::{Esmtp, SmtpParser};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    smtp_server(&args.addrs, |email| println!("{email}")).await
}

pub type DeliverMail = fn(email: &str) -> ();

async fn smtp_server(bind_addrs: &str, deliver_mail: DeliverMail) -> Result<()> {
    let service = Builder
        + DebugService::default()
        + Esmtp.with(SmtpParser)
        + MailHandler::new(deliver_mail)?;

    TcpServer::on(bind_addrs).serve(service.build()).await
}
