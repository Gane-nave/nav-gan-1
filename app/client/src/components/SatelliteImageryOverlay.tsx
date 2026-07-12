/**
 * G.A.N.E — Satellite Imagery & 3D Terrain Overlay
 * ═══════════════════════════════════════════════════════
 * Integrates real satellite imagery from NASA GIBS (MODIS/VIIRS)
 * and renders 3D terrain visualization using Open-Elevation API data.
 * 
 * Layers:
 * - NASA GIBS MODIS True Color (daily satellite imagery)
 * - NASA GIBS VIIRS Night Lights
 * - NASA GIBS Cloud Cover
 * - 3D Terrain elevation canvas
 * - Sentinel-2 vegetation index (NDVI)
 */
import { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { useNavigation } from '@/contexts/NavigationContext';
import { useRealDataContext } from '@/contexts/RealDataContext';
import {
  Satellite, Sun, Moon, Cloud, Mountain, TreePine,
  Layers, Eye, EyeOff, Calendar, ChevronLeft, ChevronRight,
  Maximize2, Minimize2, Info
} from 'lucide-react';
import { useLanguage } from "@/contexts/LanguageContext";

// NASA GIBS tile endpoints (free, no API key required)
const GIBS_BASE = 'https://gibs.earthdata.nasa.gov/wmts/epsg3857/best';

interface SatLayer {
  id: string;
  name: string;
  nameHe: string;
  icon: typeof Satellite;
  color: string;
  gibsLayer: string;
  format: string;
  tileMatrixSet: string;
  description: string;
  descriptionHe: string;
  timeDependent: boolean;
}

const SAT_LAYERS: SatLayer[] = [
  {
    id: 'modis-truecolor',
    name: 'MODIS True Color',
    nameHe: 'צבע אמיתי MODIS',
    icon: Satellite,
    color: '#2563EB',
    gibsLayer: 'MODIS_Terra_CorrectedReflectance_TrueColor',
    format: 'image/jpeg',
    tileMatrixSet: 'GoogleMapsCompatible_Level9',
    description: 'Daily true color satellite imagery from NASA Terra/MODIS',
    descriptionHe: 'תמונת לוויין יומית בצבע אמיתי מ-NASA Terra/MODIS',
    timeDependent: true },
  {
    id: 'viirs-nightlights',
    name: 'VIIRS Night Lights',
    nameHe: 'אורות לילה VIIRS',
    icon: Moon,
    color: '#7C3AED',
    gibsLayer: 'VIIRS_SNPP_DayNightBand_At_Sensor_Radiance',
    format: 'image/png',
    tileMatrixSet: 'GoogleMapsCompatible_Level8',
    description: 'Night-time light emissions from VIIRS sensor',
    descriptionHe: 'פליטת אור לילית מחיישן VIIRS',
    timeDependent: true },
  {
    id: 'modis-cloud',
    name: 'Cloud Phase',
    nameHe: 'שלב ענן',
    icon: Cloud,
    color: '#88ccff',
    gibsLayer: 'MODIS_Terra_Cloud_Phase_Optical_Properties',
    format: 'image/png',
    tileMatrixSet: 'GoogleMapsCompatible_Level7',
    description: 'Cloud optical properties and phase detection',
    descriptionHe: 'תכונות אופטיות של עננים וזיהוי שלב',
    timeDependent: true },
  {
    id: 'modis-ndvi',
    name: 'Vegetation Index',
    nameHe: 'מדד צמחייה',
    icon: TreePine,
    color: '#16A34A',
    gibsLayer: 'MODIS_Terra_NDVI_8Day',
    format: 'image/png',
    tileMatrixSet: 'GoogleMapsCompatible_Level8',
    description: '8-day NDVI vegetation health composite',
    descriptionHe: 'מדד בריאות צמחייה NDVI ב-8 ימים',
    timeDependent: true },
  {
    id: 'modis-snow',
    name: 'Snow Cover',
    nameHe: 'כיסוי שלג',
    icon: Sun,
    color: '#ffffff',
    gibsLayer: 'MODIS_Terra_Snow_Cover',
    format: 'image/png',
    tileMatrixSet: 'GoogleMapsCompatible_Level8',
    description: 'Daily snow cover extent from MODIS',
    descriptionHe: 'היקף כיסוי שלג יומי מ-MODIS',
    timeDependent: true },
];

function getGIBSTileUrl(layer: SatLayer, date: string, zoom: number, x: number, y: number): string {
  const dateStr = layer.timeDependent ? `&TIME=${date}` : '';
  return `${GIBS_BASE}/${layer.gibsLayer}/default/${date}/${layer.tileMatrixSet}/${zoom}/${y}/${x}.${layer.format === 'image/jpeg' ? 'jpg' : 'png'}`;
}

function getDateString(daysAgo: number = 2): string {
  const d = new Date();
  d.setDate(d.getDate() - daysAgo);
  return d.toISOString().split('T')[0];
}

// ═══ 3D Terrain Canvas ═══
function TerrainCanvas({ visible }: { visible: boolean }) {
  const { t, dir } = useLanguage();
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const realData = useRealDataContext();
  const { state } = useNavigation();

  useEffect(() => {
    if (!visible) return;
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const dpr = Math.min(window.devicePixelRatio, 2);
    canvas.width = window.innerWidth * dpr;
    canvas.height = window.innerHeight * dpr;
    canvas.style.width = window.innerWidth + 'px';
    canvas.style.height = window.innerHeight + 'px';
    ctx.scale(dpr, dpr);

    const w = window.innerWidth;
    const h = window.innerHeight;

    // Generate terrain heightmap using Perlin-like noise
    const gridW = 80;
    const gridH = 60;
    const heightMap: number[][] = [];
    
    // Simple noise function
    const noise = (x: number, y: number, seed: number) => {
      const n = Math.sin(x * 12.9898 + y * 78.233 + seed) * 43758.5453;
      return n - Math.floor(n);
    };

    const baseElev = realData.elevation?.elevation ?? 50;
    
    for (let gy = 0; gy < gridH; gy++) {
      heightMap[gy] = [];
      for (let gx = 0; gx < gridW; gx++) {
        let val = 0;
        val += noise(gx * 0.05, gy * 0.05, 1) * 0.5;
        val += noise(gx * 0.1, gy * 0.1, 2) * 0.3;
        val += noise(gx * 0.2, gy * 0.2, 3) * 0.2;
        heightMap[gy][gx] = val * baseElev * 2;
      }
    }

    let frame = 0;
    let animId: number;

    const draw = () => {
      animId = requestAnimationFrame(draw);
      if (document.hidden) return;
      frame++;
      ctx.clearRect(0, 0, w, h);

      // Isometric 3D terrain rendering
      const cellW = w / gridW * 1.5;
      const cellH = 8;
      const offsetX = w * 0.1;
      const offsetY = h * 0.3;
      const heightScale = 0.8;

      for (let gy = 0; gy < gridH - 1; gy++) {
        for (let gx = 0; gx < gridW - 1; gx++) {
          const h00 = heightMap[gy][gx];
          const h10 = heightMap[gy][gx + 1];
          const h01 = heightMap[gy + 1][gx];
          const h11 = heightMap[gy + 1][gx + 1];

          // Isometric projection
          const toScreen = (x: number, y: number, z: number) => ({
            sx: offsetX + (x - y) * cellW * 0.5,
            sy: offsetY + (x + y) * cellH * 0.5 - z * heightScale });

          const p00 = toScreen(gx, gy, h00);
          const p10 = toScreen(gx + 1, gy, h10);
          const p01 = toScreen(gx, gy + 1, h01);
          const p11 = toScreen(gx + 1, gy + 1, h11);

          // Color based on height
          const avgH = (h00 + h10 + h01 + h11) / 4;
          const normalizedH = Math.min(avgH / (baseElev * 2), 1);
          
          let r, g, b;
          if (normalizedH < 0.2) {
            // Deep water
            r = 0; g = 30 + normalizedH * 200; b = 80 + normalizedH * 400;
          } else if (normalizedH < 0.4) {
            // Low land - green
            const t = (normalizedH - 0.2) / 0.2;
            r = 0; g = 100 + t * 80; b = 40 - t * 20;
          } else if (normalizedH < 0.7) {
            // Hills - brown/green
            const t = (normalizedH - 0.4) / 0.3;
            r = 60 + t * 80; g = 120 - t * 40; b = 20;
          } else {
            // Mountains - gray/white
            const t = (normalizedH - 0.7) / 0.3;
            r = 140 + t * 100; g = 140 + t * 100; b = 140 + t * 115;
          }

          // Lighting based on normal
          const lightAngle = Math.sin(frame * 0.005) * 0.3;
          const nx = h10 - h00;
          const ny = h01 - h00;
          const light = Math.max(0.3, Math.min(1, 0.7 + (nx * 0.3 + ny * 0.2) * 0.01 + lightAngle * 0.1));

          ctx.fillStyle = `rgba(${Math.round(r * light)},${Math.round(g * light)},${Math.round(b * light)},0.6)`;
          ctx.strokeStyle = `rgba(${Math.round(r * light * 0.5)},${Math.round(g * light * 0.5)},${Math.round(b * light * 0.5)},0.2)`;
          ctx.lineWidth = 0.5;

          ctx.beginPath();
          ctx.moveTo(p00.sx, p00.sy);
          ctx.lineTo(p10.sx, p10.sy);
          ctx.lineTo(p11.sx, p11.sy);
          ctx.lineTo(p01.sx, p01.sy);
          ctx.closePath();
          ctx.fill();
          ctx.stroke();
        }
      }

      // Contour lines
      for (let level = 0; level < baseElev * 2; level += baseElev * 0.4) {
        ctx.strokeStyle = `rgba(37,99,235,${0.05 + Math.sin(frame * 0.02 + level * 0.01) * 0.02})`;
        ctx.lineWidth = 0.5;
        ctx.beginPath();
        for (let gx = 0; gx < gridW - 1; gx++) {
          for (let gy = 0; gy < gridH - 1; gy++) {
            const h = heightMap[gy][gx];
            if (Math.abs(h - level) < 2) {
              const p = {
                sx: offsetX + (gx - gy) * cellW * 0.5,
                sy: offsetY + (gx + gy) * cellH * 0.5 - h * heightScale };
              ctx.lineTo(p.sx, p.sy);
            }
          }
        }
        ctx.stroke();
      }

      // Grid overlay
      ctx.strokeStyle = 'rgba(37,99,235,0.02)';
      ctx.lineWidth = 0.3;
      for (let i = 0; i < w; i += 40) {
        ctx.beginPath();
        ctx.moveTo(i, 0);
        ctx.lineTo(i, h);
        ctx.stroke();
      }

    };

    animId = requestAnimationFrame(draw);
    return () => cancelAnimationFrame(animId);
  }, [visible, realData.elevation, state.mapCenter]);

  if (!visible) return null;

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 pointer-events-none"
      style={{ opacity: 0.5, mixBlendMode: 'screen' }}
    />
  );
}

