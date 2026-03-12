use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Exchange {
    pub id: String,
    pub name: String,
    pub country: String,
    pub currency: String,
    pub mic: String,
    pub timezone: String,
    pub open_time_utc: String, // HH:MM
    pub close_time_utc: String, // HH:MM
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub id: i64,
    pub exchange_id: String,
    pub symbol: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}

