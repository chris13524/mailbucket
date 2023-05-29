mod email_handler;
mod http_server;
mod smtp_server;

pub use email_handler::EmailHandler;

use crate::args::Args;
use smtp_server::Email;
use std::{collections::HashMap, sync::Arc};
use tokio::{select, sync::Mutex};

pub async fn run(args: Args) {
    let emails = Arc::new(Mutex::new(HashMap::<String, Email>::new()));

    let http_server = http_server::run(&args.bind_http_addrs, emails.clone());

    let smtp_server = smtp_server::run(&args.bind_smtp_addrs, EmailHandler::new(emails.clone()));

    select! {
        _ = http_server => {},
        e = smtp_server => e.unwrap(),
    }
}
