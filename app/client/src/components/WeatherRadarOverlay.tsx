/**
 * G.A.N.E — Weather Radar Tile Overlay
 * ═══════════════════════════════════════════
 * Fetches real-time radar data from RainViewer API
 * and renders precipitation tiles on the map.
 * 
 * RainViewer provides free global radar coverage
 * with 5-minute update intervals.
 * 
 * When Google Maps is available, tiles are added
 * as a map overlay. When using fallback, tiles
 * are rendered on a canvas.
 */
import { useEffect, useRef, useState, useCallback } from 'react';
import { useNavigation } from '@/contexts/NavigationContext';
import { useRealDataContext } from '@/contexts/RealDataContext';

interface RainViewerData {
  version: string;
  generated: number;
  host: string;
  radar: {
    past: { time: number; path: string }[];
    nowcast: { time: number; path: string }[];
  };
  satellite: {
    infrared: { time: number; path: string }[];
  };
}

// ═══ Radar Tile Manager ═══
function useRadarTiles() {
  const [radarData, setRadarData] = useState<RainViewerData | null>(null);
  const [currentFrame, setCurrentFrame] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchRadarData = useCallback(async (retries = 2) => {
    for (let attempt = 0; attempt <= retries; attempt++) {
      try {
        const controller = new AbortController();
        const timeout = setTimeout(() => controller.abort(), 8000);
        const res = await fetch('https://api.rainviewer.com/public/weather-maps.json', { signal: controller.signal });
        clearTimeout(timeout);
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const data: RainViewerData = await res.json();
        setRadarData(data);
        setError(null);
        if (data.radar.past.length > 0) {
          setCurrentFrame(data.radar.past.length - 1);
        }
        return;
      } catch (err) {
        if (attempt < retries) {
          await new Promise(r => setTimeout(r, 2000 * (attempt + 1)));
        } else {
          setError('Radar data unavailable');
          console.warn('RainViewer API error (all retries exhausted):', err);
        }
      }
    }
  }, []);

  // CONSOLIDATED: Single interval for data fetch + animation playback
  useEffect(() => {
    fetchRadarData();
    let tick = 0;
    const interval = setInterval(() => {
      tick++;
      // Animation playback at 500ms intervals
      if (isPlaying && radarData) {
        const totalFrames = radarData.radar.past.length + radarData.radar.nowcast.length;
        setCurrentFrame(prev => (prev + 1) % totalFrames);
      }
      // Refresh radar data every 5 minutes (600 ticks * 500ms)
      if (tick % 600 === 0) {
        fetchRadarData();
      }
    }, 500);
    return () => clearInterval(interval);
  }, [fetchRadarData, isPlaying, radarData]);

  const allFrames = radarData ? [...radarData.radar.past, ...radarData.radar.nowcast] : [];
  const currentPath = allFrames[currentFrame]?.path || null;
  const currentTime = allFrames[currentFrame]?.time || null;

  return {
    radarData,
    currentFrame,
    setCurrentFrame,
    isPlaying,
    setIsPlaying,
    currentPath,
    currentTime,
    allFrames,
    error,
    refresh: fetchRadarData };
}

