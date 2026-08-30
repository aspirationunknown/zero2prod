//! src/startup.rs

use crate::routes::health_check;
use crate::routes::subscribe;
use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

pub fn run(
    listener: std::net::TcpListener,
) -> std::io::Result<impl std::future::Future<Output = std::io::Result<()>> + Send + 'static> {
    // application currently has a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!\n" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(listener)?;

    Ok(async move { axum::serve(listener, app).await })
}
