use std::net::TcpListener;

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

fn spawn_app() -> String {
    // -- port 0 tells the OS to find any available port for us to use
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to address");
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter::run(listener).expect("Failed to run the http server");
    let _ = tokio::spawn(server);

    format!("http://localhost:{port}")
}
