/**
 * G.A.N.E — Full-Screen Navigation Map
 * Dark-styled Google Maps with all services initialized
 * Rich animated fallback with simulated streets, buildings, and POIs
 */
import { useEffect, useRef, useCallback, useState } from "react";
import { motion } from "framer-motion";
import { usePersistFn } from "@/hooks/usePersistFn";
import { useNavigation } from "@/contexts/NavigationContext";
import { createMarker, updateMarkerPosition } from "@/lib/mapCompat";
import { darkMapStyle } from "@/lib/navStore";

const API_KEY = import.meta.env.VITE_FRONTEND_FORGE_API_KEY;
const FORGE_BASE_URL = import.meta.env.VITE_FRONTEND_FORGE_API_URL || "https://forge.butterfly-effect.dev";
const MAPS_PROXY_URL = `${FORGE_BASE_URL}/v1/maps/proxy`;

function loadMapScript(): Promise<void> {
  return new Promise((resolve, reject) => {
    if (window.google?.maps) { resolve(); return; }
    const script = document.createElement("script");
    script.src = `${MAPS_PROXY_URL}/maps/api/js?key=${API_KEY}&v=weekly&libraries=marker,places,geocoding,geometry,routes`;
    script.async = true;
    script.crossOrigin = "anonymous";
    script.onload = () => { resolve(); script.remove(); };
    script.onerror = () => reject(new Error("Failed to load Google Maps"));
    document.head.appendChild(script);
  });
}

