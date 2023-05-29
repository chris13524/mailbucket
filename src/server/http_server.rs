use super::email_handler::Emails;
use std::net::ToSocketAddrs;
use warp::{
    hyper::StatusCode,
    reply::{json, Response},
    Filter, Reply,
};

pub async fn run(bind_http_addrs: &str, emails: Emails) {
    let http_socket_addr = bind_http_addrs.to_socket_addrs().unwrap().next().unwrap();

    let hello =
        warp::path!(String).then(move |email_address| get_email(emails.clone(), email_address));

    warp::serve(hello).run(http_socket_addr).await
}

async fn get_email(emails: Emails, email_address: String) -> Response {
    let emails = emails.lock().await;
    let email = emails.get(&email_address);
    if let Some(email) = email {
        json(&email).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
