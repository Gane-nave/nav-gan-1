/**
 * G.A.N.E — Night Mode Components (Lightweight)
 * 
 * Extracted from NewFeatures.tsx to allow dynamic import of the
 * heavier RouteHistoryPanel and SmartAlertsPanel without pulling
 * these small UI components into the same chunk.
 */
import { useState, useEffect, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Moon, Sun } from "lucide-react";

// ═══════════════════════════════════════════════════
// NIGHT MODE CONTROLLER — Auto day/night with smooth transition
// ═══════════════════════════════════════════════════
export function NightModeController({ children }: { children: (isNight: boolean, toggle: () => void) => React.ReactNode }) {
  const [isNight, setIsNight] = useState(() => {
    const hour = new Date().getHours();
    return hour < 6 || hour >= 19;
  });
  const [isAuto, setIsAuto] = useState(true);

  useEffect(() => {
    if (!isAuto) return;
    const check = () => {
      const hour = new Date().getHours();
      setIsNight(hour < 6 || hour >= 19);
    };
    const interval = setInterval(check, 60000);
    return () => clearInterval(interval);
  }, [isAuto]);

  const toggle = useCallback(() => {
    setIsAuto(false);
    setIsNight(prev => !prev);
  }, []);

  return <>{children(isNight, toggle)}</>;
}

// ═══════════════════════════════════════════════════
// NIGHT MODE TOGGLE BUTTON — Animated sun/moon morph
// ═══════════════════════════════════════════════════
export function NightModeToggle({ isNight, onToggle }: { isNight: boolean; onToggle: () => void }) {
  return (
    <motion.button
      onClick={onToggle}
      aria-label={isNight ? 'Switch to day mode' : 'Switch to night mode'}
      className="relative w-10 h-10 rounded-xl flex items-center justify-center overflow-hidden cursor-pointer"
      style={{
        background: isNight
          ? 'linear-gradient(135deg, oklch(0.15 0.04 264), oklch(0.08 0.02 264))'
          : 'linear-gradient(135deg, oklch(0.85 0.12 80), oklch(0.75 0.15 60))',
        border: `1px solid ${isNight ? 'oklch(0.35 0.06 264)' : 'oklch(0.80 0.10 80)'}`,
        boxShadow: isNight
          ? '0 0 15px oklch(0.50 0.12 264 / 30%), inset 0 0 8px oklch(0.50 0.12 264 / 10%)'
          : '0 0 15px oklch(0.80 0.15 60 / 30%), inset 0 0 8px oklch(0.80 0.15 60 / 10%)' }}
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9 }}
    >
      <AnimatePresence mode="wait">
        {isNight ? (
          <motion.div
            key="moon"
            initial={{ rotate: -90, scale: 0, opacity: 0 }}
            animate={{ rotate: 0, scale: 1, opacity: 1 }}
            exit={{ rotate: 90, scale: 0, opacity: 0 }}
            transition={{ duration: 0.4, type: 'spring', stiffness: 200 }}
          >
            <Moon className="w-5 h-5 text-blue-200" />
          </motion.div>
        ) : (
          <motion.div
            key="sun"
            initial={{ rotate: 90, scale: 0, opacity: 0 }}
            animate={{ rotate: 0, scale: 1, opacity: 1 }}
            exit={{ rotate: -90, scale: 0, opacity: 0 }}
            transition={{ duration: 0.4, type: 'spring', stiffness: 200 }}
          >
            <Sun className="w-5 h-5 text-amber-600" />
          </motion.div>
        )}
      </AnimatePresence>
      {/* Ambient glow */}
      <motion.div
        className="absolute inset-0 pointer-events-none rounded-xl"
        animate={{
          boxShadow: isNight
            ? ['inset 0 0 10px oklch(0.50 0.12 264 / 10%)', 'inset 0 0 20px oklch(0.50 0.12 264 / 20%)', 'inset 0 0 10px oklch(0.50 0.12 264 / 10%)']
            : ['inset 0 0 10px oklch(0.80 0.15 60 / 10%)', 'inset 0 0 20px oklch(0.80 0.15 60 / 20%)', 'inset 0 0 10px oklch(0.80 0.15 60 / 10%)'] }}
        transition={{ duration: 3, repeat: Infinity, ease: 'easeInOut' }}
      />
    </motion.button>
  );
}
