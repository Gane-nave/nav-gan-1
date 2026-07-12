/**
 * G.A.N.E — Holographic Effects Layer
 * ALL CSS-ONLY — zero canvas, zero requestAnimationFrame
 * Maximum visual impact with zero performance cost
 */
import { memo } from "react";

// ═══════════════════════════════════════════════════
// PARTICLE FIELD — Pure CSS floating particles (no canvas!)
// Uses CSS animations for zero JS overhead
// ═══════════════════════════════════════════════════
export const ParticleField = memo(function ParticleField({
  intensity = 0.5 }: {
  intensity?: number;
  interactive?: boolean;
}) {
  const count = Math.floor(16 * intensity);
  const particles = Array.from({ length: count }, (_, i) => {
    const colors = [
      'rgba(0, 229, 255, 0.4)',
      'rgba(170, 102, 255, 0.35)',
      'rgba(0, 255, 136, 0.3)',
      'rgba(68, 136, 255, 0.35)',
    ];
    const size = 1 + Math.random() * 2.5;
    const x = Math.random() * 100;
    const y = Math.random() * 100;
    const duration = 20 + Math.random() * 40;
    const delay = Math.random() * -30;
    const color = colors[i % colors.length];
    const type = i % 4; // 0=dot, 1=glow, 2=ring, 3=diamond
    return { size, x, y, duration, delay, color, type, i };
  });

  return (
    <div className="fixed inset-0 pointer-events-none z-[1]" style={{ opacity: intensity }}>
      {particles.map((p) => (
        <div
          key={p.i}
          className="absolute rounded-full"
          style={{
            left: `${p.x}%`,
            top: `${p.y}%`,
            width: p.type === 2 ? `${p.size * 5}px` : `${p.size}px`,
            height: p.type === 2 ? `${p.size * 5}px` : `${p.size}px`,
            background: p.type === 2 ? 'transparent' : p.color,
            border: p.type === 2 ? `0.5px solid ${p.color}` : 'none',
            borderRadius: p.type === 3 ? '0' : '50%',
            transform: p.type === 3 ? 'rotate(45deg)' : 'none',
            boxShadow: p.type === 1 ? `0 0 ${p.size * 4}px ${p.color}` : 'none',
            animation: `particle-float-${p.i % 4} ${p.duration}s ease-in-out ${p.delay}s infinite`,
            opacity: 0.6 }}
        />
      ))}
    </div>
  );
});

// ═══════════════════════════════════════════════════
// ENERGY GRID — Subtle scan line (CSS-only)
// ═══════════════════════════════════════════════════
export const EnergyGrid = memo(function EnergyGrid() {
  return (
    <div className="fixed inset-0 pointer-events-none z-[0] overflow-hidden">
      <div
        className="absolute left-0 right-0 h-[40px]"
        style={{
          background: 'linear-gradient(180deg, transparent, rgba(37,99,235,0.015), transparent)',
          animation: 'energy-scan 12s linear infinite' }}
      />
      <div
        className="absolute w-[120vw] h-[120vw] rounded-full"
        style={{
          left: '-30%', bottom: '-60%',
          border: '1px solid rgba(37,99,235,0.03)',
          animation: 'energy-pulse-ring 8s ease-out infinite' }}
      />
    </div>
  );
});

// ═══════════════════════════════════════════════════
// HOLOGRAPHIC CORNERS — Static SVG (no animation cost)
// ═══════════════════════════════════════════════════
export function HolographicCorners() {
  return (
    <div className="fixed inset-0 pointer-events-none z-[2]">
      <svg className="absolute top-2 left-2 w-16 h-16 opacity-20" viewBox="0 0 64 64">
        <path d="M2 32 L2 2 L32 2" fill="none" stroke="rgba(37,99,235,0.5)" strokeWidth="1" />
        <circle cx="2" cy="2" r="2" fill="rgba(37,99,235,0.4)" />
      </svg>
      <svg className="absolute top-2 right-2 w-16 h-16 opacity-20" viewBox="0 0 64 64">
        <path d="M32 2 L62 2 L62 32" fill="none" stroke="rgba(124,58,237,0.5)" strokeWidth="1" />
        <circle cx="62" cy="2" r="2" fill="rgba(124,58,237,0.4)" />
      </svg>
      <svg className="absolute bottom-2 left-2 w-16 h-16 opacity-20" viewBox="0 0 64 64">
        <path d="M2 32 L2 62 L32 62" fill="none" stroke="rgba(22,163,74,0.5)" strokeWidth="1" />
        <circle cx="2" cy="62" r="2" fill="rgba(22,163,74,0.4)" />
      </svg>
      <svg className="absolute bottom-2 right-2 w-16 h-16 opacity-20" viewBox="0 0 64 64">
        <path d="M32 62 L62 62 L62 32" fill="none" stroke="rgba(68,136,255,0.5)" strokeWidth="1" />
        <circle cx="62" cy="62" r="2" fill="rgba(68,136,255,0.4)" />
      </svg>
    </div>
  );
}

// ═══════════════════════════════════════════════════
// AMBIENT GLOW — CSS-only blobs (no JS cost)
// ═══════════════════════════════════════════════════
export const AmbientGlow = memo(function AmbientGlow() {
  return (
    <div className="fixed inset-0 pointer-events-none z-[0] overflow-hidden">
      <div className="absolute w-[800px] h-[600px] rounded-full"
        style={{ left: '-10%', bottom: '-20%', background: 'radial-gradient(ellipse, rgba(37,99,235,0.04) 0%, transparent 60%)', animation: 'gane-drift-1 25s ease-in-out infinite' }} />
      <div className="absolute w-[600px] h-[500px] rounded-full"
        style={{ right: '-5%', top: '-15%', background: 'radial-gradient(ellipse, rgba(124,58,237,0.035) 0%, transparent 60%)', animation: 'gane-drift-2 30s ease-in-out infinite' }} />
      <div className="absolute w-[500px] h-[400px] rounded-full"
        style={{ left: '30%', top: '20%', background: 'radial-gradient(ellipse, rgba(22,163,74,0.02) 0%, transparent 60%)', animation: 'gane-drift-3 35s ease-in-out infinite' }} />
    </div>
  );
});
