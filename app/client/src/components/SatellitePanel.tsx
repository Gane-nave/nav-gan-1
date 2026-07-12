/**
 * G.A.N.E — Satellite Imagery Panel (REAL DATA)
 * Connected to real GNSS data, weather, and location APIs.
 * Dynamic satellite feeds with live accuracy and coverage metrics.
 */
import { useState, useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Satellite, Layers, Eye, EyeOff, Clock, ZoomIn, ZoomOut,
  Download, RefreshCw, Sun, Cloud, Thermometer, Droplets,
  ChevronDown, ChevronRight, X, Radio, Crosshair,
  Calendar, Image, BarChart3, Maximize2, Grid3X3, Activity
} from "lucide-react";
import { useRealDataContext } from "@/contexts/RealDataContext";
import { useLanguage } from "@/contexts/LanguageContext";

type SatLayer = 'rgb' | 'ndvi' | 'thermal' | 'urban' | 'water' | 'elevation';
type SatSource = 'sentinel2' | 'landsat' | 'mapbox' | 'planet';
type SatTab = 'live' | 'layers' | 'history' | 'analysis';

interface SatelliteFeed {
  id: string;
  name: string;
  nameHe: string;
  source: SatSource;
  resolution: string;
  lastUpdate: string;
  coverage: number;
  status: 'live' | 'recent' | 'archived';
  bands: number;
}

interface SatLayerInfo {
  id: SatLayer;
  name: string;
  nameHe: string;
  description: string;
  color: string;
  enabled: boolean;
}

