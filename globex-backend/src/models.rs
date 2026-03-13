use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Exchange {
    pub id: String,
    pub name: String,
    pub country: String,
    pub currency: String,
    pub mic: String,
    pub timezone: String,
    pub open_time_utc: String, // HH:MM
    pub close_time_utc: String, // HH:MM
    pub index_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Quote {
    pub id: i64,
    pub exchange_id: String,
    pub symbol: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct MarketIndex {
    pub exchange_id: String,
    pub name: String,
    pub current_price: f64,
    pub prev_close: f64,
    pub change_percent: f64,
    pub market_cap: Option<f64>, // Normalized to USD
    pub volume: Option<i64>,
    pub fear_greed_score: Option<i32>,
    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TopStock {
    pub id: i64,
    pub exchange_id: String,
    pub rank: i32,
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub market_cap: f64, // Normalized to USD
    pub change_percent: f64,
    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct HistoricalPrice {
    pub exchange_id: String,
    pub date: NaiveDate,
    pub close_price: f64,
    pub volume: Option<i64>,
}
