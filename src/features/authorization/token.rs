use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AuthorizationData {
    code: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    session_id: String,
}

#[derive(Deserialize, FromRow)]
struct AuthorizationCode {
    code: String,
    user_id: Uuid,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

pub async fn handler(
    State(db): State<PgPool>,
    Json(data): Json<AuthorizationData>,
) -> Result<Json<ResponseData>, axum::http::StatusCode> {
    let _authorization_code: AuthorizationCode = sqlx::query_as(
        "SELECT code, user_id, expires_at, created_at FROM authorization_codes WHERE code=$1 ORDER BY created_at DESC",
    )
    .bind(&data.code)
    .fetch_one(&db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseData {
        session_id: format!("TODO: session ID based on auth code: {}", data.code),
    }))
}
