/**
 * SatelliteCoverageMap — Global constellation coverage visualization
 * Shows coverage zones for all 7 GNSS systems + SBAS augmentation
 * Scientific-grade visualization with real orbital coverage data
 */
import { useState, useMemo } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  X, Globe, Satellite, Signal, ChevronDown, ChevronUp,
  MapPin, Radio, Shield, Zap, Eye, EyeOff
} from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";

// ═══════════════════════════════════════════════════════════
// CONSTELLATION DATA — Real coverage parameters
// ═══════════════════════════════════════════════════════════

interface ConstellationInfo {
  id: string;
  name: string;
  country: string;
  flag: string;
  satellites: number;
  orbitalPlanes: number;
  altitude: number; // km
  inclination: number; // degrees
  accuracy: string;
  coverageType: 'global' | 'regional' | 'augmentation';
  color: string;
  colorLight: string;
  description: string;
  features: string[];
  // SVG coverage zone (simplified polygon on Mercator projection)
  coverageZone: { cx: number; cy: number; rx: number; ry: number }[];
  // Enhanced coverage region (where accuracy is best)
  enhancedZone?: { cx: number; cy: number; rx: number; ry: number };
}

const CONSTELLATIONS: ConstellationInfo[] = [
  {
    id: 'gps',
    name: 'GPS',
    country: 'United States',
    flag: '🇺🇸',
    satellites: 31,
    orbitalPlanes: 6,
    altitude: 20200,
    inclination: 55,
    accuracy: '3-5m',
    coverageType: 'global',
    color: '#3B82F6',
    colorLight: 'rgba(59,130,246,0.15)',
    description: 'Global Positioning System — the most widely used GNSS worldwide',
    features: ['L1 C/A', 'L2C', 'L5', 'CNAV', 'Military P(Y)'],
    coverageZone: [{ cx: 400, cy: 200, rx: 395, ry: 195 }],
  },
  {
    id: 'galileo',
    name: 'Galileo',
    country: 'European Union',
    flag: '🇪🇺',
    satellites: 30,
    orbitalPlanes: 3,
    altitude: 23222,
    inclination: 56,
    accuracy: '< 1m',
    coverageType: 'global',
    color: '#8B5CF6',
    colorLight: 'rgba(139,92,246,0.15)',
    description: 'European GNSS — highest civilian accuracy with Search & Rescue',
    features: ['E1', 'E5a', 'E5b', 'E6', 'SAR/Galileo', 'HAS'],
    coverageZone: [{ cx: 400, cy: 200, rx: 395, ry: 195 }],
    enhancedZone: { cx: 350, cy: 175, rx: 120, ry: 60 },
  },
  {
    id: 'glonass',
    name: 'GLONASS',
    country: 'Russia',
    flag: '🇷🇺',
    satellites: 24,
    orbitalPlanes: 3,
    altitude: 19100,
    inclination: 64.8,
    accuracy: '3-7m',
    coverageType: 'global',
    color: '#EF4444',
    colorLight: 'rgba(239,68,68,0.15)',
    description: 'Russian GNSS — excellent at high latitudes (>55°N)',
    features: ['L1OF', 'L2OF', 'L3OC', 'CDMA', 'FDMA'],
    coverageZone: [{ cx: 400, cy: 200, rx: 395, ry: 195 }],
    enhancedZone: { cx: 450, cy: 80, rx: 200, ry: 50 },
  },
  {
    id: 'beidou',
    name: 'BeiDou',
    country: 'China',
    flag: '🇨🇳',
    satellites: 35,
    orbitalPlanes: 3,
    altitude: 21528,
    inclination: 55,
    accuracy: '3-5m',
    coverageType: 'global',
    color: '#F59E0B',
    colorLight: 'rgba(245,158,11,0.15)',
    description: 'Chinese GNSS — global coverage with enhanced Asia-Pacific + messaging',
    features: ['B1I', 'B1C', 'B2a', 'B2b', 'B3I', 'RDSS Messaging'],
    coverageZone: [{ cx: 400, cy: 200, rx: 395, ry: 195 }],
    enhancedZone: { cx: 560, cy: 170, rx: 100, ry: 70 },
  },
  {
    id: 'qzss',
    name: 'QZSS',
    country: 'Japan',
    flag: '🇯🇵',
    satellites: 4,
    orbitalPlanes: 3,
    altitude: 32000,
    inclination: 43,
    accuracy: '< 1m (augmented)',
    coverageType: 'augmentation',
    color: '#EC4899',
    colorLight: 'rgba(236,72,153,0.15)',
    description: 'Quasi-Zenith Satellite System — GPS augmentation for Japan/Asia-Pacific',
    features: ['L1C/A', 'L1C', 'L2C', 'L5', 'SLAS', 'CLAS'],
    coverageZone: [{ cx: 610, cy: 165, rx: 80, ry: 70 }],
    enhancedZone: { cx: 620, cy: 155, rx: 30, ry: 25 },
  },
  {
    id: 'navic',
    name: 'NavIC',
    country: 'India',
    flag: '🇮🇳',
    satellites: 7,
    orbitalPlanes: 2,
    altitude: 36000,
    inclination: 29,
    accuracy: '< 5m (regional)',
    coverageType: 'regional',
    color: '#10B981',
    colorLight: 'rgba(16,185,129,0.15)',
    description: 'Indian Regional Navigation Satellite System — high accuracy over India',
    features: ['L5', 'S-band', 'NavIC SPS', 'NavIC RS'],
    coverageZone: [{ cx: 520, cy: 195, rx: 60, ry: 55 }],
    enhancedZone: { cx: 520, cy: 190, rx: 30, ry: 30 },
  },
  {
    id: 'sbas',
    name: 'SBAS',
    country: 'Multi-national',
    flag: '🌐',
    satellites: 20,
    orbitalPlanes: 0,
    altitude: 35786,
    inclination: 0,
    accuracy: '< 1m (augmented)',
    coverageType: 'augmentation',
    color: '#06B6D4',
    colorLight: 'rgba(6,182,212,0.15)',
    description: 'Satellite-Based Augmentation Systems — WAAS, EGNOS, GAGAN, MSAS',
    features: ['WAAS (US)', 'EGNOS (EU)', 'GAGAN (India)', 'MSAS (Japan)', 'SDCM (Russia)'],
    coverageZone: [
      { cx: 170, cy: 155, rx: 100, ry: 55 },  // WAAS - North America
      { cx: 350, cy: 160, rx: 80, ry: 45 },    // EGNOS - Europe
      { cx: 520, cy: 190, rx: 50, ry: 40 },    // GAGAN - India
      { cx: 620, cy: 155, rx: 40, ry: 35 },    // MSAS - Japan
    ],
  },
];

