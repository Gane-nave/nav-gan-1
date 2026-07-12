/**
 * G.A.N.E — Futuristic Features (2950 Edition)
 * =================================================
 * AI Route Optimizer, Offline Mode, Social Navigation
 */
import { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  X, BrainCircuit, Wifi, WifiOff, Users, MapPin, Clock, Route,
  Zap, TrendingUp, Shield, Sparkles, Globe, Signal, Download,
  Share2, MessageCircle, Heart, Star, Navigation, ChevronRight,
  Activity, Cpu, Database, Eye
} from 'lucide-react';
import { useRealDataContext } from '@/contexts/RealDataContext';
import { useLanguage } from "@/contexts/LanguageContext";

/* ═══════════════════════════════════════════════════════════════
   AI ROUTE OPTIMIZER
   ═══════════════════════════════════════════════════════════════ */
export function AIRouteOptimizer({ onClose }: { onClose: () => void }) {
  const { lang } = useLanguage();
  const realData = useRealDataContext();
  const [isAnalyzing, setIsAnalyzing] = useState(true);
  const [progress, setProgress] = useState(0);
  const [recommendations, setRecommendations] = useState<{
    title: string; titleHe: string; savings: string; confidence: number; color: string; icon: typeof Zap;
  }[]>([]);
  const [patterns, setPatterns] = useState<{ label: string; value: string; trend: string; color: string }[]>([]);

  useEffect(() => {
    // Simulate AI analysis with progress
    const interval = setInterval(() => {
      setProgress(prev => {
        if (prev >= 100) {
          clearInterval(interval);
          setIsAnalyzing(false);
          return 100;
        }
        return prev + Math.random() * 15;
      });
    }, 200);

    // Generate recommendations based on real data
    const timer = setTimeout(() => {
      const recs = [
        { title: 'Optimal Departure', titleHe: 'זמן יציאה אופטימלי', savings: '-12 min', confidence: 94, color: '#16A34A', icon: Clock },
        { title: 'Weather-Aware Route', titleHe: 'מסלול מותאם מזג אוויר', savings: 'Safer', confidence: 87, color: '#2563EB', icon: Shield },
        { title: 'Eco Route', titleHe: 'מסלול חסכוני', savings: '-18% fuel', confidence: 91, color: '#7C3AED', icon: TrendingUp },
        { title: 'Low Congestion', titleHe: 'עומס נמוך', savings: '-8 min', confidence: 82, color: '#F97316', icon: Route },
      ];

      if (realData.weather && realData.weather.precipitation > 0) {
        recs.push({ title: 'Rain Avoidance', titleHe: 'הימנעות מגשם', savings: 'Dry route', confidence: 78, color: '#2563EB', icon: Globe });
      }

      setRecommendations(recs);
      setPatterns([
        { label: 'Avg. Commute', value: '32 min', trend: '-3 min vs last week', color: '#16A34A' },
        { label: 'Preferred Route', value: 'Ayalon → Begin', trend: '68% of trips', color: '#2563EB' },
        { label: 'Peak Hours', value: '07:30-09:00', trend: 'Avoid Tuesdays', color: '#F97316' },
        { label: 'Fuel Saved', value: '12.4L', trend: 'This month', color: '#7C3AED' },
      ]);
    }, 2500);

    return () => { clearInterval(interval); clearTimeout(timer); };
  }, [realData.weather]);

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: 'spring', damping: 30, stiffness: 300 }}
      className="expand-panel scrollbar-none"
    >
      <div className="sticky top-0 z-10 p-4 pb-3" style={{
        background: 'rgba(4,8,18,0.95)',
        borderBottom: '1px solid rgba(124,58,237,0.1)' }}>
        <div className="flex items-center justify-between mb-2">
          <div className="flex items-center gap-3">
            <motion.div
              animate={{ rotate: isAnalyzing ? [0, 360] : 0 }}
              transition={{ duration: 3, repeat: isAnalyzing ? Infinity : 0, ease: 'linear' }}
              className="w-10 h-10 rounded-xl flex items-center justify-center"
              style={{ background: 'rgba(124,58,237,0.1)', border: '1px solid rgba(124,58,237,0.2)' }}
            >
              <BrainCircuit size={18} style={{ color: '#7C3AED' }} />
            </motion.div>
            <div>
              <h3 className="text-sm font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: '#7C3AED' }}>AI Optimizer</h3>
              <p className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>
                {isAnalyzing ? 'Analyzing patterns...' : 'אופטימיזציה מבוססת AI'}
              </p>
            </div>
          </div>
          <button onClick={onClose} className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
            style={{ background: 'rgba(229,231,235,0.5)', border: '1px solid rgba(209,213,219,0.5)' }}>
            <X size={14} style={{ color: 'rgba(107,114,128,0.9)' }} />
          </button>
        </div>

        {/* Analysis progress */}
        <AnimatePresence>
          {isAnalyzing && (
            <motion.div exit={{ opacity: 0, height: 0 }} className="mt-3">
              <div className="flex items-center justify-between mb-1.5">
                <span className="text-[10px] font-mono" style={{ color: 'rgba(124,58,237,0.6)' }}>Neural analysis</span>
                <span className="text-[10px] font-mono font-bold" style={{ color: '#7C3AED' }}>{Math.min(progress, 100).toFixed(0)}%</span>
              </div>
              <div className="w-full h-1.5 rounded-full overflow-hidden" style={{ background: 'rgba(229,231,235,0.4)' }}>
                <motion.div className="h-full rounded-full"
                  style={{ background: 'linear-gradient(90deg, #7C3AED, #2563EB)', boxShadow: '0 0 8px rgba(124,58,237,0.4)' }}
                  animate={{ width: `${Math.min(progress, 100)}%` }} />
              </div>
              <div className="flex items-center gap-2 mt-2">
                {['Traffic patterns', 'Weather impact', 'Historical data', 'User preferences'].map((step, i) => (
                  <motion.span key={i}
                    animate={{ opacity: progress > i * 25 ? 1 : 0.3 }}
                    className="text-[8px] px-1.5 py-0.5 rounded" style={{ background: 'rgba(124,58,237,0.08)', color: '#7C3AED' }}>
                    {step}
                  </motion.span>
                ))}
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      <div className="px-4 pb-6 space-y-4 pt-3">
        {/* Recommendations */}
        {!isAnalyzing && (
          <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="space-y-4">
            <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(124,58,237,0.6)' }}>AI Recommendations</div>
            {recommendations.map((rec, i) => (
              <motion.div
                key={i}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: i * 0.08 }}
                className="relative overflow-hidden rounded-xl p-3 cursor-pointer"
                style={{ background: `${rec.color}06`, border: `1px solid ${rec.color}15` }}
                whileHover={{ scale: 1.01, borderColor: `${rec.color}30` }}
              >
                <div className="flex items-center gap-3">
                  <div className="w-9 h-9 rounded-lg flex items-center justify-center flex-shrink-0"
                    style={{ background: `${rec.color}10`, border: `1px solid ${rec.color}20` }}>
                    <rec.icon size={16} style={{ color: rec.color }} />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-xs font-bold" style={{ color: 'rgba(17,24,39,0.8)' }}>{rec.title}</div>
                    <div className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>{lang === 'he' ? rec.titleHe : rec.title}</div>
                  </div>
                  <div className="flex flex-col items-end">
                    <span className="text-xs font-bold font-mono" style={{ color: rec.color }}>{rec.savings}</span>
                    <span className="text-[9px]" style={{ color: 'rgba(156,163,175,0.9)' }}>{rec.confidence}% conf</span>
                  </div>
                </div>
                {/* Confidence bar */}
                <div className="mt-2 w-full h-1 rounded-full overflow-hidden" style={{ background: 'rgba(243,244,246,0.5)' }}>
                  <motion.div className="h-full rounded-full"
                    initial={{ width: 0 }}
                    animate={{ width: `${rec.confidence}%` }}
                    transition={{ delay: i * 0.08 + 0.3, duration: 0.8 }}
                    style={{ background: rec.color, boxShadow: `0 0 4px ${rec.color}40` }} />
                </div>
              </motion.div>
            ))}

            {/* Driving Patterns */}
            <div className="text-[10px] font-bold tracking-wider uppercase mt-4" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(124,58,237,0.6)' }}>Your Patterns</div>
            <div className="grid grid-cols-2 gap-2">
              {patterns.map((p, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, scale: 0.95 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: i * 0.06 + 0.5 }}
                  className="rounded-xl p-3"
                  style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.5)' }}
                >
                  <div className="text-[9px] uppercase tracking-wider mb-1" style={{ color: `${p.color}70` }}>{p.label}</div>
                  <div className="text-sm font-bold font-mono" style={{ color: p.color }}>{p.value}</div>
                  <div className="text-[9px] mt-1" style={{ color: 'rgba(156,163,175,0.9)' }}>{p.trend}</div>
                </motion.div>
              ))}
            </div>
          </motion.div>
        )}
      </div>
    </motion.div>
  );
}

