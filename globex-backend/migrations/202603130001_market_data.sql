-- Migration: 202603130001_market_data.sql
-- Goal: Add tables for market indices, top stocks, and historical prices.

-- Enable WAL mode for SQLite to handle concurrent access
PRAGMA journal_mode = WAL;

-- Table for current market index information and Fear & Greed score
CREATE TABLE IF NOT EXISTS market_indices (
    exchange_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    current_price REAL NOT NULL,
    prev_close REAL NOT NULL,
    change_percent REAL NOT NULL,
    market_cap REAL, -- Normalized to USD
    volume INTEGER,
    fear_greed_score INTEGER,
    last_updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Table for top 10 stocks by market cap per exchange
CREATE TABLE IF NOT EXISTS top_stocks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    exchange_id TEXT NOT NULL,
    rank INTEGER NOT NULL,
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    market_cap REAL NOT NULL, -- Normalized to USD
    change_percent REAL NOT NULL,
    last_updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (exchange_id) REFERENCES market_indices(exchange_id),
    UNIQUE(exchange_id, rank)
);

-- Table for historical prices (last 125+ days) for Fear & Greed calculation
CREATE TABLE IF NOT EXISTS historical_prices (
    exchange_id TEXT NOT NULL,
    date DATE NOT NULL,
    close_price REAL NOT NULL,
    volume INTEGER,
    PRIMARY KEY (exchange_id, date),
    FOREIGN KEY (exchange_id) REFERENCES market_indices(exchange_id)
);

-- Add index_symbol column to exchanges table if it doesn't exist
-- Note: Assuming 'exchanges' table was created in 202603120000_init.sql
ALTER TABLE exchanges ADD COLUMN index_symbol TEXT;
