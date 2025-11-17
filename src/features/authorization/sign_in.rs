use argon2::password_hash::SaltString;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::rand_core::OsRng,
};
use axum::{Form, extract::State, response::Redirect};
use serde::Deserialize;
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AuthorizeFormData {
    email: String,
    password: String,
}

pub async fn handler(
    State(db): State<PgPool>,
    cookies: Cookies,
    Form(body): Form<AuthorizeFormData>,
) -> Result<Redirect, axum::http::StatusCode> {
    let user_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(&body.email)
            .fetch_one(&db)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id: Uuid = if !user_exists {
        let id = Uuid::new_v4();
        let created_at = chrono::Utc::now();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(body.password.as_bytes(), &salt)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
            .to_string();

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(&body.email)
        .bind(&password_hash)
        .bind(created_at)
        .execute(&db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        id
    } else {
        let stored_password_hash: String =
            sqlx::query_scalar("SELECT password_hash FROM users WHERE email = $1")
                .bind(&body.email)
                .fetch_one(&db)
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        let parsed_hash = PasswordHash::new(&stored_password_hash)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        let argon2 = Argon2::default();
        argon2
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

        let user_id: Uuid = sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
            .bind(&body.email)
            .fetch_one(&db)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        user_id
    };

    let mut cookie = Cookie::new("authorization_session_id", user_id.to_string());
    cookie.set_path("/authorization");
    cookies.add(cookie);

    Ok(Redirect::to("/authorization/authorize"))
}
