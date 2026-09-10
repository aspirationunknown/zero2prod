//! src/routes/subscriptions.rs

use axum::{Form, extract::State, http::StatusCode};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
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
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_name = %subscription_form.name,
        subscriber_email = %subscription_form.email
    );
    // _request_span_guard is dropped naturally at the end of this function
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
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
    // This call to instrument handles the calling of enter() on the query_span.
    // This is how spans are handled when we want them to be entered and exited automatically
    // until the query is complete and the span is closed.
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "Successfully added request_id: {}; name: '{}'; email: '{}' as a subscriber.",
                request_id,
                subscription_form.name,
                subscription_form.email
            );
            StatusCode::OK
        }
        Err(err) => {
            tracing::error!(
                "Failed to execute query; request_id: {}; name: '{}'; email: '{}'; error: {:?}",
                request_id,
                subscription_form.name,
                subscription_form.email,
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
