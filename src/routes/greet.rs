use actix_files::NamedFile;
use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
    // NamedFile::open_async("static/404.html").await
}
