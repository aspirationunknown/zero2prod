//! src/routes/subscriptions.rs

use axum::{Form, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(subscription_form: Form<FormData>) -> StatusCode {
    let name = subscription_form.0.name;
    let email = subscription_form.0.email;
    println!("Subscriber info: name = {name}, email = {email}");
    StatusCode::OK
}
