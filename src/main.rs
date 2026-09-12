//! src/main.rs

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to set logger");
    // This logger uses the RUST_LOG environment variable to determine log filtering level
    // or if it is not set then it defaults to "info" level logging
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // Output the formatted spans to stdout
        std::io::stdout,
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
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
            tracing::error!("Failed to bind to {address}: {err}");
            return Err(err);
        }
    };

    run(listener, connection_pool)?.await
}
