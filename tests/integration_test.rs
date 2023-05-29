use mailbucket::{args::Args, server};
use samotop_delivery::types::{EmailAddress, Envelope};
use std::time::Duration;

#[tokio::test]
async fn send_email_then_request() {
    let smtp_server = "localhost:2525";
    let http_server = "localhost:8080";

    let _server = tokio::task::spawn(server::run(Args {
        bind_smtp_addrs: smtp_server.to_string(),
        bind_http_addrs: http_server.to_string(),
    }));
    tokio::time::sleep(Duration::from_millis(100)).await; // TODO more reliable test

    let from = "from@example.com";
    let to: &str = "to@example.com";
    let body = "the body";

    samotop_delivery::smtp::SmtpClient::new(smtp_server)
        .unwrap()
        .connect_and_send(
            Envelope::new(
                Some(EmailAddress::new(from.to_string()).unwrap()),
                vec![EmailAddress::new(to.to_string()).unwrap()],
                "".to_string(),
            )
            .unwrap(),
            body.as_bytes(),
        )
        .await
        .unwrap();

    let email = reqwest::get(&format!("http://{http_server}/{to}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(email.contains(body));
}
