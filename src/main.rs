//! src/main.rs

use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Cannot connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = match std::net::TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Failed to bind to {address}: {err}");
            return Err(err);
        }
    };

    run(listener, connection_pool)?.await
}
