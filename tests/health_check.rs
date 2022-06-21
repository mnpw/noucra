use std::net::TcpListener;

use noucra::{
    configuration::{get_configuration, DatabaseSettings},
    startup,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "debug".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

struct TestApp {
    address: String,
    db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // Initialize tracing setup only once
    Lazy::force(&TRACING);

    // create a tcp listner for server
    let listener = TcpListener::bind("0:0").expect("Failed to bind to a port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://0:{}", port);

    // create a new db, and its connection pool
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(configuration.database).await;

    // start the server
    let server =
        startup::run(listener, connection_pool.clone()).expect("Failed to start the server.");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

async fn configure_database(db_config: DatabaseSettings) -> PgPool {
    // create a new database
    let mut connection = PgConnection::connect(&db_config.connection_url_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // migrate database
    let connection_pool = PgPool::connect(&db_config.connection_url())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Request failed.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_for_valid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=mrinal%20paliwal&email=dummy%40mail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Request failed.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "dummy@mail.com");
    assert_eq!(saved.name, "mrinal paliwal");
}

#[tokio::test]
async fn subscribe_for_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=mrinal%20paliwal", "missing email"),
        ("email=dummy%40mail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, case) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
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
