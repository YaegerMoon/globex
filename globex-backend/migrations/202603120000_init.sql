CREATE TABLE IF NOT EXISTS exchanges (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    country TEXT NOT NULL,
    currency TEXT NOT NULL,
    mic TEXT NOT NULL UNIQUE,
    timezone TEXT NOT NULL,
    open_time_utc TEXT NOT NULL,
    close_time_utc TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS quotes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    exchange_id TEXT NOT NULL,
    symbol TEXT NOT NULL,
    price REAL NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id)
);

CREATE INDEX idx_quotes_symbol ON quotes(symbol);
CREATE INDEX idx_quotes_exchange ON quotes(exchange_id);

INSERT INTO exchanges (id, name, country, currency, mic, timezone, open_time_utc, close_time_utc) VALUES
('NYSE', 'New York Stock Exchange', 'USA', 'USD', 'XNYS', 'America/New_York', '14:30', '21:00'),
('NASDAQ', 'NASDAQ', 'USA', 'USD', 'XNAS', 'America/New_York', '14:30', '21:00'),
('SSE', 'Shanghai Stock Exchange', 'China', 'CNY', 'XSHG', 'Asia/Shanghai', '01:30', '07:00'),
('EURONEXT', 'Euronext', 'Europe', 'EUR', 'XAMS', 'Europe/Paris', '07:00', '15:30'),
('JPX', 'Japan Exchange Group', 'Japan', 'JPY', 'XJPX', 'Asia/Tokyo', '00:00', '06:00'),
('SZSE', 'Shenzhen Stock Exchange', 'China', 'CNY', 'XSHE', 'Asia/Shanghai', '01:30', '07:00'),
('HKEX', 'Hong Kong Stock Exchange', 'Hong Kong', 'HKD', 'XHKG', 'Asia/Hong_Kong', '01:30', '08:00'),
('BSE', 'Bombay Stock Exchange', 'India', 'INR', 'XBOM', 'Asia/Kolkata', '03:45', '10:00'),
('NSE', 'National Stock Exchange of India', 'India', 'INR', 'XNSE', 'Asia/Kolkata', '03:45', '10:00'),
('LSE', 'London Stock Exchange', 'UK', 'GBP', 'XLON', 'Europe/London', '08:00', '16:30'),
('TSX', 'Toronto Stock Exchange', 'Canada', 'CAD', 'XTSE', 'America/Toronto', '14:30', '21:00'),
('TADAWUL', 'Saudi Stock Exchange', 'Saudi Arabia', 'SAR', 'XSAU', 'Asia/Riyadh', '07:00', '12:00'),
('FWB', 'Frankfurt Stock Exchange', 'Germany', 'EUR', 'XFRA', 'Europe/Berlin', '07:00', '18:30'),
('KRX', 'Korea Exchange', 'South Korea', 'KRW', 'XKRX', 'Asia/Seoul', '00:00', '06:30'),
('SIX', 'SIX Swiss Exchange', 'Switzerland', 'CHF', 'XSWX', 'Europe/Zurich', '08:00', '16:20'),
('ASX', 'Australian Securities Exchange', 'Australia', 'AUD', 'XASX', 'Australia/Sydney', '23:00', '05:00'),
('TWSE', 'Taiwan Stock Exchange', 'Taiwan', 'TWD', 'XTAI', 'Asia/Taipei', '01:00', '05:30'),
('B3', 'B3 - Brasil Bolsa Balcão', 'Brazil', 'BRL', 'BVMF', 'America/Sao_Paulo', '13:00', '20:00'),
('JSE', 'Johannesburg Stock Exchange', 'South Africa', 'ZAR', 'XJSE', 'Africa/Johannesburg', '07:00', '15:00'),
('BME', 'Bolsas y Mercados Españoles', 'Spain', 'EUR', 'BMEX', 'Europe/Madrid', '08:00', '16:30'),
('IDX', 'Indonesia Stock Exchange', 'Indonesia', 'IDR', 'XIDX', 'Asia/Jakarta', '02:00', '09:00'),
('SGX', 'Singapore Exchange', 'Singapore', 'SGD', 'XSES', 'Asia/Singapore', '01:00', '09:00'),
('MOEX', 'Moscow Exchange', 'Russia', 'RUB', 'MISX', 'Europe/Moscow', '06:50', '20:50'),
('SET', 'Stock Exchange of Thailand', 'Thailand', 'THB', 'XBKK', 'Asia/Bangkok', '03:00', '09:30'),
('BIST', 'Borsa Istanbul', 'Turkey', 'TRY', 'XIST', 'Europe/Istanbul', '07:00', '15:00'),
('MYX', 'Bursa Malaysia', 'Malaysia', 'MYR', 'XKLS', 'Asia/Kuala_Lumpur', '01:00', '09:00'),
('BMV', 'Bolsa Mexicana de Valores', 'Mexico', 'MXN', 'XMEX', 'America/Mexico_City', '14:30', '21:00'),
('TASE', 'Tel Aviv Stock Exchange', 'Israel', 'ILS', 'XTAE', 'Asia/Jerusalem', '08:00', '15:25'),
('WSE', 'Warsaw Stock Exchange', 'Poland', 'PLN', 'XWAR', 'Europe/Warsaw', '08:00', '15:50'),
('PSE', 'Philippine Stock Exchange', 'Philippines', 'PHP', 'XPHS', 'Asia/Manila', '01:30', '07:00');
