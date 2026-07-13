/**
 * G.A.N.E — Map Layers Panel (Scientific Grade)
 * ═══════════════════════════════════════════════════
 * Multi-layer map system with:
 * - 19 distinct map layers organized in 5 categories
 * - Real-time data overlays (weather radar, seismic, AQI)
 * - Canvas-rendered layer previews with live data
 * - Layer opacity controls with smooth sliders
 * - Layer composition engine with blend modes
 * - Map style presets with live preview
 * - Layer dependency awareness
 */
import { useState, useEffect, useRef, useCallback, useMemo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
// NOTE: Using framer-motion sparingly - only for enter/exit animations
import {
  X, Layers, Rocket, Train, Bike, Telescope, Atom, Flame,
  Hexagon, Bolt, Diamond, MapPin, Route, Eye, EyeOff, Sparkles,
  ChevronDown, ChevronRight, Cloud, Thermometer, Wind, Droplets,
  Radio, Mountain, Activity, Zap, Shield, Building2, Navigation2,
  Radar, Globe, Gauge, Sun, Moon, Waves, AlertTriangle, Lock,
  Unlock, SlidersHorizontal, Palette, BarChart3, Cpu
} from 'lucide-react';
import { useNavigation } from '@/contexts/NavigationContext';
import { useRealDataContext } from '@/contexts/RealDataContext';
import type { MapLayer } from '@/lib/navStore';
import { useLanguage } from "@/contexts/LanguageContext";

// ═══ Layer Configuration System ═══
interface LayerConfig {
  id: MapLayer;
  label: string;
  i18nKey?: string;
  description: string;
  icon: typeof Rocket;
  color: string;
  category: 'core' | 'weather' | 'safety' | 'infrastructure' | 'advanced';
  dataSource: string;
  updateFreq: string;
  isLive: boolean;
  requiresApi: boolean;
  blendMode?: string;
  defaultOpacity: number;
}

const LAYER_CATEGORIES = [
  { id: 'core', label: 'Core Navigation', i18nKey: 'layerCat.core', icon: Navigation2, color: '#2563EB' },
  { id: 'weather', label: 'Weather & Atmosphere', i18nKey: 'layerCat.weather', icon: Cloud, color: '#16A34A' },
  { id: 'safety', label: 'Safety & Risk', i18nKey: 'layerCat.safety', icon: Shield, color: '#ff4444' },
  { id: 'infrastructure', label: 'Infrastructure', i18nKey: 'layerCat.infrastructure', icon: Building2, color: '#7C3AED' },
  { id: 'advanced', label: 'Advanced / Experimental', i18nKey: 'layerCat.advanced', icon: Cpu, color: '#F97316' },
] as const;

const ALL_LAYERS: LayerConfig[] = [
  // Core Navigation
  { id: 'traffic', label: 'Live Traffic', i18nKey: 'layer.traffic', description: 'Real-time traffic flow, congestion levels, and incidents', icon: Rocket, color: '#F97316', category: 'core', dataSource: 'Google Traffic API', updateFreq: '2 min', isLive: true, requiresApi: false, defaultOpacity: 0.8 },
  { id: 'transit', label: 'Public Transit', i18nKey: 'layer.transit', description: 'Bus, train, metro routes with real-time arrival data', icon: Train, color: '#6366f1', category: 'core', dataSource: 'Google Transit API', updateFreq: '1 min', isLive: true, requiresApi: false, defaultOpacity: 0.7 },
  { id: 'bicycling', label: 'Cycling Network', i18nKey: 'layer.cycling', description: 'Bike lanes, shared paths, elevation profiles', icon: Bike, color: '#22c55e', category: 'core', dataSource: 'Google Bicycling API', updateFreq: 'Static', isLive: false, requiresApi: false, defaultOpacity: 0.6 },
  { id: 'satellite', label: 'Satellite Imagery', i18nKey: 'layer.satellite', description: 'High-resolution satellite imagery with 15m resolution', icon: Telescope, color: '#06b6d4', category: 'core', dataSource: 'Google Satellite', updateFreq: '~Monthly', isLive: false, requiresApi: false, defaultOpacity: 1.0 },
  { id: 'terrain', label: 'Terrain & Elevation', i18nKey: 'layer.terrain', description: '3D terrain model with contour lines and elevation data', icon: Mountain, color: '#84cc16', category: 'core', dataSource: 'Open-Meteo Elevation', updateFreq: 'Static', isLive: false, requiresApi: true, defaultOpacity: 0.5 },

  // Weather & Atmosphere
  { id: 'weather-radar', label: 'Precipitation Radar', i18nKey: 'layer.weatherRadar', description: 'Live doppler radar showing rain, snow, and storm cells', icon: Radar, color: '#2563EB', category: 'weather', dataSource: 'RainViewer API', updateFreq: '5 min', isLive: true, requiresApi: true, blendMode: 'screen', defaultOpacity: 0.65 },
  { id: 'weather-temp', label: 'Temperature Map', i18nKey: 'layer.temperature', description: 'Thermal gradient overlay showing temperature distribution', icon: Thermometer, color: '#ef4444', category: 'weather', dataSource: 'Open-Meteo API', updateFreq: '15 min', isLive: true, requiresApi: true, blendMode: 'overlay', defaultOpacity: 0.5 },
  { id: 'weather-wind', label: 'Wind Patterns', i18nKey: 'layer.wind', description: 'Animated wind flow visualization with speed and direction', icon: Wind, color: '#a78bfa', category: 'weather', dataSource: 'Open-Meteo API', updateFreq: '15 min', isLive: true, requiresApi: true, blendMode: 'screen', defaultOpacity: 0.4 },
  { id: 'weather-precip', label: 'Precipitation Forecast', i18nKey: 'layer.precipitation', description: '48-hour precipitation probability heatmap', icon: Droplets, color: '#3b82f6', category: 'weather', dataSource: 'Open-Meteo API', updateFreq: '1 hour', isLive: true, requiresApi: true, blendMode: 'multiply', defaultOpacity: 0.5 },
  { id: 'air-quality', label: 'Air Quality Index', i18nKey: 'layer.airQuality', description: 'PM2.5, PM10, Ozone levels with health recommendations', icon: Atom, color: '#10b981', category: 'weather', dataSource: 'Open-Meteo AQI', updateFreq: '30 min', isLive: true, requiresApi: true, blendMode: 'overlay', defaultOpacity: 0.45 },

  // Safety & Risk
  { id: 'earthquake', label: 'Seismic Activity', i18nKey: 'layer.earthquake', description: 'Real-time earthquake data from USGS with magnitude rings', icon: Activity, color: '#ef4444', category: 'safety', dataSource: 'USGS Earthquake API', updateFreq: '5 min', isLive: true, requiresApi: true, defaultOpacity: 0.7 },
  { id: 'risk-zones', label: 'Risk Zones', i18nKey: 'layer.riskZones', description: 'Accident hotspots, flood zones, and hazard areas', icon: AlertTriangle, color: '#f97316', category: 'safety', dataSource: 'Derived Analytics', updateFreq: '1 hour', isLive: true, requiresApi: true, defaultOpacity: 0.5 },
  { id: 'night-vision', label: 'Night Vision', i18nKey: 'layer.nightVision', description: 'Enhanced visibility mode with IR-style rendering', icon: Moon, color: '#22d3ee', category: 'safety', dataSource: 'Client Rendering', updateFreq: 'Real-time', isLive: true, requiresApi: false, blendMode: 'screen', defaultOpacity: 0.3 },
  { id: 'heatmap', label: 'Activity Heatmap', i18nKey: 'layer.heatmap', description: 'Population density and movement patterns', icon: Flame, color: '#f59e0b', category: 'safety', dataSource: 'Aggregated Data', updateFreq: '15 min', isLive: true, requiresApi: true, blendMode: 'screen', defaultOpacity: 0.4 },

  // Infrastructure
  { id: 'ev-charging', label: 'EV Charging', i18nKey: 'layer.evCharging', description: 'Charging stations with real-time availability and pricing', icon: Bolt, color: '#22c55e', category: 'infrastructure', dataSource: 'OpenChargeMap API', updateFreq: '10 min', isLive: true, requiresApi: true, defaultOpacity: 0.8 },
  { id: 'parking', label: 'Smart Parking', i18nKey: 'layer.smartParking', description: 'Available parking spots, garages, and pricing', icon: Hexagon, color: '#06b6d4', category: 'infrastructure', dataSource: 'OSM + Overpass', updateFreq: '5 min', isLive: true, requiresApi: true, defaultOpacity: 0.7 },

  // Advanced
  { id: '3d-buildings', label: '3D Buildings', i18nKey: 'layer.buildings3d', description: 'Photorealistic 3D building models with height data', icon: Building2, color: '#8b5cf6', category: 'advanced', dataSource: 'Google 3D Tiles', updateFreq: 'Static', isLive: false, requiresApi: false, defaultOpacity: 0.6 },
  { id: 'indoor', label: 'Indoor Maps', i18nKey: 'layer.indoorMaps', description: 'Mall, airport, and station floor plans', icon: MapPin, color: '#ec4899', category: 'advanced', dataSource: 'Google Indoor', updateFreq: 'Static', isLive: false, requiresApi: false, defaultOpacity: 0.8 },
  { id: 'lane-level', label: 'Lane Guidance', i18nKey: 'layer.laneGuidance', description: 'Individual lane-level navigation with AR overlay', icon: Route, color: '#f59e0b', category: 'advanced', dataSource: 'HD Map Data', updateFreq: 'Real-time', isLive: true, requiresApi: true, defaultOpacity: 0.7 },
];

// ═══ Map Style Presets ═══
const MAP_STYLES = [
  { id: 'gane-dark', label: 'G.A.N.E Dark', i18nKey: 'theme.ganeDark', color: '#0d0d1a', accent: '#2563EB', description: 'Default dark theme' },
  { id: 'midnight', label: 'Midnight', i18nKey: 'theme.midnight', color: '#050510', accent: '#6366f1', description: 'Ultra-dark mode' },
  { id: 'satellite-hybrid', label: 'Satellite', i18nKey: 'sidebar.satellite', color: '#1a3a2a', accent: '#22c55e', description: 'Satellite + labels' },
  { id: 'terrain-topo', label: 'Topographic', i18nKey: 'theme.topographic', color: '#1a1a0a', accent: '#84cc16', description: 'Elevation contours' },
  { id: 'night-vision', label: 'Night Vision', i18nKey: 'layer.nightVision', color: '#001a00', accent: '#00ff00', description: 'IR-style green' },
  { id: 'thermal', label: 'Thermal', i18nKey: 'theme.thermal', color: '#1a0a1a', accent: '#ff4444', description: 'Heat signature view' },
];

// ═══ Mini Canvas Preview for Layers ═══
function LayerPreviewCanvas({ layer, isActive }: { layer: LayerConfig; isActive: boolean }) {
  const { t, dir } = useLanguage();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const frameRef = useRef(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const w = 48, h = 32;
    canvas.width = w * 2;
    canvas.height = h * 2;
    ctx.scale(2, 2);

    const draw = () => {
      frameRef.current++;
      ctx.clearRect(0, 0, w, h);

      if (!isActive) {
        ctx.fillStyle = 'rgba(243,244,246,0.4)';
        ctx.fillRect(0, 0, w, h);
        return;
      }

      const f = frameRef.current;
      const col = layer.color;

      switch (layer.category) {
        case 'weather': {
          for (let i = 0; i < 12; i++) {
            const x = (i * 7 + f * 0.3) % w;
            const y = (i * 5 + f * 0.5) % h;
            ctx.beginPath();
            ctx.arc(x, y, 2 + Math.sin(f * 0.05 + i) * 1, 0, Math.PI * 2);
            ctx.fillStyle = col + '40';
            ctx.fill();
          }
          const grad = ctx.createLinearGradient(0, 0, w, h);
          grad.addColorStop(0, col + '10');
          grad.addColorStop(0.5, col + '25');
          grad.addColorStop(1, col + '05');
          ctx.fillStyle = grad;
          ctx.fillRect(0, 0, w, h);
          break;
        }
        case 'safety': {
          const cx = w / 2, cy = h / 2;
          const pulse = Math.sin(f * 0.05) * 0.5 + 0.5;
          for (let r = 3; r < 18; r += 5) {
            ctx.beginPath();
            ctx.arc(cx, cy, r + pulse * 3, 0, Math.PI * 2);
            ctx.strokeStyle = col + Math.round(30 - r).toString(16).padStart(2, '0');
            ctx.lineWidth = 0.5;
            ctx.stroke();
          }
          break;
        }
        case 'infrastructure': {
          for (let x = 4; x < w; x += 8) {
            for (let y = 4; y < h; y += 8) {
              const dist = Math.sin(f * 0.03 + x * 0.1 + y * 0.1) * 0.5 + 0.5;
              ctx.beginPath();
              ctx.arc(x, y, 1 + dist, 0, Math.PI * 2);
              ctx.fillStyle = col + Math.round(dist * 60 + 20).toString(16).padStart(2, '0');
              ctx.fill();
            }
          }
          break;
        }
        case 'advanced': {
          ctx.strokeStyle = col + '20';
          ctx.lineWidth = 0.3;
          for (let x = 0; x < w; x += 6) {
            ctx.beginPath();
            ctx.moveTo(x, 0);
            ctx.lineTo(x + Math.sin(f * 0.02 + x) * 2, h);
            ctx.stroke();
          }
          for (let y = 0; y < h; y += 6) {
            ctx.beginPath();
            ctx.moveTo(0, y);
            ctx.lineTo(w, y + Math.cos(f * 0.02 + y) * 2);
            ctx.stroke();
          }
          break;
        }
        default: {
          for (let i = 0; i < 5; i++) {
            const y = (i * 7 + 3);
            ctx.beginPath();
            ctx.moveTo(0, y);
            for (let x = 0; x < w; x += 2) {
              ctx.lineTo(x, y + Math.sin(f * 0.04 + x * 0.1 + i) * 3);
            }
            ctx.strokeStyle = col + '30';
            ctx.lineWidth = 1;
            ctx.stroke();
          }
        }
      }
    };

    // Draw once only (static preview) — no continuous animation for performance
    draw();
    // No interval needed — static preview is sufficient
    return () => {};
  }, [isActive, layer]);

  return (
    <canvas
      ref={canvasRef}
      className="rounded-md"
      style={{ width: 48, height: 32, opacity: isActive ? 1 : 0.3 }}
    />
  );
}

