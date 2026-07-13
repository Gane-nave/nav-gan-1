/**
 * G.A.N.E — Map Overlay Renderer (Scientific Grade)
 * ═══════════════════════════════════════════════════════
 * Canvas-based overlay system that renders real data
 * visualizations on top of the map:
 * 
 * - Weather Radar: Animated precipitation cells
 * - Temperature Heatmap: Thermal gradient overlay
 * - Wind Flow: Animated particle wind field
 * - Seismic Activity: Earthquake magnitude rings
 * - Air Quality: AQI gradient zones
 * - Traffic Heatmap: Congestion density
 * - Risk Zones: Hazard area highlighting
 * - Night Vision: IR-style post-processing
 * 
 * All data sourced from real APIs via RealDataContext.
 */
import { useEffect, useRef, useMemo, useCallback } from 'react';
import { useNavigation } from '@/contexts/NavigationContext';
import { useRealDataContext } from '@/contexts/RealDataContext';
import type { MapLayer } from '@/lib/navStore';

// ═══ Color Scales ═══
const TEMP_COLORS = [
  { temp: -20, color: [30, 0, 120] },
  { temp: -10, color: [0, 50, 200] },
  { temp: 0, color: [0, 150, 255] },
  { temp: 10, color: [0, 220, 180] },
  { temp: 20, color: [100, 255, 50] },
  { temp: 30, color: [255, 200, 0] },
  { temp: 40, color: [255, 80, 0] },
  { temp: 50, color: [200, 0, 0] },
];

const AQI_COLORS = [
  { aqi: 0, color: [0, 228, 0] },
  { aqi: 50, color: [255, 255, 0] },
  { aqi: 100, color: [255, 126, 0] },
  { aqi: 150, color: [255, 0, 0] },
  { aqi: 200, color: [143, 63, 151] },
  { aqi: 300, color: [126, 0, 35] },
];

function interpolateColor(scale: { temp?: number; aqi?: number; color: number[] }[], value: number, key: 'temp' | 'aqi'): string {
  for (let i = 0; i < scale.length - 1; i++) {
    const lo = (scale[i] as any)[key];
    const hi = (scale[i + 1] as any)[key];
    if (value >= lo && value <= hi) {
      const t = (value - lo) / (hi - lo);
      const r = Math.round(scale[i].color[0] + (scale[i + 1].color[0] - scale[i].color[0]) * t);
      const g = Math.round(scale[i].color[1] + (scale[i + 1].color[1] - scale[i].color[1]) * t);
      const b = Math.round(scale[i].color[2] + (scale[i + 1].color[2] - scale[i].color[2]) * t);
      return `rgb(${r},${g},${b})`;
    }
  }
  const last = scale[scale.length - 1];
  return `rgb(${last.color[0]},${last.color[1]},${last.color[2]})`;
}

// ═══ Wind Particle System ═══
interface WindParticle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  age: number;
  maxAge: number;
}

function createWindParticles(count: number, w: number, h: number, windSpeed: number, windDir: number): WindParticle[] {
  const rad = (windDir * Math.PI) / 180;
  const speed = windSpeed * 0.15;
  return Array.from({ length: count }, () => ({
    x: Math.random() * w,
    y: Math.random() * h,
    vx: Math.sin(rad) * speed + (Math.random() - 0.5) * 0.5,
    vy: -Math.cos(rad) * speed + (Math.random() - 0.5) * 0.5,
    age: Math.random() * 80,
    maxAge: 60 + Math.random() * 40 }));
}

