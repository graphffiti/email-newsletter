use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(greet)
        .service(greet_hi)
        .service(health_check)
        .route("/hello", web::get().to(|| async { "Hello World!" }))
        .route(
            "/hi",
            web::get().to(|| async {
                "<!DOCTYPE html>
                <html>
                <head>
                    <title>Page AWESOME Title</title>
                </head>
                <body>
                    
                    <h1>My First Heading</h1>
                    <p>My first paragraph.</p>
                    
                </body>
                </html>"
            }),
        );
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    //format!("Hello {name}!")
    NamedFile::open_async("static/404.html").await
}

#[get("/hi/{name}")]
async fn greet_hi(name: web::Path<String>) -> HttpResponse {
    // format!("Hi {name}!")
    HttpResponse::Ok().body(format!(
        "<!DOCTYPE html>
            <html>
            <head>
            <title>Page Title</title>
            </head>
            <body>
            
            <h1>Hello {}</h1>
            <p>My first paragraph.</p>
            
            </body>
            </html>",
        name
    ))
}

#[get("/health-check")]
async fn health_check() -> HttpResponseBuilder {
    HttpResponse::Ok()
}
