use axum::{Router, extract::Path, http::StatusCode, routing::get};
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!\n")
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn run(
    listener: std::net::TcpListener,
) -> std::io::Result<impl std::future::Future<Output = std::io::Result<()>> + Send + 'static> {
    // application currently has a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!\n" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check));

    listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(listener)?;

    Ok(async move { axum::serve(listener, app).await })
}
