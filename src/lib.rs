mod routes;
use actix_files::NamedFile;
use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().configure(routes::config))
        .listen(listener)?
        .run();

    Ok(server)
}
