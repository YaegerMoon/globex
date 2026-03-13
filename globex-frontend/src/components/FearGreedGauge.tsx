'use client';

interface FearGreedGaugeProps {
  score: number;
}

export default function FearGreedGauge({ score }: FearGreedGaugeProps) {
  const rotation = (score / 100) * 180 - 90; // Rotate from -90 to 90

  const getStatus = (s: number) => {
    if (s < 25) return { label: 'EXTREME FEAR', color: 'text-red-600' };
    if (s < 45) return { label: 'FEAR', color: 'text-orange-500' };
    if (s < 55) return { label: 'NEUTRAL', color: 'text-yellow-600' };
    if (s < 75) return { label: 'GREED', color: 'text-green-500' };
    return { label: 'EXTREME GREED', color: 'text-green-600' };
  };

  const status = getStatus(score);

  return (
    <div className="flex flex-col items-center">
      <div className="relative w-64 h-32 overflow-hidden">
        {/* Semi-circle Gauge Background */}
        <div className="absolute top-0 left-0 w-64 h-64 border-[16px] border-gray-100 rounded-full"></div>
        
        {/* Color segments (simplified using a conic gradient approach or multiple arcs) */}
        <div 
          className="absolute top-0 left-0 w-64 h-64 border-[16px] rounded-full border-transparent"
          style={{
            background: 'conic-gradient(from 270deg, #ef4444 0deg, #f97316 45deg, #eab308 81deg, #84cc16 135deg, #16a34a 180deg, transparent 180deg)',
            maskImage: 'radial-gradient(circle at 50% 50%, transparent 58%, black 60%)',
            WebkitMaskImage: 'radial-gradient(circle at 50% 50%, transparent 58%, black 60%)',
          }}
        ></div>

        {/* Needle */}
        <div 
          className="absolute bottom-0 left-1/2 w-1 h-28 bg-gray-800 origin-bottom transition-transform duration-1000 ease-out"
          style={{ transform: `translateX(-50%) rotate(${rotation}deg)` }}
        >
          <div className="absolute top-0 left-1/2 -translate-x-1/2 w-3 h-3 bg-gray-800 rounded-full -mt-1"></div>
        </div>
      </div>
      
      <div className="mt-4 text-center">
        <div className="text-5xl font-black text-gray-900">{score}</div>
        <div className={`text-xl font-bold mt-1 ${status.color}`}>{status.label}</div>
      </div>
    </div>
  );
}
