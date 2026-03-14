use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub id: i64,
    pub exchange_id: String,
    pub symbol: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}
