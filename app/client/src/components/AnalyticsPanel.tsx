/**
 * G.A.N.E — Analytics & Intelligence Panel (2950 Edition)
 * Live data from real APIs, animated sparklines, pulsing metrics, holographic design
 */
import { useState, useEffect, useCallback, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { X, TrendingUp, ShieldCheck, Zap, ScanEye, Sparkles, Signal, Users, Clock, Activity, Cpu, Globe, Wifi, Database } from 'lucide-react';
import { useRealDataContext } from '@/contexts/RealDataContext';

/* ── Animated Circular Gauge ── */
function AnimatedGauge({ value, size = 80, strokeWidth = 6, color, label }: { value: number; size?: number; strokeWidth?: number; color: string; label: string }) {
  const [animValue, setAnimValue] = useState(0);
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const offset = circumference - (animValue / 100) * circumference;

  useEffect(() => {
    const timer = setTimeout(() => setAnimValue(value), 100);
    return () => clearTimeout(timer);
  }, [value]);

  return (
    <div className="flex flex-col items-center gap-2">
      <div className="relative">
        <svg width={size} height={size} className="transform -rotate-90">
          <circle cx={size / 2} cy={size / 2} r={radius} fill="none" strokeWidth={strokeWidth}
            stroke="rgba(229,231,235,0.4)" />
          <circle cx={size / 2} cy={size / 2} r={radius} fill="none" strokeWidth={strokeWidth} stroke={color}
            strokeDasharray={circumference} strokeDashoffset={offset} strokeLinecap="round"
            style={{ transition: 'stroke-dashoffset 1.5s cubic-bezier(0.16, 1, 0.3, 1)', filter: `drop-shadow(0 0 4px ${color}60)` }} />
        </svg>
        <div className="absolute inset-0 flex items-center justify-center">
          <span className="text-sm font-bold" style={{ fontFamily: 'JetBrains Mono, monospace', color }}>{animValue.toFixed(0)}</span>
        </div>
      </div>
      <span className="text-[9px] uppercase tracking-wider" style={{ color: `${color}80` }}>{label}</span>
    </div>
  );
}

/* ── Live Sparkline ── */
function Sparkline({ data, color, height = 32 }: { data: number[]; color: string; height?: number }) {
  if (data.length < 2) return null;
  const max = Math.max(...data);
  const min = Math.min(...data);
  const range = max - min || 1;
  const w = 100;
  const points = data.map((v, i) => `${(i / (data.length - 1)) * w},${height - ((v - min) / range) * (height - 4) - 2}`).join(' ');
  const areaPoints = `0,${height} ${points} ${w},${height}`;

  return (
    <svg viewBox={`0 0 ${w} ${height}`} className="w-full" style={{ height }}>
      <defs>
        <linearGradient id={`spark-${color.replace(/[^a-z0-9]/gi, '')}`} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor={color} stopOpacity="0.3" />
          <stop offset="100%" stopColor={color} stopOpacity="0" />
        </linearGradient>
      </defs>
      <polygon points={areaPoints} fill={`url(#spark-${color.replace(/[^a-z0-9]/gi, '')})`} />
      <polyline points={points} fill="none" stroke={color} strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"
        style={{ filter: `drop-shadow(0 0 3px ${color}60)` }} />
      {/* Pulsing dot at end */}
      <circle cx={w} cy={parseFloat(points.split(' ').pop()?.split(',')[1] || '0')} r="2.5" fill={color}>
        <animate attributeName="r" values="2;3.5;2" dur="2s" repeatCount="indefinite" />
        <animate attributeName="opacity" values="1;0.5;1" dur="2s" repeatCount="indefinite" />
      </circle>
    </svg>
  );
}

/* ── Pulsing Metric Card ── */
function LiveMetric({ label, value, unit, color, sparkData, icon: Icon }: {
  label: string; value: string; unit?: string; color: string; sparkData?: number[]; icon?: typeof Activity;
}) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      className="relative overflow-hidden rounded-xl p-3"
      style={{ background: 'rgba(243,244,246,0.4)', border: `1px solid ${color}15` }}
    >
      {/* Ambient glow */}
      <div className="absolute top-0 right-0 w-12 h-12 rounded-full pointer-events-none"
        style={{ background: `radial-gradient(circle, ${color}08, transparent)` }} />

      <div className="flex items-center gap-2 mb-1.5">
        {Icon && <Icon size={12} style={{ color: `${color}70` }} />}
        <span className="text-[9px] uppercase tracking-wider font-bold" style={{ color: `${color}80`, fontFamily: 'Syne, sans-serif' }}>{label}</span>
      </div>
      <div className="flex items-baseline gap-1">
        <span className="text-lg font-bold" style={{ fontFamily: 'JetBrains Mono, monospace', color }}>{value}</span>
        {unit && <span className="text-[10px]" style={{ color: `${color}60` }}>{unit}</span>}
      </div>
      {sparkData && sparkData.length > 2 && (
        <div className="mt-2">
          <Sparkline data={sparkData} color={color} height={24} />
        </div>
      )}
    </motion.div>
  );
}

