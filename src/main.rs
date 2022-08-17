use email_newsletter::run;

#[tokio::main] // or #[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