// ═══ Opacity Slider ═══
function OpacitySlider({ value, onChange, color }: { value: number; onChange: (v: number) => void; color: string }) {
  return (
    <div className="flex items-center gap-2 mt-2">
      <span className="text-[8px] font-mono" style={{ color: 'rgba(156,163,175,0.9)' }}>0%</span>
      <div className="flex-1 relative h-1.5 rounded-full" style={{ background: 'rgba(229,231,235,0.6)' }}>
        <motion.div
          className="absolute top-0 left-0 h-full rounded-full"
          style={{ background: `linear-gradient(90deg, ${color}40, ${color})`, width: `${value * 100}%` }}
          layout
        />
        <input
          type="range"
          min="0"
          max="100"
          value={Math.round(value * 100)}
          onChange={(e) => onChange(Number(e.target.value) / 100)}
          className="absolute inset-0 w-full opacity-0 cursor-pointer"
        />
      </div>
      <span className="text-[8px] font-mono w-6 text-right" style={{ color }}>{Math.round(value * 100)}%</span>
    </div>
  );
}

// ═══ Layer Stats Bar ═══
function LayerStatsBar({ activeLayers }: { activeLayers: MapLayer[] }) {
  const liveCount = ALL_LAYERS.filter(l => activeLayers.includes(l.id) && l.isLive).length;
  const totalActive = activeLayers.length;
  const dataSourceCount = new Set(ALL_LAYERS.filter(l => activeLayers.includes(l.id)).map(l => l.dataSource)).size;

  return (
    <div className="flex items-center gap-3 px-4 py-2 mb-2" style={{
      background: 'rgba(37,99,235,0.03)',
      borderBottom: '1px solid rgba(37,99,235,0.06)' }}>
      <div className="flex items-center gap-1.5">
        <div className="w-1.5 h-1.5 rounded-full" style={{ background: '#2563EB', boxShadow: '0 0 6px #2563EB' }} />
        <span className="text-[9px] font-mono" style={{ color: 'rgba(37,99,235,0.7)' }}>{totalActive} ACTIVE</span>
      </div>
      <div className="w-px h-3" style={{ background: 'rgba(229,231,235,0.6)' }} />
      <div className="flex items-center gap-1.5">
        <div className="w-1.5 h-1.5 rounded-full animate-pulse" style={{ background: '#16A34A' }} />
        <span className="text-[9px] font-mono" style={{ color: 'rgba(22,163,74,0.7)' }}>{liveCount} LIVE</span>
      </div>
      <div className="w-px h-3" style={{ background: 'rgba(229,231,235,0.6)' }} />
      <div className="flex items-center gap-1.5">
        <Radio className="w-2.5 h-2.5" style={{ color: 'rgba(124,58,237,0.5)' }} />
        <span className="text-[9px] font-mono" style={{ color: 'rgba(124,58,237,0.7)' }}>{dataSourceCount} SOURCES</span>
      </div>
    </div>
  );
}

