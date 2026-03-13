'use client';

import Link from 'next/link';
import { MarketIndex } from '@/services/marketService';
import { TrendingUp, TrendingDown, Gauge } from 'lucide-react';

interface MarketCardProps {
  market: MarketIndex;
}

export default function MarketCard({ market }: MarketCardProps) {
  const isPositive = market.change_percent >= 0;
  
  const getFearGreedColor = (score?: number) => {
    if (score === undefined) return 'bg-gray-200';
    if (score < 25) return 'bg-red-500';
    if (score < 45) return 'bg-orange-500';
    if (score < 55) return 'bg-yellow-500';
    if (score < 75) return 'bg-green-400';
    return 'bg-green-600';
  };

  const getFearGreedLabel = (score?: number) => {
    if (score === undefined) return 'N/A';
    if (score < 25) return 'Extreme Fear';
    if (score < 45) return 'Fear';
    if (score < 55) return 'Neutral';
    if (score < 75) return 'Greed';
    return 'Extreme Greed';
  };

  return (
    <Link href={`/market/${market.exchange_id}`}>
      <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-6 hover:shadow-md transition-shadow cursor-pointer">
        <div className="flex justify-between items-start mb-4">
          <div>
            <h3 className="text-lg font-bold text-gray-900">{market.exchange_id}</h3>
            <p className="text-sm text-gray-500">{market.name}</p>
          </div>
          <div className={`flex items-center gap-1 ${isPositive ? 'text-green-600' : 'text-red-600'} font-semibold`}>
            {isPositive ? <TrendingUp size={16} /> : <TrendingDown size={16} />}
            {Math.abs(market.change_percent).toFixed(2)}%
          </div>
        </div>

        <div className="text-2xl font-bold text-gray-900 mb-6">
          {market.current_price.toLocaleString()}
        </div>

        <div className="pt-4 border-t border-gray-50">
          <div className="flex justify-between items-center mb-2">
            <div className="flex items-center gap-2 text-sm text-gray-600">
              <Gauge size={14} />
              <span>Fear & Greed</span>
            </div>
            <span className={`text-xs font-bold px-2 py-0.5 rounded-full text-white ${getFearGreedColor(market.fear_greed_score)}`}>
              {getFearGreedLabel(market.fear_greed_score)}
            </span>
          </div>
          <div className="w-full bg-gray-100 rounded-full h-1.5">
            <div 
              className={`h-1.5 rounded-full ${getFearGreedColor(market.fear_greed_score)} transition-all duration-500`} 
              style={{ width: `${market.fear_greed_score ?? 50}%` }}
            />
          </div>
        </div>
      </div>
    </Link>
  );
}
