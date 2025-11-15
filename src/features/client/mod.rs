use axum::{
    Router,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};

async fn authorization_callback() -> impl IntoResponse {
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/authorization/token")
        .send()
        .await
        .unwrap();

    println!("{response:?}");

    Redirect::to("/client")
}

async fn landing_page() -> impl IntoResponse {
    Html(
        r#"
        <H1>Sign In</H1>
        <a href="http://localhost:3000/authorization/sign_in">Sign in with Rådell</a>
        "#,
    )
}
pub fn router() -> Router {
    Router::new()
        .route("/authorization_callback", get(authorization_callback))
        .route("/", get(landing_page))
}
