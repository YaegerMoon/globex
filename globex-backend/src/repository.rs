use sqlx::{Pool, Sqlite};
use crate::models::{Exchange, MarketIndex, TopStock, HistoricalPrice};

pub async fn get_all_exchanges(pool: &Pool<Sqlite>) -> Result<Vec<Exchange>, sqlx::Error> {
    sqlx::query_as::<_, Exchange>("SELECT * FROM exchanges")
        .fetch_all(pool)
        .await
}

pub async fn upsert_market_index(pool: &Pool<Sqlite>, index: &MarketIndex) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO market_indices (exchange_id, name, current_price, prev_close, change_percent, market_cap, volume, fear_greed_score, last_updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(exchange_id) DO UPDATE SET
            current_price=excluded.current_price,
            prev_close=excluded.prev_close,
            change_percent=excluded.change_percent,
            market_cap=excluded.market_cap,
            volume=excluded.volume,
            fear_greed_score=excluded.fear_greed_score,
            last_updated_at=excluded.last_updated_at"
    )
    .bind(&index.exchange_id)
    .bind(&index.name)
    .bind(index.current_price)
    .bind(index.prev_close)
    .bind(index.change_percent)
    .bind(index.market_cap)
    .bind(index.volume)
    .bind(index.fear_greed_score)
    .bind(index.last_updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn upsert_historical_prices(pool: &Pool<Sqlite>, prices: &[HistoricalPrice]) -> Result<(), sqlx::Error> {
    for price in prices {
        sqlx::query(
            "INSERT INTO historical_prices (exchange_id, date, close_price, volume)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(exchange_id, date) DO UPDATE SET
                close_price=excluded.close_price,
                volume=excluded.volume"
        )
        .bind(&price.exchange_id)
        .bind(price.date)
        .bind(price.close_price)
        .bind(price.volume)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn get_historical_prices(pool: &Pool<Sqlite>, exchange_id: &str, limit: i64) -> Result<Vec<HistoricalPrice>, sqlx::Error> {
    sqlx::query_as::<_, HistoricalPrice>(
        "SELECT * FROM historical_prices WHERE exchange_id = ? ORDER BY date DESC LIMIT ?"
    )
    .bind(exchange_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn get_market_summaries(pool: &Pool<Sqlite>) -> Result<Vec<MarketIndex>, sqlx::Error> {
    sqlx::query_as::<_, MarketIndex>("SELECT * FROM market_indices")
        .fetch_all(pool)
        .await
}

pub async fn get_market_detail(pool: &Pool<Sqlite>, exchange_id: &str) -> Result<Option<MarketIndex>, sqlx::Error> {
    sqlx::query_as::<_, MarketIndex>("SELECT * FROM market_indices WHERE exchange_id = ?")
        .bind(exchange_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_top_stocks(pool: &Pool<Sqlite>, exchange_id: &str) -> Result<Vec<TopStock>, sqlx::Error> {
    sqlx::query_as::<_, TopStock>("SELECT * FROM top_stocks WHERE exchange_id = ? ORDER BY rank ASC")
        .bind(exchange_id)
        .fetch_all(pool)
        .await
}
