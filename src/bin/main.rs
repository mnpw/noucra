use std::net::TcpListener;

use noucra::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0:8000").expect("Failed to bind to a port.");

    run(listener)?.await
}
