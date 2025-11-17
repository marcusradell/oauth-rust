use axum::{http::StatusCode, response::Redirect};
use tower_cookies::Cookies;

pub async fn handler(cookies: Cookies) -> Result<Redirect, StatusCode> {
    let session_id_cookie: String = cookies
        .get("authorization_session_id")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or("".to_string());

    if session_id_cookie.is_empty() {
        return Ok(Redirect::to("/authorization/sign_in"));
    }

    Ok(Redirect::to("/authorization/sign_in"))
}
