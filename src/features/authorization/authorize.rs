use axum::{Form, extract::State, response::Redirect};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AuthorizeFormData {
    email: String,
    password: String,
}

pub async fn handler(
    State(db): State<PgPool>,
    Form(body): Form<AuthorizeFormData>,
) -> Result<Redirect, axum::http::StatusCode> {
    let user_exists =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(&body.email)
            .fetch_one(&db)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if !user_exists {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    let expected_email = std::env::var("SIGN_IN_EMAIL").unwrap();
    let expected_password = std::env::var("SIGN_IN_PASSWORD").unwrap();

    if body.email != expected_email || body.password != expected_password {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    Ok(Redirect::to(
        "http://localhost:3000/client/authorization_callback?code=123",
    ))
}
