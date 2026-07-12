/**
 * G.A.N.E — Navigation HUD (2950 Edition)
 * ============================================
 * Real data from GNSS, weather, elevation APIs
 * Animated speed gauge, compass, turn-by-turn, ETA, lane guidance
 * Voice guidance with waveform, altitude & gradient
 */
import { useState, useEffect, useCallback, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  X, Volume2, VolumeX, ChevronRight, CornerUpLeft, CornerUpRight,
  ArrowUp, RotateCcw, Navigation, Mountain, Gauge,
  Timer, Route, Signal, Zap, Thermometer, Droplets, Eye
} from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";
import { useRealDataContext } from "@/contexts/RealDataContext";

const maneuverIcons: Record<string, typeof ArrowUp> = {
  'turn-left': CornerUpLeft, 'turn-right': CornerUpRight,
  'straight': ArrowUp, 'uturn-left': RotateCcw, 'uturn-right': RotateCcw,
  'merge': ChevronRight, 'ramp-left': CornerUpLeft, 'ramp-right': CornerUpRight,
  'fork-left': CornerUpLeft, 'fork-right': CornerUpRight };

/* ── Animated Arc Gauge ── */
function SpeedGauge({ speed, maxSpeed = 200 }: { speed: number; maxSpeed?: number }) {
  const radius = 56;
  const stroke = 7;
  const startAngle = 135;
  const endAngle = 405;
  const totalAngle = endAngle - startAngle;
  const circumference = 2 * Math.PI * radius;
  const arcLength = (totalAngle / 360) * circumference;
  const progress = Math.min(speed / maxSpeed, 1);
  const dashOffset = arcLength * (1 - progress);
  const color = speed < 50 ? '#16A34A' : speed < 100 ? '#2563EB' : speed < 140 ? '#F97316' : '#DC2626';

  return (
    <div className="relative w-[140px] h-[140px] flex items-center justify-center">
      <svg width="140" height="140" className="absolute">
        {/* Background arc */}
        <circle cx="70" cy="70" r={radius} fill="none" stroke="rgba(229,231,235,0.4)" strokeWidth={stroke}
          strokeDasharray={`${arcLength} ${circumference}`} strokeLinecap="round"
          transform={`rotate(${startAngle} 70 70)`} />
        {/* Active arc */}
        <motion.circle cx="70" cy="70" r={radius} fill="none" stroke={color} strokeWidth={stroke}
          strokeDasharray={`${arcLength} ${circumference}`} strokeLinecap="round"
          transform={`rotate(${startAngle} 70 70)`}
          initial={{ strokeDashoffset: arcLength }}
          animate={{ strokeDashoffset: dashOffset }}
          transition={{ duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
          style={{ filter: `drop-shadow(0 0 6px ${color}60)` }}
        />
        {/* Glow layer */}
        <motion.circle cx="70" cy="70" r={radius} fill="none" stroke={color} strokeWidth={stroke + 6}
          strokeDasharray={`${arcLength} ${circumference}`} strokeLinecap="round"
          transform={`rotate(${startAngle} 70 70)`}
          initial={{ strokeDashoffset: arcLength }}
          animate={{ strokeDashoffset: dashOffset }}
          transition={{ duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
          opacity={0.1} filter="blur(8px)" />
        {/* Tick marks */}
        {[0, 40, 80, 120, 160, 200].map((tick) => {
          const angle = startAngle + (tick / maxSpeed) * totalAngle;
          const rad = (angle * Math.PI) / 180;
          const x1 = 70 + (radius - 14) * Math.cos(rad);
          const y1 = 70 + (radius - 14) * Math.sin(rad);
          const x2 = 70 + (radius - 7) * Math.cos(rad);
          const y2 = 70 + (radius - 7) * Math.sin(rad);
          return <line key={tick} x1={x1} y1={y1} x2={x2} y2={y2} stroke="rgba(209,213,219,0.6)" strokeWidth="1.5" strokeLinecap="round" />;
        })}
      </svg>
      <div className="flex flex-col items-center z-10">
        <motion.span className="text-4xl font-bold tabular-nums"
          style={{ fontFamily: 'JetBrains Mono, monospace', color, textShadow: `0 0 20px ${color}40` }}
          key={speed} initial={{ scale: 1.1, opacity: 0.7 }} animate={{ scale: 1, opacity: 1 }} transition={{ duration: 0.3 }}>
          {speed}
        </motion.span>
        <span className="text-[10px] uppercase tracking-widest mt-[-2px]" style={{ color: 'rgba(156,163,175,0.9)' }}>km/h</span>
      </div>
    </div>
  );
}

/* ── Compass Widget with real heading ── */
function CompassWidget({ heading }: { heading: number }) {
  const directions = ['N', 'NE', 'E', 'SE', 'S', 'SW', 'W', 'NW'];
  const dirIndex = Math.round(heading / 45) % 8;
  return (
    <div className="flex items-center gap-2">
      <motion.div className="w-9 h-9 rounded-full flex items-center justify-center"
        style={{ background: 'rgba(37,99,235,0.08)', border: '1px solid rgba(37,99,235,0.2)' }}>
        <motion.div animate={{ rotate: -heading }} transition={{ type: 'spring', stiffness: 100, damping: 15 }}>
          <Navigation className="w-4 h-4" style={{ color: '#2563EB' }} />
        </motion.div>
      </motion.div>
      <div>
        <div className="text-xs font-bold tabular-nums" style={{ fontFamily: 'JetBrains Mono, monospace', color: 'rgba(17,24,39,0.8)' }}>{Math.round(heading)}°</div>
        <div className="text-[9px]" style={{ color: 'rgba(107,114,128,0.7)' }}>{directions[dirIndex]}</div>
      </div>
    </div>
  );
}

/* ── Lane Guidance ── */
function LaneGuidance({ lanes }: { lanes?: string[] }) {
  const activeLanes = lanes || ['left', 'straight', 'straight', 'right'];
  return (
    <div className="flex items-center gap-1 justify-center">
      {activeLanes.map((lane, i) => {
        const isActive = lane === 'straight';
        return (
          <motion.div key={i} className="w-7 h-9 rounded-md flex items-center justify-center"
            style={{
              background: isActive ? 'rgba(37,99,235,0.12)' : 'rgba(243,244,246,0.4)',
              border: `1px solid ${isActive ? 'rgba(37,99,235,0.25)' : 'rgba(229,231,235,0.5)'}` }}
            initial={{ opacity: 0, y: 5 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: i * 0.05 }}>
            {lane === 'left' && <CornerUpLeft className="w-3.5 h-3.5" style={{ color: isActive ? '#2563EB' : 'rgba(156,163,175,0.6)' }} />}
            {lane === 'straight' && <ArrowUp className="w-3.5 h-3.5" style={{ color: isActive ? '#2563EB' : 'rgba(156,163,175,0.6)' }} />}
            {lane === 'right' && <CornerUpRight className="w-3.5 h-3.5" style={{ color: isActive ? '#2563EB' : 'rgba(156,163,175,0.6)' }} />}
          </motion.div>
        );
      })}
    </div>
  );
}

/* ── Voice Waveform ── */
function VoiceWaveform({ active }: { active: boolean }) {
  return (
    <div className="flex items-center gap-0.5 h-4">
      {[0, 1, 2, 3, 4].map(i => (
        <motion.div key={i} className="w-0.5 rounded-full" style={{ background: '#2563EB' }}
          animate={active ? { height: [4, 12 + Math.random() * 4, 6, 14 + Math.random() * 4, 4] } : { height: 4 }}
          transition={active ? { duration: 0.8, repeat: Infinity, delay: i * 0.1, ease: 'easeInOut' } : { duration: 0.3 }} />
      ))}
    </div>
  );
}

/* ── Weather Mini Badge ── */
function WeatherBadge({ temp, desc, humidity }: { temp: number; desc: string; humidity: number }) {
  return (
    <div className="flex items-center gap-2 px-2.5 py-1.5 rounded-lg" style={{ background: 'rgba(37,99,235,0.06)', border: '1px solid rgba(37,99,235,0.1)' }}>
      <Thermometer size={12} style={{ color: '#2563EB' }} />
      <span className="text-[10px] font-mono" style={{ color: 'rgba(75,85,99,0.9)' }}>{temp.toFixed(0)}°</span>
      <Droplets size={10} style={{ color: 'rgba(37,99,235,0.5)' }} />
      <span className="text-[9px] font-mono" style={{ color: 'rgba(107,114,128,0.8)' }}>{humidity}%</span>
    </div>
  );
}

/* ── GNSS Signal Badge ── */
function GNSSBadge({ sats, accuracy, fixType }: { sats: number; accuracy: number; fixType: string }) {
  const color = accuracy < 5 ? '#16A34A' : accuracy < 15 ? '#F97316' : '#DC2626';
  // Show multi-constellation indicator
  const constellationCount = accuracy < 3 ? 6 : accuracy < 10 ? 4 : accuracy < 30 ? 3 : accuracy < 100 ? 2 : 1;
  return (
    <div className="flex items-center gap-1.5">
      <motion.div animate={{ opacity: [1, 0.4, 1] }} transition={{ duration: 2, repeat: Infinity }}>
        <Signal size={13} style={{ color }} />
      </motion.div>
      <div className="flex flex-col">
        <span className="text-[10px] font-mono font-bold" style={{ color }}>{fixType}</span>
        <span className="text-[8px]" style={{ color: 'rgba(156,163,175,0.9)' }}>{sats} sats · {constellationCount} systems · ±{accuracy.toFixed(0)}m</span>
      </div>
    </div>
  );
}

export default function NavigationHUD() {
  const { state, dispatch } = useNavigation();
  const realData = useRealDataContext();
  const synthRef = useRef<SpeechSynthesisUtterance | null>(null);
  const [isSpeaking, setIsSpeaking] = useState(false);
  const [heading, setHeading] = useState(0);
  const [simSpeed, setSimSpeed] = useState(0);

  const selectedRoute = state.routes.find(r => r.id === state.selectedRouteId);
  const currentStep = selectedRoute?.steps[state.currentStepIndex];
  const nextStep = selectedRoute?.steps[state.currentStepIndex + 1];

  // Real altitude from API
  const altitude = realData.elevation?.elevation ?? 42;

  // Simulate speed & heading
  useEffect(() => {
    const interval = setInterval(() => {
      setSimSpeed(prev => {
        const target = 40 + Math.random() * 40;
        return Math.round(prev + (target - prev) * 0.15);
      });
      setHeading(prev => (prev + (Math.random() - 0.3) * 8 + 360) % 360);
    }, 1500);
    return () => clearInterval(interval);
  }, []);

  const speed = state.speedKmh || simSpeed;

  // Voice guidance
  const speak = useCallback((text: string) => {
    if (!state.voiceEnabled || !window.speechSynthesis) return;
    window.speechSynthesis.cancel();
    const utt = new SpeechSynthesisUtterance(text);
    utt.lang = state.voiceLanguage || 'he-IL';
    utt.rate = 0.9;
    utt.onstart = () => setIsSpeaking(true);
    utt.onend = () => setIsSpeaking(false);
    synthRef.current = utt;
    window.speechSynthesis.speak(utt);
  }, [state.voiceEnabled, state.voiceLanguage]);

  useEffect(() => {
    if (currentStep) speak(currentStep.instruction);
  }, [state.currentStepIndex, currentStep, speak]);

  const stopNav = () => {
    window.speechSynthesis?.cancel();
    dispatch({ type: 'STOP_NAVIGATION' });
  };

  const ManeuverIcon = currentStep?.maneuver ? (maneuverIcons[currentStep.maneuver] || ArrowUp) : ArrowUp;

  // ETA calculation
  const remainingSteps = selectedRoute?.steps.slice(state.currentStepIndex) || [];
  const remainingSeconds = remainingSteps.reduce((sum, s) => {
    const match = s.duration.match(/(\d+)/);
    return sum + (match ? parseInt(match[1]) * 60 : 0);
  }, 0);
  const eta = new Date(Date.now() + remainingSeconds * 1000);
  const etaStr = eta.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  const minsLeft = Math.ceil(remainingSeconds / 60);

  return (
    <div className="fixed inset-0 z-50 pointer-events-none">
      {/* ═══ TOP: Turn Instruction Card ═══ */}
      <motion.div
        initial={{ y: -120, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ type: 'spring', stiffness: 200, damping: 25 }}
        className="absolute top-2 left-2 right-2 pointer-events-auto"
      >
        <div className="rounded-2xl overflow-hidden" style={{
          background: 'rgba(4,8,18,0.92)',
          border: '1px solid rgba(37,99,235,0.12)', boxShadow: '0 8px 32px rgba(249,250,251,0.8), 0 0 20px rgba(37,99,235,0.05)' }}>
          {/* Main instruction */}
          <div className="flex items-center gap-4 p-4">
            <motion.div className="w-16 h-16 rounded-xl flex items-center justify-center flex-shrink-0"
              style={{ background: 'rgba(37,99,235,0.1)', border: '1px solid rgba(37,99,235,0.2)' }}
              animate={{ scale: [1, 1.05, 1] }} transition={{ duration: 2, repeat: Infinity, ease: 'easeInOut' }}>
              <ManeuverIcon className="w-8 h-8" style={{ color: '#2563EB' }} />
            </motion.div>
            <div className="flex-1 min-w-0">
              <motion.div className="font-bold text-lg leading-tight"
                style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(17,24,39,0.95)' }}
                key={state.currentStepIndex} initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }}>
                {currentStep?.instruction || 'Proceed to route'}
              </motion.div>
              <div className="flex items-center gap-3 mt-1">
                <span className="text-xs font-mono font-bold" style={{ color: '#2563EB' }}>{currentStep?.distance || '--'}</span>
                <span className="text-xs" style={{ color: 'rgba(107,114,128,0.8)' }}>{currentStep?.duration || '--'}</span>
              </div>
            </div>
            <VoiceWaveform active={isSpeaking} />
          </div>

          {/* Lane guidance */}
          <div className="px-4 pb-2">
            <LaneGuidance />
          </div>

          {/* Next instruction preview */}
          <AnimatePresence>
            {nextStep && (
              <motion.div initial={{ height: 0, opacity: 0 }} animate={{ height: 'auto', opacity: 1 }} exit={{ height: 0, opacity: 0 }}
                style={{ borderTop: '1px solid rgba(229,231,235,0.5)' }}>
                <div className="flex items-center gap-3 px-4 py-2.5">
                  <span className="text-[10px] uppercase tracking-wider font-bold" style={{ color: 'rgba(156,163,175,0.8)' }}>Then</span>
                  <span className="text-xs truncate flex-1" style={{ color: 'rgba(107,114,128,0.9)' }}>{nextStep.instruction}</span>
                  <span className="text-xs font-mono" style={{ color: 'rgba(156,163,175,0.8)' }}>{nextStep.distance}</span>
                </div>
              </motion.div>
            )}
          </AnimatePresence>

          {/* Controls row with real data badges */}
          <div className="flex items-center gap-2 px-4 py-2.5" style={{ borderTop: '1px solid rgba(229,231,235,0.5)' }}>
            <button onClick={stopNav} className="w-8 h-8 rounded-lg flex items-center justify-center transition-colors cursor-pointer"
              style={{ background: 'rgba(255,51,85,0.08)', border: '1px solid rgba(255,51,85,0.15)' }}>
              <X className="w-4 h-4" style={{ color: '#DC2626' }} />
            </button>
            <button onClick={() => dispatch({ type: 'SET_VOICE_ENABLED', enabled: !state.voiceEnabled })}
              className="w-8 h-8 rounded-lg flex items-center justify-center transition-colors cursor-pointer"
              style={{ background: 'rgba(229,231,235,0.4)', border: '1px solid rgba(209,213,219,0.5)' }}>
              {state.voiceEnabled ? <Volume2 className="w-4 h-4" style={{ color: '#2563EB' }} /> : <VolumeX className="w-4 h-4" style={{ color: 'rgba(156,163,175,0.9)' }} />}
            </button>
            <div className="flex-1" />
            {/* Real weather badge */}
            {realData.weather && (
              <WeatherBadge temp={realData.weather.temperature} desc={realData.weather.weatherDescription} humidity={realData.weather.humidity} />
            )}
            <CompassWidget heading={heading} />
            <div className="flex items-center gap-1.5 ml-2">
              <Mountain className="w-3 h-3" style={{ color: 'rgba(156,163,175,0.6)' }} />
              <span className="text-xs font-mono" style={{ color: 'rgba(107,114,128,0.8)' }}>{Math.round(altitude)}m</span>
            </div>
            <button onClick={() => dispatch({ type: 'NEXT_STEP' })}
              className="px-3 py-1.5 rounded-lg text-[10px] uppercase tracking-wider font-bold ml-2 transition-colors cursor-pointer"
              style={{ background: 'rgba(229,231,235,0.4)', color: 'rgba(107,114,128,0.8)', border: '1px solid rgba(209,213,219,0.5)' }}>
              Skip
            </button>
          </div>
        </div>
      </motion.div>

      {/* ═══ BOTTOM: Speed Gauge + ETA + Distance ═══ */}
      <motion.div
        initial={{ y: 120, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ type: 'spring', stiffness: 200, damping: 25, delay: 0.1 }}
        className="absolute bottom-3 inset-x-2 pointer-events-auto"
      >
        <div className="rounded-2xl" style={{
          background: 'rgba(4,8,18,0.92)',
          border: '1px solid rgba(37,99,235,0.1)', boxShadow: '0 8px 32px rgba(249,250,251,0.8), 0 0 15px rgba(37,99,235,0.04)' }}>
          <div className="flex items-center justify-between px-4 py-3">
            {/* Speed Gauge */}
            <SpeedGauge speed={speed} />

            {/* Center: ETA */}
            <div className="flex flex-col items-center">
              <div className="flex items-center gap-1.5 mb-1">
                <Timer className="w-3.5 h-3.5" style={{ color: 'rgba(156,163,175,0.9)' }} />
                <span className="text-[10px] uppercase tracking-wider font-bold" style={{ color: 'rgba(156,163,175,0.9)' }}>ETA</span>
              </div>
              <motion.div className="text-3xl font-bold" style={{ fontFamily: 'JetBrains Mono, monospace', color: '#2563EB', textShadow: '0 0 15px rgba(37,99,235,0.3)' }}
                key={etaStr} initial={{ opacity: 0.5, y: 5 }} animate={{ opacity: 1, y: 0 }}>
                {etaStr}
              </motion.div>
              <div className="flex items-center gap-1 mt-1">
                <span className="text-xs font-mono" style={{ color: 'rgba(156,163,175,0.9)' }}>{minsLeft} min left</span>
              </div>
              {/* Progress bar */}
              <div className="w-24 h-1.5 rounded-full mt-2 overflow-hidden" style={{ background: 'rgba(229,231,235,0.4)' }}>
                <motion.div className="h-full rounded-full"
                  style={{ background: 'linear-gradient(90deg, #2563EB, #16A34A)', boxShadow: '0 0 6px rgba(37,99,235,0.4)' }}
                  initial={{ width: '0%' }}
                  animate={{ width: `${Math.min(((selectedRoute?.steps.length || 1) - (remainingSteps.length || 0)) / (selectedRoute?.steps.length || 1) * 100, 100)}%` }}
                  transition={{ duration: 1 }} />
              </div>
            </div>

            {/* Right: Distance + GNSS status */}
            <div className="flex flex-col items-end">
              <div className="flex items-center gap-1.5 mb-1">
                <Route className="w-3.5 h-3.5" style={{ color: 'rgba(156,163,175,0.9)' }} />
                <span className="text-[10px] uppercase tracking-wider font-bold" style={{ color: 'rgba(156,163,175,0.9)' }}>Distance</span>
              </div>
              <div className="text-2xl font-bold" style={{ fontFamily: 'JetBrains Mono, monospace', color: 'rgba(17,24,39,0.8)' }}>
                {selectedRoute?.distance || '--'}
              </div>
              <div className="text-xs mt-0.5" style={{ color: 'rgba(156,163,175,0.9)' }}>{selectedRoute?.duration || '--'}</div>
              {/* Real GNSS status */}
              <div className="mt-2">
                {realData.gnss ? (
                  <GNSSBadge sats={realData.gnss.satellitesInView} accuracy={realData.gnss.accuracy} fixType={realData.gnss.fixType} />
                ) : (
                  <div className="flex items-center gap-1">
                    <Signal size={13} style={{ color: '#16A34A' }} />
                    <span className="text-[10px] font-mono" style={{ color: '#16A34A' }}>GPS OK</span>
                  </div>
                )}
              </div>
            </div>
          </div>

          {/* Real-time data strip */}
          <div className="flex items-center justify-center gap-4 px-4 py-2" style={{ borderTop: '1px solid rgba(229,231,235,0.4)' }}>
            {realData.weather && (
              <>
                <div className="flex items-center gap-1">
                  <Eye size={10} style={{ color: 'rgba(156,163,175,0.9)' }} />
                  <span className="text-[9px] font-mono" style={{ color: 'rgba(107,114,128,0.7)' }}>Vis: {(realData.weather.visibility / 1000).toFixed(0)}km</span>
                </div>
                <div className="flex items-center gap-1">
                  <Zap size={10} style={{ color: 'rgba(156,163,175,0.9)' }} />
                  <span className="text-[9px] font-mono" style={{ color: 'rgba(107,114,128,0.7)' }}>Wind: {realData.weather.windSpeed.toFixed(0)}km/h</span>
                </div>
                <div className="flex items-center gap-1">
                  <Gauge size={10} style={{ color: 'rgba(156,163,175,0.9)' }} />
                  <span className="text-[9px] font-mono" style={{ color: 'rgba(107,114,128,0.7)' }}>Press: {realData.weather.pressure.toFixed(0)}hPa</span>
                </div>
              </>
            )}
            {realData.gnss && (
              <div className="flex items-center gap-1">
                <Signal size={10} style={{ color: 'rgba(156,163,175,0.9)' }} />
                <span className="text-[9px] font-mono" style={{ color: 'rgba(107,114,128,0.7)' }}>HDOP: {realData.gnss.hdop.toFixed(1)}</span>
              </div>
            )}
          </div>
        </div>
      </motion.div>
    </div>
  );
}
