use crate::models::HistoricalPrice;
use itertools::Itertools;

pub struct IndexCalculator;

impl IndexCalculator {
    pub fn calculate_fear_greed_score(historical_data: &[HistoricalPrice], current_price: f64) -> Option<i32> {
        if historical_data.len() < 125 {
            return None;
        }

        // 1. Price Momentum (50% weight)
        let ma_125 = historical_data.iter().take(125).map(|p| p.close_price).sum::<f64>() / 125.0;
        let momentum_score = Self::map_momentum_to_score(current_price, ma_125);

        // 2. Volatility (50% weight)
        let recent_volatility = Self::calculate_volatility(&historical_data[0..30]);
        let yearly_volatility = Self::calculate_volatility(historical_data);
        let volatility_score = Self::map_volatility_to_score(recent_volatility, yearly_volatility);

        let final_score = (momentum_score as f32 * 0.5 + volatility_score as f32 * 0.5) as i32;
        Some(final_score.clamp(0, 100))
    }

    fn calculate_volatility(data: &[HistoricalPrice]) -> f64 {
        if data.len() < 2 { return 0.0; }
        
        let returns: Vec<f64> = data.iter().tuple_windows()
            .map(|(a, b)| (b.close_price - a.close_price) / a.close_price)
            .collect();
            
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64;
        variance.sqrt()
    }

    fn map_momentum_to_score(current: f64, ma: f64) -> i32 {
        let diff_percent = (current - ma) / ma * 100.0;
        // Map -10% to 0, +10% to 100
        let score = (diff_percent + 10.0) * 5.0;
        score.clamp(0.0, 100.0) as i32
    }

    fn map_volatility_to_score(recent: f64, yearly: f64) -> i32 {
        if yearly == 0.0 { return 50; }
        let ratio = recent / yearly;
        // Map ratio 1.5 to 0 (high volatility = fear), 0.5 to 100 (low volatility = greed)
        // Formula: score = (1.5 - ratio) / (1.5 - 0.5) * 100
        let score = (1.5 - ratio) * 100.0;
        score.clamp(0.0, 100.0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn mock_historical_data(n: usize, start_price: f64, trend: f64) -> Vec<HistoricalPrice> {
        (0..n).map(|i| {
            HistoricalPrice {
                exchange_id: "TEST".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                close_price: start_price + (i as f64 * trend),
                volume: Some(1000),
            }
        }).collect()
    }

    #[test]
    fn test_fear_greed_calculation() {
        let data = mock_historical_data(125, 100.0, 0.1); // Steady uptrend
        let score = IndexCalculator::calculate_fear_greed_score(&data, 115.0);
        assert!(score.is_some());
        assert!(score.unwrap() > 50); // Should be greedy
    }
}
