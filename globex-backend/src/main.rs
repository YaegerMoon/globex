use sqlx::sqlite::SqlitePoolOptions;
use std::time::Duration;
use globex_backend::{services::market_collector::MarketCollector, services::index_calculator::IndexCalculator, services::registry::MARKET_REGISTRY, repository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://globex.db".to_string());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Enable WAL mode
    sqlx::query("PRAGMA journal_mode = WAL").execute(&pool).await?;

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let collector = match MarketCollector::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to initialize MarketCollector: {}", e);
                return;
            }
        };
        let mut interval = tokio::time::interval(Duration::from_secs(180)); // 3 minutes

        loop {
            interval.tick().await;
            println!("Starting market data collection batch...");

            for config in MARKET_REGISTRY {
                // 1. Fetch current index data
                match collector.fetch_index_data(config).await {
                    Ok(mut index) => {
                        // 2. Fetch historical data if needed for Fear & Greed
                        let history = repository::get_historical_prices(&pool_clone, config.exchange_id, 250).await.unwrap_or_default();
                        
                        if history.len() < 125 {
                            println!("Syncing historical data for {}...", config.exchange_id);
                            if let Ok(new_history) = collector.fetch_historical_data(config, 250).await {
                                let _ = repository::upsert_historical_prices(&pool_clone, &new_history).await;
                            }
                        }

                        // 3. Calculate Fear & Greed
                        let updated_history = repository::get_historical_prices(&pool_clone, config.exchange_id, 250).await.unwrap_or_default();
                        index.fear_greed_score = IndexCalculator::calculate_fear_greed_score(&updated_history, index.current_price);

                        // 4. Save to DB
                        if let Err(e) = repository::upsert_market_index(&pool_clone, &index).await {
                            eprintln!("Failed to save market index for {}: {}", config.exchange_id, e);
                        }
                    }
                    Err(e) => eprintln!("Failed to fetch data for {}: {}", config.exchange_id, e),
                }
            }
            println!("Market data collection batch completed.");
        }
    });

    let app = globex_backend::create_app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("Backend server running on http://localhost:3001");
    axum::serve(listener, app).await?;

    Ok(())
}
