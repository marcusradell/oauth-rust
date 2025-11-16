use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_sign_in_route() {
    let router = super::router();

    let response = router
        .oneshot(
            Request::builder()
                .uri("/sign_in")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}
