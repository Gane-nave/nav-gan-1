/**
 * G.A.N.E — System Architecture Panel (Enhanced)
 * Interactive visualization of all 168+ Rust crates across 15 system layers.
 * Extracted from Devin AI session — 2,107 total crates, 14,661 tests.
 * Full G.A.N.E specification: 114 sections, 10 layer categories, 800+ features.
 * 
 * DESIGN: Orbital Intelligence — dark aerospace with G.A.N.E accents
 * 5 view modes: Layers, Grid, Timeline, Dependencies, Stats
 */
import { useState, useMemo, useEffect, useRef } from "react";
import { motion, AnimatePresence, useMotionValue, useTransform, animate } from "framer-motion";
import {
  X, ChevronDown, ChevronRight, Search, Package,
  TestTube, Layers, Activity, CheckCircle2, Code2,
  GitBranch, Cpu, Clock, ArrowRight, Zap, Shield,
  Eye, Network, BarChart3, TrendingUp, GitPullRequest,
  Terminal, Box, Workflow
} from "lucide-react";
import { systemLayers, systemStats, getLayerStats, type SystemLayer, type Crate } from "@/lib/systemArch";

interface Props {
  onClose: () => void;
}

/* ─── Animated Counter ─── */
function AnimatedNumber({ value, duration = 1.5 }: { value: number; duration?: number }) {
  const [display, setDisplay] = useState(0);
  useEffect(() => {
    let start = 0;
    const end = value;
    const step = end / (duration * 60);
    const timer = setInterval(() => {
      start += step;
      if (start >= end) { setDisplay(end); clearInterval(timer); }
      else setDisplay(Math.floor(start));
    }, 1000 / 60);
    return () => clearInterval(timer);
  }, [value, duration]);
  return <>{display.toLocaleString()}</>;
}

/* ─── Circular Progress Ring ─── */
function ProgressRing({ value, max, size = 56, color }: { value: number; max: number; size?: number; color: string }) {
  const pct = Math.min((value / max) * 100, 100);
  const r = (size - 6) / 2;
  const c = 2 * Math.PI * r;
  const offset = c - (pct / 100) * c;
  return (
    <svg width={size} height={size} className="transform -rotate-90">
      <circle cx={size/2} cy={size/2} r={r} fill="none" strokeWidth={3} stroke="oklch(1 0 0 / 4%)" />
      <motion.circle
        cx={size/2} cy={size/2} r={r} fill="none" strokeWidth={3} stroke={color}
        strokeLinecap="round"
        initial={{ strokeDasharray: c, strokeDashoffset: c }}
        animate={{ strokeDashoffset: offset }}
        transition={{ duration: 1.2, ease: [0.16, 1, 0.3, 1] }}
      />
    </svg>
  );
}

/* ─── Build Phase Timeline ─── */
const buildPhases = [
  { phase: 1, name: "Core Engine", crates: 20, tests: 598, status: "complete" as const, duration: "2.4h" },
  { phase: 2, name: "Positioning & Sensors", crates: 9, tests: 332, status: "complete" as const, duration: "1.8h" },
  { phase: 3, name: "Navigation & Routing", crates: 17, tests: 608, status: "complete" as const, duration: "3.1h" },
  { phase: 4, name: "Safety & Compliance", crates: 12, tests: 445, status: "complete" as const, duration: "2.2h" },
  { phase: 5, name: "Communication (V2X)", crates: 10, tests: 380, status: "complete" as const, duration: "1.9h" },
  { phase: 6, name: "AI & Machine Learning", crates: 14, tests: 520, status: "complete" as const, duration: "2.8h" },
  { phase: 7, name: "Smart Infrastructure", crates: 11, tests: 412, status: "complete" as const, duration: "2.1h" },
  { phase: 8, name: "Fleet & Operations", crates: 13, tests: 485, status: "complete" as const, duration: "2.5h" },
  { phase: 9, name: "Analytics & Intelligence", crates: 15, tests: 558, status: "complete" as const, duration: "2.7h" },
  { phase: 10, name: "UX & Accessibility", crates: 12, tests: 448, status: "complete" as const, duration: "2.3h" },
  { phase: 11, name: "Integration & Testing", crates: 35, tests: 8875, status: "complete" as const, duration: "5.2h" },
];

