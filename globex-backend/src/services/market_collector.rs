use std::error::Error;
use yahoo_finance_api as yahoo;
use crate::models::{MarketIndex, TopStock, HistoricalPrice};
use crate::services::registry::MarketConfig;
use chrono::{Utc, TimeZone};
use backoff::{ExponentialBackoff, future::retry};
use time::OffsetDateTime;

pub struct MarketCollector {
    provider: yahoo::YahooConnector,
}

impl MarketCollector {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            provider: yahoo::YahooConnector::new()?,
        })
    }

    pub async fn fetch_index_data(&self, config: &MarketConfig) -> Result<MarketIndex, Box<dyn Error + Send + Sync>> {
        let now = OffsetDateTime::now_utc();
        let start = now - time::Duration::days(5); // Fetch last few days to ensure we get at least 2 quotes

        let response: yahoo::YResponse = retry(ExponentialBackoff::default(), || async {
            self.provider.get_quote_history(config.index_symbol, start, now)
                .await
                .map_err(backoff::Error::transient)
        }).await?;

        let quotes = response.quotes()?;
        if quotes.is_empty() {
            return Err("No quotes found".into());
        }

        let latest = quotes.last().unwrap();
        let prev_close = if quotes.len() > 1 {
            quotes[quotes.len() - 2].close
        } else {
            // If only one quote, try to get it from metadata
            response.metadata()?.previous_close.unwrap_or(latest.close)
        };

        let change_percent = if prev_close != 0.0 {
            (latest.close - prev_close) / prev_close * 100.0
        } else {
            0.0
        };

        Ok(MarketIndex {
            exchange_id: config.exchange_id.to_string(),
            name: config.exchange_id.to_string(),
            country_code: Some(config.country_code.to_string()),
            current_price: latest.close,
            prev_close,
            change_percent,
            market_cap: Some(config.base_market_cap),
            volume: Some(latest.volume as i64),
            fear_greed_score: None,
            last_updated_at: Utc::now(),
            open_time_utc: None,
            close_time_utc: None,
        })
    }

    pub async fn fetch_historical_data(&self, config: &MarketConfig, days: i64) -> Result<Vec<HistoricalPrice>, Box<dyn Error + Send + Sync>> {
        let now = OffsetDateTime::now_utc();
        let start = now - time::Duration::days(days);

        let response: yahoo::YResponse = retry(ExponentialBackoff::default(), || async {
            self.provider.get_quote_history(config.index_symbol, start, now)
                .await
                .map_err(backoff::Error::transient)
        }).await?;

        let quotes = response.quotes()?;
        
        let historical_prices = quotes.into_iter().map(|q| {
            let date = Utc.timestamp_opt(q.timestamp as i64, 0)
                .unwrap()
                .date_naive();
            
            HistoricalPrice {
                exchange_id: config.exchange_id.to_string(),
                date,
                close_price: q.close,
                volume: Some(q.volume as i64),
            }
        }).collect();

        Ok(historical_prices)
    }

    pub async fn fetch_top_stocks(&self, _exchange_id: &str) -> Result<Vec<TopStock>, Box<dyn Error + Send + Sync>> {
        Ok(vec![])
    }
}
