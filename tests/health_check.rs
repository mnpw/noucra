use std::net::TcpListener;

use noucra::{configuration::get_configuration, startup};
use sqlx::{Connection, PgConnection};

fn spawn_app() -> String {
    // create a tcp listner for server
    let listener = TcpListener::bind("0:0").expect("Failed to bind to a port.");
    let port = listener.local_addr().unwrap().port();

    // start the server
    let server = startup::run(listener).expect("Failed to start the server.");
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

#[tokio::test]
async fn subscribe_for_valid_data() {
    let app_addr = spawn_app();
    let config = get_configuration().expect("Failed to read configuration.");
    let db_connection_string = config.database.connection_url();
    let mut connection = PgConnection::connect(&db_connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let client = reqwest::Client::new();

    let body = "name=mrinal%20paliwal&email=dummy%40mail.com";
    let response = client
        .post(format!("{}/subscriptions", app_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Request failed.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "dummy@mail.com");
    assert_eq!(saved.name, "mrinal paliwal");
}

#[tokio::test]
async fn subscribe_for_invalid_data() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=mrinal%20paliwal", "missing email"),
        ("email=dummy%40mail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, case) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Request failed.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "Did not fail with payload {}",
            case
        );
    }
}
