mod args;
mod email_handler;
mod smtp;

use crate::email_handler::EmailHandler;
use args::Args;
use clap::Parser;
use smtp::{smtp_server, Email};
use std::{collections::HashMap, net::ToSocketAddrs, sync::Arc};
use tokio::{select, sync::Mutex};
use warp::{hyper::StatusCode, reply::json, Filter, Reply};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let emails = Arc::new(Mutex::new(HashMap::<String, Email>::new()));

    let hello = warp::path!(String).then({
        let emails = emails.clone();
        move |email_address| {
            let emails = emails.clone();
            async move {
                let emails = emails.lock().await;
                let email = emails.get(&email_address);
                if let Some(email) = email {
                    json(&email).into_response()
                } else {
                    StatusCode::NOT_FOUND.into_response()
                }
            }
        }
    });

    let http_socket_addr = args
        .bind_http_addrs
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let http_server = warp::serve(hello).run(http_socket_addr);

    let smtp_server = smtp_server(&args.bind_smtp_addrs, EmailHandler::new(emails.clone()));

    select! {
        _ = http_server => {},
        e = smtp_server => e.unwrap(),
    }

    Ok(())
}
