use email_newsletter::{
    configuration::{get_configuration, DatabaseSettings},
    startup,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;

struct TestApp {
    address: String,
    db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let mut configuration = get_configuration().expect("Failed to load configuration");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();
    // -- port 0 tells the OS to find any available port for us to use
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to address");
    let port = listener.local_addr().unwrap().port();

    let db_pool = configure_database(&configuration.database).await;

    let server = startup::run(listener, db_pool.clone()).expect("Failed to run the http server");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://localhost:{port}"),
        db_pool,
    }
}

async fn configure_database(configuration_database: &DatabaseSettings) -> PgPool {
    let mut connection =
        PgConnection::connect(&configuration_database.connection_string_without_db())
            .await
            .expect("Failed to connect to Postgres");

    connection
        .execute(
            format!(
                r#"CREATE DATABASE "{}";"#,
                configuration_database.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to execute the query");

    let connection_pool = PgPool::connect(&configuration_database.connection_string())
        .await
        .expect("Failed to connect to DB.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // -- ACTION
    let response = client
        .get(&format!("{}/health-check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // -- TESTS
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // -- ARRANGE
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=graphffiti%20witti&email=graphffiti.witti%40gmail.com";

    // -- ACTION
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Fail to execute request.");

    // -- TESTS
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!("graphffiti witti", saved.name);
    assert_eq!("graphffiti.witti@gmail.com", saved.email);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=graphffiti%20witti", "missing name"),
        ("email=graphffiti.witti%40gmail.com", "missing email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // -- TESTS
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {error_message}"
        );
    }
}
