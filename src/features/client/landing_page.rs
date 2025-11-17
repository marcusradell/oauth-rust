use axum::extract::State;
use axum::response::{Html, IntoResponse};
use tower_cookies::Cookies;

use super::router::AppState;

pub async fn handler(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let access_token = cookies
        .get("client_access_token")
        .and_then(|c| c.value().parse().ok())
        .unwrap_or("".to_string());

    if access_token.is_empty() {
        let authorize_url = format!("{}/authorization/authorize", state.api_base_url);
        let html = include_str!("../../../static/client/sign_in.html")
            .replace("{{authorize_url}}", &authorize_url);
        Html(html)
    } else {
        let sign_out_url = format!("{}/authorization/sign_out", state.api_base_url);
        let html = include_str!("../../../static/client/welcome.html")
            .replace("{{sign_out_url}}", &sign_out_url);
        Html(html)
    }
}
