/**
 * G.A.N.E — Battery Status & Optimization Panel
 * ================================================
 * Real-time battery monitoring with power mode controls.
 * Connected to BatteryOptimizer engine.
 */
import { useState, useEffect, useCallback, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  X, Battery, BatteryCharging, BatteryWarning,
  Zap, Gauge, Clock, MapPin,
  TrendingDown, Shield, Activity
} from "lucide-react";
import { BatteryOptimizer, type PowerMode, type BatteryState } from "@/engine/batteryOptimizer";
import { useLanguage } from "@/contexts/LanguageContext";

const ACCENT = 'oklch(0.75 0.18 150)';
const ACCENT_AMBER = 'oklch(0.80 0.16 75)';
const ACCENT_RED = 'oklch(0.65 0.22 25)';
const ACCENT_CYAN = 'oklch(0.82 0.15 192)';

function PanelWrapper({ title, titleHe, icon: Icon, onClose, accentColor, children }: {
  title: string; titleHe?: string; icon: any; onClose: () => void; accentColor: string; children: React.ReactNode;
}) {
  const { t, dir } = useLanguage();
  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: "spring", damping: 28, stiffness: 300 }}
      className="expand-panel"
    >
      <div className="sticky top-0 z-10 px-5 pt-5 pb-3 relative overflow-hidden" style={{
        background: 'linear-gradient(180deg, oklch(0.07 0.015 264 / 99%) 0%, oklch(0.07 0.015 264 / 92%) 80%, transparent 100%)' }}>
        <motion.div className="absolute inset-0 pointer-events-none"
          style={{ background: `radial-gradient(ellipse at 20% 50%, color-mix(in oklch, ${accentColor}, transparent 92%), transparent 70%)` }}
          animate={{ opacity: [0.3, 0.6, 0.3] }}
          transition={{ duration: 4, repeat: Infinity, ease: 'easeInOut' }} />
        <motion.div className="absolute left-0 right-0 h-px pointer-events-none"
          style={{ background: `linear-gradient(90deg, transparent, ${accentColor}, transparent)` }}
          animate={{ top: ['0%', '100%'] }}
          transition={{ duration: 3, repeat: Infinity, ease: 'linear' }} />
        <div className="flex items-center gap-3 relative z-10">
          <motion.div
            initial={{ scale: 0.5, opacity: 0, rotate: -20 }}
            animate={{ scale: 1, opacity: 1, rotate: 0 }}
            transition={{ delay: 0.1, type: 'spring', stiffness: 350, damping: 15 }}
            className="w-10 h-10 rounded-xl flex items-center justify-center relative"
            style={{
              background: `color-mix(in oklch, ${accentColor}, transparent 82%)`,
              border: `1px solid color-mix(in oklch, ${accentColor}, transparent 60%)`,
              boxShadow: `0 0 20px color-mix(in oklch, ${accentColor}, transparent 75%)` }}>
            <Icon className="w-5 h-5" style={{ color: accentColor }} />
            <motion.div className="absolute inset-[-3px] rounded-[14px] pointer-events-none"
              style={{ border: `1px solid ${accentColor}` }}
              animate={{ scale: [1, 1.12, 1], opacity: [0.3, 0, 0.3] }}
              transition={{ duration: 2, repeat: Infinity }} />
          </motion.div>
          <div className="flex-1">
            <motion.h2 initial={{ opacity: 0, x: -12 }} animate={{ opacity: 1, x: 0 }}
              className="text-base font-bold text-white/90" style={{ fontFamily: 'Syne, sans-serif' }}>{title}</motion.h2>
            {titleHe && <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}
              className="text-[10px] text-white/25 mt-0.5" style={{ direction: dir }}>{titleHe}</motion.div>}
          </div>
          <motion.button onClick={onClose} whileHover={{ scale: 1.15, rotate: 90 }} whileTap={{ scale: 0.85 }}
            className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
            style={{ color: 'oklch(0.40 0.01 264)' }}>
            <X className="w-4 h-4" />
          </motion.button>
        </div>
        <motion.div className="mt-3 h-px w-full relative overflow-hidden" style={{ background: 'oklch(1 0 0 / 3%)' }}>
          <motion.div className="absolute inset-0"
            style={{ background: `linear-gradient(90deg, transparent, ${accentColor}, transparent)`, backgroundSize: '200% 100%' }}
            animate={{ backgroundPosition: ['200% 0', '-200% 0'] }}
            transition={{ duration: 4, repeat: Infinity, ease: 'linear' }} />
        </motion.div>
      </div>
      <div className="absolute inset-0 pointer-events-none quantum-grid opacity-40" />
      <motion.div className="px-5 pb-6 futuristic-scroll relative z-[1]"
        initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }}>
        {children}
      </motion.div>
    </motion.div>
  );
}

