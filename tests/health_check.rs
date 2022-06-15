use std::net::TcpListener;

use noucra::run;

fn spawn_app() -> String {
    // create a tcp listner for server
    let listener = TcpListener::bind("0:0").expect("Failed to bind to a port.");
    let port = listener.local_addr().unwrap().port();

    // start the server
    let server = run(listener).expect("Failed to start the server.");
    let _ = tokio::spawn(server);

    format!("http://0:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Request failed.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
