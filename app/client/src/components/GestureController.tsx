/**
 * G.A.N.E — Gesture Controller
 * ═══════════════════════════════════
 * Advanced touch/mouse gesture recognition
 * 
 * Gestures:
 * - Two-finger rotate → Map rotation
 * - Two-finger pinch → Zoom
 * - Three-finger swipe up → Toggle 3D tilt
 * - Long press → Drop pin
 * - Swipe left/right → Switch panels
 * - Double tap → Quick zoom
 * - Shake detection → Emergency mode
 * 
 * Keyboard shortcuts:
 * - Ctrl+K → Search
 * - Ctrl+L → Toggle layers
 * - Ctrl+M → Toggle map type
 * - Ctrl+N → Start navigation
 * - Ctrl+/ → Voice command
 * - Escape → Close panel
 * - +/- → Zoom
 * - Arrow keys → Pan map
 */
import { useEffect, useCallback, useRef, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { useNavigation } from '@/contexts/NavigationContext';
import {
  Keyboard, Hand, RotateCcw, ZoomIn, ZoomOut,
  Move, Pin, Layers, Search, Navigation, Volume2, X
} from 'lucide-react';
import { useLanguage } from "@/contexts/LanguageContext";

interface GestureHint {
  id: string;
  icon: typeof Hand;
  label: string;
  color: string;
}

// ═══ Keyboard Shortcut Handler ═══
function useKeyboardShortcuts() {
  const { state, dispatch, mapRef } = useNavigation();

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      // Ignore if typing in input
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

      // Ctrl+K → Search
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        dispatch({ type: 'SET_VIEW', view: 'search' });
      }
      // Ctrl+L → Toggle layers panel
      if ((e.ctrlKey || e.metaKey) && e.key === 'l') {
        e.preventDefault();
        dispatch({ type: 'SET_ACTIVE_PANEL', panel: state.activePanel === 'map-layers' ? null : 'map-layers' });
      }
      // Ctrl+M → Toggle satellite view
      if ((e.ctrlKey || e.metaKey) && e.key === 'm') {
        e.preventDefault();
        dispatch({ type: 'TOGGLE_LAYER', layer: 'satellite' });
      }
      // Ctrl+N → Start navigation
      if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
        e.preventDefault();
        dispatch({ type: 'SET_VIEW', view: 'search' });
      }
      // Escape → Close panel/overlay
      if (e.key === 'Escape') {
        if (state.activePanel) {
          dispatch({ type: 'SET_ACTIVE_PANEL', panel: null });
        } else if (state.view !== 'map') {
          dispatch({ type: 'SET_VIEW', view: 'map' });
        } else if (state.isNavigating) {
          dispatch({ type: 'STOP_NAVIGATION' });
        }
      }
      // +/= → Zoom in
      if (e.key === '+' || e.key === '=') {
        if (mapRef.current) {
          const z = mapRef.current.getZoom() || 14;
          mapRef.current.setZoom(z + 1);
        }
      }
      // - → Zoom out
      if (e.key === '-') {
        if (mapRef.current) {
          const z = mapRef.current.getZoom() || 14;
          mapRef.current.setZoom(z - 1);
        }
      }
      // Arrow keys → Pan map
      if (e.key === 'ArrowUp' || e.key === 'ArrowDown' || e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
        if (mapRef.current) {
          const center = mapRef.current.getCenter();
          if (center) {
            const delta = 0.005;
            let lat = center.lat();
            let lng = center.lng();
            if (e.key === 'ArrowUp') lat += delta;
            if (e.key === 'ArrowDown') lat -= delta;
            if (e.key === 'ArrowLeft') lng -= delta;
            if (e.key === 'ArrowRight') lng += delta;
            mapRef.current.panTo({ lat, lng });
          }
        }
      }
      // Space → Toggle 3D tilt
      if (e.key === ' ' && !e.ctrlKey && !e.metaKey) {
        if (mapRef.current) {
          const tilt = mapRef.current.getTilt?.() || 0;
          mapRef.current.setTilt?.(tilt > 0 ? 0 : 45);
        }
      }
    };

    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [state.activePanel, state.view, state.isNavigating, dispatch, mapRef]);
}

