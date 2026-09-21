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

#[tracing::instrument (
    name = "Adding a new subscriber",
    skip(pool, subscription_form),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %subscription_form.email,
        subscriber_name = %subscription_form.name
    )
)]
pub async fn subscribe(
    State(pool): State<PgPool>,
    Form(subscription_form): Form<FormData>,
) -> StatusCode {
    match insert_subscriber(&pool, &subscription_form).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        err
        // Using '?' operator to return early
    })?;
    Ok(())
}