/* ═══════════════════════════════════════════════════════════════
   OFFLINE MODE PANEL
   ═══════════════════════════════════════════════════════════════ */
export function OfflineModePanel({ onClose }: { onClose: () => void }) {
  const [isOnline, setIsOnline] = useState(navigator.onLine);
  const [cachedMaps, setCachedMaps] = useState([
    { name: 'Tel Aviv Metro', size: '124 MB', lastSync: '2 hours ago', coverage: 95 },
    { name: 'Jerusalem', size: '89 MB', lastSync: '5 hours ago', coverage: 88 },
    { name: 'Haifa Bay', size: '67 MB', lastSync: '1 day ago', coverage: 82 },
    { name: 'Negev Routes', size: '156 MB', lastSync: '3 days ago', coverage: 71 },
  ]);
  const [totalCached, setTotalCached] = useState(436);

  useEffect(() => {
    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);
    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: 'spring', damping: 30, stiffness: 300 }}
      className="expand-panel scrollbar-none"
    >
      <div className="sticky top-0 z-10 p-4 pb-3" style={{
        background: 'rgba(4,8,18,0.95)',
        borderBottom: `1px solid ${isOnline ? 'rgba(22,163,74,0.1)' : 'rgba(255,51,85,0.1)'}` }}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <motion.div
              animate={isOnline ? {} : { scale: [1, 1.1, 1] }}
              transition={{ duration: 1.5, repeat: Infinity }}
              className="w-10 h-10 rounded-xl flex items-center justify-center"
              style={{ background: isOnline ? 'rgba(22,163,74,0.1)' : 'rgba(255,51,85,0.1)', border: `1px solid ${isOnline ? 'rgba(22,163,74,0.2)' : 'rgba(255,51,85,0.2)'}` }}
            >
              {isOnline ? <Wifi size={18} style={{ color: '#16A34A' }} /> : <WifiOff size={18} style={{ color: '#DC2626' }} />}
            </motion.div>
            <div>
              <h3 className="text-sm font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: isOnline ? '#16A34A' : '#DC2626' }}>
                {isOnline ? 'Online' : 'Offline'}
              </h3>
              <p className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>
                {isOnline ? 'מחובר — סנכרון פעיל' : 'לא מחובר — מצב אופליין'}
              </p>
            </div>
          </div>
          <button onClick={onClose} className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
            style={{ background: 'rgba(229,231,235,0.5)', border: '1px solid rgba(209,213,219,0.5)' }}>
            <X size={14} style={{ color: 'rgba(107,114,128,0.9)' }} />
          </button>
        </div>
      </div>

      <div className="px-4 pb-6 space-y-4 pt-3">
        {/* Storage overview */}
        <div className="rounded-xl p-4" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.5)' }}>
          <div className="flex items-center justify-between mb-3">
            <span className="text-xs font-bold" style={{ color: 'rgba(75,85,99,0.9)' }}>Cached Storage</span>
            <span className="text-sm font-bold font-mono" style={{ color: '#2563EB' }}>{totalCached} MB</span>
          </div>
          <div className="w-full h-2 rounded-full overflow-hidden" style={{ background: 'rgba(229,231,235,0.4)' }}>
            <motion.div className="h-full rounded-full"
              initial={{ width: 0 }}
              animate={{ width: `${(totalCached / 1024) * 100}%` }}
              transition={{ duration: 1 }}
              style={{ background: 'linear-gradient(90deg, #2563EB, #16A34A)', boxShadow: '0 0 6px rgba(37,99,235,0.3)' }} />
          </div>
          <div className="flex justify-between mt-1.5">
            <span className="text-[9px]" style={{ color: 'rgba(156,163,175,0.9)' }}>{totalCached} MB used</span>
            <span className="text-[9px]" style={{ color: 'rgba(156,163,175,0.9)' }}>1 GB limit</span>
          </div>
        </div>

        {/* Cached maps */}
        <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(37,99,235,0.6)' }}>Cached Maps</div>
        {cachedMaps.map((map, i) => (
          <motion.div
            key={i}
            initial={{ opacity: 0, x: -10 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: i * 0.06 }}
            className="rounded-xl p-3"
            style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.5)' }}
          >
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <Download size={13} style={{ color: '#2563EB' }} />
                <span className="text-xs font-bold" style={{ color: 'rgba(55,65,81,0.9)' }}>{map.name}</span>
              </div>
              <span className="text-[10px] font-mono" style={{ color: 'rgba(107,114,128,0.8)' }}>{map.size}</span>
            </div>
            <div className="w-full h-1 rounded-full overflow-hidden" style={{ background: 'rgba(229,231,235,0.4)' }}>
              <div className="h-full rounded-full" style={{ width: `${map.coverage}%`, background: map.coverage > 90 ? '#16A34A' : map.coverage > 80 ? '#2563EB' : '#F97316' }} />
            </div>
            <div className="flex justify-between mt-1.5">
              <span className="text-[9px]" style={{ color: 'rgba(156,163,175,0.9)' }}>Coverage: {map.coverage}%</span>
              <span className="text-[9px]" style={{ color: 'rgba(156,163,175,0.9)' }}>Synced {map.lastSync}</span>
            </div>
          </motion.div>
        ))}

        {/* Offline capabilities */}
        <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(37,99,235,0.6)' }}>Offline Capabilities</div>
        {[
          { name: 'Navigation', status: true, desc: 'Full turn-by-turn' },
          { name: 'Search', status: true, desc: 'Cached POIs only' },
          { name: 'Traffic', status: false, desc: 'Requires connection' },
          { name: 'Weather', status: false, desc: 'Last known data' },
          { name: 'GNSS', status: true, desc: 'Full satellite tracking' },
        ].map((cap, i) => (
          <div key={i} className="flex items-center gap-3 p-2.5 rounded-xl" style={{ background: 'rgba(243,244,246,0.4)' }}>
            <div className="w-2 h-2 rounded-full" style={{ background: cap.status ? '#16A34A' : 'rgba(209,213,219,0.8)' }} />
            <span className="text-xs flex-1" style={{ color: 'rgba(75,85,99,0.9)' }}>{cap.name}</span>
            <span className="text-[10px]" style={{ color: 'rgba(156,163,175,0.9)' }}>{cap.desc}</span>
          </div>
        ))}
      </div>
    </motion.div>
  );
}

