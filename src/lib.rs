use axum::{Router, extract::Path, http::StatusCode, routing::get};
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!\n").to_string()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn run()
-> std::io::Result<impl std::future::Future<Output = std::io::Result<()>> + Send + 'static> {
    // application currently has a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!\n" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check));

    // run app with hyper, listening globally on port 3000
    let listener = std::net::TcpListener::bind("127.0.0.1:8000")?;
    listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(listener)?;

    Ok(async move { axum::serve(listener, app).await })
}
