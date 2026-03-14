use crate::models::HistoricalPrice;
use itertools::Itertools;

pub struct IndexCalculator;

impl IndexCalculator {
    pub fn calculate_fear_greed_score(historical_data: &[HistoricalPrice], current_price: f64) -> Option<i32> {
        let len = historical_data.len();
        if len < 90 {
            return None;
        }

        // 1. Price Momentum (50% weight) - Use up to 125 days, or whatever is available
        let momentum_period = len.min(125);
        let ma = historical_data.iter().take(momentum_period).map(|p| p.close_price).sum::<f64>() / momentum_period as f64;
        let momentum_score = Self::map_momentum_to_score(current_price, ma);

        // 2. Volatility (50% weight)
        let vol_period_short = (len / 4).min(30).max(10); // Scale short volatility based on available data
        let recent_volatility = Self::calculate_volatility(&historical_data[0..vol_period_short]);
        let yearly_volatility = Self::calculate_volatility(historical_data);
        let volatility_score = Self::map_volatility_to_score(recent_volatility, yearly_volatility);

        let final_score = (momentum_score as f32 * 0.5 + volatility_score as f32 * 0.5) as i32;
        
        Some(final_score.clamp(0, 100))
    }

    fn calculate_volatility(data: &[HistoricalPrice]) -> f64 {
        if data.len() < 2 { return 0.0; }
        
        // Data is DESC (newest first), so we reverse it to get ASC (oldest first)
        let returns: Vec<f64> = data.iter().rev().tuple_windows()
            .map(|(a, b)| (b.close_price - a.close_price) / a.close_price)
            .collect();
            
        if returns.is_empty() { return 0.0; }
            
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64;
        variance.sqrt()
    }

    fn map_momentum_to_score(current: f64, ma: f64) -> i32 {
        let diff_percent = (current - ma) / ma * 100.0;
        // More sensitive mapping: -5% to 0, +5% to 100
        // Formula: score = (diff_percent + 5.0) * 10.0
        let score = (diff_percent + 5.0) * 10.0;
        score.clamp(0.0, 100.0) as i32
    }

    fn map_volatility_to_score(recent: f64, yearly: f64) -> i32 {
        if yearly == 0.0 { return 50; }
        let ratio = recent / yearly;

        // Better mapping for ratio:
        // ratio 1.0 -> 50
        // ratio 2.0 -> 0 (Extreme Fear)
        // ratio 0.5 -> 100 (Extreme Greed)
        let score = if ratio >= 1.0 {
            // Scale from 1.0 (50) to 2.0 (0)
            50.0 - (ratio - 1.0) * 50.0
        } else {
            // Scale from 1.0 (50) to 0.5 (100)
            50.0 + (1.0 - ratio) * 100.0
        };

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

    #[test]
    fn test_fear_greed_market_conditions() {
        let base_price = 100.0;
        
        // 1. Steady uptrend (Greed)
        let data_uptrend: Vec<HistoricalPrice> = (0..200).map(|i| {
            HistoricalPrice {
                exchange_id: "UP".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                close_price: base_price + (i as f64 * 0.1),
                volume: Some(1000),
            }
        }).rev().collect();
        let score_uptrend = IndexCalculator::calculate_fear_greed_score(&data_uptrend, 120.0).unwrap();
        assert!(score_uptrend > 50);

        // 2. Volatile downtrend (Fear)
        let data_downtrend: Vec<HistoricalPrice> = (0..200).map(|i| {
            let volatility = if i % 2 == 0 { 2.0 } else { -2.0 };
            HistoricalPrice {
                exchange_id: "DOWN".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                close_price: base_price - (i as f64 * 0.1) + volatility,
                volume: Some(1000),
            }
        }).rev().collect();
        let score_downtrend = IndexCalculator::calculate_fear_greed_score(&data_downtrend, 80.0).unwrap();
        assert!(score_downtrend < 50);

        // 3. Flat market (Neutral)
        let data_flat: Vec<HistoricalPrice> = (0..200).map(|_| {
            HistoricalPrice {
                exchange_id: "FLAT".to_string(),
                date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                close_price: base_price,
                volume: Some(1000),
            }
        }).rev().collect();
        let score_flat = IndexCalculator::calculate_fear_greed_score(&data_flat, base_price).unwrap();
        println!("Flat score: {}", score_flat);
        assert!(score_flat >= 40 && score_flat <= 60);
    }
}
