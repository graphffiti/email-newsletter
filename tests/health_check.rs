#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    // -- ACTION
    let response = client
        .get("http://localhost:8080/health-check")
        .send()
        .await
        .expect("Failed to execute request.");

    // -- TESTS
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = email_newsletter::run().expect("Failed to run the http server");

    let _ = tokio::spawn(server);
}
