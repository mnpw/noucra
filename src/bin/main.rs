use std::net::TcpListener;

use noucra::{configuration, startup, telemetry};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("noucra".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.connection_url().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener, connection_pool)?.await
}
