use super::routes;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().configure(routes::config))
        .listen(listener)?
        .run();

    Ok(server)
}
