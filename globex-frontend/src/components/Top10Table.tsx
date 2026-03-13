'use client';

import { TopStock } from '@/services/marketService';

interface Top10TableProps {
  stocks: TopStock[];
}

export default function Top10Table({ stocks }: Top10TableProps) {
  const formatMarketCap = (cap: number) => {
    if (cap >= 1e12) return `$${(cap / 1e12).toFixed(2)}T`;
    if (cap >= 1e9) return `$${(cap / 1e9).toFixed(2)}B`;
    return `$${(cap / 1e6).toFixed(2)}M`;
  };

  return (
    <div className="overflow-x-auto">
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Rank</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Symbol</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Name</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Market Cap</th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Change</th>
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {stocks.length > 0 ? (
            stocks.map((stock) => (
              <tr key={stock.symbol} className="hover:bg-gray-50 transition-colors">
                <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">{stock.rank}</td>
                <td className="px-6 py-4 whitespace-nowrap text-sm font-bold text-blue-600">{stock.symbol}</td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">{stock.name}</td>
                <td className="px-6 py-4 whitespace-nowrap text-sm font-semibold text-gray-900">{formatMarketCap(stock.market_cap)}</td>
                <td className={`px-6 py-4 whitespace-nowrap text-sm font-medium ${stock.change_percent >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                  {stock.change_percent >= 0 ? '+' : ''}{stock.change_percent.toFixed(2)}%
                </td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={5} className="px-6 py-10 text-center text-gray-400">
                No stock data available for this market.
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}
