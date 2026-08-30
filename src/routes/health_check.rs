//! src/routes/health_check.rs

use axum::http::StatusCode;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
