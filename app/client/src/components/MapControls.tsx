/**
 * G.A.N.E — Map Controls (Premium)
 * ====================================
 * Floating action buttons with:
 * - Animated compass with heading rotation
 * - 3D tilt control
 * - Zoom with level indicator
 * - Quick layer toggles with glow
 * - Driving profile selector with animated icons
 * - Connection status indicator
 */
import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Plus, Minus, Layers, Locate, Compass, Rocket, Bike,
  TowerControl, Zap, Leaf, Telescope, Navigation,
  Box, Signal, ChevronUp, ChevronDown
} from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";
import type { MapLayer, DrivingProfile } from "@/lib/navStore";

const layerOptions: { id: MapLayer; label: string; icon: typeof Layers; color: string }[] = [
  { id: 'traffic', label: 'Traffic', icon: Rocket, color: 'oklch(0.80 0.16 75)' },
  { id: 'transit', label: 'Transit', icon: Bike, color: 'oklch(0.55 0.22 264)' },
  { id: 'bicycling', label: 'Cycling', icon: Bike, color: 'oklch(0.75 0.18 150)' },
  { id: 'satellite', label: 'Satellite', icon: Telescope, color: 'oklch(0.72 0.14 180)' },
];

const profileConfig: Record<DrivingProfile, { icon: typeof Rocket; label: string; color: string }> = {
  standard: { icon: Rocket, label: 'Standard', color: 'oklch(0.82 0.15 192)' },
  eco: { icon: Leaf, label: 'Eco', color: 'oklch(0.75 0.18 150)' },
  fast: { icon: Zap, label: 'Fast', color: 'oklch(0.80 0.16 75)' },
  truck: { icon: TowerControl, label: 'Truck', color: 'oklch(0.55 0.22 264)' },
  motorcycle: { icon: Bike, label: 'Moto', color: 'oklch(0.60 0.25 300)' } };

