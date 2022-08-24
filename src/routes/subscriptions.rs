use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]

struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<FormData>, db_conn: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name) VALUES ($1, $2, $3)"#,
        Uuid::new_v4(),
        form.email,
        form.name
    )
    .execute(db_conn.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            println!("Failed to execute query: {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
