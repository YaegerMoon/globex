'use client';

import { useQuery } from '@tanstack/react-query';
import { marketService } from '@/services/marketService';
import FearGreedGauge from '@/components/FearGreedGauge';
import Top10Table from '@/components/Top10Table';
import { ChevronLeft, Info, TrendingUp, TrendingDown, DollarSign, BarChart2 } from 'lucide-react';
import Link from 'next/link';

export default function MarketDetailPage({ params }: { params: { id: string } }) {
  const { data, isLoading, isError } = useQuery({
    queryKey: ['market', params.id],
    queryFn: () => marketService.getMarketById(params.id),
  });

  if (isLoading) {
    return <div className="p-8 text-center">Loading market details...</div>;
  }

  if (isError || !data) {
    return <div className="p-8 text-center text-red-500">Error loading market details.</div>;
  }

  const { market, top_stocks } = data;
  const isPositive = market.change_percent >= 0;

  return (
    <div className="pb-20">
      <Link href="/" className="flex items-center gap-1 text-blue-600 font-bold mb-6 hover:underline">
        <ChevronLeft size={20} />
        Back to Dashboard
      </Link>

      <div className="flex flex-col md:flex-row justify-between items-start md:items-center mb-10 gap-4">
        <div>
          <h2 className="text-4xl font-black text-gray-900 tracking-tighter uppercase">{market.exchange_id}</h2>
          <p className="text-lg text-gray-500 font-medium">{market.name}</p>
        </div>
        <div className="flex flex-col items-end">
          <div className="text-4xl font-black text-gray-900">
            {market.current_price.toLocaleString()}
          </div>
          <div className={`flex items-center gap-1 text-xl font-bold ${isPositive ? 'text-green-600' : 'text-red-600'}`}>
            {isPositive ? <TrendingUp size={24} /> : <TrendingDown size={24} />}
            {Math.abs(market.change_percent).toFixed(2)}%
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-12">
        <div className="lg:col-span-1 bg-white rounded-2xl p-8 shadow-sm border border-gray-100 flex flex-col items-center justify-center">
          <div className="flex items-center gap-2 mb-6 text-gray-400 font-bold text-sm uppercase tracking-widest">
            <Info size={16} />
            Market Sentiment
          </div>
          <FearGreedGauge score={market.fear_greed_score ?? 50} />
        </div>

        <div className="lg:col-span-2 bg-white rounded-2xl p-8 shadow-sm border border-gray-100">
          <h3 className="text-xl font-bold text-gray-900 mb-8 border-b border-gray-100 pb-4">Key Market Stats</h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-x-12 gap-y-8">
            <div className="flex items-start gap-4">
              <div className="p-3 bg-blue-50 text-blue-600 rounded-xl">
                <DollarSign size={24} />
              </div>
              <div>
                <p className="text-sm font-bold text-gray-400 uppercase tracking-wider">Market Cap (USD)</p>
                <p className="text-2xl font-black text-gray-900">
                  {market.market_cap ? `$${(market.market_cap / 1e12).toFixed(2)}T` : 'N/A'}
                </p>
              </div>
            </div>
            <div className="flex items-start gap-4">
              <div className="p-3 bg-purple-50 text-purple-600 rounded-xl">
                <BarChart2 size={24} />
              </div>
              <div>
                <p className="text-sm font-bold text-gray-400 uppercase tracking-wider">24h Volume</p>
                <p className="text-2xl font-black text-gray-900">
                  {market.volume ? (market.volume / 1e6).toFixed(1) + 'M' : 'N/A'}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
        <div className="px-8 py-6 border-b border-gray-100 flex justify-between items-center">
          <h3 className="text-xl font-bold text-gray-900">Top 10 Stocks by Market Cap</h3>
          <span className="text-xs font-bold text-gray-400 bg-gray-50 px-3 py-1 rounded-full uppercase tracking-tighter">Updated 3m ago</span>
        </div>
        <Top10Table stocks={top_stocks} />
      </div>
    </div>
  );
}
