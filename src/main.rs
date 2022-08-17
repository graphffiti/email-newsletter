use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

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

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(greet);
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
            )
            .configure(config)
            .service(greet_hi)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
