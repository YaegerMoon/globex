# Market Comparison Service Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 각 국가별 주식시장의 지수, 시가총액, Top 10 종목 및 자체 산출한 Fear & Greed Index를 제공하는 풀스택 기능을 구축한다.

**Architecture:** Rust(Axum) 백엔드에서 3분 주기로 Yahoo Finance API를 통해 데이터를 수집하여 SQLite에 저장하고, Next.js 프런트엔드에서 대시보드와 상세 페이지를 통해 시각화한다.

**Tech Stack:** Rust (Axum, sqlx, tokio, yahoo_finance_api), SQLite, Next.js (App Router, Tailwind CSS, Recharts/Lucide).

---

## Chunk 1: Backend Foundation & Database

**Files:**
- Create: `globex-backend/migrations/202603130001_market_data.sql`
- Modify: `globex-backend/src/models.rs`
- Modify: `globex-backend/src/models/exchange.rs` (Add symbols)
- Create: `globex-backend/src/models/market_data.rs`

- [ ] **Step 1: Create Database Migration for Market Data**
  - Create `globex-backend/migrations/202603130001_market_data.sql` with tables for `market_indices`, `top_stocks`, and `historical_prices`.
  - Enable WAL mode in the migration.

- [ ] **Step 2: Run Migration and Verify Schema**
  - Run: `cd globex-backend && sqlx migrate run`
  - Verify tables are created in `globex.db`.

- [ ] **Step 3: Define Rust Models**
  - Update `globex-backend/src/models.rs` and create `globex-backend/src/models/market_data.rs` with `MarketIndex`, `TopStock`, and `HistoricalPrice` structs.
  - Add `index_symbol` to `Exchange` model.

- [ ] **Step 4: Commit Chunk 1**
  - `git add globex-backend/migrations globex-backend/src/models`
  - `git commit -m "feat: add database schema and models for market data"`

---

## Chunk 2: Data Collection & Fear & Greed Engine

**Files:**
- Modify: `globex-backend/Cargo.toml` (Add `yahoo_finance_api`, `itertools`, `backoff`)
- Create: `globex-backend/src/services/market_collector.rs`
- Create: `globex-backend/src/services/index_calculator.rs`
- Create: `globex-backend/src/services/registry.rs` (Market Symbol Registry)

- [ ] **Step 1: Add Dependencies**
  - Update `globex-backend/Cargo.toml` with `yahoo_finance_api`, `backoff`, and `itertools`.

- [ ] **Step 2: Define Market Symbol Registry**
  - Create `globex-backend/src/services/registry.rs` to map Exchange IDs to Yahoo Finance symbols (e.g., `NYSE` -> `^GSPC`, `KRX` -> `^KS11`).

- [ ] **Step 3: Implement Yahoo Finance Collector with Resilience**
  - Create `globex-backend/src/services/market_collector.rs`.
  - Implement `fetch_market_data` with **Exponential Backoff** using the `backoff` crate.
  - Include **Currency Normalization** logic to convert all market caps to USD using current FX rates.

- [ ] **Step 4: Implement Initial Historical Sync**
  - Add logic to check if `historical_prices` has 125 days of data; if not, perform a one-time bulk fetch for each market in the registry.

- [ ] **Step 5: Implement Fear & Greed Calculator**
  - Create `globex-backend/src/services/index_calculator.rs` using the 125-day percentile logic for momentum and volatility.

- [ ] **Step 6: Write Tests for Calculator**
  - Add unit tests to `globex-backend/src/services/index_calculator.rs` to verify score mapping.

- [ ] **Step 7: Implement 3-Minute Scheduler**
  - Update `globex-backend/src/main.rs` to spawn the background task, handling **Market Holidays** by checking if the latest data is already up-to-date.

- [ ] **Step 8: Commit Chunk 2**
  - `git add globex-backend/Cargo.toml globex-backend/src/services globex-backend/src/main.rs`
  - `git commit -m "feat: implement resilient market data collector and fear & greed calculator"`

---

## Chunk 3: Backend API Endpoints

**Files:**
- Modify: `globex-backend/src/handlers.rs`
- Modify: `globex-backend/src/repository.rs`
- Modify: `globex-backend/src/lib.rs` (Register routes)

- [ ] **Step 1: Implement Repository Methods**
  - Add `get_market_summaries` and `get_market_detail` to `repository.rs`.

- [ ] **Step 2: Create API Handlers**
  - Add `get_markets` and `get_market_by_id` handlers to `handlers.rs`.

- [ ] **Step 3: Register Routes**
  - Update `lib.rs` to include `/api/markets` and `/api/markets/:id` endpoints.

- [ ] **Step 4: Verify API with `curl`**
  - Run server and test endpoints.

- [ ] **Step 5: Commit Chunk 3**
  - `git add globex-backend/src/handlers.rs globex-backend/src/repository.rs globex-backend/src/lib.rs`
  - `git commit -m "feat: add api endpoints for market data"`

---

## Chunk 4: Frontend Routing & Service Layer

**Files:**
- Create: `globex-frontend/src/services/marketService.ts`
- Create: `globex-frontend/src/app/page.tsx` (Update Dashboard)
- Create: `globex-frontend/src/app/market/[id]/page.tsx` (Detail Page)

- [ ] **Step 1: Implement Frontend API Service**
  - Create `marketService.ts` to fetch data from the backend.

- [ ] **Step 2: Setup Dynamic Routing**
  - Create `globex-frontend/src/app/market/[id]/page.tsx` skeleton.

- [ ] **Step 3: Commit Chunk 4**
  - `git add globex-frontend/src`
  - `git commit -m "feat: setup frontend routing and api service"`

---

## Chunk 5: UI Implementation & Polish

**Files:**
- Create: `globex-frontend/src/components/MarketCard.tsx`
- Create: `globex-frontend/src/components/FearGreedGauge.tsx`
- Create: `globex-frontend/src/components/Top10Table.tsx`
- Modify: `globex-frontend/src/app/page.tsx`
- Modify: `globex-frontend/src/app/market/[id]/page.tsx`

- [ ] **Step 1: Build MarketCard Component**
  - Implement the summary card for the dashboard.

- [ ] **Step 2: Build FearGreedGauge Component**
  - Create a visual gauge using SVG or CSS.

- [ ] **Step 3: Build Top10Table Component**
  - Implement a clean table for the top 10 stocks.

- [ ] **Step 4: Integrate Components into Pages**
  - Finalize Dashboard and Detail View.

- [ ] **Step 5: Final Verification & Commit**
  - Run E2E check and commit final UI changes.
  - `git add globex-frontend/src/components globex-frontend/src/app`
  - `git commit -m "feat: complete ui implementation for market comparison"`
