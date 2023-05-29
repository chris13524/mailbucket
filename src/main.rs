mod args;
mod smtp;

use args::Args;
use clap::Parser;
use smtp::smtp_server;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    smtp_server(&args.addrs, |email| {
        println!(
            "Received email (from:{}, to:{}) body: {}",
            email.envelope.from().unwrap(),
            email.envelope.to()[0],
            email.body
        )
    })
    .await
}