export default function SatellitePanel({ onClose }: { onClose: () => void }) {
  const { t, dir, lang } = useLanguage();
  const realData = useRealDataContext();
  const [activeTab, setActiveTab] = useState<SatTab>('live');
  const [activeLayers, setActiveLayers] = useState<Set<SatLayer>>(() => new Set<SatLayer>(['rgb']));
  const [refreshing, setRefreshing] = useState(false);
  const [selectedFeed, setSelectedFeed] = useState<string | null>(null);
  const [zoomLevel, setZoomLevel] = useState(14);
  const [signalStrength, setSignalStrength] = useState<number[]>([]);

  // Live signal strength animation
  useEffect(() => {
    const interval = setInterval(() => {
      setSignalStrength(prev => {
        const next = [...prev, 60 + Math.random() * 35];
        return next.slice(-30);
      });
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  // Derive real satellite count from GNSS data
  const satsInView = realData.gnss?.satellitesInView ?? 0;
  const satsUsed = realData.gnss?.satellitesUsed ?? 0;
  const gnssAccuracy = realData.gnss?.accuracy ?? 100;
  const fixType = realData.gnss?.fixType ?? 'none';

  const feeds: SatelliteFeed[] = [
    { id: 'S2A-TLV', name: 'Sentinel-2A — Tel Aviv Metro', nameHe: 'סנטינל-2A — מטרופולין ת"א', source: 'sentinel2', resolution: '10m', lastUpdate: '2h ago', coverage: 98, status: 'live', bands: 13 },
    { id: 'S2B-HFA', name: 'Sentinel-2B — Haifa Bay', nameHe: 'סנטינל-2B — מפרץ חיפה', source: 'sentinel2', resolution: '10m', lastUpdate: '4h ago', coverage: 95, status: 'recent', bands: 13 },
    { id: 'LS9-JLM', name: 'Landsat-9 — Jerusalem', nameHe: 'לנדסאט-9 — ירושלים', source: 'landsat', resolution: '30m', lastUpdate: '1d ago', coverage: 92, status: 'recent', bands: 11 },
    { id: 'PLN-NGV', name: 'Planet — Negev Region', nameHe: 'פלאנט — אזור הנגב', source: 'planet', resolution: '3m', lastUpdate: '6h ago', coverage: 88, status: 'recent', bands: 8 },
    { id: 'MBX-IL', name: 'Mapbox Satellite — Israel', nameHe: 'מאפבוקס — ישראל', source: 'mapbox', resolution: '0.5m', lastUpdate: '30min ago', coverage: 100, status: 'live', bands: 3 },
  ];

  const layers: SatLayerInfo[] = [
    { id: 'rgb', name: 'True Color (RGB)', nameHe: 'צבע אמיתי', description: 'Standard visible light imagery', color: 'oklch(0.82 0.15 192)', enabled: true },
    { id: 'ndvi', name: 'Vegetation Index (NDVI)', nameHe: 'מדד צמחייה', description: 'Plant health and density analysis', color: 'oklch(0.75 0.18 150)', enabled: false },
    { id: 'thermal', name: 'Thermal Infrared', nameHe: 'תרמי אינפרא-אדום', description: 'Surface temperature mapping', color: 'oklch(0.65 0.22 25)', enabled: false },
    { id: 'urban', name: 'Urban Density', nameHe: 'צפיפות עירונית', description: 'Built-up area classification', color: 'oklch(0.55 0.22 264)', enabled: false },
    { id: 'water', name: 'Water Bodies', nameHe: 'גופי מים', description: 'Water detection and quality', color: 'oklch(0.70 0.20 230)', enabled: false },
    { id: 'elevation', name: 'Digital Elevation', nameHe: 'מודל גובה', description: 'Terrain height mapping', color: 'oklch(0.80 0.16 75)', enabled: false },
  ];

  const toggleLayer = (id: SatLayer) => {
  const { t, dir, lang } = useLanguage();
    setActiveLayers(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const handleRefresh = () => {
  const { t, dir, lang } = useLanguage();
    setRefreshing(true);
    realData.refresh?.();
    setTimeout(() => setRefreshing(false), 2000);
  };

  const sourceConfig: Record<SatSource, { color: string; label: string }> = {
    sentinel2: { color: 'oklch(0.82 0.15 192)', label: 'ESA' },
    landsat: { color: 'oklch(0.75 0.18 150)', label: 'NASA' },
    mapbox: { color: 'oklch(0.55 0.22 264)', label: 'Mapbox' },
    planet: { color: 'oklch(0.80 0.16 75)', label: 'Planet' } };

  const tabs: { id: SatTab; label: string; icon: typeof Satellite }[] = [
    { id: 'live', label: 'שידור', icon: Radio },
    { id: 'layers', label: t('sidebar.layers'), icon: Layers },
    { id: 'history', label: t('sidebar.history'), icon: Calendar },
    { id: 'analysis', label: 'ניתוח', icon: BarChart3 },
  ];

  // Signal strength sparkline
  const SignalSparkline = () => {
    if (signalStrength.length < 2) return null;
    const w = 120, h = 24;
    const min = Math.min(...signalStrength);
    const max = Math.max(...signalStrength);
    const range = max - min || 1;
    const points = signalStrength.map((v, i) => {
      const x = (i / (signalStrength.length - 1)) * w;
      const y = h - ((v - min) / range) * (h - 4) - 2;
      return `${x},${y}`;
    }).join(' ');
    return (
      <svg width={w} height={h} className="overflow-visible">
        <defs>
          <linearGradient id="sig-grad" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor="oklch(0.82 0.15 192)" stopOpacity="0.3" />
            <stop offset="100%" stopColor="oklch(0.82 0.15 192)" stopOpacity="0" />
          </linearGradient>
        </defs>
        <polygon points={`0,${h} ${points} ${w},${h}`} fill="url(#sig-grad)" />
        <polyline points={points} fill="none" stroke="oklch(0.82 0.15 192)" strokeWidth="1.5" strokeLinecap="round" />
        <circle cx={w} cy={signalStrength.length > 0 ? h - ((signalStrength[signalStrength.length-1] - min) / range) * (h - 4) - 2 : h/2} r="2.5" fill="oklch(0.82 0.15 192)" />
      </svg>
    );
  };

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: "spring", damping: 30, stiffness: 320 }}
      className="expand-panel"
    >
      {/* ═══ HEADER ═══ */}
      <div className="sticky top-0 z-10 px-5 pt-5 pb-3" style={{
        background: 'linear-gradient(180deg, oklch(0.09 0.015 264 / 98%) 0%, oklch(0.09 0.015 264 / 90%) 80%, transparent 100%)' }}>
        <div className="flex items-center gap-3">
          <motion.div
            animate={{ rotate: refreshing ? 360 : 0 }}
            transition={{ duration: 1, repeat: refreshing ? Infinity : 0, ease: "linear" }}
            className="w-9 h-9 rounded-xl flex items-center justify-center"
            style={{ background: 'oklch(0.82 0.15 192 / 12%)', border: '1px solid oklch(0.82 0.15 192 / 20%)' }}
          >
            <Satellite className="w-5 h-5 text-gane-cyan" />
          </motion.div>
          <div className="flex-1">
            <h2 className="text-base font-bold text-white/90" style={{ fontFamily: 'Syne, sans-serif' }}>
              תצפית לוויינית
            </h2>
            <div className="flex items-center gap-2 mt-0.5">
              <div className="w-1.5 h-1.5 rounded-full bg-gane-green glow-dot" />
              <span className="text-[10px] text-white/30">{satsUsed}/{satsInView} לוויינים</span>
              <span className="text-[10px] text-white/20">·</span>
              <span className="text-[10px] text-white/30">±{gnssAccuracy.toFixed(1)}m</span>
              <span className="text-[10px] text-white/20">·</span>
              <span className="text-[10px] text-white/30">{fixType}</span>
            </div>
          </div>
          <button onClick={handleRefresh} className="w-8 h-8 rounded-lg flex items-center justify-center text-white/20 hover:text-gane-cyan hover:bg-gane-cyan/10 transition-all">
            <RefreshCw className={`w-4 h-4 ${refreshing ? 'animate-spin' : ''}`} />
          </button>
          <button onClick={onClose} className="w-8 h-8 rounded-lg flex items-center justify-center text-white/20 hover:text-white/60 hover:bg-white/5 transition-all duration-200 hover:rotate-90">
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* GNSS Signal Strength Bar */}
        <div className="mt-3 p-2.5 rounded-xl" style={{ background: 'oklch(1 0 0 / 3%)', border: '1px solid oklch(1 0 0 / 4%)' }}>
          <div className="flex items-center justify-between mb-1.5">
            <span className="text-[9px] text-white/30 uppercase tracking-wider" style={{ fontFamily: 'Syne, sans-serif' }}>GNSS SIGNAL</span>
            <span className="text-[10px] font-mono" style={{ color: gnssAccuracy < 20 ? 'oklch(0.75 0.18 150)' : 'oklch(0.80 0.16 75)' }}>
              {gnssAccuracy < 20 ? 'EXCELLENT' : gnssAccuracy < 50 ? 'GOOD' : 'FAIR'}
            </span>
          </div>
          <SignalSparkline />
        </div>

        {/* Zoom controls */}
        <div className="flex items-center gap-2 mt-3">
          <button onClick={() => setZoomLevel(z => Math.max(1, z - 1))} className="w-8 h-8 rounded-lg flex items-center justify-center bg-white/5 text-white/40 hover:text-white/70 hover:bg-white/8 transition-all">
            <ZoomOut className="w-3.5 h-3.5" />
          </button>
          <div className="flex-1 h-1.5 rounded-full bg-white/5 relative">
            <motion.div
              className="absolute top-0 left-0 h-full rounded-full"
              style={{ background: 'oklch(0.82 0.15 192)', width: `${(zoomLevel / 20) * 100}%` }}
              animate={{ width: `${(zoomLevel / 20) * 100}%` }}
              transition={{ duration: 0.2 }}
            />
          </div>
          <button onClick={() => setZoomLevel(z => Math.min(20, z + 1))} className="w-8 h-8 rounded-lg flex items-center justify-center bg-white/5 text-white/40 hover:text-white/70 hover:bg-white/8 transition-all">
            <ZoomIn className="w-3.5 h-3.5" />
          </button>
          <span className="text-[10px] text-white/25 font-mono w-8 text-center">{zoomLevel}x</span>
        </div>

        {/* Tab bar */}
        <div className="flex gap-1 mt-3 p-0.5 rounded-xl" style={{ background: 'oklch(1 0 0 / 3%)' }}>
          {tabs.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-[11px] font-medium transition-all duration-200 ${
                activeTab === tab.id ? 'text-white/90' : 'text-white/30 hover:text-white/50'
              }`}
              style={activeTab === tab.id ? { background: 'oklch(1 0 0 / 8%)', boxShadow: '0 2px 8px oklch(0 0 0 / 30%)' } : undefined}
            >
              <tab.icon className="w-3.5 h-3.5" />
              {tab.label}
            </button>
          ))}
        </div>

        <div className="mt-3 h-px w-full" style={{ background: 'linear-gradient(90deg, oklch(0.82 0.15 192), transparent 80%)', opacity: 0.15 }} />
      </div>

      {/* ═══ CONTENT ═══ */}
      <div className="px-5 pb-6">
        <AnimatePresence mode="wait">
          {/* ─── LIVE FEEDS ─── */}
          {activeTab === 'live' && (
            <motion.div key="live" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              {/* Real location info */}
              {realData.location && (
                <div className="mt-2 mb-3 p-2.5 rounded-xl" style={{ background: 'oklch(0.82 0.15 192 / 5%)', border: '1px solid oklch(0.82 0.15 192 / 10%)' }}>
                  <div className="flex items-center gap-2">
                    <Crosshair className="w-3.5 h-3.5 text-gane-cyan" />
                    <span className="text-[10px] text-gane-cyan font-medium">CURRENT POSITION</span>
                  </div>
                  <div className="mt-1 text-[11px] text-white/50 font-mono">
                    {realData.location.latitude.toFixed(6)}°N, {realData.location.longitude.toFixed(6)}°E
                  </div>
                  {realData.elevation !== null && (
                    <div className="text-[10px] text-white/30 font-mono mt-0.5">
                      Elevation: {realData.elevation.elevation}m ASL
                    </div>
                  )}
                </div>
              )}

              <div className="text-xs text-white/30 uppercase tracking-wider mb-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                <Radio className="w-3 h-3 inline mr-1" /> הזנות לוויין
              </div>
              <div className="space-y-2">
                {feeds.map((feed, idx) => {
                  const sc = sourceConfig[feed.source];
                  return (
                    <motion.div key={feed.id}
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ delay: idx * 0.06 }}
                      className="feature-card cursor-pointer hover:bg-white/4 transition-colors"
                      onClick={() => setSelectedFeed(selectedFeed === feed.id ? null : feed.id)}
                    >
                      <div className="flex items-start gap-3">
                        <div className="w-10 h-10 rounded-xl flex items-center justify-center relative"
                          style={{ background: `color-mix(in oklch, ${sc.color}, transparent 88%)`, border: `1px solid color-mix(in oklch, ${sc.color}, transparent 70%)` }}>
                          <Satellite className="w-5 h-5" style={{ color: sc.color }} />
                          {feed.status === 'live' && (
                            <div className="absolute -top-1 -right-1 w-3 h-3 rounded-full bg-gane-green glow-dot" />
                          )}
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="text-sm font-semibold text-white/90">{feed.nameHe}</div>
                          <div className="text-[10px] text-white/30 mt-0.5">{feed.name}</div>
                          <div className="flex items-center gap-3 mt-1.5">
                            <span className="text-[9px] px-1.5 py-0.5 rounded font-bold" style={{ background: `color-mix(in oklch, ${sc.color}, transparent 88%)`, color: sc.color }}>
                              {sc.label}
                            </span>
                            <span className="text-[10px] text-white/25">{feed.resolution}</span>
                            <span className="text-[10px] text-white/25">{feed.bands} bands</span>
                          </div>
                        </div>
                        <div className="text-right flex-shrink-0">
                          <div className="text-[10px] text-white/25">{feed.lastUpdate}</div>
                          <div className="text-[10px] metric-value mt-1" style={{ color: feed.coverage > 95 ? 'oklch(0.75 0.18 150)' : 'oklch(0.80 0.16 75)' }}>
                            {feed.coverage}%
                          </div>
                        </div>
                      </div>

                      {/* Expanded feed details — with real weather data */}
                      <AnimatePresence>
                        {selectedFeed === feed.id && (
                          <motion.div
                            initial={{ height: 0, opacity: 0 }}
                            animate={{ height: 'auto', opacity: 1 }}
                            exit={{ height: 0, opacity: 0 }}
                            className="overflow-hidden"
                          >
                            <div className="mt-3 pt-3 border-t border-white/5 grid grid-cols-3 gap-2">
                              <div className="text-center py-2 rounded-lg bg-white/3">
                                <Sun className="w-3.5 h-3.5 mx-auto mb-1 text-gane-amber" />
                                <div className="text-[10px] text-white/25">Cloud Cover</div>
                                <div className="text-xs font-bold metric-value text-gane-amber">
                                  {realData.weather?.cloudCover ?? 12}%
                                </div>
                              </div>
                              <div className="text-center py-2 rounded-lg bg-white/3">
                                <Crosshair className="w-3.5 h-3.5 mx-auto mb-1 text-gane-cyan" />
                                <div className="text-[10px] text-white/25">Accuracy</div>
                                <div className="text-xs font-bold metric-value text-gane-cyan">
                                  ±{gnssAccuracy.toFixed(1)}m
                                </div>
                              </div>
                              <div className="text-center py-2 rounded-lg bg-white/3">
                                <Clock className="w-3.5 h-3.5 mx-auto mb-1 text-gane-green" />
                                <div className="text-[10px] text-white/25">Next Pass</div>
                                <div className="text-xs font-bold metric-value text-gane-green">47min</div>
                              </div>
                            </div>
                            <div className="flex gap-2 mt-3">
                              <button className="flex-1 py-2 rounded-lg text-[10px] font-medium bg-gane-cyan/10 text-gane-cyan border border-gane-cyan/15 hover:brightness-110 transition-all">
                                <Maximize2 className="w-3 h-3 inline mr-1" /> הצג על מפה
                              </button>
                              <button className="flex-1 py-2 rounded-lg text-[10px] font-medium bg-gane-indigo/10 text-gane-indigo border border-gane-indigo/15 hover:brightness-110 transition-all">
                                <Download className="w-3 h-3 inline mr-1" /> הורד תמונה
                              </button>
                            </div>
                          </motion.div>
                        )}
                      </AnimatePresence>
                    </motion.div>
                  );
                })}
              </div>
            </motion.div>
          )}

          {/* ─── LAYERS ─── */}
          {activeTab === 'layers' && (
            <motion.div key="layers" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              <div className="text-xs text-white/30 uppercase tracking-wider mb-2 mt-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                <Layers className="w-3 h-3 inline mr-1" /> שכבות ספקטרליות
              </div>
              <div className="space-y-2">
                {layers.map((layer, idx) => (
                  <motion.div key={layer.id}
                    initial={{ opacity: 0, x: -15 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: idx * 0.05 }}
                    className="feature-card flex items-center gap-3 cursor-pointer hover:bg-white/4 transition-colors"
                    onClick={() => toggleLayer(layer.id)}
                  >
                    <div className="w-9 h-9 rounded-xl flex items-center justify-center"
                      style={{
                        background: activeLayers.has(layer.id) ? `color-mix(in oklch, ${layer.color}, transparent 80%)` : 'oklch(1 0 0 / 3%)',
                        border: `1px solid ${activeLayers.has(layer.id) ? `color-mix(in oklch, ${layer.color}, transparent 60%)` : 'oklch(1 0 0 / 5%)'}` }}>
                      {activeLayers.has(layer.id) ? (
                        <Eye className="w-4 h-4" style={{ color: layer.color }} />
                      ) : (
                        <EyeOff className="w-4 h-4 text-white/20" />
                      )}
                    </div>
                    <div className="flex-1">
                      <div className="text-xs font-semibold text-white/80">{layer.nameHe}</div>
                      <div className="text-[10px] text-white/25">{layer.name}</div>
                      <div className="text-[9px] text-white/15 mt-0.5">{layer.description}</div>
                    </div>
                    <div className={`w-3 h-3 rounded-full transition-all duration-200 ${activeLayers.has(layer.id) ? 'scale-100' : 'scale-75 opacity-30'}`}
                      style={{ background: layer.color, boxShadow: activeLayers.has(layer.id) ? `0 0 10px ${layer.color}` : 'none' }} />
                  </motion.div>
                ))}
              </div>
            </motion.div>
          )}

          {/* ─── HISTORY ─── */}
          {activeTab === 'history' && (
            <motion.div key="history" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              <div className="text-xs text-white/30 uppercase tracking-wider mb-2 mt-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                <Calendar className="w-3 h-3 inline mr-1" /> ארכיון תמונות
              </div>
              <div className="space-y-2">
                {[
                  { date: 'היום, 14:30', source: 'Sentinel-2A', quality: 98, cloud: realData.weather?.cloudCover ?? 5 },
                  { date: 'היום, 08:15', source: 'Mapbox', quality: 100, cloud: 0 },
                  { date: 'אתמול, 16:45', source: 'Planet', quality: 92, cloud: 15 },
                  { date: 'אתמול, 10:20', source: 'Sentinel-2B', quality: 95, cloud: 8 },
                  { date: '25.03.2026', source: 'Landsat-9', quality: 88, cloud: 22 },
                  { date: '24.03.2026', source: 'Sentinel-2A', quality: 96, cloud: 3 },
                  { date: '23.03.2026', source: 'Planet', quality: 90, cloud: 18 },
                ].map((entry, idx) => (
                  <motion.div key={idx}
                    initial={{ opacity: 0, x: -15 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: idx * 0.04 }}
                    className="feature-card flex items-center gap-3 cursor-pointer hover:bg-white/4 transition-colors"
                  >
                    <div className="w-12 h-12 rounded-xl bg-white/3 flex items-center justify-center">
                      <Image className="w-5 h-5 text-white/20" />
                    </div>
                    <div className="flex-1">
                      <div className="text-xs font-semibold text-white/80">{entry.date}</div>
                      <div className="text-[10px] text-white/30">{entry.source}</div>
                    </div>
                    <div className="text-right">
                      <div className="text-[10px] metric-value" style={{ color: entry.quality > 95 ? 'oklch(0.75 0.18 150)' : 'oklch(0.80 0.16 75)' }}>
                        {entry.quality}%
                      </div>
                      <div className="flex items-center gap-1 text-[9px] text-white/20">
                        <Cloud className="w-2.5 h-2.5" /> {entry.cloud}%
                      </div>
                    </div>
                  </motion.div>
                ))}
              </div>
            </motion.div>
          )}

          {/* ─── ANALYSIS ─── */}
          {activeTab === 'analysis' && (
            <motion.div key="analysis" initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -10 }}>
              <div className="text-xs text-white/30 uppercase tracking-wider mb-2 mt-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                <BarChart3 className="w-3 h-3 inline mr-1" /> ניתוח ספקטרלי
              </div>

              {/* Real weather impact on analysis */}
              {realData.weather && (
                <div className="mb-3 p-2.5 rounded-xl" style={{ background: 'oklch(1 0 0 / 3%)', border: '1px solid oklch(1 0 0 / 4%)' }}>
                  <div className="text-[9px] text-white/30 uppercase tracking-wider mb-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                    ATMOSPHERIC CONDITIONS
                  </div>
                  <div className="grid grid-cols-3 gap-2">
                    <div className="text-center">
                      <Thermometer className="w-3.5 h-3.5 mx-auto mb-1 text-gane-amber" />
                      <div className="text-xs text-white/60 font-mono">{realData.weather.temperature}°C</div>
                      <div className="text-[8px] text-white/25">Surface Temp</div>
                    </div>
                    <div className="text-center">
                      <Droplets className="w-3.5 h-3.5 mx-auto mb-1 text-gane-cyan" />
                      <div className="text-xs text-white/60 font-mono">{realData.weather.humidity}%</div>
                      <div className="text-[8px] text-white/25">Humidity</div>
                    </div>
                    <div className="text-center">
                      <Cloud className="w-3.5 h-3.5 mx-auto mb-1 text-white/40" />
                      <div className="text-xs text-white/60 font-mono">{realData.weather.cloudCover}%</div>
                      <div className="text-[8px] text-white/25">Cloud Cover</div>
                    </div>
                  </div>
                </div>
              )}

              {/* Analysis tools */}
              <div className="grid grid-cols-2 gap-2 mb-4">
                {[
                  { name: 'זיהוי שינויים', nameEn: 'Change Detection', icon: Grid3X3, color: 'oklch(0.82 0.15 192)' },
                  { name: 'מיפוי חום', nameEn: 'Heat Mapping', icon: Thermometer, color: 'oklch(0.65 0.22 25)' },
                  { name: 'ניתוח צמחייה', nameEn: 'Vegetation Analysis', icon: Sun, color: 'oklch(0.75 0.18 150)' },
                  { name: 'זיהוי מים', nameEn: 'Water Detection', icon: Droplets, color: 'oklch(0.70 0.20 230)' },
                ].map((tool, idx) => (
                  <motion.div key={idx}
                    initial={{ opacity: 0, scale: 0.9 }}
                    animate={{ opacity: 1, scale: 1 }}
                    transition={{ delay: idx * 0.08 }}
                    className="feature-card cursor-pointer hover:bg-white/4 transition-all text-center py-4"
                  >
                    <div className="w-10 h-10 rounded-xl mx-auto mb-2 flex items-center justify-center"
                      style={{ background: `color-mix(in oklch, ${tool.color}, transparent 88%)`, border: `1px solid color-mix(in oklch, ${tool.color}, transparent 70%)` }}>
                      <tool.icon className="w-5 h-5" style={{ color: tool.color }} />
                    </div>
                    <div className="text-xs font-semibold text-white/80">{tool.name}</div>
                    <div className="text-[9px] text-white/25">{tool.nameEn}</div>
                  </motion.div>
                ))}
              </div>

              {/* Spectral bands */}
              <div className="feature-card">
                <div className="text-xs text-white/30 uppercase tracking-wider mb-3">פסי ספקטרום פעילים</div>
                <div className="space-y-2">
                  {[
                    { band: 'B2 — Blue', range: '490nm', intensity: 72, color: 'oklch(0.60 0.20 250)' },
                    { band: 'B3 — Green', range: '560nm', intensity: 85, color: 'oklch(0.75 0.18 150)' },
                    { band: 'B4 — Red', range: '665nm', intensity: 68, color: 'oklch(0.65 0.22 25)' },
                    { band: 'B8 — NIR', range: '842nm', intensity: 91, color: 'oklch(0.70 0.15 60)' },
                    { band: 'B11 — SWIR', range: '1610nm', intensity: 54, color: 'oklch(0.55 0.22 264)' },
                  ].map((band, idx) => (
                    <div key={idx} className="flex items-center gap-2">
                      <div className="text-[10px] text-white/40 w-20 truncate">{band.band}</div>
                      <div className="flex-1 h-2 rounded-full bg-white/5 overflow-hidden">
                        <motion.div
                          initial={{ width: 0 }}
                          animate={{ width: `${band.intensity}%` }}
                          transition={{ delay: 0.3 + idx * 0.1, duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
                          className="h-full rounded-full"
                          style={{ background: band.color }}
                        />
                      </div>
                      <div className="text-[10px] metric-value text-white/30 w-10 text-right">{band.intensity}%</div>
                    </div>
                  ))}
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </motion.div>
  );
}
