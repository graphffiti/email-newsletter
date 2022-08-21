use email_newsletter::startup;
use std::net::TcpListener;

fn spawn_app() -> String {
    // -- port 0 tells the OS to find any available port for us to use
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to address");
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("Failed to run the http server");
    let _ = tokio::spawn(server);

    format!("http://localhost:{port}")
}

#[tokio::test]
async fn health_check_works() {
    let uri = spawn_app();
    let client = reqwest::Client::new();

    // -- ACTION
    let response = client
        .get(&format!("{uri}/health-check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // -- TESTS
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let uri = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=graphffiti%20witti&email=graphffiti.witti%40gmail.com";

    // -- ACTION
    let response = client
        .post(format!("{uri}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Fail to execute request.");

    // -- TESTS
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let uri = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=graphffiti%20witti", "missing name"),
        ("email=graphffiti.witti%40gmail.com", "missing email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{uri}/subscriptions"))
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
