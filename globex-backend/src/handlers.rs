use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use sqlx::{Pool, Sqlite};
use crate::repository;
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn get_markets(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    match repository::get_market_summaries(&pool).await {
        Ok(markets) => (StatusCode::OK, Json(markets)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

pub async fn get_market_by_id(Path(id): Path<String>, State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    match repository::get_market_detail(&pool, &id).await {
        Ok(Some(market)) => {
            let top_stocks = repository::get_top_stocks(&pool, &id).await.unwrap_or_default();
            (StatusCode::OK, Json(json!({
                "market": market,
                "top_stocks": top_stocks
            }))).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Market not found"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}
