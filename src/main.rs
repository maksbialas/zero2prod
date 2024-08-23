use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind(format!("localhost:{}", config.application_port))
        .expect("Failed to bind to address");
    run(listener)?.await
}
