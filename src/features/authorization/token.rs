use axum::{Json, extract::State};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AuthorizationData {
    code: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    access_token: String,
    refresh_token: String,
}

#[derive(Deserialize, FromRow)]
struct AuthorizationCode {
    user_id: Uuid,
    expires_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: i64,
}

pub async fn handler(
    State(db): State<PgPool>,
    Json(data): Json<AuthorizationData>,
) -> Result<Json<ResponseData>, axum::http::StatusCode> {
    let authorization_code: AuthorizationCode = sqlx::query_as(
        "SELECT user_id, expires_at FROM authorization_codes WHERE code=$1 ORDER BY created_at DESC",
    )
    .bind(&data.code)
    .fetch_one(&db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let now = Utc::now();
    if authorization_code.expires_at < now {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    let jwt_secret =
        std::env::var("JWT_SECRET").map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let access_token_claims = Claims {
        sub: authorization_code.user_id,
        exp: (now + Duration::minutes(15)).timestamp(),
    };

    let access_token = encode(
        &Header::default(),
        &access_token_claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut refresh_token_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut refresh_token_bytes);
    let refresh_token = URL_SAFE_NO_PAD.encode(refresh_token_bytes);

    let refresh_token_created_at = now;
    let refresh_token_expires_at = now + Duration::days(30);

    sqlx::query(
        "INSERT INTO refresh_tokens (token, user_id, created_at, expires_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(&refresh_token)
    .bind(authorization_code.user_id)
    .bind(refresh_token_created_at)
    .bind(refresh_token_expires_at)
    .execute(&db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseData {
        access_token,
        refresh_token,
    }))
}