// ─── Battery ring visualization ───
function BatteryRing({ level, charging, size = 120 }: { level: number; charging: boolean; size?: number }) {
  const strokeWidth = 6;
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const offset = circumference - (level / 100) * circumference;
  const color = level > 50 ? ACCENT : level > 20 ? ACCENT_AMBER : ACCENT_RED;

  return (
    <div className="relative" style={{ width: size, height: size }}>
      <svg width={size} height={size} className="-rotate-90">
        <circle cx={size / 2} cy={size / 2} r={radius} fill="none" stroke="oklch(1 0 0 / 5%)" strokeWidth={strokeWidth} />
        <motion.circle
          cx={size / 2} cy={size / 2} r={radius} fill="none" stroke={color} strokeWidth={strokeWidth}
          strokeDasharray={circumference} strokeLinecap="round"
          initial={{ strokeDashoffset: circumference }}
          animate={{ strokeDashoffset: offset }}
          transition={{ duration: 1.5, ease: "easeOut" }}
        />
      </svg>
      <div className="absolute inset-0 flex flex-col items-center justify-center">
        {charging ? (
          <motion.div animate={{ scale: [1, 1.1, 1] }} transition={{ duration: 1.5, repeat: Infinity }}>
            <BatteryCharging className="w-6 h-6" style={{ color }} />
          </motion.div>
        ) : (
          <Battery className="w-6 h-6" style={{ color }} />
        )}
        <div className="text-xl font-black mt-1" style={{ color, fontFamily: 'JetBrains Mono, monospace' }}>
          {level}%
        </div>
        {charging && <div className="text-[9px] text-white/30">Charging</div>}
      </div>
    </div>
  );
}

const POWER_MODES: { id: PowerMode; label: string; i18nKey?: string; icon: typeof Zap; color: string; desc: string }[] = [
  { id: 'high_accuracy', label: 'Performance', i18nKey: 'power.performance', icon: Zap, color: ACCENT_CYAN, desc: '1s GPS, full sensors, max accuracy' },
  { id: 'balanced', label: 'Balanced', i18nKey: 'power.balanced', icon: Gauge, color: ACCENT, desc: '3s GPS, smart sensors, good accuracy' },
  { id: 'low_power', label: 'Power Saver', i18nKey: 'power.saver', icon: TrendingDown, color: ACCENT_AMBER, desc: '10s GPS, minimal sensors, basic nav' },
  { id: 'ultra_low', label: 'Ultra Saver', i18nKey: 'power.ultraSaver', icon: Shield, color: ACCENT_RED, desc: '60s GPS, essential only, max battery' },
];

