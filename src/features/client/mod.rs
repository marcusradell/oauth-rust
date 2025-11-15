use axum::{
    Router,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};
use tower_cookies::CookieManagerLayer;
use tower_cookies::{Cookie, Cookies};

async fn authorization_callback(cookies: Cookies) -> impl IntoResponse {
    let cookie: Cookie = Cookie::build(("session", "[[TODO: session ID]]"))
        .path("/")
        .secure(true)
        .http_only(true)
        .into();

    cookies.add(cookie);

    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/authorization/token")
        .send()
        .await
        .unwrap();

    println!("{response:?}");

    Redirect::to("/client")
}

async fn landing_page(cookies: Cookies) -> impl IntoResponse {
    let session = cookies
        .get("session")
        .and_then(|c| c.value().parse().ok())
        .unwrap_or("".to_string());

    if session.is_empty() {
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
            "#,
        )
    }
}
pub fn router() -> Router {
    Router::new()
        .route("/authorization_callback", get(authorization_callback))
        .route("/", get(landing_page))
        .layer(CookieManagerLayer::new())
}