// ═══════════════════════════════════════════════════════════
// WORLD MAP SVG — Simplified continents
// ═══════════════════════════════════════════════════════════

function WorldMapSVG() {
  return (
    <g className="world-continents" opacity={0.3}>
      {/* Grid lines */}
      {[0, 50, 100, 150, 200, 250, 300, 350, 400].map(y => (
        <line key={`h${y}`} x1={0} y1={y} x2={800} y2={y} stroke="currentColor" strokeWidth={0.3} opacity={0.15} />
      ))}
      {[0, 100, 200, 300, 400, 500, 600, 700, 800].map(x => (
        <line key={`v${x}`} x1={x} y1={0} x2={x} y2={400} stroke="currentColor" strokeWidth={0.3} opacity={0.15} />
      ))}
      {/* Equator */}
      <line x1={0} y1={200} x2={800} y2={200} stroke="currentColor" strokeWidth={0.5} opacity={0.25} strokeDasharray="4 4" />
      {/* North America */}
      <path d="M80,80 L200,60 L220,90 L200,120 L220,150 L200,180 L180,200 L140,210 L120,190 L100,200 L80,180 L60,140 L70,100 Z" fill="currentColor" opacity={0.12} />
      {/* South America */}
      <path d="M180,220 L210,210 L230,230 L240,270 L230,310 L210,340 L190,350 L170,330 L160,290 L165,250 Z" fill="currentColor" opacity={0.12} />
      {/* Europe */}
      <path d="M320,70 L380,60 L400,80 L390,100 L370,110 L350,120 L330,110 L310,100 L320,80 Z" fill="currentColor" opacity={0.12} />
      {/* Africa */}
      <path d="M330,140 L380,130 L410,150 L420,190 L410,240 L390,280 L370,300 L350,290 L340,260 L330,220 L320,180 Z" fill="currentColor" opacity={0.12} />
      {/* Asia */}
      <path d="M400,60 L500,50 L580,60 L640,80 L660,100 L650,130 L620,150 L580,160 L540,170 L500,180 L460,170 L430,150 L410,130 L400,100 Z" fill="currentColor" opacity={0.12} />
      {/* India */}
      <path d="M500,160 L530,150 L540,170 L530,200 L510,220 L490,210 L485,190 Z" fill="currentColor" opacity={0.12} />
      {/* Southeast Asia */}
      <path d="M580,170 L620,160 L640,180 L630,200 L610,210 L590,200 Z" fill="currentColor" opacity={0.12} />
      {/* Australia */}
      <path d="M600,260 L660,250 L690,270 L680,300 L650,310 L620,300 L600,280 Z" fill="currentColor" opacity={0.12} />
      {/* Japan */}
      <path d="M645,110 L655,100 L660,115 L650,125 Z" fill="currentColor" opacity={0.15} />
      {/* Labels */}
      <text x={140} y={140} fontSize={8} fill="currentColor" opacity={0.2} textAnchor="middle">N. America</text>
      <text x={195} y={280} fontSize={8} fill="currentColor" opacity={0.2} textAnchor="middle">S. America</text>
      <text x={355} y={95} fontSize={8} fill="currentColor" opacity={0.2} textAnchor="middle">Europe</text>
      <text x={370} y={220} fontSize={8} fill="currentColor" opacity={0.2} textAnchor="middle">Africa</text>
      <text x={540} y={110} fontSize={8} fill="currentColor" opacity={0.2} textAnchor="middle">Asia</text>
      <text x={650} y={285} fontSize={8} fill="currentColor" opacity={0.2} textAnchor="middle">Australia</text>
    </g>
  );
}