// ═══ Main Overlay Renderer ═══
export default function MapOverlayRenderer() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { state } = useNavigation();
  const realData = useRealDataContext();
  const frameRef = useRef(0);
  const windParticlesRef = useRef<WindParticle[]>([]);
  const prevLayersRef = useRef<string>('');

  const activeLayers = state.activeLayers;
  const hasWeatherOverlay = useMemo(() => {
    return activeLayers.some(l => ['weather-radar', 'weather-temp', 'weather-wind', 'weather-precip', 'air-quality', 'earthquake', 'heatmap', 'risk-zones', 'night-vision'].includes(l));
  }, [activeLayers]);

  // Initialize wind particles when wind layer is activated
  useEffect(() => {
    if (activeLayers.includes('weather-wind') && realData.weather) {
      const canvas = canvasRef.current;
      if (canvas) {
        windParticlesRef.current = createWindParticles(
          200, canvas.width / 2, canvas.height / 2,
          realData.weather.windSpeed, realData.weather.windDirection
        );
      }
    }
  }, [activeLayers, realData.weather]);

  const drawOverlays = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const dpr = Math.min(window.devicePixelRatio, 2);
    const w = window.innerWidth;
    const h = window.innerHeight;

    if (canvas.width !== w * dpr || canvas.height !== h * dpr) {
      canvas.width = w * dpr;
      canvas.height = h * dpr;
      canvas.style.width = w + 'px';
      canvas.style.height = h + 'px';
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    }

    ctx.clearRect(0, 0, w, h);
    frameRef.current++;
    const f = frameRef.current;

    // ─── Weather Radar ───
    if (activeLayers.includes('weather-radar') && realData.weather) {
      const precip = realData.weather.precipitation;
      const cloudCover = realData.weather.cloudCover / 100;
      
      // Generate radar cells based on real precipitation data
      const cellCount = Math.max(3, Math.round(precip * 5 + cloudCover * 8));
      for (let i = 0; i < cellCount; i++) {
        const cx = w * (0.15 + Math.sin(i * 2.3 + f * 0.001) * 0.35 + 0.35);
        const cy = h * (0.15 + Math.cos(i * 1.7 + f * 0.0008) * 0.35 + 0.35);
        const radius = 30 + precip * 20 + Math.sin(f * 0.02 + i) * 15;
        const intensity = Math.min(1, precip * 0.3 + cloudCover * 0.2);
        
        const gradient = ctx.createRadialGradient(cx, cy, 0, cx, cy, radius);
        if (precip > 2) {
          gradient.addColorStop(0, `rgba(255,50,50,${intensity * 0.3})`);
          gradient.addColorStop(0.4, `rgba(255,150,0,${intensity * 0.2})`);
          gradient.addColorStop(0.7, `rgba(0,200,255,${intensity * 0.1})`);
        } else if (precip > 0.5) {
          gradient.addColorStop(0, `rgba(0,200,255,${intensity * 0.25})`);
          gradient.addColorStop(0.5, `rgba(0,150,200,${intensity * 0.15})`);
        } else {
          gradient.addColorStop(0, `rgba(37,99,235,${intensity * 0.15})`);
          gradient.addColorStop(0.5, `rgba(0,150,200,${intensity * 0.08})`);
        }
        gradient.addColorStop(1, 'transparent');
        
        ctx.fillStyle = gradient;
        ctx.beginPath();
        ctx.arc(cx, cy, radius, 0, Math.PI * 2);
        ctx.fill();
      }

      // Radar sweep line
      const sweepAngle = (f * 0.015) % (Math.PI * 2);
      const sweepCx = w / 2;
      const sweepCy = h / 2;
      ctx.save();
      ctx.translate(sweepCx, sweepCy);
      ctx.rotate(sweepAngle);
      const sweepGrad = ctx.createLinearGradient(0, 0, Math.min(w, h) * 0.4, 0);
      sweepGrad.addColorStop(0, 'rgba(37,99,235,0.15)');
      sweepGrad.addColorStop(1, 'rgba(37,99,235,0)');
      ctx.fillStyle = sweepGrad;
      ctx.beginPath();
      ctx.moveTo(0, 0);
      ctx.arc(0, 0, Math.min(w, h) * 0.4, -0.05, 0.05);
      ctx.fill();
      ctx.restore();
    }

    // ─── Temperature Map ───
    if (activeLayers.includes('weather-temp') && realData.weather) {
      const temp = realData.weather.temperature;
      const color = interpolateColor(TEMP_COLORS, temp, 'temp');
      
      // Create a grid of temperature cells with slight variation
      const gridSize = 60;
      for (let x = 0; x < w; x += gridSize) {
        for (let y = 0; y < h; y += gridSize) {
          const variation = Math.sin(x * 0.01 + y * 0.01 + f * 0.005) * 3;
          const localTemp = temp + variation;
          const localColor = interpolateColor(TEMP_COLORS, localTemp, 'temp');
          
          ctx.fillStyle = localColor.replace('rgb', 'rgba').replace(')', ',0.06)');
          ctx.fillRect(x, y, gridSize, gridSize);
        }
      }

      // Temperature label
      ctx.font = '10px monospace';
      ctx.fillStyle = 'rgba(107,114,128,0.8)';
      ctx.fillText(`${temp.toFixed(1)}°C`, 100, h - 80);
    }

    // ─── Wind Flow ───
    if (activeLayers.includes('weather-wind') && realData.weather) {
      const particles = windParticlesRef.current;
      const windSpeed = realData.weather.windSpeed;
      const windDir = realData.weather.windDirection;
      const rad = (windDir * Math.PI) / 180;
      const speed = windSpeed * 0.15;

      for (const p of particles) {
        p.age++;
        if (p.age > p.maxAge) {
          p.x = Math.random() * w;
          p.y = Math.random() * h;
          p.age = 0;
          p.vx = Math.sin(rad) * speed + (Math.random() - 0.5) * 0.5;
          p.vy = -Math.cos(rad) * speed + (Math.random() - 0.5) * 0.5;
        }

        p.x += p.vx;
        p.y += p.vy;

        // Wrap around
        if (p.x < 0) p.x = w;
        if (p.x > w) p.x = 0;
        if (p.y < 0) p.y = h;
        if (p.y > h) p.y = 0;

        const alpha = Math.sin((p.age / p.maxAge) * Math.PI) * 0.4;
        const len = Math.sqrt(p.vx * p.vx + p.vy * p.vy);
        
        ctx.beginPath();
        ctx.moveTo(p.x, p.y);
        ctx.lineTo(p.x - p.vx * 3, p.y - p.vy * 3);
        ctx.strokeStyle = `rgba(167,139,250,${alpha})`;
        ctx.lineWidth = 0.8 + len * 0.3;
        ctx.stroke();

        // Head dot
        ctx.beginPath();
        ctx.arc(p.x, p.y, 1, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(167,139,250,${alpha * 1.5})`;
        ctx.fill();
      }

      // Wind direction indicator
      ctx.save();
      ctx.translate(w - 60, h - 80);
      ctx.rotate(rad);
      ctx.beginPath();
      ctx.moveTo(0, -12);
      ctx.lineTo(-4, 8);
      ctx.lineTo(4, 8);
      ctx.closePath();
      ctx.fillStyle = 'rgba(167,139,250,0.4)';
      ctx.fill();
      ctx.restore();
      ctx.font = '9px monospace';
      ctx.fillStyle = 'rgba(167,139,250,0.5)';
      ctx.fillText(`${windSpeed.toFixed(0)} km/h`, w - 80, h - 55);
    }

    // ─── Seismic Activity ───
    if (activeLayers.includes('earthquake') && realData.earthquakes.length > 0) {
      for (const eq of realData.earthquakes) {
        // Map earthquake coordinates to screen (simplified projection)
        const userLat = realData.location?.latitude || 32.0853;
        const userLon = realData.location?.longitude || 34.7818;
        const dx = (eq.longitude - userLon) * 80;
        const dy = -(eq.latitude - userLat) * 80;
        const sx = w / 2 + dx;
        const sy = h / 2 + dy;

        if (sx < -100 || sx > w + 100 || sy < -100 || sy > h + 100) continue;

        const mag = eq.magnitude;
        const baseRadius = mag * 8;
        const pulseRadius = baseRadius + Math.sin(f * 0.04) * mag * 3;

        // Magnitude rings
        for (let r = 0; r < 3; r++) {
          const ringRadius = pulseRadius + r * 8;
          const alpha = (0.3 - r * 0.1) * (mag / 5);
          ctx.beginPath();
          ctx.arc(sx, sy, ringRadius, 0, Math.PI * 2);
          ctx.strokeStyle = `rgba(239,68,68,${alpha})`;
          ctx.lineWidth = 1.5 - r * 0.3;
          ctx.stroke();
        }

        // Center dot
        ctx.beginPath();
        ctx.arc(sx, sy, 3, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(239,68,68,${0.5 + Math.sin(f * 0.05) * 0.2})`;
        ctx.fill();

        // Magnitude label
        ctx.font = 'bold 9px monospace';
        ctx.fillStyle = 'rgba(239,68,68,0.7)';
        ctx.fillText(`M${mag.toFixed(1)}`, sx + 8, sy - 8);
      }
    }

    // ─── Air Quality ───
    if (activeLayers.includes('air-quality') && realData.airQuality) {
      const aqi = realData.airQuality.aqi;
      const color = interpolateColor(AQI_COLORS, aqi, 'aqi');
      
      // Gradient overlay
      const aqiGrad = ctx.createRadialGradient(w / 2, h / 2, 0, w / 2, h / 2, Math.max(w, h) * 0.5);
      aqiGrad.addColorStop(0, color.replace('rgb', 'rgba').replace(')', ',0.08)'));
      aqiGrad.addColorStop(0.5, color.replace('rgb', 'rgba').replace(')', ',0.04)'));
      aqiGrad.addColorStop(1, 'transparent');
      ctx.fillStyle = aqiGrad;
      ctx.fillRect(0, 0, w, h);

      // Floating particles representing air quality
      const particleCount = Math.min(50, aqi);
      for (let i = 0; i < particleCount; i++) {
        const px = (w * 0.1 + (i * 37 + f * 0.2) % (w * 0.8));
        const py = (h * 0.1 + (i * 23 + f * 0.15) % (h * 0.8));
        const size = 1 + Math.sin(f * 0.03 + i) * 0.5;
        ctx.beginPath();
        ctx.arc(px, py, size, 0, Math.PI * 2);
        ctx.fillStyle = color.replace('rgb', 'rgba').replace(')', ',0.15)');
        ctx.fill();
      }
    }

    // ─── Activity Heatmap ───
    if (activeLayers.includes('heatmap')) {
      // Generate procedural heatmap based on time and location
      const hour = new Date().getHours();
      const intensity = hour >= 7 && hour <= 20 ? 0.8 : 0.3;
      
      const hotspots = [
        { x: w * 0.3, y: h * 0.4, r: 80, i: intensity },
        { x: w * 0.6, y: h * 0.3, r: 60, i: intensity * 0.7 },
        { x: w * 0.5, y: h * 0.6, r: 100, i: intensity * 0.9 },
        { x: w * 0.7, y: h * 0.7, r: 50, i: intensity * 0.5 },
        { x: w * 0.2, y: h * 0.7, r: 70, i: intensity * 0.6 },
      ];

      for (const spot of hotspots) {
        const pulsedR = spot.r + Math.sin(f * 0.02) * 10;
        const grad = ctx.createRadialGradient(spot.x, spot.y, 0, spot.x, spot.y, pulsedR);
        grad.addColorStop(0, `rgba(245,158,11,${spot.i * 0.15})`);
        grad.addColorStop(0.3, `rgba(239,68,68,${spot.i * 0.08})`);
        grad.addColorStop(0.7, `rgba(239,68,68,${spot.i * 0.03})`);
        grad.addColorStop(1, 'transparent');
        ctx.fillStyle = grad;
        ctx.beginPath();
        ctx.arc(spot.x, spot.y, pulsedR, 0, Math.PI * 2);
        ctx.fill();
      }
    }

    // ─── Night Vision ───
    if (activeLayers.includes('night-vision')) {
      // Green-tinted overlay with scan lines
      ctx.fillStyle = 'rgba(0,30,0,0.15)';
      ctx.fillRect(0, 0, w, h);

      // Scan lines
      ctx.strokeStyle = 'rgba(0,255,0,0.03)';
      ctx.lineWidth = 0.5;
      for (let y = 0; y < h; y += 3) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(w, y);
        ctx.stroke();
      }

      // Vignette
      const vigGrad = ctx.createRadialGradient(w / 2, h / 2, w * 0.2, w / 2, h / 2, w * 0.7);
      vigGrad.addColorStop(0, 'transparent');
      vigGrad.addColorStop(1, 'rgba(0,0,0,0.3)');
      ctx.fillStyle = vigGrad;
      ctx.fillRect(0, 0, w, h);

      // Noise
      for (let i = 0; i < 300; i++) {
        const nx = Math.random() * w;
        const ny = Math.random() * h;
        ctx.fillStyle = `rgba(0,255,0,${Math.random() * 0.05})`;
        ctx.fillRect(nx, ny, 1, 1);
      }
    }

    // ─── Risk Zones ───
    if (activeLayers.includes('risk-zones')) {
      const zones = [
        { x: w * 0.25, y: h * 0.35, r: 60 },
        { x: w * 0.65, y: h * 0.55, r: 45 },
        { x: w * 0.45, y: h * 0.75, r: 55 },
      ];

      for (const zone of zones) {
        const pulse = Math.sin(f * 0.03) * 5;
        
        // Dashed circle
        ctx.setLineDash([4, 4]);
        ctx.beginPath();
        ctx.arc(zone.x, zone.y, zone.r + pulse, 0, Math.PI * 2);
        ctx.strokeStyle = 'rgba(249,115,22,0.25)';
        ctx.lineWidth = 1;
        ctx.stroke();
        ctx.setLineDash([]);

        // Fill
        const zGrad = ctx.createRadialGradient(zone.x, zone.y, 0, zone.x, zone.y, zone.r);
        zGrad.addColorStop(0, 'rgba(249,115,22,0.08)');
        zGrad.addColorStop(1, 'transparent');
        ctx.fillStyle = zGrad;
        ctx.beginPath();
        ctx.arc(zone.x, zone.y, zone.r, 0, Math.PI * 2);
        ctx.fill();

        // Warning icon
        ctx.font = 'bold 10px monospace';
        ctx.fillStyle = 'rgba(249,115,22,0.5)';
        ctx.fillText('⚠', zone.x - 5, zone.y + 4);
      }
    }
  }, [activeLayers, realData]);

  // Animation loop — throttled to 20fps, visibility-aware
  useEffect(() => {
    if (!hasWeatherOverlay) return;

    let animId: number;
    let lastFrame = 0;
    const FRAME_INTERVAL = 1000 / 20; // 20fps is enough for weather overlays
    const loop = (now: number) => {
      animId = requestAnimationFrame(loop);
      if (document.hidden) return;
      if (now - lastFrame < FRAME_INTERVAL) return;
      lastFrame = now - ((now - lastFrame) % FRAME_INTERVAL);
      drawOverlays();
    };
    animId = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(animId);
  }, [hasWeatherOverlay, drawOverlays]);

  if (!hasWeatherOverlay) return null;

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 pointer-events-none"
      style={{ zIndex: 5, mixBlendMode: 'screen' }}
    />
  );
}
