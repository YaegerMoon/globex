'use client';

import { useQuery } from '@tanstack/react-query';
import { marketService } from '@/services/marketService';
import MarketCard from '@/components/MarketCard';
import { RefreshCw, LayoutGrid } from 'lucide-react';

export default function Dashboard() {
  const { data: markets, isLoading, isError, refetch } = useQuery({
    queryKey: ['markets'],
    queryFn: marketService.getMarkets,
    refetchInterval: 60000, // Refetch every minute
  });

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center min-h-[60vh]">
        <RefreshCw className="w-10 h-10 text-blue-500 animate-spin mb-4" />
        <p className="text-gray-500 font-medium">Loading market data...</p>
      </div>
    );
  }

  if (isError) {
    return (
      <div className="bg-red-50 border border-red-200 rounded-lg p-8 text-center">
        <h2 className="text-xl font-bold text-red-700 mb-2">Error loading data</h2>
        <p className="text-red-600 mb-4">Failed to connect to the server. Please make sure the backend is running.</p>
        <button 
          onClick={() => refetch()}
          className="bg-red-600 text-white px-6 py-2 rounded-lg font-bold hover:bg-red-700 transition-colors"
        >
          Try Again
        </button>
      </div>
    );
  }

  return (
    <div>
      <div className="flex justify-between items-center mb-8">
        <div>
          <h2 className="text-3xl font-black text-gray-900 tracking-tight">Market Dashboard</h2>
          <p className="text-gray-500 mt-1">Real-time global indices and sentiment analysis</p>
        </div>
        <div className="flex items-center gap-2 bg-white border border-gray-200 rounded-lg p-2 shadow-sm">
          <LayoutGrid className="text-gray-400" size={20} />
          <span className="text-sm font-semibold text-gray-700">Grid View</span>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {markets?.map((market) => (
          <MarketCard key={market.exchange_id} market={market} />
        ))}
      </div>
    </div>
  );
}
