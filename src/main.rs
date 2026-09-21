//! src/main.rs

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // This logger uses the RUST_LOG environment variable to determine log filtering level
    // or if it is not set then it defaults to "info" level logging
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    // init_subscriber should only be ever called once
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(configuration.database.connection_string().expose_secret())
        .await
        .expect("Cannot connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = match std::net::TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!("Failed to bind to {address}: {err}");
            return Err(err);
        }
    };

    run(listener, connection_pool)?.await
}
