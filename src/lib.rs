use axum::{
    Form, Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!\n")
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(subscription_form: Form<FormData>) -> StatusCode {
    let name = subscription_form.0.name;
    let email = subscription_form.0.email;
    println!("Subscriber info: name = {name}, email = {email}");
    StatusCode::OK
}

pub fn run(
    listener: std::net::TcpListener,
) -> std::io::Result<impl std::future::Future<Output = std::io::Result<()>> + Send + 'static> {
    // application currently has a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!\n" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(listener)?;

    Ok(async move { axum::serve(listener, app).await })
}
