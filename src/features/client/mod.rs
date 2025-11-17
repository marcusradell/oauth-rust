use axum::extract::Query;
use axum::routing::post;
use axum::{
    Router,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use serde::{Deserialize, Serialize};
use tower_cookies::CookieManagerLayer;
use tower_cookies::{Cookie, Cookies};

#[derive(Serialize, Deserialize)]
struct QueryData {
    code: String,
}

#[derive(Deserialize)]
struct TokenData {
    access_token: String,
    refresh_token: String,
}

async fn authorization_callback(
    Query(query_data): Query<QueryData>,
    cookies: Cookies,
) -> Result<Redirect, axum::http::StatusCode> {
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/authorization/token")
        .json(&query_data)
        .send()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let token_data: TokenData = response
        .json()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let access_cookie: Cookie = Cookie::build(("client_access_token", token_data.access_token))
        .path("/")
        .secure(true)
        .http_only(true)
        .into();

    cookies.add(access_cookie);

    let refresh_cookie: Cookie = Cookie::build(("client_refresh_token", token_data.refresh_token))
        .path("/")
        .secure(true)
        .http_only(true)
        .into();

    cookies.add(refresh_cookie);

    Ok(Redirect::to("/client"))
}

async fn landing_page(cookies: Cookies) -> impl IntoResponse {
    let access_token = cookies
        .get("client_access_token")
        .and_then(|c| c.value().parse().ok())
        .unwrap_or("".to_string());

    if access_token.is_empty() {
        Html(
            r#"
        <H1>Sign In</H1>
        <a href="http://localhost:3000/authorization/sign_in">Sign in with Rådell</a>
        "#,
        )
    } else {
        Html(
            r#"
                <H1>Welcome</H1>
                <p>You are signed in!</p>

                <form action="/client/log_out" method="post">
                    <button type="submit">Log out</button>
                </form>
            "#,
        )
    }
}

async fn log_out(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::from("client_access_token");
    cookie.set_path("/");
    cookies.remove(cookie);

    let mut cookie = Cookie::from("client_refresh_token");
    cookie.set_path("/");
    cookies.remove(cookie);

    Redirect::to("/client")
}

pub fn router() -> Router {
    Router::new()
        .route("/authorization_callback", get(authorization_callback))
        .route("/", get(landing_page))
        .route("/log_out", post(log_out))
        .layer(CookieManagerLayer::new())
}
