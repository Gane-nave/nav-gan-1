/**
 * G.A.N.E — Quantum Boot Sequence
 * 
 * Cinematic startup animation with particle canvas,
 * spinning rings, progress bar, and system status lines.
 */
import { useState, useEffect, useRef } from "react";
import { motion } from "framer-motion";
import { Navigation } from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";
import { COLORS } from "./homeConstants";

export default function QuantumBoot({ onComplete }: { onComplete: () => void }) {
  const [phase, setPhase] = useState(0);
  const [progress, setProgress] = useState(0);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { t } = useLanguage();

  useEffect(() => {
    const steps = [
      { delay: 300, phase: 1 },
      { delay: 900, phase: 2 },
      { delay: 1500, phase: 3 },
      { delay: 2100, phase: 4 },
      { delay: 2700, phase: 5 },
      { delay: 3200, phase: 6 },
      { delay: 3600, phase: 7 },
    ];
    steps.forEach(s => setTimeout(() => setPhase(s.phase), s.delay));

    let p = 0;
    const interval = setInterval(() => {
      p += 1.8;
      setProgress(Math.min(p, 100));
      if (p >= 100) {
        clearInterval(interval);
        setTimeout(onComplete, 500);
      }
    }, 50);
    return () => clearInterval(interval);
  }, [onComplete]);

  // Boot particle canvas
  useEffect(() => {
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

    const particles: { x: number; y: number; vx: number; vy: number; size: number; color: string; life: number }[] = [];
    for (let i = 0; i < 60; i++) {
      particles.push({
        x: Math.random() * window.innerWidth,
        y: Math.random() * window.innerHeight,
        vx: (Math.random() - 0.5) * 0.5,
        vy: (Math.random() - 0.5) * 0.5,
        size: Math.random() * 1.5 + 0.5,
        color: ['rgba(37,99,235,', 'rgba(124,58,237,', 'rgba(22,163,74,'][Math.floor(Math.random() * 3)],
        life: Math.random() * 100,
      });
    }

    let animId: number;
    const animate = () => {
      animId = requestAnimationFrame(animate);
      if (document.hidden) return;
      const w = window.innerWidth;
      const h = window.innerHeight;
      ctx.clearRect(0, 0, w, h);
      for (const p of particles) {
        p.x += p.vx;
        p.y += p.vy;
        p.life += 0.5;
        if (p.x < 0) p.x = w;
        if (p.x > w) p.x = 0;
        if (p.y < 0) p.y = h;
        if (p.y > h) p.y = 0;
        const alpha = 0.3 + Math.sin(p.life * 0.05) * 0.2;
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
        ctx.fillStyle = p.color + alpha + ')';
        ctx.fill();
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.size * 4, 0, Math.PI * 2);
        ctx.fillStyle = p.color + (alpha * 0.15) + ')';
        ctx.fill();
      }
      // Connection lines
      for (let i = 0; i < particles.length; i++) {
        for (let j = i + 1; j < particles.length; j++) {
          const dx = particles[i].x - particles[j].x;
          const dy = particles[i].y - particles[j].y;
          const dist = Math.sqrt(dx * dx + dy * dy);
          if (dist < 120) {
            ctx.beginPath();
            ctx.moveTo(particles[i].x, particles[i].y);
            ctx.lineTo(particles[j].x, particles[j].y);
            ctx.strokeStyle = `rgba(37,99,235,${(1 - dist / 120) * 0.08})`;
            ctx.lineWidth = 0.5;
            ctx.stroke();
          }
        }
      }
    };
    animId = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(animId);
  }, []);

  const bootLines = [
    { text: 'Quantum Core', i18nKey: 'boot.quantumCore' as const, threshold: 1, color: COLORS.cyan },
    { text: '2,246 System Crates', i18nKey: 'boot.systemCrates' as const, threshold: 2, color: COLORS.purple },
    { text: 'Neural Routing Engine', i18nKey: 'boot.neuralRouting' as const, threshold: 3, color: COLORS.blue },
    { text: 'GNSS Constellation Lock', i18nKey: 'boot.gnssLock' as const, threshold: 4, color: COLORS.green },
    { text: 'Real-Time Data Streams', i18nKey: 'boot.dataStreams' as const, threshold: 5, color: COLORS.orange },
    { text: 'Holographic UI Layer', i18nKey: 'boot.holoUI' as const, threshold: 6, color: COLORS.pink },
  ];

  return (
    <motion.div
      className="fixed inset-0 z-[200] flex items-center justify-center overflow-hidden"
      style={{ background: '#F9FAFB' }}
      exit={{ opacity: 0, scale: 1.05, filter: 'blur(20px)' }}
      transition={{ duration: 1 }}
    >
      {/* Particle canvas */}
      <canvas ref={canvasRef} className="absolute inset-0" style={{ mixBlendMode: 'screen' }} />

      {/* Radial grid */}
      <div className="absolute inset-0" style={{
        background: `radial-gradient(circle at 50% 50%, rgba(37,99,235,0.04) 0%, transparent 50%),
                     radial-gradient(circle at 20% 80%, rgba(124,58,237,0.03) 0%, transparent 40%),
                     radial-gradient(circle at 80% 20%, rgba(22,163,74,0.03) 0%, transparent 40%)`,
      }} />

      {/* Spinning rings with dots */}
      {[400, 320, 240, 160].map((size, i) => (
        <motion.div
          key={size}
          className="absolute rounded-full"
          style={{
            width: size, height: size,
            border: `1px solid rgba(37,99,235,${0.08 - i * 0.015})`,
          }}
          animate={{ rotate: i % 2 === 0 ? 360 : -360 }}
          transition={{ duration: 15 + i * 5, repeat: Infinity, ease: 'linear' }}
        >
          <motion.div
            className="absolute w-1.5 h-1.5 rounded-full"
            style={{
              top: -2, left: '50%', marginLeft: -3,
              background: [COLORS.cyan, COLORS.purple, COLORS.green, COLORS.blue][i],
              boxShadow: `0 0 6px ${[COLORS.cyan, COLORS.purple, COLORS.green, COLORS.blue][i]}40`,
            }}
          />
        </motion.div>
      ))}

      {/* Center content */}
      <div className="relative z-10 flex flex-col items-center">
        {/* Logo icon */}
        <motion.div
          className="w-28 h-28 rounded-3xl flex items-center justify-center mb-10 relative"
          style={{
            background: 'linear-gradient(135deg, rgba(37,99,235,0.08), rgba(124,58,237,0.08))',
            border: '1px solid rgba(37,99,235,0.20)',
            boxShadow: '0 4px 24px rgba(37,99,235,0.12)',
          }}
          initial={{ scale: 0, rotate: -180, opacity: 0 }}
          animate={{ scale: 1, rotate: 0, opacity: 1 }}
          transition={{ type: 'spring', damping: 12, stiffness: 80, delay: 0.1 }}
        >
          <Navigation className="w-14 h-14" style={{ color: COLORS.cyan }} />
          {[0, 1, 2, 3].map(i => (
            <motion.div
              key={i}
              className="absolute rounded-full"
              style={{
                width: i < 2 ? 6 : 4,
                height: i < 2 ? 6 : 4,
                background: [COLORS.cyan, COLORS.purple, COLORS.green, COLORS.blue][i],
                boxShadow: `0 0 8px ${[COLORS.cyan, COLORS.purple, COLORS.green, COLORS.blue][i]}40`,
              }}
              animate={{
                rotate: 360,
                x: [0, 50 + i * 5, 0, -(50 + i * 5), 0],
                y: [-(50 + i * 5), 0, 50 + i * 5, 0, -(50 + i * 5)],
              }}
              transition={{ duration: 3 + i * 0.8, repeat: Infinity, ease: 'linear', delay: i * 0.5 }}
            />
          ))}
        </motion.div>

        {/* Title */}
        <motion.div
          className="text-center mb-10"
          initial={{ opacity: 0, y: 30 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4, duration: 0.8 }}
        >
          <h1 className="text-5xl font-black tracking-[0.25em] mb-3 relative" style={{ fontFamily: 'Syne, sans-serif', color: '#111827' }}>
            G.A.N.E
            <motion.span
              className="absolute inset-0 text-5xl font-black tracking-[0.25em]"
              style={{ fontFamily: 'Syne, sans-serif', color: COLORS.cyan, opacity: 0.10, filter: 'blur(4px)' }}
              animate={{ x: [-2, 2, -2] }}
              transition={{ duration: 2, repeat: Infinity }}
            >
              G.A.N.E
            </motion.span>
          </h1>
          <div className="text-sm tracking-[0.5em] font-mono" style={{ color: '#6B7280' }}>
            GLOBAL MOBILITY INTELLIGENCE
          </div>
          <motion.div
            className="text-[10px] tracking-[0.5em] font-mono mt-2"
            style={{ color: '#9CA3AF' }}
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.8 }}
          >
            NETWORK v3.0
          </motion.div>
        </motion.div>

        {/* Status lines */}
        <div className="w-96 space-y-2.5 mb-10">
          {bootLines.map((item, i) => (
            <motion.div
              key={i}
              className="flex items-center gap-3 text-xs font-mono relative"
              initial={{ opacity: 0, x: -30 }}
              animate={phase > item.threshold ? { opacity: 1, x: 0 } : { opacity: 0.15, x: -15 }}
              transition={{ duration: 0.4, type: 'spring', damping: 20 }}
            >
              <motion.div
                className="w-2.5 h-2.5 rounded-full flex-shrink-0 relative"
                style={{
                  background: phase > item.threshold ? item.color : '#E5E7EB',
                  boxShadow: phase > item.threshold ? `0 0 8px ${item.color}40` : 'none',
                }}
              >
                {phase > item.threshold && (
                  <motion.div
                    className="absolute inset-[-4px] rounded-full"
                    style={{ border: `1px solid ${item.color}30` }}
                    animate={{ scale: [1, 1.5, 1], opacity: [0.5, 0, 0.5] }}
                    transition={{ duration: 2, repeat: Infinity }}
                  />
                )}
              </motion.div>
              <span style={{ color: phase > item.threshold ? '#374151' : '#D1D5DB' }}>
                {item.text}
              </span>
              <span className="text-[9px] mr-auto" style={{ color: phase > item.threshold ? '#9CA3AF' : 'transparent' }}>
                {t(item.i18nKey)}
              </span>
              <span className="font-bold tracking-wider" style={{ color: phase > item.threshold ? item.color : '#E5E7EB' }}>
                {phase > item.threshold ? 'ONLINE' : '···'}
              </span>
            </motion.div>
          ))}
        </div>

        {/* Progress bar */}
        <div className="w-96 relative">
          <div className="h-2 rounded-full overflow-hidden" style={{ background: '#E5E7EB', border: '1px solid #D1D5DB' }}>
            <motion.div
              className="h-full rounded-full relative"
              style={{
                width: `${progress}%`,
                background: `linear-gradient(90deg, ${COLORS.cyan}, ${COLORS.purple}, ${COLORS.green}, ${COLORS.cyan})`,
                backgroundSize: '200% 100%',
                animation: 'gane-shimmer 3s linear infinite',
              }}
            />
          </div>
          <motion.div
            className="absolute -bottom-2 h-4 rounded-full pointer-events-none"
            style={{
              width: `${progress}%`,
              background: `linear-gradient(90deg, ${COLORS.cyan}30, ${COLORS.purple}30, ${COLORS.green}30)`,
              filter: 'blur(8px)',
            }}
          />
          <div className="flex items-center justify-between mt-4">
            <span className="text-[10px] font-mono" style={{ color: '#9CA3AF' }}>INITIALIZING</span>
            <span className="text-sm font-mono font-bold" style={{ color: COLORS.cyan }}>
              {Math.round(progress)}%
            </span>
          </div>
        </div>
      </div>
    </motion.div>
  );
}
