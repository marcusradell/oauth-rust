use axum::{
    Router,
    routing::{get, post},
};
use tower_cookies::CookieManagerLayer;

pub fn router() -> Router {
    Router::new()
        .route(
            "/authorization_callback",
            get(super::authorization_callback::handler),
        )
        .route("/", get(super::landing_page::handler))
        .route("/log_out", post(super::log_out))
        .layer(CookieManagerLayer::new())
}
