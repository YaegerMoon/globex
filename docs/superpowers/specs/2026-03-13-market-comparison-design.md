# Spec: Global Market Comparison Service (Globex)

**Date:** 2026-03-13
**Topic:** 각 국가별 주식시장 비교 및 Fear & Greed Index 제공 서비스

## 1. Overview
Globex 서비스에 전 세계 주요 주식시장의 지수 흐름, 시가총액 규모, 시장별 시가총액 Top 10 종목, 그리고 자체적으로 산출한 Fear & Greed Index를 제공하는 기능을 추가한다.

## 2. Goals & Success Criteria
- **Goals**:
    - Yahoo Finance API를 활용하여 신뢰할 수 있는 시장 데이터를 수집한다.
    - 3분 주기로 데이터를 자동 갱신하는 안정적인 배치 시스템을 구축한다.
    - 자체 알고리즘을 통해 각 시장의 심리 지수(Fear & Greed)를 산출한다.
    - 사용자 친화적인 대시보드와 상세 페이지 UI를 제공한다.
- **Success Criteria**:
    - 모든 데이터는 3분 이내의 지연 시간을 유지하며 업데이트되어야 한다.
    - 대시보드에서 각 시장 카드를 클릭하면 해당 시장의 상세 페이지로 정확히 이동해야 한다.
    - Fear & Greed Index가 0~100 사이의 값으로 정상 산출되어 시각화되어야 한다.

## 3. Architecture

### Backend (Rust/Axum)
- **Data Collector**: `yahoo_finance_api` 크레이트를 사용하여 비동기로 데이터를 수집한다.
- **Scheduler**: `tokio::time::interval`을 사용하여 3분 주기로 배치 작업을 실행한다.
- **Index Calculator**: 수집된 가격 모멘텀과 변동성 데이터를 기반으로 Fear & Greed 점수를 계산한다.
- **Database (SQLite)**: 
    - `market_indices`: 최신 시장 요약 정보 및 Fear & Greed 점수.
    - `top_stocks`: 시장별 시가총액 상위 10개 종목.
    - `historical_prices`: Fear & Greed 계산을 위한 과거 125일간의 종가 데이터 캐시.

### Frontend (Next.js/App Router)
- **Pages**:
    - `/`: 대시보드 페이지 (모든 시장 요약 카드 그리드).
    - `/market/[id]`: 시장 상세 페이지 (Fear & Greed 게이지, 상세 통계, Top 10 종목).
- **Components**:
    - `MarketCard`: 요약 정보를 보여주는 클릭 가능한 카드 컴포넌트.
    - `FearGreedGauge`: 점수에 따라 바늘이 움직이는 게이지 UI.
    - `Top10Table`: 시총 상위 10개 종목 리스트 테이블.

## 4. Implementation Details

### Fear & Greed Calculation Logic
- **지표 A (가격 모멘텀, 50%)**: `(현재가 / 125일 이동평균선) - 1`. 단순히 ±10% 고정이 아닌, 해당 시장의 역사적 이격도 분포(Percentile)를 기준으로 0~100점을 산출하여 시장별 특성을 반영한다.
- **지표 B (시장 변동성, 50%)**: `최근 30일 변동성 / 1년 평균 변동성`. 이 역시 역사적 변동성 분포를 기반으로 점수화한다.
- **최종 점수**: 두 지표의 가중 평균값.

### Data Update & Storage Strategy
- 3분마다 최신 시세와 Top 10 종목을 업데이트한다.
- **Currency Normalization**: 모든 시가총액 데이터는 비교 가능하도록 USD로 환산하여 저장한다 (Yahoo Finance의 환율 데이터 활용).
- **SQLite Optimization**: 3분 주기의 빈번한 쓰기 작업과 읽기 작업의 충돌을 방지하기 위해 **WAL(Write-Ahead Logging) 모드**를 활성화한다.
- **Market Hours**: 각 시장의 개장 시간에 맞춰 API 호출 빈도를 조절하여 불필요한 호출을 줄이고 Rate Limit을 관리한다.

## 5. Resilience & Edge Cases
- **API Rate Limiting**: Yahoo Finance API 호출 시 `429 Too Many Requests` 발생을 대비하여 지수 백오프(Exponential Backoff) 재시도 전략을 구현한다.
- **Market Holidays**: 시장 휴장일에는 마지막 종가를 유지하고 배치를 건너뛰도록 처리한다.
- **Data Inconsistency**: 특정 종목이나 지수 데이터가 누락될 경우, 이전 데이터를 유지하고 관리자 로그를 남긴다.
- **Error Handling**: API 호출 실패나 계산 오류 시 사용자에게 "데이터 갱신 중"임을 표시하고 마지막 성공 데이터를 보여준다.

## 6. Testing Strategy
- **Backend**:
    - Yahoo Finance API 연동 유닛 테스트.
    - Fear & Greed 계산 로직의 경계값(Extreme Fear/Greed) 테스트.
    - DB 저장 및 조회 통합 테스트.
- **Frontend**:
    - 페이지 이동 및 동적 라우팅 테스트.
    - 게이지 UI 및 데이터 렌더링 검증.
- **E2E**:
    - 배치 작업 실행 후 프런트엔드에 반영되는 전체 흐름 확인.

## 6. Milestones
1. Backend 데이터 수집 및 DB 스키마 구축.
2. Fear & Greed 계산 엔진 구현 및 배치 스케줄러 설정.
3. API 엔드포인트 개발.
4. Frontend 기본 레이아웃 및 대시보드 구현.
5. 상세 페이지 및 Fear & Greed 게이지 시각화 완성.
