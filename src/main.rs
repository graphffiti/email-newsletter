use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main] // or #[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:8080";
    let listener = TcpListener::bind(address).expect("Failed to bind to address {address}");

    run(listener)?.await
}
