use std::net::TcpListener;

use noucra::{configuration, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = configuration::get_configuration().expect("Failed to read configuration.");

    let address = format!("0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener)?.await
}