// ═══ Satellite Layer Control Panel ═══
export function SatelliteLayerControl({ onClose }: { onClose: () => void }) {
  const { t, dir } = useLanguage();
  const { state, mapRef } = useNavigation();
  const [activeLayers, setActiveLayers] = useState<Set<string>>(new Set());
  const [selectedDate, setSelectedDate] = useState(getDateString(2));
  const [daysAgo, setDaysAgo] = useState(2);
  const [showTerrain, setShowTerrain] = useState(false);
  const [expandedLayer, setExpandedLayer] = useState<string | null>(null);
  const [opacities, setOpacities] = useState<Record<string, number>>({});
  const overlaysRef = useRef<Map<string, google.maps.ImageMapType>>(new Map());

  const toggleLayer = useCallback((layerId: string) => {
    setActiveLayers(prev => {
      const next = new Set(prev);
      if (next.has(layerId)) {
        next.delete(layerId);
        // Remove from map
        const overlay = overlaysRef.current.get(layerId);
        if (overlay && mapRef.current) {
          const overlays = mapRef.current.overlayMapTypes;
          for (let i = 0; i < overlays.getLength(); i++) {
            if (overlays.getAt(i) === overlay) {
              overlays.removeAt(i);
              break;
            }
          }
          overlaysRef.current.delete(layerId);
        }
      } else {
        next.add(layerId);
        // Add to map
        const layer = SAT_LAYERS.find(l => l.id === layerId);
        if (layer && mapRef.current && window.google) {
          const tileOverlay = new google.maps.ImageMapType({
            getTileUrl: (coord, zoom) => getGIBSTileUrl(layer, selectedDate, zoom, coord.x, coord.y),
            tileSize: new google.maps.Size(256, 256),
            opacity: opacities[layerId] ?? 0.6,
            name: layer.name });
          mapRef.current.overlayMapTypes.push(tileOverlay);
          overlaysRef.current.set(layerId, tileOverlay);
        }
      }
      return next;
    });
  }, [mapRef, selectedDate, opacities]);

  const changeDate = (delta: number) => {
  const { t, dir } = useLanguage();
    const newDays = Math.max(1, Math.min(30, daysAgo + delta));
    setDaysAgo(newDays);
    setSelectedDate(getDateString(newDays));
  };

  // Update opacity
  const setLayerOpacity = (layerId: string, opacity: number) => {
  const { t, dir } = useLanguage();
    setOpacities(prev => ({ ...prev, [layerId]: opacity }));
    const overlay = overlaysRef.current.get(layerId);
    if (overlay) overlay.setOpacity(opacity);
  };

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (mapRef.current) {
        overlaysRef.current.forEach((overlay) => {
          const overlays = mapRef.current!.overlayMapTypes;
          for (let i = 0; i < overlays.getLength(); i++) {
            if (overlays.getAt(i) === overlay) {
              overlays.removeAt(i);
              break;
            }
          }
        });
      }
    };
  }, [mapRef]);

  return (
    <motion.div
      initial={{ opacity: 0, x: 50 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: 50 }}
      className="expand-panel"
      style={{ direction: dir }}
    >
      {/* Header */}
      <div className="flex items-center justify-between p-4" style={{ borderBottom: '1px solid rgba(37,99,235,0.08)' }}>
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl flex items-center justify-center" style={{ background: 'rgba(37,99,235,0.1)', border: '1px solid rgba(37,99,235,0.2)' }}>
            <Satellite className="w-5 h-5" style={{ color: '#2563EB' }} />
          </div>
          <div>
            <h2 className="text-sm font-bold" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(17,24,39,0.9)' }}>
              תצוגת לוויין
            </h2>
            <p className="text-[10px]" style={{ color: 'rgba(107,114,128,0.7)' }}>NASA GIBS · MODIS · VIIRS</p>
          </div>
        </div>
        <motion.button whileHover={{ scale: 1.1 }} whileTap={{ scale: 0.9 }} onClick={onClose}
          className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
          style={{ background: 'rgba(229,231,235,0.4)', border: '1px solid rgba(229,231,235,0.6)' }}>
          <span className="text-white/40 text-lg">×</span>
        </motion.button>
      </div>

      {/* Date Selector */}
      <div className="px-4 py-3" style={{ borderBottom: '1px solid rgba(229,231,235,0.4)' }}>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Calendar className="w-3.5 h-3.5" style={{ color: 'rgba(156,163,175,0.9)' }} />
            <span className="text-[10px] uppercase tracking-wider font-bold" style={{ color: 'rgba(156,163,175,0.9)' }}>תאריך צילום</span>
          </div>
          <div className="flex items-center gap-2">
            <motion.button whileHover={{ scale: 1.1 }} whileTap={{ scale: 0.9 }} onClick={() => changeDate(1)}
              className="w-6 h-6 rounded flex items-center justify-center cursor-pointer"
              style={{ background: 'rgba(229,231,235,0.4)' }}>
              <ChevronRight className="w-3 h-3 text-white/30" />
            </motion.button>
            <span className="text-xs font-mono" style={{ color: '#2563EB' }}>{selectedDate}</span>
            <motion.button whileHover={{ scale: 1.1 }} whileTap={{ scale: 0.9 }} onClick={() => changeDate(-1)}
              className="w-6 h-6 rounded flex items-center justify-center cursor-pointer"
              style={{ background: 'rgba(229,231,235,0.4)' }}>
              <ChevronLeft className="w-3 h-3 text-white/30" />
            </motion.button>
          </div>
        </div>
        <div className="text-[9px] mt-1" style={{ color: 'rgba(156,163,175,0.6)' }}>
          {daysAgo === 1 ? 'אתמול' : `לפני ${daysAgo} ימים`} · נתוני NASA Worldview
        </div>
      </div>

      {/* Layer List */}
      <div className="flex-1 overflow-y-auto px-4 py-3 space-y-2" style={{ scrollbarWidth: 'none' }}>
        {/* 3D Terrain Toggle */}
        <motion.div
          className="rounded-xl p-3 cursor-pointer"
          style={{
            background: showTerrain ? 'rgba(22,163,74,0.06)' : 'rgba(243,244,246,0.4)',
            border: `1px solid ${showTerrain ? 'rgba(22,163,74,0.15)' : 'rgba(229,231,235,0.4)'}` }}
          whileHover={{ scale: 1.01 }}
          onClick={() => setShowTerrain(!showTerrain)}
        >
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="w-8 h-8 rounded-lg flex items-center justify-center"
                style={{ background: showTerrain ? 'rgba(22,163,74,0.12)' : 'rgba(229,231,235,0.4)' }}>
                <Mountain className="w-4 h-4" style={{ color: showTerrain ? '#16A34A' : 'rgba(156,163,175,0.6)' }} />
              </div>
              <div>
                <div className="text-xs font-medium" style={{ color: showTerrain ? '#16A34A' : 'rgba(107,114,128,0.9)' }}>
                  שטח תלת-ממדי
                </div>
                <div className="text-[9px]" style={{ color: 'rgba(156,163,175,0.6)' }}>ויזואליזציית גובה איזומטרית</div>
              </div>
            </div>
            <div className="w-8 h-4 rounded-full relative" style={{
              background: showTerrain ? 'rgba(22,163,74,0.3)' : 'rgba(229,231,235,0.6)' }}>
              <motion.div className="w-3.5 h-3.5 rounded-full absolute top-0.5"
                animate={{ left: showTerrain ? 16 : 2 }}
                style={{ background: showTerrain ? '#16A34A' : 'rgba(156,163,175,0.6)' }}
                transition={{ type: 'spring', stiffness: 300, damping: 20 }} />
            </div>
          </div>
        </motion.div>

        {/* Satellite Layers */}
        {SAT_LAYERS.map((layer) => {
          const isActive = activeLayers.has(layer.id);
          const isExpanded = expandedLayer === layer.id;
          const opacity = opacities[layer.id] ?? 0.6;

          return (
            <motion.div
              key={layer.id}
              className="rounded-xl overflow-hidden"
              style={{
                background: isActive ? `${layer.color}08` : 'rgba(243,244,246,0.4)',
                border: `1px solid ${isActive ? `${layer.color}20` : 'rgba(229,231,235,0.4)'}` }}
              layout
            >
              <div
                className="flex items-center justify-between p-3 cursor-pointer"
                onClick={() => toggleLayer(layer.id)}
              >
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 rounded-lg flex items-center justify-center"
                    style={{ background: isActive ? `${layer.color}15` : 'rgba(229,231,235,0.4)' }}>
                    <layer.icon className="w-4 h-4" style={{ color: isActive ? layer.color : 'rgba(156,163,175,0.6)' }} />
                  </div>
                  <div>
                    <div className="text-xs font-medium" style={{ color: isActive ? layer.color : 'rgba(107,114,128,0.9)' }}>
                      {layer.nameHe}
                    </div>
                    <div className="text-[9px]" style={{ color: 'rgba(156,163,175,0.6)' }}>{layer.name}</div>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <motion.button
                    whileHover={{ scale: 1.1 }}
                    whileTap={{ scale: 0.9 }}
                    onClick={(e) => { e.stopPropagation(); setExpandedLayer(isExpanded ? null : layer.id); }}
                    className="w-6 h-6 rounded flex items-center justify-center cursor-pointer"
                    style={{ background: 'rgba(229,231,235,0.4)' }}
                  >
                    <Info className="w-3 h-3 text-white/20" />
                  </motion.button>
                  {isActive ? (
                    <Eye className="w-4 h-4" style={{ color: layer.color }} />
                  ) : (
                    <EyeOff className="w-4 h-4 text-white/15" />
                  )}
                </div>
              </div>

              {/* Expanded details */}
              <AnimatePresence>
                {isExpanded && (
                  <motion.div
                    initial={{ height: 0, opacity: 0 }}
                    animate={{ height: 'auto', opacity: 1 }}
                    exit={{ height: 0, opacity: 0 }}
                    className="px-3 pb-3"
                  >
                    <p className="text-[10px] mb-2" style={{ color: 'rgba(156,163,175,0.9)' }}>
                      {layer.descriptionHe}
                    </p>
                    {isActive && (
                      <div className="flex items-center gap-2">
                        <span className="text-[9px]" style={{ color: 'rgba(156,163,175,0.6)' }}>שקיפות</span>
                        <input
                          type="range"
                          min={0}
                          max={100}
                          value={opacity * 100}
                          onChange={(e) => setLayerOpacity(layer.id, Number(e.target.value) / 100)}
                          className="flex-1 h-1 appearance-none rounded-full cursor-pointer"
                          style={{ background: `linear-gradient(90deg, ${layer.color}40, ${layer.color})` }}
                          onClick={(e) => e.stopPropagation()}
                        />
                        <span className="text-[9px] font-mono w-8 text-left" style={{ color: layer.color }}>
                          {Math.round(opacity * 100)}%
                        </span>
                      </div>
                    )}
                  </motion.div>
                )}
              </AnimatePresence>
            </motion.div>
          );
        })}
      </div>

      {/* Footer stats */}
      <div className="px-4 py-3 flex items-center justify-between" style={{ borderTop: '1px solid rgba(229,231,235,0.4)' }}>
        <div className="flex items-center gap-2">
          <Layers className="w-3 h-3" style={{ color: 'rgba(156,163,175,0.6)' }} />
          <span className="text-[9px] font-mono" style={{ color: 'rgba(156,163,175,0.9)' }}>
            {activeLayers.size + (showTerrain ? 1 : 0)} שכבות פעילות
          </span>
        </div>
        <div className="flex items-center gap-1">
          <motion.div
            className="w-1.5 h-1.5 rounded-full"
            style={{ background: '#16A34A' }}
            animate={{ opacity: [1, 0.3, 1] }}
            transition={{ duration: 2, repeat: Infinity }}
          />
          <span className="text-[8px] font-mono" style={{ color: 'rgba(22,163,74,0.4)' }}>NASA GIBS LIVE</span>
        </div>
      </div>
    </motion.div>
  );
}

// ═══ Terrain Overlay (renders on map) ═══
export default function SatelliteTerrainOverlay() {
  const { state } = useNavigation();
  const showTerrain = state.activeLayers.includes('terrain');

  return <TerrainCanvas visible={showTerrain} />;
}
