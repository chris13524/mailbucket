use clap::Parser;
use mailbucket::args::Args;
use mailbucket::server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    server::run(args).await
}
