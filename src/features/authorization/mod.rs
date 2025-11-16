mod authorize;
mod sign_in;
mod token;

use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/sign_in", get(sign_in::sign_in))
        .route("/authorize", post(authorize::authorize))
        .route("/token", post(token::token))
}
