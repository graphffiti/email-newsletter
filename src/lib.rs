mod routes;
use actix_files::NamedFile;
use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub fn run() -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().configure(routes::config))
        .bind(("0.0.0.0", 8080))?
        .run();

    Ok(server)
}