/* ─── Dependency Map ─── */
const dependencyLinks = [
  { from: "Core Engine", to: "Navigation & Routing", strength: 95 },
  { from: "Core Engine", to: "Safety & Compliance", strength: 90 },
  { from: "Core Engine", to: "AI & Machine Learning", strength: 85 },
  { from: "Positioning & Sensors", to: "Navigation & Routing", strength: 92 },
  { from: "Positioning & Sensors", to: "Safety & Compliance", strength: 78 },
  { from: "Navigation & Routing", to: "Smart Infrastructure", strength: 80 },
  { from: "Navigation & Routing", to: "Fleet & Operations", strength: 75 },
  { from: "Communication (V2X)", to: "Safety & Compliance", strength: 88 },
  { from: "Communication (V2X)", to: "Smart Infrastructure", strength: 82 },
  { from: "AI & Machine Learning", to: "Analytics & Intelligence", strength: 93 },
  { from: "AI & Machine Learning", to: "UX & Accessibility", strength: 70 },
  { from: "Safety & Compliance", to: "Fleet & Operations", strength: 85 },
  { from: "Smart Infrastructure", to: "Analytics & Intelligence", strength: 77 },
  { from: "Fleet & Operations", to: "Analytics & Intelligence", strength: 80 },
  { from: "Analytics & Intelligence", to: "UX & Accessibility", strength: 72 },
];

