use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthorizationData {
    code: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    access_token: String,
}

pub async fn handler(Json(data): Json<AuthorizationData>) -> impl IntoResponse {
    Json(ResponseData {
        access_token: format!("TODO: access token based on auth code: {}", data.code),
    })
}
