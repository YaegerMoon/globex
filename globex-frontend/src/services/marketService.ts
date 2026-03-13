import axios from 'axios';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001/api';

export interface MarketIndex {
  exchange_id: string;
  name: string;
  current_price: number;
  prev_close: number;
  change_percent: number;
  market_cap?: number;
  volume?: number;
  fear_greed_score?: number;
  last_updated_at: string;
}

export interface TopStock {
  id: number;
  exchange_id: string;
  rank: number;
  symbol: string;
  name: string;
  price: number;
  market_cap: number;
  change_percent: number;
}

export interface MarketDetail {
  market: MarketIndex;
  top_stocks: TopStock[];
}

export const marketService = {
  getMarkets: async (): Promise<MarketIndex[]> => {
    const response = await axios.get(`${API_BASE_URL}/markets`);
    return response.data;
  },

  getMarketById: async (id: string): Promise<MarketDetail> => {
    const response = await axios.get(`${API_BASE_URL}/markets/${id}`);
    return response.data;
  }
};
