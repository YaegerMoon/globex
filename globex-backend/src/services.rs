use crate::{error::AppError, models::Quote};

pub trait StockExchange {
    // Defines standard fetching behavior for a particular exchange API implementation.
    fn fetch_quote(&self, symbol: &str) -> impl std::future::Future<Output = Result<Quote, AppError>> + Send;
}
