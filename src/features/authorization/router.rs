use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use tower_cookies::CookieManagerLayer;

pub fn router(db: PgPool) -> Router {
    Router::new()
        .route("/authorize", get(super::authorize::handler))
        .route("/sign_in", get(super::sign_in_page::handler))
        .route("/sign_in", post(super::sign_in::handler))
        .route("/token", post(super::token::handler))
        .with_state(db)
        .layer(CookieManagerLayer::new())
}
