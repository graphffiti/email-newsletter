use actix_web::web;

use super::greet::*;
use super::health_check::*;
use super::subscriptions::*;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(greet)
        .service(health_check)
        .service(subscribe)
        .route("/hello", web::get().to(|| async { "Hello World!" }));
}