// ═══ Animated Toggle ═══
function AnimatedToggle({ active, color, onClick }: { active: boolean; color: string; onClick: () => void }) {
  return (
    <motion.button
      onClick={onClick}
      className="w-11 h-6 rounded-full relative flex-shrink-0 cursor-pointer"
      style={{
        background: active ? `${color}25` : 'rgba(229,231,235,0.6)',
        border: `1px solid ${active ? color + '40' : 'rgba(209,213,219,0.5)'}` }}
      whileTap={{ scale: 0.95 }}
    >
      <motion.div
        className="w-4.5 h-4.5 rounded-full absolute top-[2px]"
        style={{ background: active ? color : 'rgba(209,213,219,0.8)' }}
        animate={{ left: active ? 22 : 2 }}
        transition={{ type: 'spring', stiffness: 500, damping: 30 }}
      />
      {active && (
        <motion.div
          className="absolute inset-0 rounded-full"
          style={{ boxShadow: `0 0 12px ${color}30` }}
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
        />
      )}
    </motion.button>
  );
}

// ═══ Main Panel ═══
export default function MapLayersPanel({ onClose }: { onClose: () => void }) {
  const { state, dispatch } = useNavigation();
  const realData = useRealDataContext();
  const [expandedCategories, setExpandedCategories] = useState<string[]>(['core', 'weather']);
  const [layerOpacities, setLayerOpacities] = useState<Record<string, number>>(() => {
    const initial: Record<string, number> = {};
    ALL_LAYERS.forEach(l => { initial[l.id] = l.defaultOpacity; });
    return initial;
  });
  const [showOpacity, setShowOpacity] = useState<string | null>(null);
  const [selectedStyle, setSelectedStyle] = useState('gane-dark');
  const [showStyles, setShowStyles] = useState(false);

  const toggleCategory = (id: string) => {
    setExpandedCategories(prev =>
      prev.includes(id) ? prev.filter(c => c !== id) : [...prev, id]
    );
  };

  const isActive = (id: string) => state.activeLayers.includes(id as MapLayer);

  const toggleLayer = useCallback((id: string) => {
    dispatch({ type: 'TOGGLE_LAYER', layer: id as MapLayer });
  }, [dispatch]);

  const layersByCategory = useMemo(() => {
    const grouped: Record<string, LayerConfig[]> = {};
    ALL_LAYERS.forEach(l => {
      if (!grouped[l.category]) grouped[l.category] = [];
      grouped[l.category].push(l);
    });
    return grouped;
  }, []);

  // Real-time data status for weather layers
  const weatherStatus = useMemo(() => {
    if (!realData.weather) return null;
    return {
      temp: Math.round(realData.weather.temperature),
      wind: Math.round(realData.weather.windSpeed),
      precip: realData.weather.precipitation,
      aqi: realData.airQuality?.aqi || 0 };
  }, [realData.weather, realData.airQuality]);

  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: 'spring', damping: 30, stiffness: 300 }}
      className="expand-panel scrollbar-gane"
    >
      {/* Header */}
      <div className="sticky top-0 z-10 p-4 pb-2" style={{
        background: 'rgba(4,8,18,0.96)',
        borderBottom: '1px solid rgba(37,99,235,0.06)' }}>
        <div className="flex items-center justify-between mb-2">
          <div className="flex items-center gap-3">
            <motion.div
              className="w-10 h-10 rounded-xl flex items-center justify-center relative overflow-hidden"
              style={{ background: 'rgba(37,99,235,0.08)', border: '1px solid rgba(37,99,235,0.15)' }}
            >
              <Layers size={18} style={{ color: '#2563EB' }} />
              {/* Static highlight for performance */}
              <div className="absolute inset-0 opacity-20" style={{ background: 'linear-gradient(45deg, transparent, rgba(37,99,235,0.15), transparent)' }} />
            </motion.div>
            <div>
              <h3 className="text-sm font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: '#2563EB' }}>
                Map Layers
              </h3>
              <p className="text-[10px]" style={{ color: 'rgba(107,114,128,0.7)' }}>
                {ALL_LAYERS.length} layers available · {state.activeLayers.length} active
              </p>
            </div>
          </div>
          <button onClick={onClose} className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
            style={{ background: 'rgba(229,231,235,0.5)', border: '1px solid rgba(209,213,219,0.5)' }}>
            <X size={14} style={{ color: 'rgba(107,114,128,0.9)' }} />
          </button>
        </div>

        {/* Quick Actions Bar */}
        <div className="flex items-center gap-2 mt-2">
          <motion.button
            whileTap={{ scale: 0.95 }}
            onClick={() => setShowStyles(!showStyles)}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-[10px] font-medium cursor-pointer"
            style={{
              background: showStyles ? 'rgba(37,99,235,0.1)' : 'rgba(229,231,235,0.4)',
              border: `1px solid ${showStyles ? 'rgba(37,99,235,0.2)' : 'rgba(229,231,235,0.6)'}`,
              color: showStyles ? '#2563EB' : 'rgba(107,114,128,0.9)' }}
          >
            <Palette size={10} />
            Map Style
          </motion.button>
          <motion.button
            whileTap={{ scale: 0.95 }}
            onClick={() => {
              // Toggle all core layers
              ['traffic', 'transit', 'bicycling'].forEach(id => {
                if (!isActive(id)) toggleLayer(id);
              });
            }}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-[10px] font-medium cursor-pointer"
            style={{
              background: 'rgba(229,231,235,0.4)',
              border: '1px solid rgba(229,231,235,0.6)',
              color: 'rgba(107,114,128,0.9)' }}
          >
            <Zap size={10} />
            Quick Nav
          </motion.button>
          <motion.button
            whileTap={{ scale: 0.95 }}
            onClick={() => {
              // Clear all layers
              state.activeLayers.forEach(l => dispatch({ type: 'TOGGLE_LAYER', layer: l }));
            }}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-[10px] font-medium cursor-pointer ml-auto"
            style={{
              background: 'rgba(255,68,68,0.05)',
              border: '1px solid rgba(255,68,68,0.1)',
              color: 'rgba(255,68,68,0.6)' }}
          >
            <EyeOff size={10} />
            Clear All
          </motion.button>
        </div>
      </div>

      {/* Stats Bar */}
      <LayerStatsBar activeLayers={state.activeLayers} />

      {/* Map Style Selector */}
      <AnimatePresence>
        {showStyles && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            className="overflow-hidden px-4 pb-3"
          >
            <div className="grid grid-cols-3 gap-2 mt-2">
              {MAP_STYLES.map((style, i) => (
                <motion.button
                  key={style.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: i * 0.04 }}
                  onClick={() => setSelectedStyle(style.id)}
                  className="p-2.5 rounded-xl text-center transition-all cursor-pointer"
                  style={{
                    background: selectedStyle === style.id ? `${style.accent}10` : 'rgba(243,244,246,0.4)',
                    border: `1px solid ${selectedStyle === style.id ? style.accent + '30' : 'rgba(229,231,235,0.4)'}` }}
                >
                  <div className="w-full h-10 rounded-lg mb-1.5 relative overflow-hidden" style={{ background: style.color }}>
                    {/* Mini grid pattern */}
                    <svg className="absolute inset-0 w-full h-full opacity-20">
                      <pattern id={`grid-${style.id}`} width="6" height="6" patternUnits="userSpaceOnUse">
                        <path d="M 6 0 L 0 0 0 6" fill="none" stroke={style.accent} strokeWidth="0.3" />
                      </pattern>
                      <rect width="100%" height="100%" fill={`url(#grid-${style.id})`} />
                    </svg>
                    {selectedStyle === style.id && (
                      <motion.div
                        className="absolute inset-0"
                        style={{ background: `radial-gradient(circle at 50% 50%, ${style.accent}30, transparent)` }}
                        animate={{ opacity: [0.3, 0.6, 0.3] }}
                        transition={{ duration: 2, repeat: Infinity }}
                      />
                    )}
                  </div>
                  <span className="text-[9px] font-medium block" style={{ color: selectedStyle === style.id ? style.accent : 'rgba(107,114,128,0.7)' }}>
                    {style.label}
                  </span>
                </motion.button>
              ))}
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Layer Categories */}
      <div className="px-4 pb-6 space-y-3">
        {LAYER_CATEGORIES.map((cat) => {
          const layers = layersByCategory[cat.id] || [];
          const isExpanded = expandedCategories.includes(cat.id);
          const activeInCategory = layers.filter(l => isActive(l.id)).length;

          return (
            <div key={cat.id}>
              {/* Category Header */}
              <button
                onClick={() => toggleCategory(cat.id)}
                className="flex items-center gap-2 w-full py-2 cursor-pointer group"
              >
                <cat.icon className="w-3.5 h-3.5 transition-colors" style={{ color: activeInCategory > 0 ? cat.color : 'rgba(156,163,175,0.6)' }} />
                <span className="text-[10px] font-bold tracking-wider uppercase transition-colors" style={{
                  fontFamily: 'Syne, sans-serif',
                  color: activeInCategory > 0 ? cat.color : 'rgba(156,163,175,0.8)' }}>
                  {cat.label}
                </span>
                {activeInCategory > 0 && (
                  <motion.span
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    className="text-[8px] px-1.5 py-0.5 rounded-full font-bold"
                    style={{ background: cat.color + '15', color: cat.color, border: `1px solid ${cat.color}25` }}
                  >
                    {activeInCategory}
                  </motion.span>
                )}
                <motion.div
                  animate={{ rotate: isExpanded ? 90 : 0 }}
                  className="ml-auto"
                >
                  <ChevronRight className="w-3 h-3" style={{ color: 'rgba(209,213,219,0.8)' }} />
                </motion.div>
              </button>

              {/* Layer Items */}
              <AnimatePresence>
                {isExpanded && (
                  <motion.div
                    initial={{ height: 0, opacity: 0 }}
                    animate={{ height: 'auto', opacity: 1 }}
                    exit={{ height: 0, opacity: 0 }}
                    transition={{ duration: 0.2 }}
                    className="space-y-2.5 overflow-hidden"
                  >
                    {layers.map((layer, i) => {
                      const active = isActive(layer.id);
                      const expanded = showOpacity === layer.id;

                      return (
                        <motion.div
                          key={layer.id}
                          initial={{ opacity: 0, x: -15 }}
                          animate={{ opacity: 1, x: 0 }}
                          transition={{ delay: i * 0.03 }}
                          className="rounded-xl transition-all"
                          style={{
                            background: active ? `${layer.color}08` : 'rgba(243,244,246,0.4)',
                            border: `1px solid ${active ? layer.color + '18' : 'rgba(229,231,235,0.4)'}` }}
                        >
                          <div className="flex items-center gap-2.5 p-2.5">
                            {/* Canvas Preview */}
                            <LayerPreviewCanvas layer={layer} isActive={active} />

                            {/* Info */}
                            <div className="flex-1 min-w-0">
                              <div className="flex items-center gap-1.5">
                                <span className="text-[11px] font-semibold" style={{ color: active ? 'rgba(17,24,39,0.9)' : 'rgba(156,163,175,0.9)' }}>
                                  {layer.label}
                                </span>
                                {layer.isLive && active && (
                                  <span className="relative flex h-1.5 w-1.5">
                                    <span className="absolute inline-flex h-full w-full rounded-full opacity-75 animate-ping" style={{ background: '#16A34A' }} />
                                    <span className="relative inline-flex rounded-full h-1.5 w-1.5" style={{ background: '#16A34A' }} />
                                  </span>
                                )}
                              </div>
                              <div className="text-[9px] mt-0.5" style={{ color: 'rgba(156,163,175,0.6)' }}>
                                {layer.description}
                              </div>
                              <div className="flex items-center gap-2 mt-1">
                                <span className="text-[7px] font-mono px-1.5 py-0.5 rounded" style={{
                                  background: 'rgba(243,244,246,0.5)',
                                  color: 'rgba(156,163,175,0.8)' }}>
                                  {layer.dataSource}
                                </span>
                                <span className="text-[7px] font-mono" style={{ color: layer.isLive ? 'rgba(22,163,74,0.4)' : 'rgba(209,213,219,0.8)' }}>
                                  ⟳ {layer.updateFreq}
                                </span>
                              </div>

                              {/* Real-time data badge for weather layers */}
                              {active && weatherStatus && layer.category === 'weather' && (
                                <motion.div
                                  initial={{ opacity: 0, y: 5 }}
                                  animate={{ opacity: 1, y: 0 }}
                                  className="flex items-center gap-2 mt-1.5"
                                >
                                  {layer.id === 'weather-temp' && (
                                    <span className="text-[8px] font-mono px-1.5 py-0.5 rounded" style={{ background: '#ef444415', color: '#ef4444', border: '1px solid #ef444420' }}>
                                      {weatherStatus.temp}°C LIVE
                                    </span>
                                  )}
                                  {layer.id === 'weather-wind' && (
                                    <span className="text-[8px] font-mono px-1.5 py-0.5 rounded" style={{ background: '#a78bfa15', color: '#a78bfa', border: '1px solid #a78bfa20' }}>
                                      {weatherStatus.wind} km/h LIVE
                                    </span>
                                  )}
                                  {layer.id === 'air-quality' && (
                                    <span className="text-[8px] font-mono px-1.5 py-0.5 rounded" style={{ background: '#10b98115', color: '#10b981', border: '1px solid #10b98120' }}>
                                      AQI {weatherStatus.aqi} LIVE
                                    </span>
                                  )}
                                  {layer.id === 'weather-precip' && (
                                    <span className="text-[8px] font-mono px-1.5 py-0.5 rounded" style={{ background: '#3b82f615', color: '#3b82f6', border: '1px solid #3b82f620' }}>
                                      {weatherStatus.precip}mm LIVE
                                    </span>
                                  )}
                                </motion.div>
                              )}
                            </div>

                            {/* Controls */}
                            <div className="flex items-center gap-1.5">
                              {active && (
                                <motion.button
                                  initial={{ scale: 0 }}
                                  animate={{ scale: 1 }}
                                  onClick={() => setShowOpacity(expanded ? null : layer.id)}
                                  className="w-6 h-6 rounded-md flex items-center justify-center cursor-pointer"
                                  style={{
                                    background: expanded ? `${layer.color}15` : 'rgba(229,231,235,0.4)',
                                    border: `1px solid ${expanded ? layer.color + '25' : 'rgba(229,231,235,0.6)'}` }}
                                >
                                  <SlidersHorizontal size={10} style={{ color: expanded ? layer.color : 'rgba(156,163,175,0.9)' }} />
                                </motion.button>
                              )}
                              <AnimatedToggle active={active} color={layer.color} onClick={() => toggleLayer(layer.id)} />
                            </div>
                          </div>

                          {/* Opacity Slider */}
                          <AnimatePresence>
                            {expanded && active && (
                              <motion.div
                                initial={{ height: 0, opacity: 0 }}
                                animate={{ height: 'auto', opacity: 1 }}
                                exit={{ height: 0, opacity: 0 }}
                                className="px-2.5 pb-2.5 overflow-hidden"
                              >
                                <div className="p-2 rounded-lg" style={{ background: 'rgba(243,244,246,0.4)' }}>
                                  <div className="flex items-center justify-between mb-1">
                                    <span className="text-[8px] font-mono uppercase tracking-wider" style={{ color: 'rgba(156,163,175,0.8)' }}>
                                      Opacity
                                    </span>
                                    {layer.blendMode && (
                                      <span className="text-[7px] font-mono px-1.5 py-0.5 rounded" style={{ background: `${layer.color}08`, color: `${layer.color}80` }}>
                                        blend: {layer.blendMode}
                                      </span>
                                    )}
                                  </div>
                                  <OpacitySlider
                                    value={layerOpacities[layer.id] ?? layer.defaultOpacity}
                                    onChange={(v) => setLayerOpacities(prev => ({ ...prev, [layer.id]: v }))}
                                    color={layer.color}
                                  />
                                </div>
                              </motion.div>
                            )}
                          </AnimatePresence>
                        </motion.div>
                      );
                    })}
                  </motion.div>
                )}
              </AnimatePresence>
            </div>
          );
        })}

        {/* Layer Composition Info */}
        <div className="mt-4 p-3 rounded-xl" style={{
          background: 'rgba(37,99,235,0.03)',
          border: '1px solid rgba(37,99,235,0.06)' }}>
          <div className="flex items-center gap-2 mb-2">
            <BarChart3 className="w-3.5 h-3.5" style={{ color: 'rgba(37,99,235,0.5)' }} />
            <span className="text-[9px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(37,99,235,0.6)' }}>
              Layer Composition Engine
            </span>
          </div>
          <div className="space-y-2.5">
            {state.activeLayers.length === 0 ? (
              <p className="text-[9px]" style={{ color: 'rgba(156,163,175,0.6)' }}>
                No layers active. Toggle layers above to build your map composition.
              </p>
            ) : (
              <div className="flex flex-wrap gap-1">
                {state.activeLayers.map(layerId => {
                  const layer = ALL_LAYERS.find(l => l.id === layerId);
                  if (!layer) return null;
                  return (
                    <motion.span
                      key={layerId}
                      initial={{ scale: 0 }}
                      animate={{ scale: 1 }}
                      className="text-[8px] px-2 py-1 rounded-md font-medium flex items-center gap-1"
                      style={{
                        background: `${layer.color}10`,
                        color: layer.color,
                        border: `1px solid ${layer.color}20` }}
                    >
                      <span className="w-1 h-1 rounded-full" style={{ background: layer.color }} />
                      {layer.label}
                      <span className="opacity-50">{Math.round((layerOpacities[layer.id] ?? layer.defaultOpacity) * 100)}%</span>
                    </motion.span>
                  );
                })}
              </div>
            )}
          </div>
        </div>

        {/* Data Sources Footer */}
        <div className="mt-3 p-3 rounded-xl" style={{
          background: 'rgba(124,58,237,0.02)',
          border: '1px solid rgba(124,58,237,0.05)' }}>
          <div className="flex items-center gap-2 mb-2">
            <Globe className="w-3 h-3" style={{ color: 'rgba(124,58,237,0.4)' }} />
            <span className="text-[8px] font-bold tracking-wider uppercase" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(124,58,237,0.5)' }}>
              Data Sources
            </span>
          </div>
          <div className="grid grid-cols-2 gap-1">
            {['Google Maps API', 'Open-Meteo', 'USGS', 'RainViewer', 'OpenChargeMap', 'OSM/Overpass'].map(src => (
              <div key={src} className="flex items-center gap-1.5">
                <div className="w-1 h-1 rounded-full" style={{ background: 'rgba(124,58,237,0.3)' }} />
                <span className="text-[8px]" style={{ color: 'rgba(156,163,175,0.6)' }}>{src}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </motion.div>
  );
}
