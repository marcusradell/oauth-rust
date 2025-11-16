use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/sign_in", get(super::sign_in::handler))
        .route("/authorize", post(super::authorize::handler))
        .route("/token", post(super::token::handler))
}
