//! src/routes/subscriptions.rs

use axum::{Form, extract::State, http::StatusCode};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    State(pool): State<PgPool>,
    Form(subscription_form): Form<FormData>,
) -> StatusCode {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscription_form.email,
        subscription_form.name,
        Utc::now()
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            println!("Failed to execute query: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
