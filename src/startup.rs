//! src/startup.rs

use crate::routes::{health_check, subscribe};
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use std::future::Future;
use std::io::Result;
use tokio::net::TcpListener;

pub fn run(
    listener: std::net::TcpListener,
    pool: PgPool,
) -> Result<impl Future<Output = Result<()>> + Send + 'static> {
    // application currently has a single route
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(pool);

    listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(listener)?;

    Ok(async move { axum::serve(listener, app).await })
}
