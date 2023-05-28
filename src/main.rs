mod dispatch;
mod handler;
mod stream;
mod transport;
mod args;

use args::Args;
use clap::Parser;
use handler::MailHandler;
use samotop::mail::{Builder, DebugService};
use samotop::server::TcpServer;
use samotop::smtp::{Esmtp, SmtpParser};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::init();
    let service = Builder + DebugService::default() + Esmtp.with(SmtpParser) + MailHandler::new()?;

    TcpServer::on(args.addrs)
        .serve(service.build())
        .await
}
