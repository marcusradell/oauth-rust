use axum::{Router, response::IntoResponse, routing::get};

async fn authorization_callback() -> impl IntoResponse {
    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/authorization/token")
        .send()
        .await
        .unwrap();

    println!("{response:?}");

    "Welcome!"
}

pub fn router() -> Router {
    Router::new().route("/authorization_callback", get(authorization_callback))
}