// ═══ Rich Animated Fallback Map ═══
// Simulates a real city map with streets, buildings, parks, and moving vehicles
function AnimatedGridFallback() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const dpr = Math.min(window.devicePixelRatio, 2);
    const resize = () => {
      canvas.width = window.innerWidth * dpr;
      canvas.height = window.innerHeight * dpr;
      canvas.style.width = window.innerWidth + 'px';
      canvas.style.height = window.innerHeight + 'px';
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    };
    resize();

    const w = window.innerWidth;
    const h = window.innerHeight;
    const cx = w / 2;
    const cy = h / 2;

    // ── Generate city layout ──
    // Major roads (horizontal and vertical arteries)
    const majorH = [h * 0.2, h * 0.4, h * 0.6, h * 0.8];
    const majorV = [w * 0.15, w * 0.35, w * 0.55, w * 0.75, w * 0.92];
    
    // Minor streets between major roads
    const minorH: number[] = [];
    const minorV: number[] = [];
    for (let i = 0; i < majorH.length - 1; i++) {
      const gap = majorH[i + 1] - majorH[i];
      minorH.push(majorH[i] + gap * 0.33);
      minorH.push(majorH[i] + gap * 0.67);
    }
    for (let i = 0; i < majorV.length - 1; i++) {
      const gap = majorV[i + 1] - majorV[i];
      minorV.push(majorV[i] + gap * 0.5);
    }

    // Diagonal boulevard
    const diagStart = { x: w * 0.1, y: h * 0.1 };
    const diagEnd = { x: w * 0.85, y: h * 0.9 };

    // Buildings (blocks between streets)
    interface Building {
      x: number; y: number; w: number; h: number;
      brightness: number; hasLight: boolean; lightPhase: number;
    }
    const buildings: Building[] = [];
    const allH = [...majorH, ...minorH].sort((a, b) => a - b);
    const allV = [...majorV, ...minorV].sort((a, b) => a - b);
    
    for (let vi = 0; vi < allV.length - 1; vi++) {
      for (let hi = 0; hi < allH.length - 1; hi++) {
        const bx = allV[vi] + 6;
        const by = allH[hi] + 6;
        const bw = allV[vi + 1] - allV[vi] - 12;
        const bh = allH[hi + 1] - allH[hi] - 12;
        if (bw > 15 && bh > 15 && Math.random() > 0.15) {
          // Split into smaller buildings within the block
          const cols = Math.max(1, Math.floor(bw / 35));
          const rows = Math.max(1, Math.floor(bh / 35));
          for (let c = 0; c < cols; c++) {
            for (let r = 0; r < rows; r++) {
              if (Math.random() > 0.2) {
                const margin = 2;
                buildings.push({
                  x: bx + (bw / cols) * c + margin,
                  y: by + (bh / rows) * r + margin,
                  w: bw / cols - margin * 2,
                  h: bh / rows - margin * 2,
                  brightness: 0.04 + Math.random() * 0.06,
                  hasLight: Math.random() > 0.6,
                  lightPhase: Math.random() * Math.PI * 2 });
              }
            }
          }
        }
      }
    }

    // Parks (green areas)
    const parks = [
      { x: w * 0.38, y: h * 0.22, r: 45 },
      { x: w * 0.65, y: h * 0.65, r: 55 },
      { x: w * 0.2, y: h * 0.75, r: 35 },
    ];

    // POI markers
    const pois = [
      { x: cx - 60, y: cy - 40, label: 'Dizengoff Center', icon: '🏬', color: 'rgba(37,99,235,' },
      { x: cx + 120, y: cy - 80, label: 'Rabin Square', icon: '🏛️', color: 'rgba(124,58,237,' },
      { x: cx - 150, y: cy + 100, label: 'Carmel Market', icon: '🛒', color: 'rgba(255,153,0,' },
      { x: cx + 80, y: cy + 60, label: 'Rothschild Blvd', icon: '🌳', color: 'rgba(22,163,74,' },
      { x: cx + 200, y: cy - 20, label: 'Tel Aviv Museum', icon: '🎨', color: 'rgba(255,68,170,' },
      { x: cx - 200, y: cy - 60, label: 'HaBima Theater', icon: '🎭', color: 'rgba(255,215,0,' },
    ];

    // Moving vehicles
    interface Vehicle {
      x: number; y: number; road: 'h' | 'v' | 'd'; roadIdx: number;
      speed: number; direction: 1 | -1; color: string; size: number;
    }
    const vehicles: Vehicle[] = [];
    // Vehicles on major horizontal roads
    majorH.forEach((ry, i) => {
      for (let v = 0; v < 4; v++) {
        vehicles.push({
          x: Math.random() * w, y: ry + (Math.random() - 0.5) * 3,
          road: 'h', roadIdx: i,
          speed: 0.4 + Math.random() * 0.8,
          direction: Math.random() > 0.5 ? 1 : -1,
          color: ['rgba(37,99,235,', 'rgba(255,255,255,', 'rgba(255,200,50,', 'rgba(255,100,100,'][Math.floor(Math.random() * 4)],
          size: 2 + Math.random() });
      }
    });
    // Vehicles on major vertical roads
    majorV.forEach((rx, i) => {
      for (let v = 0; v < 3; v++) {
        vehicles.push({
          x: rx + (Math.random() - 0.5) * 3, y: Math.random() * h,
          road: 'v', roadIdx: i,
          speed: 0.3 + Math.random() * 0.7,
          direction: Math.random() > 0.5 ? 1 : -1,
          color: ['rgba(37,99,235,', 'rgba(255,255,255,', 'rgba(255,200,50,'][Math.floor(Math.random() * 3)],
          size: 2 + Math.random() });
      }
    });

    // Street names
    const streetNames = [
      { x: w * 0.25, y: majorH[0] - 5, text: 'Ben Yehuda St', angle: 0 },
      { x: w * 0.55, y: majorH[1] - 5, text: 'King George St', angle: 0 },
      { x: w * 0.4, y: majorH[2] - 5, text: 'Allenby St', angle: 0 },
      { x: w * 0.7, y: majorH[3] - 5, text: 'Rothschild Blvd', angle: 0 },
      { x: majorV[0] + 5, y: h * 0.3, text: 'Dizengoff', angle: -Math.PI / 2 },
      { x: majorV[1] + 5, y: h * 0.5, text: 'Ibn Gabirol', angle: -Math.PI / 2 },
      { x: majorV[2] + 5, y: h * 0.35, text: 'Nahalat Binyamin', angle: -Math.PI / 2 },
      { x: majorV[3] + 5, y: h * 0.55, text: 'HaYarkon', angle: -Math.PI / 2 },
    ];

    let frame = 0;
    let animId: number;
    let lastFrame = 0;
    const FRAME_INTERVAL = 1000 / 12; // 12fps — ambient map doesn't need high framerate

    const animate = (now: number = 0) => {
      animId = requestAnimationFrame(animate);
      if (document.hidden) return;
      if (now - lastFrame < FRAME_INTERVAL) return;
      lastFrame = now - ((now - lastFrame) % FRAME_INTERVAL);
      frame++;

      ctx.clearRect(0, 0, w, h);

      // ── Background ──
      const bgGrad = ctx.createRadialGradient(cx, cy, 0, cx, cy, Math.max(w, h) * 0.7);
      bgGrad.addColorStop(0, '#060a14');
      bgGrad.addColorStop(0.5, '#040810');
      bgGrad.addColorStop(1, '#020406');
      ctx.fillStyle = bgGrad;
      ctx.fillRect(0, 0, w, h);

      // ── Parks (green areas) ──
      for (const park of parks) {
        const pg = ctx.createRadialGradient(park.x, park.y, 0, park.x, park.y, park.r);
        pg.addColorStop(0, 'rgba(0,80,30,0.12)');
        pg.addColorStop(0.6, 'rgba(0,60,20,0.06)');
        pg.addColorStop(1, 'rgba(0,40,10,0)');
        ctx.fillStyle = pg;
        ctx.beginPath();
        ctx.arc(park.x, park.y, park.r, 0, Math.PI * 2);
        ctx.fill();
      }

      // ── Buildings ──
      for (const b of buildings) {
        const flicker = b.hasLight ? Math.sin(frame * 0.02 + b.lightPhase) * 0.015 : 0;
        ctx.fillStyle = `rgba(20,30,50,${b.brightness + flicker})`;
        ctx.fillRect(b.x, b.y, b.w, b.h);
        // Building outline
        ctx.strokeStyle = `rgba(40,60,90,${b.brightness * 0.6})`;
        ctx.lineWidth = 0.5;
        ctx.strokeRect(b.x, b.y, b.w, b.h);
        // Window lights
        if (b.hasLight && b.w > 12 && b.h > 12) {
          const windowCols = Math.floor(b.w / 8);
          const windowRows = Math.floor(b.h / 8);
          for (let wc = 0; wc < windowCols; wc++) {
            for (let wr = 0; wr < windowRows; wr++) {
              if (Math.sin(b.lightPhase + wc * 3 + wr * 7 + frame * 0.003) > 0.3) {
                const wx = b.x + 3 + wc * 8;
                const wy = b.y + 3 + wr * 8;
                ctx.fillStyle = `rgba(255,220,120,${0.08 + Math.sin(b.lightPhase + wc + wr + frame * 0.01) * 0.04})`;
                ctx.fillRect(wx, wy, 3, 3);
              }
            }
          }
        }
      }

      // ── Major roads ──
      ctx.lineWidth = 4;
      ctx.strokeStyle = 'rgba(40,55,80,0.5)';
      for (const ry of majorH) {
        ctx.beginPath();
        ctx.moveTo(0, ry);
        ctx.lineTo(w, ry);
        ctx.stroke();
      }
      for (const rx of majorV) {
        ctx.beginPath();
        ctx.moveTo(rx, 0);
        ctx.lineTo(rx, h);
        ctx.stroke();
      }
      // Road center lines (dashed)
      ctx.setLineDash([8, 12]);
      ctx.lineWidth = 0.5;
      ctx.strokeStyle = 'rgba(100,130,170,0.2)';
      for (const ry of majorH) {
        ctx.beginPath();
        ctx.moveTo(0, ry);
        ctx.lineTo(w, ry);
        ctx.stroke();
      }
      for (const rx of majorV) {
        ctx.beginPath();
        ctx.moveTo(rx, 0);
        ctx.lineTo(rx, h);
        ctx.stroke();
      }
      ctx.setLineDash([]);

      // ── Minor roads ──
      ctx.lineWidth = 1.5;
      ctx.strokeStyle = 'rgba(30,45,65,0.35)';
      for (const ry of minorH) {
        ctx.beginPath();
        ctx.moveTo(0, ry);
        ctx.lineTo(w, ry);
        ctx.stroke();
      }
      for (const rx of minorV) {
        ctx.beginPath();
        ctx.moveTo(rx, 0);
        ctx.lineTo(rx, h);
        ctx.stroke();
      }

      // ── Diagonal boulevard ──
      ctx.lineWidth = 5;
      ctx.strokeStyle = 'rgba(50,70,100,0.4)';
      ctx.beginPath();
      ctx.moveTo(diagStart.x, diagStart.y);
      ctx.lineTo(diagEnd.x, diagEnd.y);
      ctx.stroke();
      // Boulevard trees
      const diagLen = Math.sqrt((diagEnd.x - diagStart.x) ** 2 + (diagEnd.y - diagStart.y) ** 2);
      for (let d = 0; d < diagLen; d += 25) {
        const t = d / diagLen;
        const tx = diagStart.x + (diagEnd.x - diagStart.x) * t;
        const ty = diagStart.y + (diagEnd.y - diagStart.y) * t;
        ctx.fillStyle = `rgba(0,100,40,${0.08 + Math.sin(d * 0.1 + frame * 0.02) * 0.03})`;
        ctx.beginPath();
        ctx.arc(tx + 4, ty + 4, 3, 0, Math.PI * 2);
        ctx.fill();
        ctx.beginPath();
        ctx.arc(tx - 4, ty - 4, 3, 0, Math.PI * 2);
        ctx.fill();
      }

      // ── Intersections (circles at road crossings) ──
      for (const ry of majorH) {
        for (const rx of majorV) {
          ctx.fillStyle = 'rgba(50,70,100,0.15)';
          ctx.beginPath();
          ctx.arc(rx, ry, 6, 0, Math.PI * 2);
          ctx.fill();
          // Traffic light simulation
          const lightPhase = Math.sin(frame * 0.03 + rx * 0.01 + ry * 0.01);
          const lightColor = lightPhase > 0.3 ? 'rgba(0,200,80,0.4)' : lightPhase > -0.3 ? 'rgba(255,200,0,0.4)' : 'rgba(255,50,50,0.4)';
          ctx.fillStyle = lightColor;
          ctx.beginPath();
          ctx.arc(rx, ry, 2, 0, Math.PI * 2);
          ctx.fill();
        }
      }

      // ── Street names ──
      ctx.font = '8px "Syne", monospace';
      for (const sn of streetNames) {
        ctx.save();
        ctx.translate(sn.x, sn.y);
        ctx.rotate(sn.angle);
        ctx.fillStyle = 'rgba(100,140,180,0.25)';
        ctx.fillText(sn.text, 0, 0);
        ctx.restore();
      }

      // ── Moving vehicles ──
      for (const v of vehicles) {
        if (v.road === 'h') {
          v.x += v.speed * v.direction;
          if (v.x > w + 10) v.x = -10;
          if (v.x < -10) v.x = w + 10;
        } else {
          v.y += v.speed * v.direction;
          if (v.y > h + 10) v.y = -10;
          if (v.y < -10) v.y = h + 10;
        }

        // Vehicle body
        ctx.fillStyle = v.color + '0.6)';
        ctx.beginPath();
        ctx.arc(v.x, v.y, v.size, 0, Math.PI * 2);
        ctx.fill();

        // Headlight glow
        ctx.fillStyle = v.color + '0.15)';
        ctx.beginPath();
        ctx.arc(v.x, v.y, v.size * 4, 0, Math.PI * 2);
        ctx.fill();

        // Trail
        const trailLen = v.speed * 8;
        if (v.road === 'h') {
          const grad = ctx.createLinearGradient(v.x - trailLen * v.direction, v.y, v.x, v.y);
          grad.addColorStop(0, v.color + '0)');
          grad.addColorStop(1, v.color + '0.15)');
          ctx.strokeStyle = grad;
          ctx.lineWidth = 1;
          ctx.beginPath();
          ctx.moveTo(v.x - trailLen * v.direction, v.y);
          ctx.lineTo(v.x, v.y);
          ctx.stroke();
        } else {
          const grad = ctx.createLinearGradient(v.x, v.y - trailLen * v.direction, v.x, v.y);
          grad.addColorStop(0, v.color + '0)');
          grad.addColorStop(1, v.color + '0.15)');
          ctx.strokeStyle = grad;
          ctx.lineWidth = 1;
          ctx.beginPath();
          ctx.moveTo(v.x, v.y - trailLen * v.direction);
          ctx.lineTo(v.x, v.y);
          ctx.stroke();
        }
      }

      // ── POI markers ──
      for (const poi of pois) {
        const pulse = Math.sin(frame * 0.03 + poi.x * 0.01) * 0.15;
        // Outer ring
        ctx.strokeStyle = poi.color + (0.2 + pulse) + ')';
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.arc(poi.x, poi.y, 10 + pulse * 20, 0, Math.PI * 2);
        ctx.stroke();
        // Inner dot
        ctx.fillStyle = poi.color + '0.6)';
        ctx.beginPath();
        ctx.arc(poi.x, poi.y, 3, 0, Math.PI * 2);
        ctx.fill();
        // Glow
        ctx.fillStyle = poi.color + '0.08)';
        ctx.beginPath();
        ctx.arc(poi.x, poi.y, 16, 0, Math.PI * 2);
        ctx.fill();
        // Label
        ctx.font = '7px "Syne", monospace';
        ctx.fillStyle = poi.color + '0.5)';
        ctx.fillText(poi.label, poi.x + 14, poi.y + 3);
      }

      // ── Radar sweep from center ──
      const sweepAngle = (frame * 0.008) % (Math.PI * 2);
      const gradient = ctx.createConicGradient(sweepAngle, cx, cy);
      gradient.addColorStop(0, 'rgba(37,99,235,0.03)');
      gradient.addColorStop(0.06, 'rgba(37,99,235,0)');
      gradient.addColorStop(1, 'rgba(37,99,235,0)');
      ctx.fillStyle = gradient;
      ctx.beginPath();
      ctx.arc(cx, cy, Math.min(w, h) * 0.45, 0, Math.PI * 2);
      ctx.fill();

      // ── Range rings ──
      for (let r = 1; r <= 3; r++) {
        const radius = r * Math.min(w, h) * 0.15;
        ctx.strokeStyle = `rgba(37,99,235,${0.03 - r * 0.008})`;
        ctx.lineWidth = 0.5;
        ctx.setLineDash([4, 8]);
        ctx.beginPath();
        ctx.arc(cx, cy, radius, 0, Math.PI * 2);
        ctx.stroke();
        ctx.setLineDash([]);
        // Range label
        ctx.font = '7px monospace';
        ctx.fillStyle = `rgba(37,99,235,${0.15 - r * 0.03})`;
        ctx.fillText(`${r * 500}m`, cx + radius + 4, cy - 2);
      }
    };

    animId = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(animId);
  }, []);

  return (
    <div className="absolute inset-0" style={{ background: '#F9FAFB' }}>
      <canvas ref={canvasRef} className="absolute inset-0" />
      {/* Atmospheric gradient overlay */}
      <div className="absolute inset-0 pointer-events-none" style={{
        background: `
          radial-gradient(ellipse at 50% 50%, transparent 20%, rgba(4,8,16,0.4) 60%),
          radial-gradient(circle at 30% 70%, rgba(0,100,60,0.02) 0%, transparent 40%),
          radial-gradient(circle at 70% 30%, rgba(0,100,200,0.02) 0%, transparent 40%)
        ` }} />
      {/* Center user marker */}
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 pointer-events-none">
        <motion.div
          className="w-5 h-5 rounded-full relative"
          style={{ background: 'rgba(37,99,235,0.85)', boxShadow: '0 0 24px rgba(37,99,235,0.5), 0 0 80px rgba(37,99,235,0.15)' }}
          animate={{ scale: [1, 1.15, 1] }}
          transition={{ duration: 2, repeat: Infinity }}
        >
          {/* Direction indicator */}
          <div className="absolute -top-2 left-1/2 -translate-x-1/2 w-0 h-0"
            style={{ borderLeft: '4px solid transparent', borderRight: '4px solid transparent', borderBottom: '6px solid rgba(37,99,235,0.8)' }} />
          {/* Accuracy ring */}
          <motion.div
            className="absolute inset-[-14px] rounded-full"
            style={{ border: '1.5px solid rgba(37,99,235,0.3)', background: 'rgba(37,99,235,0.04)' }}
            animate={{ scale: [1, 1.3, 1], opacity: [0.6, 0.2, 0.6] }}
            transition={{ duration: 2.5, repeat: Infinity }}
          />
          <motion.div
            className="absolute inset-[-28px] rounded-full"
            style={{ border: '1px solid rgba(37,99,235,0.12)' }}
            animate={{ scale: [1, 1.2, 1], opacity: [0.3, 0, 0.3] }}
            transition={{ duration: 3, repeat: Infinity, delay: 0.5 }}
          />
        </motion.div>
      </div>
      {/* Location info */}
      <motion.div
        className="absolute top-1/2 left-1/2 mt-10 -translate-x-1/2 text-center pointer-events-none"
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 1 }}
      >
        <div className="text-[10px] font-mono tracking-[0.2em] font-bold" style={{ color: 'rgba(37,99,235,0.5)' }}>
          TEL AVIV — QUANTUM GRID
        </div>
        <div className="text-[9px] font-mono mt-1" style={{ color: 'rgba(156,163,175,0.6)' }}>
          32.0853°N 34.7818°E · ALT 12m
        </div>
      </motion.div>
      {/* Scale bar */}
      <div className="absolute bottom-20 left-24 pointer-events-none flex items-center gap-2">
        <div className="h-px w-16" style={{ background: 'rgba(209,213,219,0.8)' }} />
        <span className="text-[8px] font-mono" style={{ color: 'rgba(156,163,175,0.6)' }}>500m</span>
      </div>
      {/* Compass rose */}
      <div className="absolute top-20 right-20 pointer-events-none">
        <motion.div
          className="w-10 h-10 flex items-center justify-center"
          animate={{ rotate: [0, 0] }}
        >
          <div className="text-[9px] font-mono font-bold" style={{ color: 'rgba(37,99,235,0.3)' }}>N</div>
        </motion.div>
      </div>
    </div>
  );
}

