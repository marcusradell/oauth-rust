use axum::body::to_bytes;
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
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

#[tokio::test]
async fn test_authorize() {
    dotenvy::from_filename("test.env").unwrap();
    let router = super::router();

    let form_data = form_urlencoded::Serializer::new(String::new())
        .append_pair("email", &std::env::var("SIGN_IN_EMAIL").unwrap())
        .append_pair("password", &std::env::var("SIGN_IN_PASSWORD").unwrap())
        .finish();

    let response = router
        .oneshot(
            Request::builder()
                .uri("/authorize")
                .method("POST")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(form_data))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get("location").unwrap(),
        "http://localhost:3000/client/authorization_callback?code=123"
    )
}

#[tokio::test]
async fn test_token() {
    let router = super::router();
    let response = router
        .oneshot(
            Request::builder()
                .uri("/token")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = std::str::from_utf8(&body_bytes).unwrap();

    assert_eq!(body, "[[TODO: token string]]");
}
