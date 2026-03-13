use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use globex_backend::handlers::health_check;
use tower::ServiceExt; // for `oneshot`

#[tokio::test]
async fn test_health_check() {
    let app = Router::new().route("/health", get(health_check));

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
