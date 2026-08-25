use axum::{Router, extract::Path, routing::get};

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!\n").to_string()
}

#[tokio::main]
async fn main() {
    // application currently has a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!\n" }))
        .route("/{name}", get(greet));

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