export default function MapControls() {
  const { state, dispatch, mapRef } = useNavigation();
  const [showLayers, setShowLayers] = useState(false);
  const [showProfiles, setShowProfiles] = useState(false);
  const [zoomLevel, setZoomLevel] = useState(15);
  const [tilt, setTilt] = useState(0);
  const [gpsStrength, setGpsStrength] = useState<'strong' | 'weak' | 'none'>('strong');

  // Track zoom level
  useEffect(() => {
    if (mapRef.current) {
      const listener = mapRef.current.addListener('zoom_changed', () => {
        const z = mapRef.current?.getZoom();
        if (z !== undefined) setZoomLevel(Math.round(z));
      });
      return () => google.maps.event.removeListener(listener);
    }
  }, [mapRef.current]);

  // Simulate GPS strength
  useEffect(() => {
    const interval = setInterval(() => {
      const r = Math.random();
      setGpsStrength(r > 0.15 ? 'strong' : r > 0.05 ? 'weak' : 'none');
    }, 8000);
    return () => clearInterval(interval);
  }, []);

  const zoomIn = () => {
    if (mapRef.current) {
      const z = mapRef.current.getZoom();
      if (z !== undefined) mapRef.current.setZoom(z + 1);
    }
  };

  const zoomOut = () => {
    if (mapRef.current) {
      const z = mapRef.current.getZoom();
      if (z !== undefined) mapRef.current.setZoom(z - 1);
    }
  };

  const recenter = () => {
    if (mapRef.current && state.userLocation) {
      mapRef.current.panTo(state.userLocation);
      mapRef.current.setZoom(16);
    }
  };

  const toggle3D = () => {
    if (mapRef.current) {
      const newTilt = tilt === 0 ? 45 : 0;
      mapRef.current.setTilt(newTilt);
      setTilt(newTilt);
    }
  };

  const gpsColor = gpsStrength === 'strong' ? 'oklch(0.75 0.18 150)' : gpsStrength === 'weak' ? 'oklch(0.80 0.16 75)' : 'oklch(0.65 0.22 25)';

  return (
    <>
      {/* ═══ Right Side Controls ═══ */}
      <div className="fixed right-4 top-1/2 -translate-y-1/2 z-20 flex flex-col gap-4">
        {/* Zoom Controls */}
        <div className="flex flex-col rounded-xl overflow-hidden orbital-panel">
          <button onClick={zoomIn} className="w-12 h-12 flex items-center justify-center hover:bg-white/5 transition-all group" aria-label="Zoom in">
            <Plus className="w-5 h-5 text-white/50 group-hover:text-gane-cyan transition-colors" />
          </button>
          {/* Zoom level indicator */}
          <div className="h-px bg-white/5 relative">
            <div className="absolute inset-x-0 flex items-center justify-center -translate-y-1/2">
              <span className="text-[8px] text-white/25 bg-[oklch(0.10_0.018_264)] px-1.5 rounded font-mono tabular-nums">{zoomLevel}</span>
            </div>
          </div>
          <button onClick={zoomOut} className="w-12 h-12 flex items-center justify-center hover:bg-white/5 transition-all group" aria-label="Zoom out">
            <Minus className="w-5 h-5 text-white/50 group-hover:text-gane-cyan transition-colors" />
          </button>
        </div>

        {/* Layers Toggle */}
        <div className="relative">
          <motion.button
            onClick={() => { setShowLayers(!showLayers); setShowProfiles(false); }}
            className="w-12 h-12 rounded-xl flex items-center justify-center orbital-panel transition-all"
            whileTap={{ scale: 0.92 }}
            style={state.activeLayers.length > 0 ? {
              borderColor: 'oklch(0.82 0.15 192 / 25%)',
              boxShadow: '0 0 15px oklch(0.82 0.15 192 / 10%)' } : {}}
            aria-label="Map layers"
          >
            <Layers className={`w-5 h-5 ${state.activeLayers.length > 0 ? 'text-gane-cyan' : 'text-white/50'}`} />
            {state.activeLayers.length > 0 && (
              <span className="absolute -top-1 -right-1 w-4 h-4 rounded-full text-[8px] font-bold flex items-center justify-center"
                style={{ background: 'oklch(0.82 0.15 192)', color: 'oklch(0.09 0.015 264)' }}>
                {state.activeLayers.length}
              </span>
            )}
          </motion.button>
          <AnimatePresence>
            {showLayers && (
              <motion.div
                initial={{ opacity: 0, x: 10, scale: 0.9 }}
                animate={{ opacity: 1, x: 0, scale: 1 }}
                exit={{ opacity: 0, x: 10, scale: 0.9 }}
                transition={{ type: 'spring', damping: 25, stiffness: 300 }}
                className="absolute right-14 top-0 orbital-panel-glow rounded-xl p-2 min-w-[160px]"
              >
                {layerOptions.map((layer, i) => {
                  const active = state.activeLayers.includes(layer.id);
                  return (
                    <motion.button
                      key={layer.id}
                      onClick={() => dispatch({ type: 'TOGGLE_LAYER', layer: layer.id })}
                      initial={{ opacity: 0, x: 10 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ delay: i * 0.05 }}
                      className="w-full flex items-center gap-2.5 px-3 py-2.5 rounded-lg text-xs transition-all"
                      style={active ? { background: `${layer.color.replace(')', ' / 10%)')}` } : {}}
                    >
                      <layer.icon className="w-4 h-4" style={{ color: active ? layer.color : 'oklch(0.50 0.01 264)' }} />
                      <span style={{ color: active ? layer.color : 'oklch(0.60 0.01 264)' }}>{layer.label}</span>
                      {active && (
                        <motion.div
                          className="w-1.5 h-1.5 rounded-full ml-auto"
                          style={{ background: layer.color }}
                          animate={{ scale: [1, 1.3, 1] }}
                          transition={{ duration: 1.5, repeat: Infinity }}
                        />
                      )}
                    </motion.button>
                  );
                })}
              </motion.div>
            )}
          </AnimatePresence>
        </div>

        {/* Recenter */}
        <motion.button
          onClick={recenter}
          className="w-12 h-12 rounded-xl flex items-center justify-center orbital-panel group"
          whileTap={{ scale: 0.92 }}
          aria-label="Recenter"
        >
          <Locate className="w-5 h-5 text-white/50 group-hover:text-gane-cyan transition-colors" />
        </motion.button>

        {/* 3D Toggle */}
        <motion.button
          onClick={toggle3D}
          className="w-12 h-12 rounded-xl flex items-center justify-center orbital-panel group"
          whileTap={{ scale: 0.92 }}
          style={tilt > 0 ? { borderColor: 'oklch(0.55 0.22 264 / 25%)', boxShadow: '0 0 12px oklch(0.55 0.22 264 / 10%)' } : {}}
          aria-label="3D view"
        >
          <Box className={`w-5 h-5 ${tilt > 0 ? 'text-gane-indigo' : 'text-white/50 group-hover:text-gane-indigo'} transition-colors`} />
        </motion.button>

        {/* Compass */}
        <motion.div
          className="w-12 h-12 rounded-xl flex items-center justify-center orbital-panel"
          style={{ cursor: 'pointer' }}
          onClick={() => {
            if (mapRef.current) mapRef.current.setHeading(0);
          }}
        >
          <motion.div
            animate={{ rotate: -(state.heading || 0) }}
            transition={{ type: 'spring', stiffness: 120, damping: 15 }}
          >
            <Navigation className="w-5 h-5 text-white/50" />
          </motion.div>
        </motion.div>

        {/* GPS Status */}
        <div className="w-12 h-6 rounded-lg flex items-center justify-center gap-1.5" style={{ background: 'oklch(0.10 0.018 264 / 60%)' }}>
          <motion.div
            className="w-1.5 h-1.5 rounded-full"
            style={{ background: gpsColor }}
            animate={{ opacity: gpsStrength === 'none' ? [1, 0.2, 1] : 1 }}
            transition={gpsStrength === 'none' ? { duration: 0.8, repeat: Infinity } : {}}
          />
          <span className="text-[7px] font-mono" style={{ color: gpsColor }}>GPS</span>
        </div>
      </div>

      {/* ═══ Bottom Left: Driving Profile ═══ */}
      {!state.isNavigating && state.view === 'map' && (
        <div className="fixed left-3 bottom-24 z-20">
          <div className="relative">
            <motion.button
              onClick={() => { setShowProfiles(!showProfiles); setShowLayers(false); }}
              className="w-12 h-12 rounded-xl flex items-center justify-center orbital-panel-glow"
              whileTap={{ scale: 0.92 }}
              aria-label="Driving profile"
            >
              {(() => {
                const config = profileConfig[state.drivingProfile];
                const Icon = config.icon;
                return <Icon className="w-5 h-5" style={{ color: config.color }} />;
              })()}
            </motion.button>
            <AnimatePresence>
              {showProfiles && (
                <motion.div
                  initial={{ opacity: 0, y: 10, scale: 0.9 }}
                  animate={{ opacity: 1, y: 0, scale: 1 }}
                  exit={{ opacity: 0, y: 10, scale: 0.9 }}
                  transition={{ type: 'spring', damping: 25, stiffness: 300 }}
                  className="absolute left-0 bottom-16 orbital-panel-glow rounded-xl p-2 min-w-[170px]"
                >
                  {(Object.keys(profileConfig) as DrivingProfile[]).map((profile, i) => {
                    const config = profileConfig[profile];
                    const Icon = config.icon;
                    const active = state.drivingProfile === profile;
                    return (
                      <motion.button
                        key={profile}
                        onClick={() => { dispatch({ type: 'SET_DRIVING_PROFILE', profile }); setShowProfiles(false); }}
                        initial={{ opacity: 0, y: 5 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ delay: i * 0.05 }}
                        className="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-xs transition-all"
                        style={active ? { background: `${config.color.replace(')', ' / 10%)')}` } : {}}
                      >
                        <Icon className="w-4 h-4" style={{ color: active ? config.color : 'oklch(0.50 0.01 264)' }} />
                        <span style={{ color: active ? config.color : 'oklch(0.60 0.01 264)' }}>{config.label}</span>
                        {active && (
                          <motion.div
                            className="w-1.5 h-1.5 rounded-full ml-auto"
                            style={{ background: config.color }}
                            layoutId="profile-indicator"
                          />
                        )}
                      </motion.button>
                    );
                  })}
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>
      )}
    </>
  );
}
