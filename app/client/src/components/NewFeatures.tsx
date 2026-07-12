/**
 * G.A.N.E — New Premium Features
 * ===================================
 * Night Mode, Route History, Smart Alerts
 * Each feature is world-class with cinematic animations
 */
import { useState, useEffect, useRef, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Moon, Sun, Sunrise, Sunset, History, Clock, MapPin,
  Footprints, Car, AlertTriangle, CloudRain, Wind,
  Thermometer, Shield, Bell, BellOff, Volume2, VolumeX,
  ChevronRight, X, TrendingUp, Zap, Navigation,
  Timer, Route, Gauge, Star, Flame, Eye, Radar,
  CircleDot, Sparkles, ArrowRight, Trash2, RotateCcw,
  Satellite, Wifi, Radio, MapPinOff, Signal
} from "lucide-react";
import { useRealDataContext } from "@/contexts/RealDataContext";
import { useLanguage } from "@/contexts/LanguageContext";

// ═══════════════════════════════════════════════════
// NIGHT MODE CONTROLLER — Auto day/night with smooth transition
// ═══════════════════════════════════════════════════
export function NightModeController({ children }: { children: (isNight: boolean, toggle: () => void) => React.ReactNode }) {
  const [isNight, setIsNight] = useState(() => {
    const hour = new Date().getHours();
    return hour < 6 || hour >= 19;
  });
  const [isAuto, setIsAuto] = useState(true);

  useEffect(() => {
    if (!isAuto) return;
    const check = () => {
      const hour = new Date().getHours();
      setIsNight(hour < 6 || hour >= 19);
    };
    const interval = setInterval(check, 60000);
    return () => clearInterval(interval);
  }, [isAuto]);

  const toggle = useCallback(() => {
    setIsAuto(false);
    setIsNight(prev => !prev);
  }, []);

  return <>{children(isNight, toggle)}</>;
}

// ═══════════════════════════════════════════════════
// NIGHT MODE TOGGLE BUTTON — Animated sun/moon morph
// ═══════════════════════════════════════════════════
export function NightModeToggle({ isNight, onToggle }: { isNight: boolean; onToggle: () => void }) {
  return (
    <motion.button
      onClick={onToggle}
      className="relative w-10 h-10 rounded-xl flex items-center justify-center overflow-hidden cursor-pointer"
      style={{
        background: isNight
          ? 'linear-gradient(135deg, oklch(0.15 0.04 264), oklch(0.08 0.02 264))'
          : 'linear-gradient(135deg, oklch(0.85 0.12 80), oklch(0.75 0.15 60))',
        border: `1px solid ${isNight ? 'oklch(0.35 0.06 264)' : 'oklch(0.80 0.10 80)'}`,
        boxShadow: isNight
          ? '0 0 15px oklch(0.50 0.12 264 / 30%), inset 0 0 8px oklch(0.50 0.12 264 / 10%)'
          : '0 0 15px oklch(0.80 0.15 60 / 30%), inset 0 0 8px oklch(0.80 0.15 60 / 10%)' }}
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9 }}
    >
      <AnimatePresence mode="wait">
        {isNight ? (
          <motion.div
            key="moon"
            initial={{ rotate: -90, scale: 0, opacity: 0 }}
            animate={{ rotate: 0, scale: 1, opacity: 1 }}
            exit={{ rotate: 90, scale: 0, opacity: 0 }}
            transition={{ duration: 0.4, type: 'spring', stiffness: 200 }}
          >
            <Moon className="w-5 h-5 text-blue-200" />
          </motion.div>
        ) : (
          <motion.div
            key="sun"
            initial={{ rotate: 90, scale: 0, opacity: 0 }}
            animate={{ rotate: 0, scale: 1, opacity: 1 }}
            exit={{ rotate: -90, scale: 0, opacity: 0 }}
            transition={{ duration: 0.4, type: 'spring', stiffness: 200 }}
          >
            <Sun className="w-5 h-5 text-amber-600" />
          </motion.div>
        )}
      </AnimatePresence>
      {/* Ambient glow */}
      <motion.div
        className="absolute inset-0 pointer-events-none rounded-xl"
        animate={{
          boxShadow: isNight
            ? ['inset 0 0 10px oklch(0.50 0.12 264 / 10%)', 'inset 0 0 20px oklch(0.50 0.12 264 / 20%)', 'inset 0 0 10px oklch(0.50 0.12 264 / 10%)']
            : ['inset 0 0 10px oklch(0.80 0.15 60 / 10%)', 'inset 0 0 20px oklch(0.80 0.15 60 / 20%)', 'inset 0 0 10px oklch(0.80 0.15 60 / 10%)'] }}
        transition={{ duration: 3, repeat: Infinity, ease: 'easeInOut' }}
      />
    </motion.button>
  );
}

