use axum::{Router, routing::get, http::StatusCode};

pub fn router() -> Router {
    Router::new().route("/", get(status_handler))
}

async fn status_handler() -> StatusCode {
    StatusCode::OK
}