/* ── Main Panel ── */
export default function AnalyticsPanel({ onClose }: { onClose: () => void }) {
  const realData = useRealDataContext();
  const [activeView, setActiveView] = useState<'overview' | 'traffic' | 'driver' | 'trust'>('overview');
  const [liveMetrics, setLiveMetrics] = useState({
    uptime: 99.97, latency: 12, coverage: 87, activeUsers: 2400000,
    routesPerMin: 847000, incidents: 0, etaAccuracy: 96.8 });
  const [sparkHistory, setSparkHistory] = useState<{ latency: number[]; users: number[]; routes: number[]; congestion: number[] }>({
    latency: [14, 12, 15, 11, 13, 12, 14, 11, 12, 13, 12, 11],
    users: [2.1, 2.2, 2.3, 2.35, 2.4, 2.38, 2.42, 2.45, 2.4, 2.43, 2.41, 2.4],
    routes: [780, 800, 820, 835, 847, 860, 855, 848, 840, 850, 847, 852],
    congestion: [30, 35, 42, 55, 68, 72, 65, 48, 35, 28, 22, 20] });
  const [liveFeed, setLiveFeed] = useState<{ icon: typeof Signal; text: string; time: string; color: string }[]>([]);
  const tickRef = useRef(0);

  // Generate live feed from real data
  useEffect(() => {
    const feed: typeof liveFeed = [];
    if (realData.weather) {
      feed.push({ icon: Globe, text: `Weather: ${realData.weather.weatherDescription}, ${realData.weather.temperature.toFixed(0)}°C`, time: 'Live', color: '#2563EB' });
    }
    if (realData.earthquakes.length > 0) {
      const eq = realData.earthquakes[0];
      feed.push({ icon: Activity, text: `Seismic: M${eq.magnitude.toFixed(1)} — ${eq.place}`, time: `${Math.round((Date.now() - new Date(eq.time).getTime()) / 60000)}m`, color: '#DC2626' });
    }
    if (realData.gnss) {
      const activeSys = ['GPS', 'Galileo', 'GLONASS', 'BeiDou', 'NavIC', 'QZSS', 'SBAS'].filter(s => (realData.gnss as any)?.[s.toLowerCase()]);
      feed.push({ icon: Wifi, text: `GNSS: ${realData.gnss.satellitesInView} sats, ${activeSys.length || 4} constellations`, time: 'Live', color: '#16A34A' });
    }
    feed.push({ icon: Cpu, text: `Edge nodes processing at ${(92 + Math.random() * 6).toFixed(1)}% capacity`, time: `${Math.floor(Math.random() * 5) + 1}m`, color: '#7C3AED' });
    feed.push({ icon: Database, text: `Data sync: ${(1.2 + Math.random() * 0.5).toFixed(1)}TB processed this hour`, time: `${Math.floor(Math.random() * 10) + 1}m`, color: '#F97316' });
    feed.push({ icon: ShieldCheck, text: `Trust verification: ${Math.floor(800 + Math.random() * 100)} evidence items validated`, time: `${Math.floor(Math.random() * 15) + 1}m`, color: '#16A34A' });
    setLiveFeed(feed);
  }, [realData]);

  // Simulate live metric updates
  useEffect(() => {
    const interval = setInterval(() => {
      tickRef.current++;
      setLiveMetrics(prev => ({
        ...prev,
        latency: Math.max(5, prev.latency + (Math.random() - 0.5) * 3),
        activeUsers: prev.activeUsers + Math.floor((Math.random() - 0.3) * 5000),
        routesPerMin: prev.routesPerMin + Math.floor((Math.random() - 0.4) * 2000),
        incidents: realData.earthquakes.length }));
      setSparkHistory(prev => ({
        latency: [...prev.latency.slice(-11), Math.max(5, 12 + (Math.random() - 0.5) * 6)],
        users: [...prev.users.slice(-11), 2.4 + (Math.random() - 0.5) * 0.1],
        routes: [...prev.routes.slice(-11), 847 + Math.floor((Math.random() - 0.5) * 40)],
        congestion: [...prev.congestion.slice(-11), Math.max(10, Math.min(90, prev.congestion[prev.congestion.length - 1] + (Math.random() - 0.5) * 15))] }));
    }, 3000);
    return () => clearInterval(interval);
  }, [realData.earthquakes.length]);

  const views = [
    { id: 'overview' as const, label: 'Overview', icon: Sparkles },
    { id: 'traffic' as const, label: 'Traffic', icon: Signal },
    { id: 'driver' as const, label: 'Driver', icon: Users },
    { id: 'trust' as const, label: 'Trust', icon: ShieldCheck },
  ];

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: 'spring', damping: 30, stiffness: 300 }}
      className="expand-panel scrollbar-none"
    >
      {/* Header */}
      <div className="sticky top-0 z-10 p-4 pb-3" style={{
        background: 'rgba(4,8,18,0.95)',
        borderBottom: '1px solid rgba(255,153,0,0.1)' }}>
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-3">
            <motion.div
              animate={{ rotate: [0, 360] }}
              transition={{ duration: 20, repeat: Infinity, ease: 'linear' }}
              className="w-10 h-10 rounded-xl flex items-center justify-center relative"
              style={{ background: 'rgba(255,153,0,0.1)', border: '1px solid rgba(255,153,0,0.2)' }}
            >
              <Sparkles size={18} style={{ color: '#F97316' }} />
            </motion.div>
            <div>
              <h3 className="text-sm font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: '#F97316' }}>Analytics</h3>
              <div className="flex items-center gap-1.5">
                <motion.div animate={{ opacity: [1, 0.3, 1] }} transition={{ duration: 1.5, repeat: Infinity }}
                  className="w-1.5 h-1.5 rounded-full" style={{ background: '#16A34A' }} />
                <p className="text-[10px]" style={{ color: 'rgba(107,114,128,0.9)' }}>Live Intelligence Feed</p>
              </div>
            </div>
          </div>
          <button onClick={onClose} className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
            style={{ background: 'rgba(229,231,235,0.5)', border: '1px solid rgba(209,213,219,0.5)' }}>
            <X size={14} style={{ color: 'rgba(107,114,128,0.9)' }} />
          </button>
        </div>

        {/* View Tabs */}
        <div className="flex gap-1 p-1 rounded-xl" style={{ background: 'rgba(243,244,246,0.5)' }}>
          {views.map(v => (
            <button key={v.id} onClick={() => setActiveView(v.id)}
              className="flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-[10px] font-bold transition-all cursor-pointer"
              style={activeView === v.id ? {
                background: 'rgba(255,153,0,0.12)', border: '1px solid rgba(255,153,0,0.25)', color: '#F97316' } : { color: 'rgba(107,114,128,0.8)' }}
            >
              <v.icon size={12} />
              {v.label}
            </button>
          ))}
        </div>
      </div>

      <div className="px-4 pb-6 space-y-4 pt-3">
        <AnimatePresence mode="wait">
          {activeView === 'overview' && (
            <motion.div key="overview" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }} className="space-y-4">
              {/* System Health Gauges */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>System Health</div>
              <div className="flex items-center justify-around py-2">
                <AnimatedGauge value={liveMetrics.uptime} size={68} strokeWidth={5} color="#16A34A" label="Uptime" />
                <AnimatedGauge value={100 - liveMetrics.latency} size={68} strokeWidth={5} color="#2563EB" label="Latency" />
                <AnimatedGauge value={liveMetrics.coverage} size={68} strokeWidth={5} color="#7C3AED" label="Coverage" />
              </div>

              {/* Key Metrics with Sparklines */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Key Metrics</div>
              <div className="grid grid-cols-2 gap-2">
                <LiveMetric label="Active Users" value={`${(liveMetrics.activeUsers / 1000000).toFixed(1)}M`} color="#2563EB" sparkData={sparkHistory.users} icon={Users} />
                <LiveMetric label="Routes/min" value={`${(liveMetrics.routesPerMin / 1000).toFixed(0)}K`} color="#16A34A" sparkData={sparkHistory.routes} icon={Activity} />
                <LiveMetric label="Latency" value={liveMetrics.latency.toFixed(0)} unit="ms" color="#7C3AED" sparkData={sparkHistory.latency} icon={Zap} />
                <LiveMetric label="ETA Accuracy" value={liveMetrics.etaAccuracy.toFixed(1)} unit="%" color="#F97316" icon={ScanEye} />
              </div>

              {/* Live Feed from Real Data */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Live Feed</div>
              <div className="space-y-1.5">
                {liveFeed.map((item, i) => (
                  <motion.div
                    key={i}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: i * 0.05 }}
                    className="flex items-center gap-3 p-2.5 rounded-xl"
                    style={{ background: `${item.color}06`, border: `1px solid ${item.color}10` }}
                  >
                    <item.icon size={13} style={{ color: item.color, flexShrink: 0 }} />
                    <div className="flex-1 min-w-0">
                      <div className="text-[11px] truncate" style={{ color: 'rgba(55,65,81,0.9)' }}>{item.text}</div>
                    </div>
                    <span className="text-[9px] flex-shrink-0 font-mono" style={{ color: `${item.color}70` }}>{item.time}</span>
                  </motion.div>
                ))}
              </div>
            </motion.div>
          )}

          {activeView === 'traffic' && (
            <motion.div key="traffic" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }} className="space-y-4">
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Traffic Intelligence</div>

              {/* Congestion Forecast with animated bars */}
              <div className="rounded-xl p-4" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.5)' }}>
                <div className="flex items-center justify-between mb-3">
                  <span className="text-xs font-bold" style={{ color: 'rgba(75,85,99,0.9)' }}>Congestion Forecast (2h)</span>
                  <TrendingUp size={14} style={{ color: '#F97316' }} />
                </div>
                <div className="flex items-end gap-1 h-20">
                  {sparkHistory.congestion.map((v, i) => (
                    <motion.div
                      key={i}
                      initial={{ height: 0 }}
                      animate={{ height: `${v}%` }}
                      transition={{ delay: i * 0.05, duration: 0.5, type: 'spring' }}
                      className="flex-1 rounded-t"
                      style={{
                        background: v > 60 ? 'rgba(255,51,85,0.6)' : v > 40 ? 'rgba(255,153,0,0.6)' : 'rgba(22,163,74,0.6)',
                        boxShadow: v > 60 ? '0 0 8px rgba(255,51,85,0.3)' : 'none' }}
                    />
                  ))}
                </div>
                <div className="flex justify-between mt-2">
                  <span className="text-[9px]" style={{ color: 'rgba(156,163,175,0.9)' }}>Now</span>
                  <span className="text-[9px]" style={{ color: 'rgba(156,163,175,0.9)' }}>+2h</span>
                </div>
              </div>

              {/* Corridor Status */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Corridor Status</div>
              {[
                { name: 'Ayalon North', status: 'Moderate', speed: 38, color: '#F97316' },
                { name: 'Highway 2 South', status: 'Free', speed: 92, color: '#16A34A' },
                { name: 'Begin Blvd', status: 'Heavy', speed: 15, color: '#DC2626' },
                { name: 'Highway 4', status: 'Free', speed: 88, color: '#16A34A' },
                { name: 'Geha Junction', status: 'Moderate', speed: 35, color: '#F97316' },
              ].map((c, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.06 }}
                  className="flex items-center gap-3 p-3 rounded-xl"
                  style={{ background: `${c.color}06`, border: `1px solid ${c.color}10` }}
                >
                  <motion.div
                    animate={c.status === 'Heavy' ? { scale: [1, 1.3, 1] } : {}}
                    transition={{ duration: 1, repeat: Infinity }}
                    className="w-2.5 h-2.5 rounded-full flex-shrink-0"
                    style={{ background: c.color, boxShadow: `0 0 6px ${c.color}60` }}
                  />
                  <div className="flex-1">
                    <div className="text-xs font-medium" style={{ color: 'rgba(55,65,81,0.9)' }}>{c.name}</div>
                    <div className="text-[10px]" style={{ color: `${c.color}80` }}>{c.status}</div>
                  </div>
                  <span className="text-sm font-bold font-mono" style={{ color: c.color }}>{c.speed}<span className="text-[9px] ml-0.5">km/h</span></span>
                </motion.div>
              ))}
            </motion.div>
          )}

          {activeView === 'driver' && (
            <motion.div key="driver" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }} className="space-y-4">
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Driver Performance</div>

              {/* Large central score */}
              <div className="flex items-center justify-center py-4">
                <AnimatedGauge value={87} size={120} strokeWidth={8} color="#2563EB" label="Overall Score" />
              </div>

              <div className="grid grid-cols-2 gap-2">
                <LiveMetric label="Safety" value="92" unit="/100" color="#16A34A" icon={ShieldCheck} />
                <LiveMetric label="Eco" value="78" unit="/100" color="#2563EB" icon={Globe} />
                <LiveMetric label="Smoothness" value="84" unit="/100" color="#7C3AED" icon={Activity} />
                <LiveMetric label="Alertness" value="95" unit="/100" color="#F97316" icon={Zap} />
              </div>

              {/* Cognitive Load */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Cognitive Load</div>
              <div className="rounded-xl p-4" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(22,163,74,0.1)' }}>
                <div className="flex items-center justify-between mb-2">
                  <span className="text-xs" style={{ color: 'rgba(75,85,99,0.9)' }}>Current Load</span>
                  <span className="text-sm font-bold font-mono" style={{ color: '#16A34A' }}>25%</span>
                </div>
                <div className="w-full h-2.5 rounded-full overflow-hidden" style={{ background: 'rgba(229,231,235,0.4)' }}>
                  <motion.div
                    initial={{ width: 0 }}
                    animate={{ width: '25%' }}
                    transition={{ duration: 1, type: 'spring' }}
                    className="h-full rounded-full"
                    style={{ background: 'linear-gradient(90deg, #16A34A, #2563EB)', boxShadow: '0 0 8px rgba(22,163,74,0.4)' }}
                  />
                </div>
                <div className="text-[10px] mt-2" style={{ color: 'rgba(107,114,128,0.8)' }}>
                  Fatigue: Low (12%) | Reaction: 320ms | Focus: High
                </div>
              </div>

              {/* Trip History */}
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Trip History</div>
              {[
                { date: 'Today', trips: 3, distance: '47 km', score: 89 },
                { date: 'Yesterday', trips: 5, distance: '82 km', score: 91 },
                { date: 'Mar 25', trips: 2, distance: '28 km', score: 85 },
              ].map((t, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.06 }}
                  className="flex items-center gap-3 p-3 rounded-xl"
                  style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.5)' }}
                >
                  <Clock size={14} style={{ color: '#2563EB' }} />
                  <div className="flex-1">
                    <div className="text-xs font-medium" style={{ color: 'rgba(55,65,81,0.9)' }}>{t.date}</div>
                    <div className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>{t.trips} trips · {t.distance}</div>
                  </div>
                  <span className="text-sm font-bold font-mono" style={{ color: t.score > 88 ? '#16A34A' : '#F97316' }}>{t.score}</span>
                </motion.div>
              ))}
            </motion.div>
          )}

          {activeView === 'trust' && (
            <motion.div key="trust" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }} className="space-y-4">
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Trust & Verification</div>

              <div className="flex items-center justify-center py-4">
                <AnimatedGauge value={94} size={120} strokeWidth={8} color="#16A34A" label="Trust Score" />
              </div>

              <div className="grid grid-cols-2 gap-2">
                <LiveMetric label="Evidence Verified" value="847" color="#16A34A" icon={ShieldCheck} />
                <LiveMetric label="Consensus Rate" value="96.2" unit="%" color="#2563EB" icon={Users} />
                <LiveMetric label="False Reports" value="0.3" unit="%" color="#DC2626" icon={ScanEye} />
                <LiveMetric label="Avg Verification" value="4.2" unit="sec" color="#7C3AED" icon={Clock} />
              </div>

              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(255,153,0,0.6)' }}>Recent Verifications</div>
              {[
                { type: 'Incident', desc: 'Accident on Ayalon verified by 3 sources', trust: 95, color: '#DC2626' },
                { type: 'Road Work', desc: 'Construction on Rothschild confirmed', trust: 98, color: '#F97316' },
                { type: 'Hazard', desc: 'Pothole on Ibn Gabirol flagged', trust: 82, color: '#2563EB' },
                { type: 'Speed Trap', desc: 'Police on Highway 20 reported', trust: 76, color: '#7C3AED' },
              ].map((v, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.06 }}
                  className="flex items-center gap-3 p-3 rounded-xl"
                  style={{ background: `${v.color}06`, border: `1px solid ${v.color}10` }}
                >
                  <ShieldCheck size={14} style={{ color: v.color }} />
                  <div className="flex-1 min-w-0">
                    <div className="text-xs font-medium" style={{ color: 'rgba(55,65,81,0.9)' }}>{v.type}</div>
                    <div className="text-[10px] truncate" style={{ color: 'rgba(107,114,128,0.8)' }}>{v.desc}</div>
                  </div>
                  <div className="text-xs font-bold font-mono" style={{ color: v.trust > 90 ? '#16A34A' : '#F97316' }}>
                    {v.trust}%
                  </div>
                </motion.div>
              ))}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}
