mod dispatch;
mod handler;
mod stream;
mod transport;

use handler::MailHandler;
use log::info;
use samotop::mail::{Builder, DebugService};
use samotop::server::TcpServer;
use samotop::smtp::{Esmtp, SmtpParser};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let service = Builder + DebugService::default() + Esmtp.with(SmtpParser) + MailHandler::new()?;

    TcpServer::on("localhost:2525".to_string())
        .serve(service.build())
        .await
}