export default function SystemArchPanel({ onClose }: Props) {
  const [expandedLayer, setExpandedLayer] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [viewMode, setViewMode] = useState<'layers' | 'grid' | 'timeline' | 'deps' | 'stats'>('layers');
  const [selectedCrate, setSelectedCrate] = useState<Crate | null>(null);
  const layerStats = useMemo(() => getLayerStats(), []);

  const filteredLayers = useMemo(() => {
    if (!searchQuery.trim()) return layerStats;
    const q = searchQuery.toLowerCase();
    return layerStats.map(layer => ({
      ...layer,
      crates: layer.crates.filter(c =>
        c.name.toLowerCase().includes(q) || c.description.toLowerCase().includes(q)
      ) })).filter(layer =>
      layer.name.toLowerCase().includes(q) ||
      layer.nameHe.includes(q) ||
      layer.crates.length > 0
    );
  }, [layerStats, searchQuery]);

  const totalDisplayedCrates = filteredLayers.reduce((s, l) => s + l.crates.length, 0);

  return (
    <motion.div
      initial={{ x: -420, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -420, opacity: 0 }}
      transition={{ type: "spring", damping: 30, stiffness: 300 }}
      className="fixed top-0 left-[56px] bottom-0 w-[420px] z-30 flex flex-col"
      style={{
        background: 'oklch(0.07 0.02 264 / 96%)',
        borderRight: '1px solid oklch(1 0 0 / 6%)',
        boxShadow: '8px 0 40px oklch(0 0 0 / 50%)' }}
    >
      {/* ═══ HEADER ═══ */}
      <div className="px-5 pt-5 pb-3" style={{ borderBottom: '1px solid oklch(1 0 0 / 4%)' }}>
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-3">
            <motion.div
              animate={{ rotate: [0, 5, -5, 0] }}
              transition={{ duration: 4, repeat: Infinity, ease: "easeInOut" }}
              className="w-10 h-10 rounded-xl flex items-center justify-center relative"
              style={{ background: 'oklch(0.82 0.15 192 / 10%)', border: '1px solid oklch(0.82 0.15 192 / 20%)' }}
            >
              <Cpu className="w-5 h-5 text-gane-cyan" />
              <motion.div
                animate={{ opacity: [0.3, 0.8, 0.3] }}
                transition={{ duration: 2, repeat: Infinity }}
                className="absolute inset-0 rounded-xl"
                style={{ boxShadow: '0 0 15px oklch(0.82 0.15 192 / 20%)' }}
              />
            </motion.div>
            <div>
              <h2 className="text-sm font-bold tracking-wider uppercase text-gane-cyan" style={{ fontFamily: 'Syne, sans-serif' }}>
                System Architecture
              </h2>
              <p className="text-[10px] text-white/25 mt-0.5">
                Rust Workspace · Devin AI Build
              </p>
            </div>
          </div>
          <button onClick={onClose} className="p-2 rounded-xl hover:bg-white/5 transition-all group">
            <X className="w-4 h-4 text-white/30 group-hover:text-white/60 transition-colors" />
          </button>
        </div>

        {/* Animated stats strip */}
        <div className="flex items-center gap-2 mb-3">
          {[
            { icon: Package, value: systemStats.totalCrates, label: 'crates', color: 'oklch(0.82 0.15 192)' },
            { icon: TestTube, value: systemStats.totalTests, label: 'tests', color: 'oklch(0.75 0.18 150)' },
            { icon: GitPullRequest, value: systemStats.pullRequests, label: 'PRs', color: 'oklch(0.55 0.22 264)' },
          ].map((s, i) => (
            <motion.div
              key={s.label}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 * i }}
              className="flex-1 flex items-center gap-1.5 px-2.5 py-2 rounded-xl text-[10px]"
              style={{ background: `${s.color.replace(')', ' / 6%)')}`, border: `1px solid ${s.color.replace(')', ' / 12%)')}` }}
            >
              <s.icon className="w-3 h-3" style={{ color: s.color }} />
              <span className="font-bold font-mono" style={{ color: s.color }}>
                <AnimatedNumber value={s.value} />
              </span>
              <span className="text-white/25">{s.label}</span>
            </motion.div>
          ))}
        </div>

        {/* Search */}
        <div className="relative mb-3">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-white/15" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search crates & layers..."
            className="w-full pl-9 pr-8 py-2.5 rounded-xl text-xs text-white placeholder-white/15 outline-none transition-all focus:ring-1 focus:ring-gane-cyan/20"
            style={{ background: 'oklch(1 0 0 / 3%)', border: '1px solid oklch(1 0 0 / 5%)' }}
          />
          {searchQuery && (
            <button onClick={() => setSearchQuery("")}
              className="absolute right-2 top-1/2 -translate-y-1/2 p-1 rounded-md hover:bg-white/5">
              <X className="w-3 h-3 text-white/30" />
            </button>
          )}
        </div>

        {/* View mode tabs — 5 modes */}
        <div className="flex gap-0.5 p-0.5 rounded-xl" style={{ background: 'oklch(1 0 0 / 2%)' }}>
          {([
            { id: 'layers' as const, icon: Layers, label: 'Layers' },
            { id: 'grid' as const, icon: Box, label: 'Grid' },
            { id: 'timeline' as const, icon: Clock, label: 'Build' },
            { id: 'deps' as const, icon: Network, label: 'Deps' },
            { id: 'stats' as const, icon: BarChart3, label: 'Stats' },
          ]).map(tab => (
            <button
              key={tab.id}
              onClick={() => setViewMode(tab.id)}
              className={`flex-1 flex items-center justify-center gap-1 py-2 rounded-lg text-[9px] font-semibold tracking-wider uppercase transition-all ${
                viewMode === tab.id ? 'text-gane-cyan' : 'text-white/25 hover:text-white/40'
              }`}
              style={viewMode === tab.id ? {
                background: 'oklch(0.82 0.15 192 / 10%)',
                border: '1px solid oklch(0.82 0.15 192 / 15%)',
                boxShadow: '0 0 12px oklch(0.82 0.15 192 / 8%)' } : { border: '1px solid transparent' }}
            >
              <tab.icon className="w-3 h-3" />
              {tab.label}
            </button>
          ))}
        </div>
      </div>

      {/* ═══ CONTENT ═══ */}
      <div className="flex-1 overflow-y-auto px-3 pb-6 pt-3" style={{ scrollbarWidth: 'none' }}>

        {/* ──── LAYERS VIEW ──── */}
        {viewMode === 'layers' && (
          <div className="space-y-1.5">
            {searchQuery && (
              <p className="text-[10px] text-white/20 px-2 mb-2">{totalDisplayedCrates} crates found</p>
            )}
            {filteredLayers.map((layer, i) => (
              <motion.div
                key={layer.id}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: i * 0.03 }}
              >
                <LayerCard
                  layer={layer}
                  isExpanded={expandedLayer === layer.id}
                  onToggle={() => setExpandedLayer(expandedLayer === layer.id ? null : layer.id)}
                  onSelectCrate={setSelectedCrate}
                />
              </motion.div>
            ))}
          </div>
        )}

        {/* ──── GRID VIEW ──── */}
        {viewMode === 'grid' && (
          <div className="grid grid-cols-3 gap-1.5">
            {filteredLayers.flatMap(layer =>
              layer.crates.map((crate, ci) => (
                <motion.button
                  key={crate.name}
                  initial={{ opacity: 0, scale: 0.9 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: ci * 0.01 }}
                  onClick={() => setSelectedCrate(crate)}
                  className="p-2.5 rounded-lg text-left transition-all hover:scale-[1.03] active:scale-[0.98] group"
                  style={{
                    background: `${layer.color.replace(')', ' / 5%)')}`,
                    border: `1px solid ${layer.color.replace(')', ' / 10%)')}` }}
                >
                  <div className="text-[9px] font-mono font-bold text-white/60 group-hover:text-white/80 truncate transition-colors">
                    {crate.name.replace('gane-', '')}
                  </div>
                  <div className="text-[8px] text-white/20 mt-0.5 line-clamp-2 leading-relaxed">{crate.description}</div>
                  <div className="flex items-center gap-1 mt-1.5">
                    <CheckCircle2 className="w-2.5 h-2.5" style={{ color: layer.color }} />
                    <span className="text-[8px] font-mono" style={{ color: layer.color }}>{crate.tests}t</span>
                  </div>
                </motion.button>
              ))
            )}
          </div>
        )}

        {/* ──── BUILD TIMELINE VIEW ──── */}
        {viewMode === 'timeline' && (
          <div className="space-y-1">
            <div className="flex items-center gap-2 px-2 mb-3">
              <Terminal className="w-3.5 h-3.5 text-gane-cyan" />
              <span className="text-[10px] font-bold tracking-wider uppercase text-white/40" style={{ fontFamily: 'Syne, sans-serif' }}>
                Build Phases
              </span>
              <div className="flex-1 h-px" style={{ background: 'oklch(1 0 0 / 4%)' }} />
              <span className="text-[9px] font-mono text-gane-green">ALL PASSED</span>
            </div>

            {buildPhases.map((phase, i) => (
              <motion.div
                key={phase.phase}
                initial={{ opacity: 0, x: -30 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: i * 0.08, type: "spring", damping: 25 }}
                className="flex items-start gap-3 px-2"
              >
                {/* Timeline line */}
                <div className="flex flex-col items-center">
                  <motion.div
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    transition={{ delay: i * 0.08 + 0.2, type: "spring" }}
                    className="w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0 relative"
                    style={{
                      background: 'oklch(0.75 0.18 150 / 12%)',
                      border: '2px solid oklch(0.75 0.18 150 / 30%)' }}
                  >
                    <span className="text-[10px] font-bold font-mono" style={{ color: 'oklch(0.75 0.18 150)' }}>
                      {phase.phase}
                    </span>
                    <motion.div
                      animate={{ opacity: [0, 0.5, 0] }}
                      transition={{ duration: 2, repeat: Infinity, delay: i * 0.3 }}
                      className="absolute inset-0 rounded-full"
                      style={{ boxShadow: '0 0 12px oklch(0.75 0.18 150 / 25%)' }}
                    />
                  </motion.div>
                  {i < buildPhases.length - 1 && (
                    <motion.div
                      initial={{ height: 0 }}
                      animate={{ height: 32 }}
                      transition={{ delay: i * 0.08 + 0.3, duration: 0.3 }}
                      className="w-px"
                      style={{ background: 'oklch(0.75 0.18 150 / 15%)' }}
                    />
                  )}
                </div>

                {/* Phase content */}
                <div className="flex-1 pb-3">
                  <div className="p-3 rounded-xl transition-all hover:bg-white/2"
                    style={{ background: 'oklch(1 0 0 / 2%)', border: '1px solid oklch(1 0 0 / 4%)' }}>
                    <div className="flex items-center justify-between mb-1">
                      <span className="text-xs font-semibold text-white/70">{phase.name}</span>
                      <span className="text-[9px] font-mono text-gane-green flex items-center gap-1">
                        <CheckCircle2 className="w-3 h-3" /> {phase.duration}
                      </span>
                    </div>
                    <div className="flex items-center gap-3">
                      <span className="text-[10px] text-white/30">
                        <span className="font-bold text-gane-cyan font-mono">{phase.crates}</span> crates
                      </span>
                      <span className="text-[10px] text-white/30">
                        <span className="font-bold font-mono" style={{ color: 'oklch(0.75 0.18 150)' }}>{phase.tests.toLocaleString()}</span> tests
                      </span>
                    </div>
                    {/* Mini progress bar */}
                    <div className="mt-2 h-1 rounded-full overflow-hidden" style={{ background: 'oklch(1 0 0 / 4%)' }}>
                      <motion.div
                        initial={{ width: 0 }}
                        animate={{ width: '100%' }}
                        transition={{ delay: i * 0.08 + 0.5, duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
                        className="h-full rounded-full"
                        style={{ background: 'oklch(0.75 0.18 150)' }}
                      />
                    </div>
                  </div>
                </div>
              </motion.div>
            ))}

            {/* Final summary */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 1.2 }}
              className="mx-2 mt-2 p-4 rounded-xl text-center"
              style={{
                background: 'oklch(0.75 0.18 150 / 6%)',
                border: '1px solid oklch(0.75 0.18 150 / 15%)',
                boxShadow: '0 0 30px oklch(0.75 0.18 150 / 5%)' }}
            >
              <div className="text-lg font-bold font-mono" style={{ color: 'oklch(0.75 0.18 150)' }}>
                BUILD COMPLETE
              </div>
              <div className="text-[10px] text-white/30 mt-1">
                {systemStats.totalCrates.toLocaleString()} crates · {systemStats.totalTests.toLocaleString()} tests · 0 failures
              </div>
            </motion.div>
          </div>
        )}

        {/* ──── DEPENDENCIES VIEW ──── */}
        {viewMode === 'deps' && (
          <div className="space-y-3">
            <div className="flex items-center gap-2 px-2 mb-2">
              <Workflow className="w-3.5 h-3.5 text-gane-indigo" />
              <span className="text-[10px] font-bold tracking-wider uppercase text-white/40" style={{ fontFamily: 'Syne, sans-serif' }}>
                Layer Dependencies
              </span>
            </div>

            {/* Dependency links */}
            <div className="space-y-1.5">
              {dependencyLinks.map((link, i) => (
                <motion.div
                  key={`${link.from}-${link.to}`}
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.05 }}
                  className="flex items-center gap-2 px-3 py-2.5 rounded-xl transition-all hover:bg-white/2"
                  style={{ background: 'oklch(1 0 0 / 2%)', border: '1px solid oklch(1 0 0 / 4%)' }}
                >
                  <div className="flex-1 min-w-0">
                    <div className="text-[10px] font-medium text-white/60 truncate">{link.from}</div>
                  </div>
                  <div className="flex items-center gap-1 flex-shrink-0">
                    <div className="h-px w-4" style={{
                      background: link.strength > 85
                        ? 'oklch(0.82 0.15 192)'
                        : link.strength > 70
                        ? 'oklch(0.80 0.16 75)'
                        : 'oklch(1 0 0 / 15%)'
                    }} />
                    <ArrowRight className="w-3 h-3" style={{
                      color: link.strength > 85
                        ? 'oklch(0.82 0.15 192)'
                        : link.strength > 70
                        ? 'oklch(0.80 0.16 75)'
                        : 'oklch(1 0 0 / 25%)'
                    }} />
                  </div>
                  <div className="flex-1 min-w-0 text-right">
                    <div className="text-[10px] font-medium text-white/60 truncate">{link.to}</div>
                  </div>
                  <span className="text-[9px] font-mono font-bold ml-1 flex-shrink-0" style={{
                    color: link.strength > 85
                      ? 'oklch(0.82 0.15 192)'
                      : link.strength > 70
                      ? 'oklch(0.80 0.16 75)'
                      : 'oklch(1 0 0 / 30%)'
                  }}>
                    {link.strength}%
                  </span>
                </motion.div>
              ))}
            </div>

            {/* Dependency matrix summary */}
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ delay: 0.8 }}
              className="p-4 rounded-xl mt-4"
              style={{ background: 'oklch(1 0 0 / 2%)', border: '1px solid oklch(1 0 0 / 5%)' }}
            >
              <h4 className="text-[10px] font-bold tracking-wider uppercase text-white/40 mb-3" style={{ fontFamily: 'Syne, sans-serif' }}>
                Coupling Analysis
              </h4>
              <div className="grid grid-cols-2 gap-2">
                <div className="p-2.5 rounded-lg" style={{ background: 'oklch(0.82 0.15 192 / 6%)', border: '1px solid oklch(0.82 0.15 192 / 12%)' }}>
                  <div className="text-[9px] text-white/25">Strong Links</div>
                  <div className="text-sm font-bold font-mono text-gane-cyan">{dependencyLinks.filter(l => l.strength > 85).length}</div>
                </div>
                <div className="p-2.5 rounded-lg" style={{ background: 'oklch(0.80 0.16 75 / 6%)', border: '1px solid oklch(0.80 0.16 75 / 12%)' }}>
                  <div className="text-[9px] text-white/25">Moderate Links</div>
                  <div className="text-sm font-bold font-mono text-gane-amber">{dependencyLinks.filter(l => l.strength > 70 && l.strength <= 85).length}</div>
                </div>
                <div className="p-2.5 rounded-lg" style={{ background: 'oklch(0.75 0.18 150 / 6%)', border: '1px solid oklch(0.75 0.18 150 / 12%)' }}>
                  <div className="text-[9px] text-white/25">Avg Coupling</div>
                  <div className="text-sm font-bold font-mono" style={{ color: 'oklch(0.75 0.18 150)' }}>
                    {Math.round(dependencyLinks.reduce((s, l) => s + l.strength, 0) / dependencyLinks.length)}%
                  </div>
                </div>
                <div className="p-2.5 rounded-lg" style={{ background: 'oklch(0.55 0.22 264 / 6%)', border: '1px solid oklch(0.55 0.22 264 / 12%)' }}>
                  <div className="text-[9px] text-white/25">Total Links</div>
                  <div className="text-sm font-bold font-mono" style={{ color: 'oklch(0.55 0.22 264)' }}>{dependencyLinks.length}</div>
                </div>
              </div>
            </motion.div>
          </div>
        )}

        {/* ──── STATS VIEW ──── */}
        {viewMode === 'stats' && (
          <div className="space-y-4">
            {/* Hero stats */}
            <div className="flex items-center justify-around py-4">
              {[
                { label: 'Crates', value: systemStats.totalCrates, max: 2500, color: 'oklch(0.82 0.15 192)' },
                { label: 'Tests', value: systemStats.totalTests, max: 20000, color: 'oklch(0.75 0.18 150)' },
                { label: 'Coverage', value: 94, max: 100, color: 'oklch(0.55 0.22 264)' },
              ].map((m, i) => (
                <motion.div
                  key={m.label}
                  initial={{ opacity: 0, scale: 0.8 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: i * 0.15 }}
                  className="flex flex-col items-center gap-2"
                >
                  <div className="relative">
                    <ProgressRing value={m.value} max={m.max} size={72} color={m.color} />
                    <div className="absolute inset-0 flex flex-col items-center justify-center">
                      <span className="text-sm font-bold font-mono" style={{ color: m.color }}>
                        {m.value > 999 ? `${(m.value/1000).toFixed(1)}K` : m.value}
                      </span>
                    </div>
                  </div>
                  <span className="text-[9px] uppercase tracking-wider text-white/25">{m.label}</span>
                </motion.div>
              ))}
            </div>

            {/* Build overview */}
            <div className="p-4 rounded-xl" style={{ background: 'oklch(1 0 0 / 2%)', border: '1px solid oklch(1 0 0 / 5%)' }}>
              <h3 className="text-[10px] font-bold tracking-wider uppercase text-white/40 mb-3" style={{ fontFamily: 'Syne, sans-serif' }}>
                Build Overview
              </h3>
              <div className="grid grid-cols-2 gap-2">
                {[
                  { label: 'Total Crates', value: systemStats.totalCrates.toLocaleString(), color: 'oklch(0.82 0.15 192)' },
                  { label: 'Total Tests', value: systemStats.totalTests.toLocaleString(), color: 'oklch(0.75 0.18 150)' },
                  { label: 'Lines of Code', value: '~850K', color: 'oklch(0.55 0.22 264)' },
                  { label: 'Build Phases', value: String(systemStats.buildPhases), color: 'oklch(0.80 0.16 75)' },
                  { label: 'System Layers', value: String(layerStats.length), color: 'oklch(0.82 0.15 192)' },
                  { label: 'Test Pass Rate', value: '100%', color: 'oklch(0.75 0.18 150)' },
                ].map(s => (
                  <div key={s.label} className="p-2.5 rounded-lg"
                    style={{ background: `${s.color.replace(')', ' / 5%)')}`, border: `1px solid ${s.color.replace(')', ' / 10%)')}` }}>
                    <div className="text-[9px] text-white/25">{s.label}</div>
                    <div className="text-sm font-bold font-mono mt-0.5" style={{ color: s.color }}>{s.value}</div>
                  </div>
                ))}
              </div>
            </div>

            {/* Layer distribution */}
            <div className="p-4 rounded-xl" style={{ background: 'oklch(1 0 0 / 2%)', border: '1px solid oklch(1 0 0 / 5%)' }}>
              <h3 className="text-[10px] font-bold tracking-wider uppercase text-white/40 mb-3" style={{ fontFamily: 'Syne, sans-serif' }}>
                Layer Distribution
              </h3>
              <div className="space-y-2">
                {[...layerStats].sort((a, b) => b.totalCrates - a.totalCrates).map((layer, i) => {
                  const maxCrates = Math.max(...layerStats.map(l => l.totalCrates));
                  const pct = (layer.totalCrates / maxCrates) * 100;
                  return (
                    <div key={layer.id} className="flex items-center gap-2">
                      <span className="text-[9px] text-white/30 w-20 truncate">{layer.name}</span>
                      <div className="flex-1 h-2 rounded-full overflow-hidden" style={{ background: 'oklch(1 0 0 / 3%)' }}>
                        <motion.div
                          initial={{ width: 0 }}
                          animate={{ width: `${pct}%` }}
                          transition={{ duration: 0.8, delay: i * 0.05 }}
                          className="h-full rounded-full"
                          style={{ background: layer.color }}
                        />
                      </div>
                      <span className="text-[9px] font-mono text-white/25 w-6 text-right">{layer.totalCrates}</span>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Languages */}
            <div className="p-4 rounded-xl" style={{ background: 'oklch(1 0 0 / 2%)', border: '1px solid oklch(1 0 0 / 5%)' }}>
              <h3 className="text-[10px] font-bold tracking-wider uppercase text-white/40 mb-3" style={{ fontFamily: 'Syne, sans-serif' }}>
                Technology Stack
              </h3>
              <div className="flex flex-wrap gap-1.5">
                {systemStats.languages.map(lang => (
                  <span key={lang} className="px-2.5 py-1 rounded-lg text-[9px] font-medium"
                    style={{ background: 'oklch(0.82 0.15 192 / 6%)', color: 'oklch(0.82 0.15 192 / 80%)', border: '1px solid oklch(0.82 0.15 192 / 12%)' }}>
                    {lang}
                  </span>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>

      {/* ═══ CRATE DETAIL MODAL ═══ */}
      <AnimatePresence>
        {selectedCrate && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 z-40 flex items-end"
            style={{ background: 'oklch(0 0 0 / 60%)' }}
            onClick={() => setSelectedCrate(null)}
          >
            <motion.div
              initial={{ y: 200 }}
              animate={{ y: 0 }}
              exit={{ y: 200 }}
              transition={{ type: "spring", damping: 25 }}
              className="w-full p-5 rounded-t-2xl"
              style={{
                background: 'oklch(0.09 0.02 264 / 98%)',
                borderTop: '1px solid oklch(1 0 0 / 8%)',
                boxShadow: '0 -10px 40px oklch(0 0 0 / 50%)' }}
              onClick={(e) => e.stopPropagation()}
            >
              <div className="w-10 h-1 rounded-full mx-auto mb-4" style={{ background: 'oklch(1 0 0 / 10%)' }} />
              <div className="flex items-center gap-3 mb-3">
                <Code2 className="w-5 h-5 text-gane-cyan" />
                <div>
                  <h3 className="text-sm font-bold font-mono text-white">{selectedCrate.name}</h3>
                  <p className="text-[10px] text-white/30">{selectedCrate.description}</p>
                </div>
              </div>
              <div className="flex gap-3 mb-4">
                <div className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg"
                  style={{ background: 'oklch(0.75 0.18 150 / 8%)', border: '1px solid oklch(0.75 0.18 150 / 15%)' }}>
                  <TestTube className="w-3 h-3" style={{ color: 'oklch(0.75 0.18 150)' }} />
                  <span className="text-[10px] font-mono font-bold" style={{ color: 'oklch(0.75 0.18 150)' }}>{selectedCrate.tests} tests</span>
                </div>
                <div className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg"
                  style={{ background: 'oklch(0.82 0.15 192 / 8%)', border: '1px solid oklch(0.82 0.15 192 / 15%)' }}>
                  <CheckCircle2 className="w-3 h-3 text-gane-cyan" />
                  <span className="text-[10px] font-mono font-bold text-gane-cyan">Passing</span>
                </div>
              </div>
              <button
                onClick={() => setSelectedCrate(null)}
                className="w-full py-2.5 rounded-xl text-xs font-medium text-white/50 hover:text-white/70 transition-colors"
                style={{ background: 'oklch(1 0 0 / 4%)', border: '1px solid oklch(1 0 0 / 6%)' }}
              >
                Close
              </button>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
}

/* ─── Layer Card Component ─── */
function LayerCard({ layer, isExpanded, onToggle, onSelectCrate }: {
  layer: ReturnType<typeof getLayerStats>[0];
  isExpanded: boolean;
  onToggle: () => void;
  onSelectCrate: (c: Crate) => void;
}) {
  return (
    <div className="rounded-xl overflow-hidden transition-all"
      style={{
        background: isExpanded ? `${layer.color.replace(')', ' / 4%)')}` : 'oklch(1 0 0 / 2%)',
        border: `1px solid ${isExpanded ? layer.color.replace(')', ' / 15%)') : 'oklch(1 0 0 / 4%)'}` }}>
      <button onClick={onToggle} className="w-full flex items-center gap-3 px-4 py-3 text-left group">
        <span className="text-lg">{layer.icon}</span>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            <span className="text-xs font-bold text-white/70 group-hover:text-white/90 transition-colors">
              {layer.name}
            </span>
            <span className="text-[9px] text-white/15">{layer.nameHe}</span>
          </div>
          <div className="flex items-center gap-3 mt-0.5">
            <span className="text-[10px] text-white/25">
              <span className="font-bold font-mono" style={{ color: layer.color }}>{layer.totalCrates}</span> crates
            </span>
            <span className="text-[10px] text-white/25">
              <span className="font-bold font-mono" style={{ color: layer.color }}>{layer.totalTests}</span> tests
            </span>
          </div>
        </div>
        {isExpanded
          ? <ChevronDown className="w-3.5 h-3.5 text-white/20" />
          : <ChevronRight className="w-3.5 h-3.5 text-white/20" />
        }
      </button>

      <AnimatePresence>
        {isExpanded && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.25 }}
            className="overflow-hidden"
          >
            <div className="px-4 pb-3 pt-1">
              <p className="text-[10px] text-white/25 mb-2 leading-relaxed">{layer.description}</p>
              <div className="space-y-0.5">
                {layer.crates.map(crate => (
                  <button
                    key={crate.name}
                    onClick={() => onSelectCrate(crate)}
                    className="w-full flex items-start gap-2 px-2.5 py-2 rounded-lg hover:bg-white/3 transition-colors text-left group"
                  >
                    <Code2 className="w-3 h-3 mt-0.5 flex-shrink-0" style={{ color: layer.color }} />
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2">
                        <span className="text-[10px] font-mono font-bold text-white/50 group-hover:text-white/70 transition-colors">
                          {crate.name}
                        </span>
                        <span className="text-[8px] px-1.5 py-0.5 rounded-md font-medium font-mono"
                          style={{ background: `${layer.color.replace(')', ' / 10%)')}`, color: layer.color }}>
                          {crate.tests}t
                        </span>
                      </div>
                      <p className="text-[9px] text-white/20 mt-0.5 leading-relaxed">{crate.description}</p>
                    </div>
                    <CheckCircle2 className="w-3 h-3 flex-shrink-0 mt-0.5" style={{ color: 'oklch(0.75 0.18 150)' }} />
                  </button>
                ))}
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