export default function BatteryStatusPanel({ onClose }: { onClose: () => void }) {
  const { t, dir } = useLanguage();
  const optimizerRef = useRef<BatteryOptimizer | null>(null);
  const [state, setState] = useState<BatteryState>({
    level: 0.75,
    isCharging: false,
    powerMode: 'balanced',
    currentIntervalMs: 3000,
    isStationary: false,
    lastSpeedKmh: 0,
    samplesInBatch: 0,
    totalSamples: 0,
    totalBatches: 0,
    estimatedHoursRemaining: 0,
    powerSavingsPercent: 0,
  });
  const [uiRec, setUiRec] = useState({
    reduceAnimations: false,
    dimBrightness: false,
    disableParticles: false,
    reduceTileQuality: false,
    disableAutoRefresh: false,
  });

  useEffect(() => {
    const opt = new BatteryOptimizer();
    optimizerRef.current = opt;
    // Poll state every 2s
    const interval = setInterval(() => {
      setState(opt.getState());
      setUiRec(opt.getUIRecommendations());
    }, 2000);
    // Initial read
    setState(opt.getState());
    setUiRec(opt.getUIRecommendations());
    return () => {
      clearInterval(interval);
      opt.destroy();
    };
  }, []);

  const handleModeChange = useCallback((mode: PowerMode) => {
    optimizerRef.current?.setPowerMode(mode);
    if (optimizerRef.current) {
      setState(optimizerRef.current.getState());
      setUiRec(optimizerRef.current.getUIRecommendations());
    }
  }, []);

  const batteryPct = Math.round(state.level * 100);
  const batteryColor = batteryPct > 50 ? ACCENT : batteryPct > 20 ? ACCENT_AMBER : ACCENT_RED;

  // Determine recommended mode based on battery level
  const recommendedMode: PowerMode =
    state.level < 0.1 ? 'ultra_low' :
    state.level < 0.2 ? 'low_power' :
    state.level < 0.5 ? 'balanced' :
    'high_accuracy';

  return (
    <PanelWrapper title="Battery & Power" titleHe="סוללה וחשמל" icon={Battery} onClose={onClose} accentColor={batteryColor}>
      {/* Battery visualization */}
      <div className="flex justify-center mb-4 mt-2">
        <BatteryRing level={batteryPct} charging={state.isCharging} />
      </div>

      {/* Quick stats */}
      <div className="grid grid-cols-3 gap-2 mb-4">
        {[
          { label: 'Est. Time', value: `${state.estimatedHoursRemaining.toFixed(1)}h`, icon: Clock, color: batteryColor },
          { label: 'GPS Rate', value: `${(state.currentIntervalMs / 1000).toFixed(0)}s`, icon: MapPin, color: ACCENT_CYAN },
          { label: 'Savings', value: `${state.powerSavingsPercent.toFixed(0)}%`, icon: Activity, color: ACCENT },
        ].map((s, i) => (
          <motion.div
            key={s.label}
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: i * 0.08 }}
            className="feature-card flex flex-col items-center py-3"
          >
            <s.icon className="w-4 h-4 mb-1" style={{ color: s.color }} />
            <div className="text-sm font-bold" style={{ color: s.color, fontFamily: 'JetBrains Mono, monospace' }}>{s.value}</div>
            <div className="text-[9px] text-white/25 mt-0.5">{s.label}</div>
          </motion.div>
        ))}
      </div>

      {/* Recommendation banner */}
      {recommendedMode !== state.powerMode && !state.isCharging && (
        <motion.div
          initial={{ opacity: 0, height: 0 }}
          animate={{ opacity: 1, height: 'auto' }}
          className="feature-card mb-4 flex items-center gap-3"
          style={{ borderColor: `color-mix(in oklch, ${ACCENT_AMBER}, transparent 50%)` }}
        >
          <BatteryWarning className="w-5 h-5 flex-shrink-0" style={{ color: ACCENT_AMBER }} />
          <div className="flex-1">
            <div className="text-xs text-white/60">
              Recommended: <span className="font-bold" style={{ color: ACCENT_AMBER }}>
                {POWER_MODES.find(m => m.id === recommendedMode)?.label}
              </span>
            </div>
            <div className="text-[10px] text-white/25" style={{ direction: dir }}>
              מומלץ לעבור למצב חיסכון
            </div>
          </div>
          <motion.button
            onClick={() => handleModeChange(recommendedMode)}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className="px-3 py-1.5 rounded-lg text-[10px] font-bold cursor-pointer"
            style={{
              background: `color-mix(in oklch, ${ACCENT_AMBER}, transparent 80%)`,
              border: `1px solid color-mix(in oklch, ${ACCENT_AMBER}, transparent 50%)`,
              color: ACCENT_AMBER,
            }}
          >
            Switch
          </motion.button>
        </motion.div>
      )}

      {/* Power mode selector */}
      <div className="text-xs text-white/30 uppercase tracking-wider mb-3">Power Mode</div>
      <div className="space-y-2 mb-4">
        {POWER_MODES.map((mode, i) => {
          const isActive = state.powerMode === mode.id;
          const isRecommended = recommendedMode === mode.id;
          return (
            <motion.button
              key={mode.id}
              onClick={() => handleModeChange(mode.id)}
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: i * 0.05 }}
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
              className="w-full feature-card flex items-center gap-3 py-3 cursor-pointer transition-all"
              style={{
                borderColor: isActive ? `color-mix(in oklch, ${mode.color}, transparent 40%)` : undefined,
                background: isActive ? `color-mix(in oklch, ${mode.color}, transparent 92%)` : undefined,
              }}
            >
              <div className="w-9 h-9 rounded-lg flex items-center justify-center flex-shrink-0"
                style={{
                  background: isActive ? `color-mix(in oklch, ${mode.color}, transparent 80%)` : 'oklch(1 0 0 / 5%)',
                  border: `1px solid ${isActive ? `color-mix(in oklch, ${mode.color}, transparent 60%)` : 'oklch(1 0 0 / 6%)'}`,
                }}>
                <mode.icon className="w-4 h-4" style={{ color: isActive ? mode.color : 'oklch(1 0 0 / 30%)' }} />
              </div>
              <div className="flex-1 text-left">
                <div className="flex items-center gap-2">
                  <span className="text-xs font-semibold" style={{ color: isActive ? 'oklch(1 0 0 / 80%)' : 'oklch(1 0 0 / 50%)' }}>
                    {mode.label}
                  </span>
                  {isRecommended && !isActive && (
                    <span className="text-[8px] px-1.5 py-0.5 rounded-full font-bold"
                      style={{ background: `color-mix(in oklch, ${ACCENT_AMBER}, transparent 80%)`, color: ACCENT_AMBER }}>
                      REC
                    </span>
                  )}
                </div>
                <div className="text-[10px] text-white/25">{mode.desc}</div>
              </div>
              <div className="text-[10px] text-white/15" style={{ direction: dir }}>{mode.i18nKey}</div>
            </motion.button>
          );
        })}
      </div>

      {/* UI Recommendations */}
      <div className="text-xs text-white/30 uppercase tracking-wider mb-3">UI Optimizations</div>
      <div className="feature-card mb-4">
        <div className="grid grid-cols-2 gap-3">
          {[
            { label: 'Reduce Animations', active: uiRec.reduceAnimations },
            { label: 'Dim Brightness', active: uiRec.dimBrightness },
            { label: 'Disable Particles', active: uiRec.disableParticles },
            { label: 'Low-Res Tiles', active: uiRec.reduceTileQuality },
          ].map(opt => (
            <div key={opt.label} className="flex items-center gap-2">
              <div className="w-2 h-2 rounded-full" style={{
                background: opt.active ? ACCENT_AMBER : 'oklch(1 0 0 / 10%)',
                boxShadow: opt.active ? `0 0 6px color-mix(in oklch, ${ACCENT_AMBER}, transparent 50%)` : 'none',
              }} />
              <span className="text-[10px]" style={{ color: opt.active ? 'oklch(1 0 0 / 50%)' : 'oklch(1 0 0 / 20%)' }}>
                {opt.label}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Telemetry stats */}
      <div className="text-xs text-white/30 uppercase tracking-wider mb-3">Telemetry</div>
      <div className="feature-card">
        <div className="grid grid-cols-3 gap-3">
          {[
            { label: 'Samples', value: state.totalSamples },
            { label: 'Batches', value: state.totalBatches },
            { label: 'In Queue', value: state.samplesInBatch },
          ].map(s => (
            <div key={s.label} className="text-center">
              <div className="text-sm font-bold text-white/50" style={{ fontFamily: 'JetBrains Mono, monospace' }}>{s.value}</div>
              <div className="text-[9px] text-white/20">{s.label}</div>
            </div>
          ))}
        </div>
        {state.isStationary && (
          <div className="mt-3 text-center">
            <span className="text-[10px] px-2 py-1 rounded-full"
              style={{ background: `color-mix(in oklch, ${ACCENT}, transparent 85%)`, color: ACCENT }}>
              Stationary — GPS interval extended
            </span>
          </div>
        )}
      </div>
    </PanelWrapper>
  );
}
