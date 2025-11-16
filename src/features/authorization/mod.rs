mod sign_in;

use axum::{
    Form, Router,
    response::{IntoResponse, Redirect},
    routing::{get, post},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthorizeFormData {
    email: String,
    password: String,
}

async fn authorize(
    Form(body): Form<AuthorizeFormData>,
) -> Result<Redirect, axum::http::StatusCode> {
    let expected_email = std::env::var("SIGN_IN_EMAIL").unwrap();
    let expected_password = std::env::var("SIGN_IN_PASSWORD").unwrap();

    if body.email != expected_email || body.password != expected_password {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    Ok(Redirect::to("/client/authorization_callback?code=123"))
}

async fn token() -> impl IntoResponse {
    "[[TODO: token string]]"
}

pub fn router() -> Router {
    Router::new()
        .route("/sign_in", get(sign_in::sign_in))
        .route("/authorize", post(authorize))
        .route("/token", post(token))
}
