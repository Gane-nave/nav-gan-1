/**
 * G.A.N.E — Onboarding Screen
 * Premium animated welcome flow with feature highlights + guided panel walkthrough.
 * Design: Orbital Intelligence — cinematic intro with G.A.N.E gradients.
 */
import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Navigation, Orbit, ShieldCheck, Zap, ChevronRight, Volume2,
  Hexagon, Bolt, Atom, Antenna, Diamond, Gauge,
  TowerControl, Gem, Crosshair, Layers, Sparkles, CircuitBoard,
  Cpu, BrainCircuit, Signal, MapPin
} from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";

const SPLASH_IMG = "https://d2xsxph8kpxj0f.cloudfront.net/97884477/Po7Xs3DDU8PjnGuUqXZUkr/gane-splash-hBzqJSjSWnGTe8e63Tu9wy.webp";

const featureSlides = [
  {
    icon: Navigation, title: "Intelligent Routing",
    description: "AI-powered probabilistic routing that learns from millions of trips. Get the fastest, safest, or most eco-friendly path — every time.",
    color: "oklch(0.82 0.15 192)" },
  {
    icon: Orbit, title: "43+ Languages",
    description: "Navigate in your language. Full voice guidance and interface translation for over 43 languages, including RTL support for Hebrew and Arabic.",
    color: "oklch(0.55 0.22 264)" },
  {
    icon: Volume2, title: "Voice Navigation",
    description: "Crystal-clear turn-by-turn voice guidance with natural speech synthesis. Hands-free navigation that keeps you safe on the road.",
    color: "oklch(0.75 0.18 150)" },
  {
    icon: ShieldCheck, title: "Safety First",
    description: "Real-time hazard detection, speed limit warnings, and driver fatigue monitoring. Your safety is our top priority.",
    color: "oklch(0.80 0.16 75)" },
  {
    icon: Zap, title: "Real-Time Traffic",
    description: "Live traffic intelligence from millions of sensors and connected vehicles. Avoid congestion before it happens.",
    color: "oklch(0.65 0.22 25)" },
];

const panelGuide = [
  { icon: Hexagon, name: "Smart Parking", desc: "Find available spots in real-time with occupancy prediction", color: "oklch(0.82 0.15 192)" },
  { icon: Bolt, name: "EV Charging", desc: "Locate charging stations with live availability and pricing", color: "oklch(0.75 0.18 150)" },
  { icon: Atom, name: "Weather", desc: "Hyper-local weather affecting your route and driving conditions", color: "oklch(0.72 0.14 180)" },
  { icon: Antenna, name: "V2X Network", desc: "Vehicle-to-everything communication for intersection safety", color: "oklch(0.55 0.22 264)" },
  { icon: Diamond, name: "Digital Twin", desc: "3D city model with real-time traffic simulation", color: "oklch(0.72 0.14 180)" },
  { icon: Gauge, name: "Driver Score", desc: "Gamified driving analytics with safety and eco scoring", color: "oklch(0.80 0.16 75)" },
  { icon: TowerControl, name: "Fleet Command", desc: "Multi-vehicle fleet management and optimization", color: "oklch(0.55 0.22 264)" },
  { icon: Gem, name: "Wallet", desc: "Integrated payments for tolls, parking, and fuel", color: "oklch(0.80 0.16 75)" },
  { icon: Crosshair, name: "Report Incident", desc: "Dashcam evidence capture and incident reporting", color: "oklch(0.65 0.22 25)" },
  { icon: Orbit, name: "Transport Modes", desc: "Multi-modal routing: bus, train, bike, scooter, and walk", color: "oklch(0.60 0.25 300)" },
  { icon: Layers, name: "Map Layers", desc: "Toggle satellite, traffic, transit, and cycling overlays", color: "oklch(0.72 0.14 180)" },
  { icon: Sparkles, name: "Analytics", desc: "Trip history, patterns, and driving insights dashboard", color: "oklch(0.80 0.16 75)" },
  { icon: CircuitBoard, name: "Spec Vault", desc: "Complete engineering specification with PDF export", color: "oklch(0.60 0.25 300)" },
  { icon: Cpu, name: "System Architecture", desc: "2,246 Rust crates across 20 layers with build pipeline", color: "oklch(0.72 0.14 120)" },
  { icon: BrainCircuit, name: "Command Center", desc: "Mission control with live metrics and AI model status", color: "oklch(0.82 0.15 192)" },
];

type Phase = 'splash' | 'features' | 'panels' | 'ready';

