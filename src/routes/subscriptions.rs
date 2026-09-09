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
    let request_id = Uuid::new_v4();
    log::info!(
        "Request_ID: {}; Adding Name: '{}'; Email: '{}' as a new subscriber.",
        request_id,
        subscription_form.name,
        subscription_form.email
    );
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
        Ok(_) => {
            log::info!(
                "Successfully added Request_ID: {}; Name: '{}'; Email: '{}' as a subscriber.",
                request_id,
                subscription_form.name,
                subscription_form.email
            );
            StatusCode::OK
        }
        Err(err) => {
            log::error!(
                "Failed to execute query: Request_ID: {}; Name: '{}'; Email: '{}'; Error: {:?}",
                request_id,
                subscription_form.name,
                subscription_form.email,
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