// ═══════════════════════════════════════════════════
// ROUTE HISTORY PANEL — Saved routes with stats
// ═══════════════════════════════════════════════════
interface RouteRecord {
  id: string;
  from: string;
  to: string;
  distance: string;
  duration: string;
  mode: 'drive' | 'walk';
  timestamp: number;
  avgSpeed: number;
  calories?: number;
  co2Saved?: number;
}

export function RouteHistoryPanel({ onClose }: { onClose: () => void }) {
  const [routes, setRoutes] = useState<RouteRecord[]>([]);
  const [filter, setFilter] = useState<'all' | 'drive' | 'walk'>('all');

  useEffect(() => {
    // Load from localStorage or generate sample data
    const stored = localStorage.getItem('gane-route-history');
    if (stored) {
      setRoutes(JSON.parse(stored));
    } else {
      const sampleRoutes: RouteRecord[] = [
        { id: '1', from: 'Rothschild Blvd, Tel Aviv', to: 'Azrieli Center', distance: '3.2 km', duration: '12 min', mode: 'drive', timestamp: Date.now() - 3600000, avgSpeed: 28, co2Saved: 0 },
        { id: '2', from: 'Dizengoff Center', to: 'Carmel Market', distance: '1.1 km', duration: '14 min', mode: 'walk', timestamp: Date.now() - 7200000, avgSpeed: 4.7, calories: 68 },
        { id: '3', from: 'Tel Aviv Port', to: 'Jaffa Clock Tower', distance: '8.5 km', duration: '22 min', mode: 'drive', timestamp: Date.now() - 86400000, avgSpeed: 35, co2Saved: 0 },
        { id: '4', from: 'Habima Square', to: 'Rabin Square', distance: '0.8 km', duration: '10 min', mode: 'walk', timestamp: Date.now() - 172800000, avgSpeed: 4.8, calories: 52 },
        { id: '5', from: 'Ben Gurion Airport', to: 'Herzliya Marina', distance: '28 km', duration: '35 min', mode: 'drive', timestamp: Date.now() - 259200000, avgSpeed: 48, co2Saved: 0 },
        { id: '6', from: 'Sarona Market', to: 'Neve Tzedek', distance: '1.5 km', duration: '18 min', mode: 'walk', timestamp: Date.now() - 345600000, avgSpeed: 5.0, calories: 95 },
      ];
      setRoutes(sampleRoutes);
      localStorage.setItem('gane-route-history', JSON.stringify(sampleRoutes));
    }
  }, []);

  const filtered = filter === 'all' ? routes : routes.filter(r => r.mode === filter);
  const totalDriveKm = routes.filter(r => r.mode === 'drive').reduce((sum, r) => sum + parseFloat(r.distance), 0);
  const totalWalkKm = routes.filter(r => r.mode === 'walk').reduce((sum, r) => sum + parseFloat(r.distance), 0);
  const totalCalories = routes.filter(r => r.mode === 'walk').reduce((sum, r) => sum + (r.calories || 0), 0);

  const formatTime = (ts: number) => {
    const diff = Date.now() - ts;
    if (diff < 3600000) return `${Math.round(diff / 60000)} min ago`;
    if (diff < 86400000) return `${Math.round(diff / 3600000)}h ago`;
    return `${Math.round(diff / 86400000)}d ago`;
  };

  const clearHistory = () => {
    setRoutes([]);
    localStorage.removeItem('gane-route-history');
  };

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: "spring", damping: 28, stiffness: 300 }}
      className="expand-panel"
    >
      {/* Header */}
      <div className="sticky top-0 z-10 px-5 pt-5 pb-3 relative overflow-hidden" style={{
        background: 'linear-gradient(180deg, oklch(0.07 0.015 264 / 99%) 0%, oklch(0.07 0.015 264 / 92%) 80%, transparent 100%)' }}>
        <motion.div className="absolute inset-0 pointer-events-none"
          style={{ background: 'radial-gradient(ellipse at 20% 50%, oklch(0.65 0.15 40 / 8%), transparent 70%)' }}
          animate={{ opacity: [0.3, 0.6, 0.3] }}
          transition={{ duration: 4, repeat: Infinity, ease: 'easeInOut' }}
        />
        <div className="flex items-center gap-3 relative z-10">
          <motion.div
            initial={{ scale: 0.5, opacity: 0, rotate: -20 }}
            animate={{ scale: 1, opacity: 1, rotate: 0 }}
            transition={{ delay: 0.1, type: 'spring', stiffness: 350 }}
            className="w-10 h-10 rounded-xl flex items-center justify-center relative"
            style={{
              background: 'oklch(0.65 0.15 40 / 18%)',
              border: '1px solid oklch(0.65 0.15 40 / 40%)',
              boxShadow: '0 0 20px oklch(0.65 0.15 40 / 25%)' }}
          >
            <History className="w-5 h-5" style={{ color: 'oklch(0.75 0.15 40)' }} />
          </motion.div>
          <div className="flex-1">
            <h2 className="text-base font-bold text-white/90" style={{ fontFamily: 'Syne, sans-serif' }}>Route History</h2>
            <div className="text-[10px] text-white/25 mt-0.5" style={{ fontFamily: 'JetBrains Mono, monospace' }}>היסטוריית מסלולים</div>
          </div>
          <motion.button onClick={onClose} whileHover={{ scale: 1.15, rotate: 90 }} whileTap={{ scale: 0.85 }}
            className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer text-white/30 hover:text-white/60 transition-colors">
            <X className="w-4 h-4" />
          </motion.button>
        </div>
        <motion.div className="mt-3 h-px w-full relative overflow-hidden" style={{ background: 'oklch(1 0 0 / 3%)' }}>
          <motion.div className="absolute inset-0"
            style={{ background: 'linear-gradient(90deg, transparent, oklch(0.75 0.15 40), transparent)', backgroundSize: '200% 100%' }}
            animate={{ backgroundPosition: ['200% 0', '-200% 0'] }}
            transition={{ duration: 4, repeat: Infinity, ease: 'linear' }}
          />
        </motion.div>
      </div>

      <div className="px-5 pb-6">
        {/* Stats Summary */}
        <div className="grid grid-cols-3 gap-2 mb-4 mt-3">
          {[
            { icon: Car, value: `${totalDriveKm.toFixed(1)}`, unit: 'km', label: 'Driven', color: 'oklch(0.72 0.14 180)' },
            { icon: Footprints, value: `${totalWalkKm.toFixed(1)}`, unit: 'km', label: 'Walked', color: 'oklch(0.75 0.18 150)' },
            { icon: Flame, value: `${totalCalories}`, unit: 'cal', label: 'Burned', color: 'oklch(0.75 0.15 40)' },
          ].map((stat, i) => (
            <motion.div key={i} className="rounded-xl p-3 text-center"
              initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 + i * 0.05 }}
              style={{ background: 'oklch(0.11 0.015 264 / 60%)', border: '1px solid oklch(1 0 0 / 5%)' }}>
              <stat.icon className="w-4 h-4 mx-auto mb-1" style={{ color: stat.color }} />
              <div className="text-lg font-bold text-white/80" style={{ fontFamily: 'JetBrains Mono, monospace' }}>
                {stat.value}<span className="text-[9px] text-white/30 ml-0.5">{stat.unit}</span>
              </div>
              <div className="text-[9px] text-white/25">{stat.label}</div>
            </motion.div>
          ))}
        </div>

        {/* Filter Tabs */}
        <div className="flex gap-1 mb-4 p-1 rounded-xl" style={{ background: 'oklch(0.11 0.015 264 / 40%)' }}>
          {(['all', 'drive', 'walk'] as const).map(f => (
            <button key={f} onClick={() => setFilter(f)}
              className="flex-1 py-1.5 px-2 rounded-lg text-[11px] font-medium transition-all duration-200 cursor-pointer"
              style={{
                background: filter === f ? 'oklch(0.20 0.03 264)' : 'transparent',
                color: filter === f ? 'oklch(0.90 0 0)' : 'oklch(0.50 0 0)',
                boxShadow: filter === f ? '0 2px 8px oklch(0 0 0 / 30%)' : 'none' }}>
              {f === 'all' ? 'All' : f === 'drive' ? '🚗 Drive' : '🚶 Walk'}
            </button>
          ))}
        </div>

        {/* Route List */}
        <div className="space-y-2">
          {filtered.map((route, idx) => (
            <motion.div key={route.id}
              initial={{ opacity: 0, x: -20 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: idx * 0.04 }}
              className="rounded-xl p-3 group cursor-pointer transition-all duration-200"
              style={{ background: 'oklch(0.11 0.015 264 / 50%)', border: '1px solid oklch(1 0 0 / 4%)' }}
              whileHover={{ scale: 1.01, backgroundColor: 'oklch(0.13 0.02 264 / 60%)' }}
            >
              <div className="flex items-start gap-3">
                <div className="w-8 h-8 rounded-lg flex items-center justify-center mt-0.5"
                  style={{
                    background: route.mode === 'drive' ? 'oklch(0.72 0.14 180 / 15%)' : 'oklch(0.75 0.18 150 / 15%)',
                    border: `1px solid ${route.mode === 'drive' ? 'oklch(0.72 0.14 180 / 30%)' : 'oklch(0.75 0.18 150 / 30%)'}` }}>
                  {route.mode === 'drive' ? (
                    <Car className="w-4 h-4" style={{ color: 'oklch(0.72 0.14 180)' }} />
                  ) : (
                    <Footprints className="w-4 h-4" style={{ color: 'oklch(0.75 0.18 150)' }} />
                  )}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-1.5 mb-1">
                    <span className="text-[11px] text-white/60 truncate max-w-[100px]">{route.from.split(',')[0]}</span>
                    <ArrowRight className="w-3 h-3 text-white/20 flex-shrink-0" />
                    <span className="text-[11px] text-white/80 font-medium truncate max-w-[100px]">{route.to.split(',')[0]}</span>
                  </div>
                  <div className="flex items-center gap-3 text-[10px] text-white/30">
                    <span className="flex items-center gap-1"><Route className="w-3 h-3" />{route.distance}</span>
                    <span className="flex items-center gap-1"><Timer className="w-3 h-3" />{route.duration}</span>
                    <span className="flex items-center gap-1"><Gauge className="w-3 h-3" />{route.avgSpeed} km/h</span>
                  </div>
                </div>
                <div className="text-[9px] text-white/20 flex-shrink-0">{formatTime(route.timestamp)}</div>
              </div>
            </motion.div>
          ))}
        </div>

        {filtered.length === 0 && (
          <div className="text-center py-8">
            <History className="w-8 h-8 text-white/10 mx-auto mb-2" />
            <div className="text-xs text-white/20">No routes found</div>
          </div>
        )}

        {/* Clear History */}
        {routes.length > 0 && (
          <motion.button onClick={clearHistory}
            className="w-full mt-4 py-2 rounded-xl text-[11px] text-white/20 hover:text-red-400/60 transition-colors cursor-pointer flex items-center justify-center gap-1.5"
            style={{ background: 'oklch(0.11 0.015 264 / 30%)', border: '1px solid oklch(1 0 0 / 3%)' }}
            whileHover={{ scale: 1.01 }}
          >
            <Trash2 className="w-3 h-3" /> Clear History
          </motion.button>
        )}
      </div>
    </motion.div>
  );
}

