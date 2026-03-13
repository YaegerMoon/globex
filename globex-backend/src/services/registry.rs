pub struct MarketConfig {
    pub exchange_id: &'static str,
    pub index_symbol: &'static str,
}

pub const MARKET_REGISTRY: &[MarketConfig] = &[
    MarketConfig { exchange_id: "NYSE", index_symbol: "^GSPC" },     // S&P 500 for NYSE
    MarketConfig { exchange_id: "NASDAQ", index_symbol: "^IXIC" },   // NASDAQ Composite
    MarketConfig { exchange_id: "KRX", index_symbol: "^KS11" },      // KOSPI
    MarketConfig { exchange_id: "JPX", index_symbol: "^N225" },      // Nikkei 225
    MarketConfig { exchange_id: "LSE", index_symbol: "^FTSE" },      // FTSE 100
    MarketConfig { exchange_id: "FWB", index_symbol: "^GDAXI" },     // DAX
    MarketConfig { exchange_id: "HKEX", index_symbol: "^HSI" },      // Hang Seng Index
    MarketConfig { exchange_id: "SSE", index_symbol: "000001.SS" },  // SSE Composite
    MarketConfig { exchange_id: "TSX", index_symbol: "^GSPTSE" },   // S&P/TSX Composite
    MarketConfig { exchange_id: "ASX", index_symbol: "^AXJO" },      // S&P/ASX 200
];
