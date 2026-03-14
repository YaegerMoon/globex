use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;
use sqlx::sqlite::SqlitePoolOptions;
use http_body_util::BodyExt;

use globex_backend::{app, AppState, models::Exchange};

#[tokio::test]
async fn test_health_check() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    let app = app(AppState { pool });
    let response = app.oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_exchanges() {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    let app = app(AppState { pool });
    let response = app.oneshot(Request::builder().uri("/api/exchanges").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let exchanges: Vec<Exchange> = serde_json::from_slice(&body).unwrap();
    assert_eq!(exchanges.len(), 30);
    assert!(exchanges.iter().any(|e| e.id == "NYSE"));
    assert!(exchanges.iter().any(|e| e.id == "KRX"));
}