export default function OnboardingScreen() {
  const { dispatch } = useNavigation();
  const [phase, setPhase] = useState<Phase>('splash');
  const [featureIdx, setFeatureIdx] = useState(0);
  const [panelPage, setPanelPage] = useState(0);
  const [typedText, setTypedText] = useState('');

  // Typing effect for the "ready" phase
  useEffect(() => {
    if (phase !== 'ready') return;
    const text = 'G.A.N.E is ready.';
    let i = 0;
    const interval = setInterval(() => {
      setTypedText(text.slice(0, i + 1));
      i++;
      if (i >= text.length) clearInterval(interval);
    }, 60);
    return () => clearInterval(interval);
  }, [phase]);

  const panelPages = [
    panelGuide.slice(0, 5),
    panelGuide.slice(5, 10),
    panelGuide.slice(10, 15),
  ];

  const next = () => {
    switch (phase) {
      case 'splash':
        setPhase('features');
        break;
      case 'features':
        if (featureIdx < featureSlides.length - 1) {
          setFeatureIdx(featureIdx + 1);
        } else {
          setPhase('panels');
        }
        break;
      case 'panels':
        if (panelPage < panelPages.length - 1) {
          setPanelPage(panelPage + 1);
        } else {
          setPhase('ready');
        }
        break;
      case 'ready':
        dispatch({ type: 'SET_ONBOARDING', show: false });
        break;
    }
  };

  const skip = () => {
    dispatch({ type: 'SET_ONBOARDING', show: false });
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 z-[100] flex flex-col"
      style={{ background: 'oklch(0.08 0.02 264)' }}
    >
      {/* G.A.N.E ambient background */}
      <div className="absolute inset-0 pointer-events-none overflow-hidden">
        <motion.div
          animate={{ scale: [1, 1.1, 1], opacity: [0.04, 0.08, 0.04] }}
          transition={{ duration: 12, repeat: Infinity, ease: 'easeInOut' }}
          className="absolute -top-1/4 -left-1/4 w-[150%] h-[150%]"
          style={{
            background: 'radial-gradient(ellipse at 30% 70%, oklch(0.82 0.15 192 / 15%) 0%, transparent 50%), radial-gradient(ellipse at 70% 30%, oklch(0.55 0.22 264 / 12%) 0%, transparent 50%)' }}
        />
      </div>

      <AnimatePresence mode="wait">
        {/* ═══ SPLASH ═══ */}
        {phase === 'splash' && (
          <motion.div key="splash" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0, scale: 1.05 }}
            className="flex-1 flex flex-col items-center justify-center relative overflow-hidden">
            <div className="absolute inset-0">
              <img src={SPLASH_IMG} alt="" className="w-full h-full object-cover opacity-25" />
              <div className="absolute inset-0" style={{ background: 'linear-gradient(to bottom, oklch(0.08 0.02 264 / 50%), oklch(0.08 0.02 264 / 95%))' }} />
            </div>

            <motion.div initial={{ y: 30, opacity: 0 }} animate={{ y: 0, opacity: 1 }} transition={{ delay: 0.3, duration: 0.8 }}
              className="relative z-10 text-center px-8">
              <motion.div initial={{ scale: 0 }} animate={{ scale: 1 }} transition={{ delay: 0.1, type: "spring", stiffness: 200 }}
                className="w-20 h-20 rounded-2xl mx-auto mb-6 flex items-center justify-center"
                style={{ background: 'oklch(0.82 0.15 192 / 15%)', border: '1px solid oklch(0.82 0.15 192 / 30%)', boxShadow: '0 0 40px oklch(0.82 0.15 192 / 15%)' }}>
                <Navigation className="w-10 h-10 text-gane-cyan" />
              </motion.div>
              <h1 className="text-4xl font-extrabold text-white mb-2" style={{ fontFamily: 'Syne, sans-serif' }}>
                G.A.N.E <span className="text-gane-cyan"></span>
              </h1>
              <p className="text-sm text-white/40 max-w-xs mx-auto leading-relaxed">
                Next-generation intelligent navigation powered by the Global Autonomous Navigation Ecosystem
              </p>
            </motion.div>

            <motion.div initial={{ y: 30, opacity: 0 }} animate={{ y: 0, opacity: 1 }} transition={{ delay: 0.8 }}
              className="relative z-10 mt-12 px-8 w-full max-w-sm">
              <button onClick={next}
                className="w-full py-4 rounded-2xl font-bold text-sm flex items-center justify-center gap-2 transition-all active:scale-[0.97]"
                style={{ background: 'oklch(0.82 0.15 192)', color: 'oklch(0.08 0.02 264)', boxShadow: '0 0 30px oklch(0.82 0.15 192 / 30%)' }}>
                Get Started <ChevronRight className="w-5 h-5" />
              </button>
              <button onClick={skip} className="w-full py-3 text-sm text-white/30 hover:text-white/50 transition-colors mt-2">Skip intro</button>
            </motion.div>
          </motion.div>
        )}

        {/* ═══ FEATURES ═══ */}
        {phase === 'features' && (
          <motion.div key={`feature-${featureIdx}`} initial={{ opacity: 0, x: 50 }} animate={{ opacity: 1, x: 0 }} exit={{ opacity: 0, x: -50 }}
            className="flex-1 flex flex-col px-8 pt-16 pb-8">
            <div className="flex-1 flex flex-col items-center justify-center">
              <motion.div initial={{ scale: 0.5, opacity: 0 }} animate={{ scale: 1, opacity: 1 }} transition={{ type: "spring", stiffness: 200 }}
                className="w-24 h-24 rounded-3xl mb-8 flex items-center justify-center"
                style={{
                  background: `color-mix(in oklch, ${featureSlides[featureIdx].color}, transparent 85%)`,
                  border: `1px solid color-mix(in oklch, ${featureSlides[featureIdx].color}, transparent 70%)`,
                  boxShadow: `0 0 40px color-mix(in oklch, ${featureSlides[featureIdx].color}, transparent 80%)`
                }}>
                {(() => { const Icon = featureSlides[featureIdx].icon; return <Icon className="w-12 h-12" style={{ color: featureSlides[featureIdx].color }} />; })()}
              </motion.div>
              <h2 className="text-2xl font-bold text-white text-center mb-4" style={{ fontFamily: 'Syne, sans-serif' }}>
                {featureSlides[featureIdx].title}
              </h2>
              <p className="text-sm text-white/40 text-center max-w-xs leading-relaxed">
                {featureSlides[featureIdx].description}
              </p>
            </div>

            {/* Progress dots */}
            <div className="flex items-center justify-center gap-2 mb-6">
              {featureSlides.map((_, idx) => (
                <div key={idx} className={`h-1.5 rounded-full transition-all duration-300 ${idx === featureIdx ? 'w-6 bg-gane-cyan' : 'w-1.5 bg-white/15'}`} />
              ))}
            </div>

            <button onClick={next}
              className="w-full py-4 rounded-2xl font-bold text-sm flex items-center justify-center gap-2 transition-all active:scale-[0.97]"
              style={{ background: 'oklch(0.82 0.15 192)', color: 'oklch(0.08 0.02 264)' }}>
              {featureIdx === featureSlides.length - 1 ? 'Explore Panels' : 'Continue'}
              <ChevronRight className="w-5 h-5" />
            </button>
            <button onClick={skip} className="w-full py-3 text-sm text-white/30 hover:text-white/50 transition-colors">Skip</button>
          </motion.div>
        )}

        {/* ═══ PANEL GUIDE ═══ */}
        {phase === 'panels' && (
          <motion.div key={`panels-${panelPage}`} initial={{ opacity: 0, x: 50 }} animate={{ opacity: 1, x: 0 }} exit={{ opacity: 0, x: -50 }}
            className="flex-1 flex flex-col px-6 pt-12 pb-8">
            {/* Header */}
            <div className="text-center mb-6">
              <div className="flex items-center justify-center gap-2 mb-2">
                <Sparkles className="w-4 h-4 text-gane-cyan" />
                <span className="text-[10px] text-gane-cyan uppercase tracking-[0.2em] font-semibold" style={{ fontFamily: 'Syne, sans-serif' }}>
                  {panelPage === 0 ? 'Core Intelligence' : panelPage === 1 ? 'Operations & Fleet' : 'Analytics & Architecture'}
                </span>
              </div>
              <h2 className="text-xl font-bold text-white" style={{ fontFamily: 'Syne, sans-serif' }}>
                16 Smart Panels
              </h2>
              <p className="text-xs text-white/30 mt-1">
                Page {panelPage + 1} of {panelPages.length} — Tap each panel in the sidebar to explore
              </p>
            </div>

            {/* Panel cards */}
            <div className="flex-1 space-y-2.5">
              {panelPages[panelPage].map((panel, i) => (
                <motion.div
                  key={panel.name}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: i * 0.08, type: 'spring', damping: 25 }}
                  className="flex items-center gap-3 p-3.5 rounded-xl transition-all"
                  style={{ background: `${panel.color}08`, border: `1px solid ${panel.color}12` }}
                >
                  <div className="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0"
                    style={{ background: `${panel.color}15`, border: `1px solid ${panel.color}25` }}>
                    <panel.icon className="w-5 h-5" style={{ color: panel.color }} />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-xs font-semibold text-white/80">{panel.name}</div>
                    <div className="text-[10px] text-white/35 mt-0.5 leading-relaxed">{panel.desc}</div>
                  </div>
                  <ChevronRight className="w-3.5 h-3.5 text-white/15 flex-shrink-0" />
                </motion.div>
              ))}
            </div>

            {/* Page dots */}
            <div className="flex items-center justify-center gap-2 mb-4 mt-4">
              {panelPages.map((_, idx) => (
                <div key={idx} className={`h-1.5 rounded-full transition-all duration-300 ${idx === panelPage ? 'w-6 bg-gane-cyan' : 'w-1.5 bg-white/15'}`} />
              ))}
            </div>

            <button onClick={next}
              className="w-full py-4 rounded-2xl font-bold text-sm flex items-center justify-center gap-2 transition-all active:scale-[0.97]"
              style={{ background: 'oklch(0.82 0.15 192)', color: 'oklch(0.08 0.02 264)' }}>
              {panelPage === panelPages.length - 1 ? 'Launch G.A.N.E' : 'Next'}
              <ChevronRight className="w-5 h-5" />
            </button>
            <button onClick={skip} className="w-full py-3 text-sm text-white/30 hover:text-white/50 transition-colors">Skip</button>
          </motion.div>
        )}

        {/* ═══ READY ═══ */}
        {phase === 'ready' && (
          <motion.div key="ready" initial={{ opacity: 0 }} animate={{ opacity: 1 }}
            className="flex-1 flex flex-col items-center justify-center px-8">
            {/* Animated orbital rings */}
            <div className="relative w-40 h-40 mb-8">
              <motion.div
                animate={{ rotate: 360 }}
                transition={{ duration: 20, repeat: Infinity, ease: 'linear' }}
                className="absolute inset-0 rounded-full"
                style={{ border: '1px solid oklch(0.82 0.15 192 / 15%)' }}
              />
              <motion.div
                animate={{ rotate: -360 }}
                transition={{ duration: 15, repeat: Infinity, ease: 'linear' }}
                className="absolute inset-4 rounded-full"
                style={{ border: '1px solid oklch(0.55 0.22 264 / 15%)' }}
              />
              <motion.div
                animate={{ rotate: 360 }}
                transition={{ duration: 10, repeat: Infinity, ease: 'linear' }}
                className="absolute inset-8 rounded-full"
                style={{ border: '1px solid oklch(0.75 0.18 150 / 15%)' }}
              />
              <div className="absolute inset-0 flex items-center justify-center">
                <motion.div
                  initial={{ scale: 0 }}
                  animate={{ scale: 1 }}
                  transition={{ delay: 0.3, type: 'spring', stiffness: 200 }}
                  className="w-16 h-16 rounded-2xl flex items-center justify-center"
                  style={{ background: 'oklch(0.82 0.15 192 / 15%)', border: '1px solid oklch(0.82 0.15 192 / 30%)', boxShadow: '0 0 40px oklch(0.82 0.15 192 / 20%)' }}
                >
                  <Navigation className="w-8 h-8 text-gane-cyan" />
                </motion.div>
              </div>
              {/* Orbiting dots */}
              {[0, 1, 2].map(i => (
                <motion.div
                  key={i}
                  animate={{ rotate: 360 }}
                  transition={{ duration: 8 + i * 4, repeat: Infinity, ease: 'linear' }}
                  className="absolute inset-0"
                  style={{ transformOrigin: 'center' }}
                >
                  <div className="absolute w-2 h-2 rounded-full"
                    style={{
                      top: i === 0 ? '0' : i === 1 ? '50%' : '100%',
                      left: '50%',
                      transform: 'translate(-50%, -50%)',
                      background: i === 0 ? 'oklch(0.82 0.15 192)' : i === 1 ? 'oklch(0.55 0.22 264)' : 'oklch(0.75 0.18 150)',
                      boxShadow: `0 0 10px ${i === 0 ? 'oklch(0.82 0.15 192 / 50%)' : i === 1 ? 'oklch(0.55 0.22 264 / 50%)' : 'oklch(0.75 0.18 150 / 50%)'}` }}
                  />
                </motion.div>
              ))}
            </div>

            <h1 className="text-2xl font-bold text-white mb-2 font-mono">
              {typedText}<span className="cursor-blink text-gane-cyan">|</span>
            </h1>
            <p className="text-sm text-white/30 text-center max-w-xs">
              16 smart panels, 168 Rust crates, 14,661 tests. Your mission control awaits.
            </p>

            <motion.div initial={{ y: 20, opacity: 0 }} animate={{ y: 0, opacity: 1 }} transition={{ delay: 1.5 }}
              className="mt-10 w-full max-w-sm">
              <button onClick={next}
                className="w-full py-4 rounded-2xl font-bold text-sm flex items-center justify-center gap-2 transition-all active:scale-[0.97]"
                style={{ background: 'oklch(0.82 0.15 192)', color: 'oklch(0.08 0.02 264)', boxShadow: '0 0 30px oklch(0.82 0.15 192 / 30%)' }}>
                <Signal className="w-5 h-5" />
                Enter Command Center
              </button>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
}
