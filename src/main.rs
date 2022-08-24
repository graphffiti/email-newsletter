use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;

use email_newsletter::configuration;
use email_newsletter::startup;

#[tokio::main] // or #[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration =
        configuration::get_configuration().expect("Failed to read configuration file");

    let connection_string = configuration.database.connection_string();
    let connection = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!(
        "{}:{}",
        configuration.application_host, configuration.application_port
    );

    let listener = TcpListener::bind(address).expect("Failed to bind to address {address}");

    startup::run(listener, connection)?.await
}
