use axum::{Router, extract::Path, http::StatusCode, routing::get};

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!\n").to_string()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    // application currently has a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!\n" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check));

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::health_check;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;

        assert_eq!(response, StatusCode::OK);
    }
}