// ═══════════════════════════════════════════════════════════
// CONSTELLATION DETAIL CARD
// ═══════════════════════════════════════════════════════════

function ConstellationCard({ c, isActive, onClick }: { c: ConstellationInfo; isActive: boolean; onClick: () => void }) {
  const [expanded, setExpanded] = useState(false);
  
  return (
    <motion.div
      layout
      className={`rounded-xl border transition-all cursor-pointer ${
        isActive 
          ? 'border-white/20 bg-white/8 shadow-lg' 
          : 'border-white/5 bg-white/3 hover:bg-white/5'
      }`}
      onClick={onClick}
    >
      <div className="flex items-center gap-3 p-3">
        <div className="w-3 h-3 rounded-full flex-shrink-0" style={{ background: c.color, boxShadow: `0 0 8px ${c.color}40` }} />
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <span className="text-sm font-bold text-white/90">{c.flag} {c.name}</span>
            <span className={`text-[9px] px-1.5 py-0.5 rounded-full font-medium ${
              c.coverageType === 'global' ? 'bg-emerald-500/15 text-emerald-400' :
              c.coverageType === 'regional' ? 'bg-amber-500/15 text-amber-400' :
              'bg-cyan-500/15 text-cyan-400'
            }`}>
              {c.coverageType.toUpperCase()}
            </span>
          </div>
          <div className="flex items-center gap-3 mt-0.5">
            <span className="text-[10px] text-white/40">{c.satellites} sats</span>
            <span className="text-[10px] text-white/40">{c.altitude.toLocaleString()} km</span>
            <span className="text-[10px] font-medium" style={{ color: c.color }}>{c.accuracy}</span>
          </div>
        </div>
        <button
          onClick={(e) => { e.stopPropagation(); setExpanded(!expanded); }}
          className="p-1 rounded-lg hover:bg-white/10 transition-colors"
        >
          {expanded ? <ChevronUp className="w-3.5 h-3.5 text-white/40" /> : <ChevronDown className="w-3.5 h-3.5 text-white/40" />}
        </button>
      </div>
      
      <AnimatePresence>
        {expanded && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            className="overflow-hidden"
          >
            <div className="px-3 pb-3 space-y-2 border-t border-white/5 pt-2">
              <p className="text-[10px] text-white/50 leading-relaxed">{c.description}</p>
              <div className="grid grid-cols-2 gap-1.5">
                <div className="text-[9px] text-white/30">
                  <span className="text-white/50">Planes:</span> {c.orbitalPlanes || 'GEO'}
                </div>
                <div className="text-[9px] text-white/30">
                  <span className="text-white/50">Incl:</span> {c.inclination}°
                </div>
                <div className="text-[9px] text-white/30">
                  <span className="text-white/50">Country:</span> {c.country}
                </div>
                <div className="text-[9px] text-white/30">
                  <span className="text-white/50">Accuracy:</span> {c.accuracy}
                </div>
              </div>
              <div className="flex flex-wrap gap-1 mt-1">
                {c.features.map(f => (
                  <span key={f} className="text-[8px] px-1.5 py-0.5 rounded-md bg-white/5 text-white/40">{f}</span>
                ))}
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
}

// ═══════════════════════════════════════════════════════════
// MAIN COMPONENT
// ═══════════════════════════════════════════════════════════

export default function SatelliteCoverageMap({ onClose }: { onClose: () => void }) {
  const { t, dir } = useLanguage();
  const [visibleLayers, setVisibleLayers] = useState<Set<string>>(
    new Set(CONSTELLATIONS.map(c => c.id))
  );
  const [selectedConstellation, setSelectedConstellation] = useState<string | null>(null);
  const [showEnhanced, setShowEnhanced] = useState(true);

  const toggleLayer = (id: string) => {
    setVisibleLayers(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const totalSatellites = useMemo(() => 
    CONSTELLATIONS.reduce((sum, c) => sum + c.satellites, 0), []
  );

  const visibleConstellations = useMemo(() =>
    CONSTELLATIONS.filter(c => visibleLayers.has(c.id)), [visibleLayers]
  );

  return (
    <motion.div
      initial={{ opacity: 0, x: 40 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: 40 }}
      className="absolute top-0 right-0 w-[520px] h-full z-50 flex flex-col overflow-hidden"
      style={{
        background: 'linear-gradient(135deg, rgba(5,5,20,0.97), rgba(10,10,30,0.97))',
        backdropFilter: 'blur(40px)',
        borderLeft: '1px solid rgba(255,255,255,0.06)',
        direction: dir,
      }}
    >
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-white/5">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-xl bg-gradient-to-br from-blue-500/20 to-purple-500/20 flex items-center justify-center">
            <Globe className="w-4 h-4 text-blue-400" />
          </div>
          <div>
            <h2 className="text-sm font-bold text-white/90">GNSS Coverage Map</h2>
            <p className="text-[10px] text-white/40">{totalSatellites} satellites · 7 constellations · Global coverage</p>
          </div>
        </div>
        <button onClick={onClose} className="p-2 rounded-xl hover:bg-white/5 transition-colors">
          <X className="w-4 h-4 text-white/40" />
        </button>
      </div>

      {/* Map */}
      <div className="px-4 py-3">
        <div className="relative rounded-2xl overflow-hidden border border-white/5" style={{ background: 'rgba(5,5,25,0.8)' }}>
          <svg viewBox="0 0 800 400" className="w-full h-auto">
            <defs>
              {CONSTELLATIONS.map(c => (
                <radialGradient key={`grad-${c.id}`} id={`coverage-${c.id}`}>
                  <stop offset="0%" stopColor={c.color} stopOpacity={0.25} />
                  <stop offset="60%" stopColor={c.color} stopOpacity={0.1} />
                  <stop offset="100%" stopColor={c.color} stopOpacity={0} />
                </radialGradient>
              ))}
              {CONSTELLATIONS.map(c => (
                <radialGradient key={`egrad-${c.id}`} id={`enhanced-${c.id}`}>
                  <stop offset="0%" stopColor={c.color} stopOpacity={0.5} />
                  <stop offset="70%" stopColor={c.color} stopOpacity={0.2} />
                  <stop offset="100%" stopColor={c.color} stopOpacity={0} />
                </radialGradient>
              ))}
            </defs>

            {/* World map background */}
            <WorldMapSVG />

            {/* Coverage zones */}
            {visibleConstellations.map(c => (
              <g key={c.id} className="transition-opacity duration-300"
                 opacity={selectedConstellation && selectedConstellation !== c.id ? 0.15 : 1}>
                {/* Main coverage */}
                {c.coverageZone.map((zone, i) => (
                  <ellipse
                    key={`${c.id}-${i}`}
                    cx={zone.cx} cy={zone.cy}
                    rx={zone.rx} ry={zone.ry}
                    fill={`url(#coverage-${c.id})`}
                    stroke={c.color}
                    strokeWidth={selectedConstellation === c.id ? 1.5 : 0.5}
                    strokeOpacity={selectedConstellation === c.id ? 0.6 : 0.2}
                    strokeDasharray={c.coverageType === 'global' ? 'none' : '4 2'}
                  />
                ))}
                {/* Enhanced coverage zone */}
                {showEnhanced && c.enhancedZone && (
                  <ellipse
                    cx={c.enhancedZone.cx} cy={c.enhancedZone.cy}
                    rx={c.enhancedZone.rx} ry={c.enhancedZone.ry}
                    fill={`url(#enhanced-${c.id})`}
                    stroke={c.color}
                    strokeWidth={1}
                    strokeOpacity={0.4}
                    strokeDasharray="2 2"
                  >
                    <animate attributeName="stroke-opacity" values="0.4;0.7;0.4" dur="3s" repeatCount="indefinite" />
                  </ellipse>
                )}
                {/* Label for regional/augmentation */}
                {c.coverageType !== 'global' && c.coverageZone.length === 1 && (
                  <text
                    x={c.coverageZone[0].cx}
                    y={c.coverageZone[0].cy - c.coverageZone[0].ry - 5}
                    fontSize={8}
                    fill={c.color}
                    textAnchor="middle"
                    opacity={0.7}
                    fontWeight="bold"
                  >
                    {c.name}
                  </text>
                )}
              </g>
            ))}

            {/* SBAS zone labels */}
            {visibleLayers.has('sbas') && (
              <g opacity={selectedConstellation && selectedConstellation !== 'sbas' ? 0.15 : 0.7}>
                <text x={170} y={130} fontSize={7} fill="#06B6D4" textAnchor="middle" fontWeight="bold">WAAS</text>
                <text x={350} y={135} fontSize={7} fill="#06B6D4" textAnchor="middle" fontWeight="bold">EGNOS</text>
                <text x={520} y={165} fontSize={7} fill="#06B6D4" textAnchor="middle" fontWeight="bold">GAGAN</text>
                <text x={620} y={135} fontSize={7} fill="#06B6D4" textAnchor="middle" fontWeight="bold">MSAS</text>
              </g>
            )}

            {/* Legend */}
            <g transform="translate(10, 370)">
              <text fontSize={7} fill="white" opacity={0.3}>Coverage: </text>
              <circle cx={55} cy={-3} r={3} fill="#10B981" opacity={0.5} />
              <text x={62} fontSize={7} fill="white" opacity={0.3}>Global</text>
              <circle cx={95} cy={-3} r={3} fill="#F59E0B" opacity={0.5} />
              <text x={102} fontSize={7} fill="white" opacity={0.3}>Regional</text>
              <circle cx={145} cy={-3} r={3} fill="#06B6D4" opacity={0.5} />
              <text x={152} fontSize={7} fill="white" opacity={0.3}>Augmentation</text>
              {showEnhanced && (
                <>
                  <circle cx={215} cy={-3} r={3} fill="white" opacity={0.3} strokeDasharray="1 1" stroke="white" strokeWidth={0.5} />
                  <text x={222} fontSize={7} fill="white" opacity={0.3}>Enhanced Zone</text>
                </>
              )}
            </g>
          </svg>

          {/* Toggle enhanced zones */}
          <button
            onClick={() => setShowEnhanced(!showEnhanced)}
            className="absolute top-2 right-2 flex items-center gap-1.5 px-2 py-1 rounded-lg bg-black/50 border border-white/10 text-[9px] text-white/50 hover:text-white/70 transition-colors"
          >
            {showEnhanced ? <Eye className="w-3 h-3" /> : <EyeOff className="w-3 h-3" />}
            Enhanced Zones
          </button>
        </div>
      </div>

      {/* Stats bar */}
      <div className="px-4 pb-2">
        <div className="grid grid-cols-4 gap-2">
          {[
            { label: 'Global', count: CONSTELLATIONS.filter(c => c.coverageType === 'global').length, icon: Globe, color: '#10B981' },
            { label: 'Regional', count: CONSTELLATIONS.filter(c => c.coverageType === 'regional').length, icon: MapPin, color: '#F59E0B' },
            { label: 'SBAS', count: CONSTELLATIONS.filter(c => c.coverageType === 'augmentation').length, icon: Radio, color: '#06B6D4' },
            { label: 'Total Sats', count: totalSatellites, icon: Satellite, color: '#8B5CF6' },
          ].map(s => (
            <div key={s.label} className="flex items-center gap-2 p-2 rounded-xl bg-white/3 border border-white/5">
              <s.icon className="w-3.5 h-3.5" style={{ color: s.color }} />
              <div>
                <div className="text-xs font-bold text-white/80">{s.count}</div>
                <div className="text-[8px] text-white/30">{s.label}</div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Constellation list */}
      <div className="flex-1 overflow-y-auto px-4 pb-4 space-y-1.5 scrollbar-gane">
        <div className="flex items-center justify-between mb-2">
          <span className="text-[10px] text-white/30 uppercase tracking-wider">Constellations</span>
          <div className="flex gap-1">
            <button
              onClick={() => setVisibleLayers(new Set(CONSTELLATIONS.map(c => c.id)))}
              className="text-[9px] px-2 py-0.5 rounded-md bg-white/5 text-white/40 hover:text-white/60 transition-colors"
            >
              Show All
            </button>
            <button
              onClick={() => setVisibleLayers(new Set())}
              className="text-[9px] px-2 py-0.5 rounded-md bg-white/5 text-white/40 hover:text-white/60 transition-colors"
            >
              Hide All
            </button>
          </div>
        </div>
        
        {CONSTELLATIONS.map(c => (
          <div key={c.id} className="flex items-start gap-2">
            <button
              onClick={() => toggleLayer(c.id)}
              className={`mt-3 w-4 h-4 rounded flex-shrink-0 flex items-center justify-center border transition-all ${
                visibleLayers.has(c.id) 
                  ? 'border-white/20' 
                  : 'border-white/10 bg-transparent'
              }`}
              style={visibleLayers.has(c.id) ? { background: c.color + '30', borderColor: c.color + '50' } : {}}
            >
              {visibleLayers.has(c.id) && (
                <div className="w-1.5 h-1.5 rounded-sm" style={{ background: c.color }} />
              )}
            </button>
            <div className="flex-1">
              <ConstellationCard
                c={c}
                isActive={selectedConstellation === c.id}
                onClick={() => setSelectedConstellation(selectedConstellation === c.id ? null : c.id)}
              />
            </div>
          </div>
        ))}
      </div>

      {/* Footer */}
      <div className="px-4 py-2 border-t border-white/5 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Shield className="w-3 h-3 text-emerald-400/50" />
          <span className="text-[9px] text-white/30">G.A.N.E Multi-Constellation Fusion Active</span>
        </div>
        <div className="flex items-center gap-1">
          <Zap className="w-3 h-3 text-amber-400/50" />
          <span className="text-[9px] text-white/30">{visibleConstellations.length}/7 visible</span>
        </div>
      </div>
    </motion.div>
  );
}
