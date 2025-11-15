use axum::{
    Router,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};

async fn sign_in() -> impl IntoResponse {
    Html(
        r#"
        <H1>Sign In</H1>
        <form action="/authorization/authorize">
        <label for="email">Email</label>
        <input id="email">
        <label for="password">Password</label>
        <input type="password">
        <button type="submit">Send</button>
        </form>
        "#,
    )
}

async fn authorize() -> impl IntoResponse {
    Redirect::to("/client/authorization_callback?code=123")
}

async fn token() -> impl IntoResponse {
    "[[TODO: token string]]"
}

pub fn router() -> Router {
    Router::new()
        .route("/sign_in", get(sign_in))
        .route("/authorize", get(authorize))
        .route("/token", post(token))
}
