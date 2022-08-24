use crate::routes::service_config;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{PgConnection, PgPool};
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_connection: PgPool) -> std::io::Result<Server> {
    let db_connection = web::Data::new(db_connection);

    let server = HttpServer::new(move || {
        App::new()
            .configure(service_config::config)
            .app_data(db_connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
