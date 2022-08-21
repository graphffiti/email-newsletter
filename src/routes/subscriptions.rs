use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
