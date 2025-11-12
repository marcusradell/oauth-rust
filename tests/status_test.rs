use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

mod features;

#[tokio::test]
async fn test_status_endpoint() {
    let app = axum::Router::new()
        .nest("/status", features::status::router());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
