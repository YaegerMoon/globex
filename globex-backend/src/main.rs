use axum::{Router, routing::get};
use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions, SqliteJournalMode}, Pool, Sqlite};
use std::str::FromStr;

pub mod error;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod services;

use handlers::health_check;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let database_url = "sqlite://globex.db";
    
    let connection_options = SqliteConnectOptions::from_str(database_url)
        .unwrap()
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .expect("Failed to create SQLite connection pool");

    let state = AppState { pool };

    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .route("/health", get(health_check))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