interface NavigationMapProps {
  onMapClick?: (lat: number, lon: number) => void;
}

export default function NavigationMap({ onMapClick }: NavigationMapProps = {}) {
  const containerRef = useRef<HTMLDivElement>(null);
  const onMapClickRef = useRef(onMapClick);
  onMapClickRef.current = onMapClick;
  const { state, dispatch, mapRef, directionsServiceRef, directionsRendererRef, geocoderRef, placesServiceRef, trafficLayerRef, transitLayerRef, bicyclingLayerRef } = useNavigation();
  const userMarkerRef = useRef<any>(null);
  const watchIdRef = useRef<number | null>(null);
  const [mapFailed, setMapFailed] = useState(false);

  const initMap = usePersistFn(async () => {
    try {
      await loadMapScript();
    } catch {
      setMapFailed(true);
      return;
    }
    if (!containerRef.current || !window.google) return;

    const map = new google.maps.Map(containerRef.current, {
      zoom: state.mapZoom,
      center: state.mapCenter,
      styles: darkMapStyle,
      mapTypeControl: false,
      fullscreenControl: false,
      zoomControl: false,
      streetViewControl: false,
      rotateControl: false,
      scaleControl: false,
      mapTypeId: "roadmap",
      gestureHandling: "greedy",
      clickableIcons: true,
      mapId: "GANE_NAV_MAP" });
    mapRef.current = map;

    // Map click listener for collaboration cursor tracking
    map.addListener('mousemove', (e: google.maps.MapMouseEvent) => {
      if (onMapClickRef.current && e.latLng) {
        onMapClickRef.current(e.latLng.lat(), e.latLng.lng());
      }
    });

    directionsServiceRef.current = new google.maps.DirectionsService();
    directionsRendererRef.current = new google.maps.DirectionsRenderer({
      map,
      suppressMarkers: true,
      polylineOptions: {
        strokeColor: "oklch(0.82 0.15 192)",
        strokeWeight: 6,
        strokeOpacity: 0.85 } });
    geocoderRef.current = new google.maps.Geocoder();

    const placesDiv = document.createElement("div");
    placesServiceRef.current = new google.maps.places.PlacesService(map);

    trafficLayerRef.current = new google.maps.TrafficLayer();
    transitLayerRef.current = new google.maps.TransitLayer();
    bicyclingLayerRef.current = new google.maps.BicyclingLayer();

    startGeolocation();
  });

  const startGeolocation = useCallback(() => {
    if (!navigator.geolocation) return;
    navigator.geolocation.getCurrentPosition(
      (pos) => {
        const loc = { lat: pos.coords.latitude, lng: pos.coords.longitude };
        dispatch({ type: 'SET_USER_LOCATION', location: loc });
        dispatch({ type: 'SET_MAP_CENTER', center: loc });
        if (mapRef.current) {
          mapRef.current.panTo(loc);
          addUserMarker(loc);
        }
      },
      () => {
        dispatch({ type: 'SET_USER_LOCATION', location: { lat: 32.0853, lng: 34.7818 } });
      },
      { enableHighAccuracy: true, timeout: 10000 }
    );

    watchIdRef.current = navigator.geolocation.watchPosition(
      (pos) => {
        const loc = { lat: pos.coords.latitude, lng: pos.coords.longitude };
        dispatch({ type: 'SET_USER_LOCATION', location: loc });
        if (pos.coords.speed) dispatch({ type: 'SET_SPEED', speed: Math.round((pos.coords.speed || 0) * 3.6) });
        if (pos.coords.heading) dispatch({ type: 'SET_HEADING', heading: pos.coords.heading });
        updateUserMarker(loc);
      },
      () => {},
      { enableHighAccuracy: true, maximumAge: 2000 }
    );
  }, [dispatch, mapRef]);

  const addUserMarker = useCallback((loc: { lat: number; lng: number }) => {
    if (!mapRef.current || !window.google) return;
    if (userMarkerRef.current) return;

    userMarkerRef.current = createMarker({
      map: mapRef.current,
      position: loc,
      icon: {
        path: google.maps.SymbolPath.CIRCLE,
        scale: 8,
        fillColor: 'oklch(0.82 0.15 192)',
        fillOpacity: 1,
        strokeColor: 'oklch(0.08 0.02 264)',
        strokeWeight: 3 },
      zIndex: 1000 }) as any;
  }, [mapRef]);

  const updateUserMarker = useCallback((loc: { lat: number; lng: number }) => {
    if (userMarkerRef.current) {
      updateMarkerPosition(userMarkerRef.current, loc);
    } else {
      addUserMarker(loc);
    }
  }, [addUserMarker]);

  // Toggle layers
  useEffect(() => {
    if (!mapRef.current) return;
    const map = mapRef.current;
    
    trafficLayerRef.current?.setMap(state.activeLayers.includes('traffic') ? map : null);
    transitLayerRef.current?.setMap(state.activeLayers.includes('transit') ? map : null);
    bicyclingLayerRef.current?.setMap(state.activeLayers.includes('bicycling') ? map : null);
    
    if (state.activeLayers.includes('satellite')) {
      map.setMapTypeId('hybrid');
    } else if (state.activeLayers.includes('terrain')) {
      map.setMapTypeId('terrain');
    } else {
      map.setMapTypeId('roadmap');
    }

    if (state.activeLayers.includes('3d-buildings')) {
      map.setTilt(45);
    } else {
      map.setTilt(0);
    }

    if (state.activeLayers.includes('night-vision')) {
      map.setOptions({ styles: [...darkMapStyle, { elementType: 'all', stylers: [{ saturation: -100 }, { lightness: -30 }] }] });
    } else {
      map.setOptions({ styles: darkMapStyle });
    }
  }, [state.activeLayers, mapRef, trafficLayerRef, transitLayerRef, bicyclingLayerRef]);

  useEffect(() => {
    initMap();
    return () => {
      if (watchIdRef.current !== null) navigator.geolocation.clearWatch(watchIdRef.current);
    };
  }, [initMap]);

  return (
    <>
      <div ref={containerRef} className="absolute inset-0 w-full h-full" style={{ display: mapFailed ? 'none' : 'block' }} />
      {mapFailed && <AnimatedGridFallback />}
    </>
  );
}