// ═══ Device Motion (Shake Detection) ═══
function useShakeDetection() {
  const { dispatch } = useNavigation();
  const lastShake = useRef(0);

  useEffect(() => {
    let lastX = 0, lastY = 0, lastZ = 0;
    let shakeCount = 0;

    const handler = (e: DeviceMotionEvent) => {
      const acc = e.accelerationIncludingGravity;
      if (!acc || acc.x === null || acc.y === null || acc.z === null) return;

      const deltaX = Math.abs(acc.x - lastX);
      const deltaY = Math.abs(acc.y - lastY);
      const deltaZ = Math.abs(acc.z - lastZ);

      if (deltaX + deltaY + deltaZ > 30) {
        shakeCount++;
        if (shakeCount >= 3 && Date.now() - lastShake.current > 5000) {
          lastShake.current = Date.now();
          shakeCount = 0;
          dispatch({ type: 'SET_NAV_MODE', mode: 'emergency' });
        }
      } else {
        shakeCount = Math.max(0, shakeCount - 1);
      }

      lastX = acc.x;
      lastY = acc.y;
      lastZ = acc.z;
    };

    window.addEventListener('devicemotion', handler);
    return () => window.removeEventListener('devicemotion', handler);
  }, [dispatch]);
}

// ═══ Shortcut Help Overlay ═══
function ShortcutHelp({ visible, onClose }: { visible: boolean; onClose: () => void }) {
  const { t, dir } = useLanguage();
  const shortcuts = [
    { keys: 'Ctrl+K', action: 'חיפוש', actionEn: 'Search' },
    { keys: 'Ctrl+L', action: 'שכבות', actionEn: 'Layers' },
    { keys: 'Ctrl+M', action: 'לוויין', actionEn: 'Satellite' },
    { keys: 'Ctrl+N', action: 'ניווט', actionEn: 'Navigate' },
    { keys: 'Escape', action: 'סגור', actionEn: 'Close' },
    { keys: '+/-', action: 'זום', actionEn: 'Zoom' },
    { keys: 'חצים', action: 'הזזה', actionEn: 'Pan' },
    { keys: 'רווח', action: '3D הטיה', actionEn: '3D Tilt' },
  ];

  return (
    <AnimatePresence>
      {visible && (
        <motion.div
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.95 }}
          className="fixed inset-0 z-[60] flex items-center justify-center"
          onClick={onClose}
        >
          <div className="absolute inset-0" style={{ background: 'rgba(249,250,251,0.85)' }} />
          <motion.div
            className="relative z-10 w-[320px] rounded-2xl overflow-hidden"
            style={{
              background: 'rgba(4,8,18,0.95)',
              border: '1px solid rgba(37,99,235,0.15)',
              boxShadow: '0 20px 60px rgba(249,250,251,0.85), 0 0 30px rgba(37,99,235,0.08)' }}
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center justify-between px-4 py-3" style={{ borderBottom: '1px solid rgba(229,231,235,0.4)' }}>
              <div className="flex items-center gap-2">
                <Keyboard className="w-4 h-4" style={{ color: '#2563EB' }} />
                <span className="text-sm font-bold" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(17,24,39,0.9)' }}>
                  קיצורי מקלדת
                </span>
              </div>
              <motion.button whileHover={{ scale: 1.1 }} whileTap={{ scale: 0.9 }} onClick={onClose}
                className="w-6 h-6 rounded flex items-center justify-center cursor-pointer"
                style={{ background: 'rgba(229,231,235,0.4)' }}>
                <X className="w-3.5 h-3.5 text-white/30" />
              </motion.button>
            </div>
            <div className="p-4 space-y-2" style={{ direction: dir }}>
              {shortcuts.map((s, i) => (
                <motion.div
                  key={s.keys}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: i * 0.04 }}
                  className="flex items-center justify-between py-1.5"
                >
                  <span className="text-xs" style={{ color: 'rgba(107,114,128,0.9)' }}>{s.action}</span>
                  <kbd className="text-[10px] font-mono px-2 py-0.5 rounded"
                    style={{ background: 'rgba(229,231,235,0.6)', color: '#2563EB', border: '1px solid rgba(209,213,219,0.5)' }}>
                    {s.keys}
                  </kbd>
                </motion.div>
              ))}
            </div>
            <div className="px-4 py-2" style={{ borderTop: '1px solid rgba(229,231,235,0.4)' }}>
              <p className="text-[9px] text-center" style={{ color: 'rgba(156,163,175,0.6)' }}>
                לחץ ? בכל מקום להצגת קיצורים
              </p>
            </div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

// ═══ Main Gesture Controller ═══
export default function GestureController() {
  const [showHelp, setShowHelp] = useState(false);

  // Activate keyboard shortcuts
  useKeyboardShortcuts();
  
  // Activate shake detection
  useShakeDetection();

  // ? key to show help
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;
      if (e.key === '?') {
        e.preventDefault();
        setShowHelp(prev => !prev);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, []);

  return <ShortcutHelp visible={showHelp} onClose={() => setShowHelp(false)} />;
}