// ═══ Main Component ═══
export default function WeatherRadarOverlay() {
  const { state } = useNavigation();
  const realData = useRealDataContext();
  const { radarData, currentPath, currentTime, allFrames, currentFrame, setCurrentFrame, isPlaying, setIsPlaying, error } = useRadarTiles();
  const { mapRef } = useNavigation();
  const overlayRef = useRef<google.maps.ImageMapType | null>(null);

  const hasRadarLayer = state.activeLayers.includes('weather-radar');

  // Add/remove radar tile overlay on Google Maps
  useEffect(() => {
    if (!mapRef.current || !window.google || !hasRadarLayer || !currentPath) {
      // Remove overlay if exists
      if (overlayRef.current && mapRef.current) {
        const overlays = mapRef.current.overlayMapTypes;
        for (let i = 0; i < overlays.getLength(); i++) {
          if (overlays.getAt(i) === overlayRef.current) {
            overlays.removeAt(i);
            break;
          }
        }
        overlayRef.current = null;
      }
      return;
    }

    // Remove previous overlay
    if (overlayRef.current) {
      const overlays = mapRef.current.overlayMapTypes;
      for (let i = 0; i < overlays.getLength(); i++) {
        if (overlays.getAt(i) === overlayRef.current) {
          overlays.removeAt(i);
          break;
        }
      }
    }

    // Create new tile overlay
    const tileSize = 256;
    const host = radarData?.host || 'https://tilecache.rainviewer.com';
    
    const radarOverlay = new google.maps.ImageMapType({
      getTileUrl: (coord, zoom) => {
        return `${host}${currentPath}/${tileSize}/${zoom}/${coord.x}/${coord.y}/2/1_1.png`;
      },
      tileSize: new google.maps.Size(tileSize, tileSize),
      opacity: 0.6,
      name: 'RainViewer' });

    mapRef.current.overlayMapTypes.push(radarOverlay);
    overlayRef.current = radarOverlay;

    return () => {
      if (overlayRef.current && mapRef.current) {
        const overlays = mapRef.current.overlayMapTypes;
        for (let i = 0; i < overlays.getLength(); i++) {
          if (overlays.getAt(i) === overlayRef.current) {
            overlays.removeAt(i);
            break;
          }
        }
        overlayRef.current = null;
      }
    };
  }, [hasRadarLayer, currentPath, mapRef, radarData]);

  // Don't render UI if radar layer is not active
  if (!hasRadarLayer) return null;

  return (
    <div className="absolute bottom-28 left-1/2 -translate-x-1/2 z-20 pointer-events-auto">
      <div className="rounded-2xl px-4 py-2.5 flex items-center gap-3" style={{
        background: 'rgba(4,8,18,0.85)',
        border: '1px solid rgba(37,99,235,0.12)',
        boxShadow: '0 8px 32px rgba(249,250,251,0.8)' }}>
        {/* Play/Pause */}
        <button
          onClick={() => setIsPlaying(!isPlaying)}
          className="w-7 h-7 rounded-lg flex items-center justify-center cursor-pointer"
          style={{
            background: isPlaying ? 'rgba(37,99,235,0.15)' : 'rgba(229,231,235,0.5)',
            border: `1px solid ${isPlaying ? 'rgba(37,99,235,0.25)' : 'rgba(209,213,219,0.5)'}` }}
        >
          <span className="text-[10px]" style={{ color: isPlaying ? '#2563EB' : 'rgba(107,114,128,0.9)' }}>
            {isPlaying ? '⏸' : '▶'}
          </span>
        </button>

        {/* Timeline scrubber */}
        <div className="flex items-center gap-2">
          <input
            type="range"
            min={0}
            max={Math.max(0, allFrames.length - 1)}
            value={currentFrame}
            onChange={(e) => { setIsPlaying(false); setCurrentFrame(Number(e.target.value)); }}
            className="w-32 h-1 appearance-none rounded-full cursor-pointer"
            style={{ background: 'rgba(37,99,235,0.15)' }}
          />
        </div>

        {/* Time label */}
        <div className="text-[9px] font-mono" style={{ color: 'rgba(37,99,235,0.6)' }}>
          {currentTime ? new Date(currentTime * 1000).toLocaleTimeString('he-IL', { hour: '2-digit', minute: '2-digit' }) : '--:--'}
        </div>

        {/* Status */}
        <div className="flex items-center gap-1">
          <div className="w-1.5 h-1.5 rounded-full animate-pulse" style={{ background: error ? '#ef4444' : '#16A34A' }} />
          <span className="text-[7px] font-mono" style={{ color: error ? 'rgba(239,68,68,0.5)' : 'rgba(22,163,74,0.4)' }}>
            {error ? 'ERR' : 'LIVE'}
          </span>
        </div>
      </div>
    </div>
  );
}