/* ═══════════════════════════════════════════════════════════════
   SOCIAL NAVIGATION PANEL
   ═══════════════════════════════════════════════════════════════ */
export function SocialNavPanel({ onClose }: { onClose: () => void }) {
  const [activeTab, setActiveTab] = useState<'friends' | 'groups' | 'meetup'>('friends');
  const [friends] = useState([
    { name: 'David K.', avatar: '🧑‍💻', status: 'driving', location: 'Ayalon North', eta: '12 min', speed: 65 },
    { name: 'Sarah M.', avatar: '👩‍🔬', status: 'walking', location: 'Rothschild Blvd', eta: '5 min', speed: 5 },
    { name: 'Yossi R.', avatar: '🧑‍🚀', status: 'idle', location: 'Dizengoff Center', eta: '--', speed: 0 },
    { name: 'Maya L.', avatar: '👩‍🎨', status: 'driving', location: 'Highway 2', eta: '25 min', speed: 92 },
  ]);
  const [groups] = useState([
    { name: 'Morning Commute TLV', members: 12, active: 8 },
    { name: 'Weekend Hikers', members: 24, active: 3 },
    { name: 'Family Circle', members: 6, active: 4 },
  ]);

  const statusColors: Record<string, string> = { driving: '#2563EB', walking: '#16A34A', idle: 'rgba(156,163,175,0.9)' };

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: 'spring', damping: 30, stiffness: 300 }}
      className="expand-panel scrollbar-none"
    >
      <div className="sticky top-0 z-10 p-4 pb-3" style={{
        background: 'rgba(4,8,18,0.95)',
        borderBottom: '1px solid rgba(37,99,235,0.1)' }}>
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl flex items-center justify-center"
              style={{ background: 'rgba(37,99,235,0.1)', border: '1px solid rgba(37,99,235,0.2)' }}>
              <Users size={18} style={{ color: '#2563EB' }} />
            </div>
            <div>
              <h3 className="text-sm font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: '#2563EB' }}>Social Nav</h3>
              <p className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>ניווט חברתי — שיתוף מיקום</p>
            </div>
          </div>
          <button onClick={onClose} className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
            style={{ background: 'rgba(229,231,235,0.5)', border: '1px solid rgba(209,213,219,0.5)' }}>
            <X size={14} style={{ color: 'rgba(107,114,128,0.9)' }} />
          </button>
        </div>

        {/* Tabs */}
        <div className="flex gap-1 p-1 rounded-xl" style={{ background: 'rgba(243,244,246,0.5)' }}>
          {[
            { id: 'friends' as const, label: 'Friends', icon: Users },
            { id: 'groups' as const, label: 'Groups', icon: Share2 },
            { id: 'meetup' as const, label: 'Meetup', icon: MapPin },
          ].map(t => (
            <button key={t.id} onClick={() => setActiveTab(t.id)}
              className="flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-[10px] font-bold transition-all cursor-pointer"
              style={activeTab === t.id ? {
                background: 'rgba(37,99,235,0.12)', border: '1px solid rgba(37,99,235,0.25)', color: '#2563EB' } : { color: 'rgba(107,114,128,0.8)' }}>
              <t.icon size={12} />
              {t.label}
            </button>
          ))}
        </div>
      </div>

      <div className="px-4 pb-6 space-y-4 pt-3">
        <AnimatePresence mode="wait">
          {activeTab === 'friends' && (
            <motion.div key="friends" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0 }} className="space-y-3">
              {friends.map((f, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.06 }}
                  className="rounded-xl p-3 cursor-pointer"
                  style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.5)' }}
                  whileHover={{ borderColor: 'rgba(37,99,235,0.2)' }}
                >
                  <div className="flex items-center gap-3">
                    <div className="text-xl">{f.avatar}</div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2">
                        <span className="text-xs font-bold" style={{ color: 'rgba(17,24,39,0.8)' }}>{f.name}</span>
                        <motion.div
                          animate={f.status === 'driving' ? { opacity: [1, 0.3, 1] } : {}}
                          transition={{ duration: 1.5, repeat: Infinity }}
                          className="w-2 h-2 rounded-full"
                          style={{ background: statusColors[f.status] }}
                        />
                      </div>
                      <div className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>{f.location}</div>
                    </div>
                    <div className="flex flex-col items-end">
                      {f.speed > 0 && (
                        <span className="text-xs font-mono font-bold" style={{ color: statusColors[f.status] }}>{f.speed} km/h</span>
                      )}
                      <span className="text-[10px]" style={{ color: 'rgba(156,163,175,0.9)' }}>ETA: {f.eta}</span>
                    </div>
                  </div>
                </motion.div>
              ))}
            </motion.div>
          )}

          {activeTab === 'groups' && (
            <motion.div key="groups" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0 }} className="space-y-3">
              {groups.map((g, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.06 }}
                  className="rounded-xl p-4 cursor-pointer"
                  style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.5)' }}
                  whileHover={{ borderColor: 'rgba(37,99,235,0.2)' }}
                >
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-xs font-bold" style={{ color: 'rgba(17,24,39,0.8)' }}>{g.name}</span>
                    <ChevronRight size={14} style={{ color: 'rgba(156,163,175,0.9)' }} />
                  </div>
                  <div className="flex items-center gap-3">
                    <div className="flex items-center gap-1">
                      <Users size={12} style={{ color: 'rgba(107,114,128,0.8)' }} />
                      <span className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>{g.members} members</span>
                    </div>
                    <div className="flex items-center gap-1">
                      <motion.div animate={{ opacity: [1, 0.3, 1] }} transition={{ duration: 1.5, repeat: Infinity }}
                        className="w-1.5 h-1.5 rounded-full" style={{ background: '#16A34A' }} />
                      <span className="text-[10px]" style={{ color: '#16A34A' }}>{g.active} active</span>
                    </div>
                  </div>
                </motion.div>
              ))}
            </motion.div>
          )}

          {activeTab === 'meetup' && (
            <motion.div key="meetup" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0 }} className="space-y-4">
              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(37,99,235,0.6)' }}>Set Meeting Point</div>
              <div className="rounded-xl p-4 text-center" style={{ background: 'rgba(37,99,235,0.04)', border: '1px solid rgba(37,99,235,0.1)' }}>
                <MapPin size={24} style={{ color: '#2563EB', margin: '0 auto 8px' }} />
                <div className="text-xs font-bold mb-1" style={{ color: 'rgba(55,65,81,0.9)' }}>Tap on map to set meetup point</div>
                <div className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>All friends will get navigation to this point</div>
              </div>

              <div className="text-[10px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(37,99,235,0.6)' }}>Quick Meetup</div>
              {[
                { name: 'Midpoint', desc: 'Meet halfway between all friends', icon: Navigation },
                { name: 'Nearest Cafe', desc: 'Closest cafe to everyone', icon: Heart },
                { name: 'Custom Location', desc: 'Choose a specific place', icon: Star },
              ].map((opt, i) => (
                <motion.div
                  key={i}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.06 }}
                  className="flex items-center gap-3 p-3 rounded-xl cursor-pointer"
                  style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.5)' }}
                  whileHover={{ borderColor: 'rgba(37,99,235,0.2)' }}
                >
                  <opt.icon size={16} style={{ color: '#2563EB' }} />
                  <div className="flex-1">
                    <div className="text-xs font-bold" style={{ color: 'rgba(55,65,81,0.9)' }}>{opt.name}</div>
                    <div className="text-[10px]" style={{ color: 'rgba(107,114,128,0.8)' }}>{opt.desc}</div>
                  </div>
                  <ChevronRight size={14} style={{ color: 'rgba(156,163,175,0.6)' }} />
                </motion.div>
              ))}
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}
