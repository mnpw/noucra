use std::net::TcpListener;

use env_logger::Env;
use noucra::{configuration, startup};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // configure Logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.connection_url())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener, connection_pool)?.await
}