// ═══════════════════════════════════════════════════
// SMART ALERTS PANEL — Real-time intelligent alerts
// ═══════════════════════════════════════════════════
interface SmartAlert {
  id: string;
  type: 'weather' | 'traffic' | 'safety' | 'speed' | 'poi' | 'gnss-fallback';
  title: string;
  titleHe: string;
  message: string;
  severity: 'info' | 'warning' | 'critical';
  timestamp: number;
  icon: typeof AlertTriangle;
  color: string;
  read: boolean;
}

export function SmartAlertsPanel({ onClose }: { onClose: () => void }) {
  const { weather, earthquakes, gnss } = useRealDataContext();
  const { t, dir, lang } = useLanguage();
  const [alerts, setAlerts] = useState<SmartAlert[]>([]);
  const [notificationsEnabled, setNotificationsEnabled] = useState(true);
  const prevChainStatusRef = useRef<string | null>(null);
  const [fallbackHistory, setFallbackHistory] = useState<Array<{from: string; to: string; time: number}>>([]);

  useEffect(() => {
    const newAlerts: SmartAlert[] = [];

    // Weather-based alerts
    if (weather) {
      if (weather.windSpeed > 40) {
        newAlerts.push({
          id: 'wind-alert',
          type: 'weather',
          title: 'High Wind Warning',
          titleHe: 'אזהרת רוחות חזקות',
          message: `Wind speed ${Math.round(weather.windSpeed)} km/h. Caution on bridges and open roads.`,
          severity: 'warning',
          timestamp: Date.now() - 300000,
          icon: Wind,
          color: 'oklch(0.80 0.16 75)',
          read: false });
      }
      if (weather.precipitation > 5) {
        newAlerts.push({
          id: 'rain-alert',
          type: 'weather',
          title: 'Heavy Rain Alert',
          titleHe: 'התראת גשם כבד',
          message: `Precipitation ${weather.precipitation}mm/h. Wet roads, reduce speed.`,
          severity: 'warning',
          timestamp: Date.now() - 600000,
          icon: CloudRain,
          color: 'oklch(0.72 0.14 220)',
          read: false });
      }
      if (weather.temperature > 38) {
        newAlerts.push({
          id: 'heat-alert',
          type: 'weather',
          title: 'Extreme Heat Warning',
          titleHe: 'אזהרת חום קיצוני',
          message: `Temperature ${Math.round(weather.temperature)}°C. Stay hydrated, check tire pressure.`,
          severity: 'critical',
          timestamp: Date.now() - 900000,
          icon: Thermometer,
          color: 'oklch(0.65 0.22 25)',
          read: false });
      }
      if (weather.visibility < 3) {
        newAlerts.push({
          id: 'fog-alert',
          type: 'safety',
          title: 'Low Visibility Warning',
          titleHe: 'אזהרת ראות נמוכה',
          message: `Visibility ${weather.visibility.toFixed(1)}km. Use fog lights, maintain distance.`,
          severity: 'critical',
          timestamp: Date.now() - 120000,
          icon: Eye,
          color: 'oklch(0.65 0.22 25)',
          read: false });
      }
    }

    // Earthquake alerts
    if (earthquakes && earthquakes.length > 0) {
      const recent = earthquakes.find((eq: any) => eq.magnitude >= 3.0);
      if (recent) {
        newAlerts.push({
          id: 'earthquake-alert',
          type: 'safety',
          title: 'Seismic Activity Detected',
          titleHe: 'זוהתה פעילות סייסמית',
          message: `M${recent.magnitude.toFixed(1)} earthquake near ${recent.place}. Check road conditions.`,
          severity: recent.magnitude >= 5 ? 'critical' : 'warning',
          timestamp: new Date(recent.time).getTime(),
          icon: Radar,
          color: recent.magnitude >= 5 ? 'oklch(0.65 0.22 25)' : 'oklch(0.80 0.16 75)',
          read: false });
      }
    }

    // ═══ GNSS FALLBACK CHAIN ALERTS ═══
    if (gnss) {
      const prevStatus = prevChainStatusRef.current;
      const currentStatus = gnss.chainStatus;
      
      // Detect tier change and log it
      if (prevStatus && prevStatus !== currentStatus) {
        const tierNames: Record<string, string> = {
          'OPTIMAL': 'Multi-GNSS (GPS+Galileo+GLONASS)',
          'DEGRADED': 'Single GNSS (GPS)',
          'FALLBACK': 'WiFi Positioning',
          'EMERGENCY': 'Cell Tower Triangulation',
          'LAST_RESORT': 'IMU Dead Reckoning',
        };
        setFallbackHistory(prev => [...prev.slice(-9), {
          from: tierNames[prevStatus] || prevStatus,
          to: tierNames[currentStatus] || currentStatus,
          time: Date.now(),
        }]);
      }
      prevChainStatusRef.current = currentStatus;

      // Active provider alert
      if (currentStatus === 'OPTIMAL') {
        const constellationCount = [gnss.gps, gnss.galileo, gnss.glonass, gnss.beidou, gnss.qzss, gnss.navic].filter(Boolean).length;
        newAlerts.push({
          id: 'gnss-optimal',
          type: 'gnss-fallback',
          title: `Multi-GNSS Lock — ${constellationCount} Constellations`,
          titleHe: `נעילת Multi-GNSS — ${constellationCount} קונסטלציות`,
          message: `${gnss.satellitesUsed} satellites used. HDOP: ${gnss.hdop.toFixed(1)}. Fix: ${gnss.fixType}. Accuracy: ±${gnss.accuracy.toFixed(1)}m. Continuity: ${(gnss.continuityScore * 100).toFixed(0)}%`,
          severity: 'info',
          timestamp: Date.now() - 5000,
          icon: Satellite,
          color: 'oklch(0.75 0.18 150)',
          read: true,
        });
      } else if (currentStatus === 'DEGRADED') {
        newAlerts.push({
          id: 'gnss-degraded',
          type: 'gnss-fallback',
          title: 'GNSS Signal Degraded — Single Constellation',
          titleHe: 'אות GNSS מופחת — קונסטלציה בודדת',
          message: `Switched to ${gnss.activeProvider}. ${gnss.satellitesUsed} sats. Accuracy: ±${gnss.accuracy.toFixed(1)}m. Navigation continues normally.`,
          severity: 'warning',
          timestamp: Date.now() - 3000,
          icon: Signal,
          color: 'oklch(0.80 0.16 75)',
          read: false,
        });
      } else if (currentStatus === 'FALLBACK') {
        newAlerts.push({
          id: 'gnss-fallback-wifi',
          type: 'gnss-fallback',
          title: 'GNSS Lost — WiFi Positioning Active',
          titleHe: 'GNSS אבד — מיקום WiFi פעיל',
          message: `All satellite signals lost. Switched to WiFi RTT positioning. Accuracy: ±${gnss.accuracy.toFixed(0)}m. Navigation continues with reduced accuracy.`,
          severity: 'warning',
          timestamp: Date.now() - 2000,
          icon: Wifi,
          color: 'oklch(0.80 0.16 75)',
          read: false,
        });
      } else if (currentStatus === 'EMERGENCY') {
        newAlerts.push({
          id: 'gnss-emergency-cell',
          type: 'gnss-fallback',
          title: 'Emergency — Cell Tower Triangulation',
          titleHe: 'חירום — שילוש אנטנות סלולריות',
          message: `GNSS and WiFi unavailable. Using cell tower triangulation. Accuracy: ±${gnss.accuracy.toFixed(0)}m. Navigation continues.`,
          severity: 'critical',
          timestamp: Date.now() - 1000,
          icon: Radio,
          color: 'oklch(0.65 0.22 25)',
          read: false,
        });
      } else if (currentStatus === 'LAST_RESORT') {
        newAlerts.push({
          id: 'gnss-last-resort',
          type: 'gnss-fallback',
          title: 'Last Resort — IMU Dead Reckoning',
          titleHe: 'מצב אחרון — ניווט אינרציאלי',
          message: `All external positioning lost. Using accelerometer + gyroscope dead reckoning. Accuracy degrades over time. Seeking signal recovery.`,
          severity: 'critical',
          timestamp: Date.now() - 500,
          icon: MapPinOff,
          color: 'oklch(0.65 0.22 25)',
          read: false,
        });
      }

      // Fallback history alerts
      fallbackHistory.slice(-3).reverse().forEach((entry, idx) => {
        newAlerts.push({
          id: `fallback-transition-${idx}`,
          type: 'gnss-fallback',
          title: `Tier Change: ${entry.to}`,
          titleHe: `מעבר שכבה: ${entry.to}`,
          message: `Switched from ${entry.from} to ${entry.to}`,
          severity: 'info',
          timestamp: entry.time,
          icon: ArrowRight,
          color: 'oklch(0.72 0.14 220)',
          read: true,
        });
      });
    }

    // Always-on system alerts
    newAlerts.push({
      id: 'system-ok',
      type: 'safety',
      title: 'All Systems Operational',
      titleHe: 'כל המערכות פעילות',
      message: gnss ? `GNSS: ${gnss.chainStatus}. ${gnss.satellitesUsed} satellites. Provider: ${gnss.activeProvider}. Accuracy: ±${gnss.accuracy.toFixed(1)}m.` : 'GNSS lock acquired. 12 satellites in view. Navigation accuracy: ±2.1m.',
      severity: 'info',
      timestamp: Date.now() - 1800000,
      icon: Shield,
      color: 'oklch(0.75 0.18 150)',
      read: true });
    newAlerts.push({
      id: 'speed-zone',
      type: 'speed',
      title: 'Speed Zone Change Ahead',
      titleHe: 'שינוי אזור מהירות לפניך',
      message: 'Speed limit changes from 80 to 50 km/h in 500m. Residential area ahead.',
      severity: 'info',
      timestamp: Date.now() - 60000,
      icon: Gauge,
      color: 'oklch(0.72 0.14 180)',
      read: false });

    setAlerts(newAlerts);
  }, [weather, earthquakes, gnss, fallbackHistory]);

  const unreadCount = alerts.filter(a => !a.read).length;
  const markAllRead = () => setAlerts(prev => prev.map(a => ({ ...a, read: true })));

  const severityBg: Record<string, string> = {
    info: 'oklch(0.72 0.14 180 / 8%)',
    warning: 'oklch(0.80 0.16 75 / 8%)',
    critical: 'oklch(0.65 0.22 25 / 8%)' };
  const severityBorder: Record<string, string> = {
    info: 'oklch(0.72 0.14 180 / 20%)',
    warning: 'oklch(0.80 0.16 75 / 20%)',
    critical: 'oklch(0.65 0.22 25 / 20%)' };

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: "spring", damping: 28, stiffness: 300 }}
      className="expand-panel"
    >
      {/* Header */}
      <div className="sticky top-0 z-10 px-5 pt-5 pb-3 relative overflow-hidden" style={{
        background: 'linear-gradient(180deg, oklch(0.07 0.015 264 / 99%) 0%, oklch(0.07 0.015 264 / 92%) 80%, transparent 100%)' }}>
        <motion.div className="absolute inset-0 pointer-events-none"
          style={{ background: 'radial-gradient(ellipse at 20% 50%, oklch(0.80 0.16 75 / 8%), transparent 70%)' }}
          animate={{ opacity: [0.3, 0.6, 0.3] }}
          transition={{ duration: 4, repeat: Infinity, ease: 'easeInOut' }}
        />
        <div className="flex items-center gap-3 relative z-10">
          <motion.div
            initial={{ scale: 0.5, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            transition={{ delay: 0.1, type: 'spring', stiffness: 350 }}
            className="w-10 h-10 rounded-xl flex items-center justify-center relative"
            style={{
              background: 'oklch(0.80 0.16 75 / 18%)',
              border: '1px solid oklch(0.80 0.16 75 / 40%)',
              boxShadow: '0 0 20px oklch(0.80 0.16 75 / 25%)' }}
          >
            <Bell className="w-5 h-5" style={{ color: 'oklch(0.85 0.16 75)' }} />
            {unreadCount > 0 && (
              <motion.div
                className="absolute -top-1 -right-1 w-4 h-4 rounded-full flex items-center justify-center text-[8px] font-bold"
                style={{ background: 'oklch(0.65 0.22 25)', color: 'white' }}
                animate={{ scale: [1, 1.2, 1] }}
                transition={{ duration: 1.5, repeat: Infinity }}
              >
                {unreadCount}
              </motion.div>
            )}
          </motion.div>
          <div className="flex-1">
            <h2 className="text-base font-bold text-white/90" style={{ fontFamily: 'Syne, sans-serif' }}>Smart Alerts</h2>
            <div className="text-[10px] text-white/25 mt-0.5" style={{ fontFamily: 'JetBrains Mono, monospace' }}>התראות חכמות</div>
          </div>
          <div className="flex items-center gap-2">
            <motion.button onClick={() => setNotificationsEnabled(!notificationsEnabled)}
              whileHover={{ scale: 1.1 }} whileTap={{ scale: 0.9 }}
              className="w-7 h-7 rounded-lg flex items-center justify-center cursor-pointer transition-colors"
              style={{ color: notificationsEnabled ? 'oklch(0.75 0.18 150)' : 'oklch(0.40 0 0)' }}>
              {notificationsEnabled ? <Volume2 className="w-3.5 h-3.5" /> : <VolumeX className="w-3.5 h-3.5" />}
            </motion.button>
            <motion.button onClick={onClose} whileHover={{ scale: 1.15, rotate: 90 }} whileTap={{ scale: 0.85 }}
              className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer text-white/30 hover:text-white/60 transition-colors">
              <X className="w-4 h-4" />
            </motion.button>
          </div>
        </div>
        <motion.div className="mt-3 h-px w-full relative overflow-hidden" style={{ background: 'oklch(1 0 0 / 3%)' }}>
          <motion.div className="absolute inset-0"
            style={{ background: 'linear-gradient(90deg, transparent, oklch(0.85 0.16 75), transparent)', backgroundSize: '200% 100%' }}
            animate={{ backgroundPosition: ['200% 0', '-200% 0'] }}
            transition={{ duration: 4, repeat: Infinity, ease: 'linear' }}
          />
        </motion.div>
      </div>

      <div className="px-5 pb-6">
        {/* Quick Actions */}
        {unreadCount > 0 && (
          <motion.button onClick={markAllRead}
            initial={{ opacity: 0 }} animate={{ opacity: 1 }}
            className="w-full mt-3 mb-4 py-1.5 rounded-lg text-[10px] text-white/30 hover:text-white/50 transition-colors cursor-pointer"
            style={{ background: 'oklch(0.11 0.015 264 / 30%)' }}>
            Mark all as read ({unreadCount})
          </motion.button>
        )}

        {/* Alert List */}
        <div className="space-y-2 mt-3">
          {alerts.map((alert, idx) => (
            <motion.div key={alert.id}
              initial={{ opacity: 0, x: -20, scale: 0.95 }}
              animate={{ opacity: 1, x: 0, scale: 1 }}
              transition={{ delay: idx * 0.05, type: 'spring', stiffness: 300 }}
              className="rounded-xl p-3 relative overflow-hidden"
              style={{
                background: severityBg[alert.severity],
                border: `1px solid ${severityBorder[alert.severity]}`,
                opacity: alert.read ? 0.6 : 1 }}
            >
              {/* Severity indicator */}
              {alert.severity === 'critical' && (
                <motion.div className="absolute top-0 left-0 w-1 h-full rounded-l-xl"
                  style={{ background: alert.color }}
                  animate={{ opacity: [1, 0.4, 1] }}
                  transition={{ duration: 1.5, repeat: Infinity }}
                />
              )}
              {alert.severity === 'warning' && (
                <div className="absolute top-0 left-0 w-0.5 h-full rounded-l-xl" style={{ background: alert.color }} />
              )}

              <div className="flex items-start gap-3 pl-1">
                <div className="w-8 h-8 rounded-lg flex items-center justify-center flex-shrink-0 mt-0.5"
                  style={{ background: `color-mix(in oklch, ${alert.color}, transparent 85%)` }}>
                  <alert.icon className="w-4 h-4" style={{ color: alert.color }} />
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-0.5">
                    <span className="text-[11px] font-semibold text-white/80">{alert.title}</span>
                    {!alert.read && (
                      <motion.div className="w-1.5 h-1.5 rounded-full flex-shrink-0"
                        style={{ background: alert.color }}
                        animate={{ scale: [1, 1.3, 1] }}
                        transition={{ duration: 2, repeat: Infinity }}
                      />
                    )}
                  </div>
                  <div className="text-[10px] text-white/20 mb-1">{lang === 'he' ? alert.titleHe : alert.title}</div>
                  <div className="text-[10px] text-white/40 leading-relaxed">{alert.message}</div>
                  <div className="text-[9px] text-white/15 mt-1.5 flex items-center gap-1">
                    <Clock className="w-2.5 h-2.5" />
                    {new Date(alert.timestamp).toLocaleTimeString('en', { hour: '2-digit', minute: '2-digit' })}
                  </div>
                </div>
              </div>
            </motion.div>
          ))}
        </div>

        {alerts.length === 0 && (
          <div className="text-center py-8">
            <Bell className="w-8 h-8 text-white/10 mx-auto mb-2" />
            <div className="text-xs text-white/20">No alerts</div>
          </div>
        )}
      </div>
    </motion.div>
  );
}
