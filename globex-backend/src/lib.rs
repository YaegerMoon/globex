pub mod error;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod services;

use axum::{
    routing::get,
    Router,
};
use sqlx::{Pool, Sqlite};
use tower_http::cors::{Any, CorsLayer};

pub fn create_app(pool: Pool<Sqlite>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/markets", get(handlers::get_markets))
        .route("/api/markets/:id", get(handlers::get_market_by_id))
        .layer(cors)
        .with_state(pool)
}
