/**
 * G.A.N.E — Gesture Hints & Ambient Intelligence v1.0
 * Visual gesture indicators, swipe hints, and ambient UI adaptations
 */
import { useState, useEffect, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
  Hand, Maximize2, Minimize2, RotateCcw,
  Zap, Sun, Moon, CloudRain, Wind
} from 'lucide-react';
import { useLanguage } from "@/contexts/LanguageContext";

// ═══════════════════════════════════════════════════
// Gesture Hint Overlay — shows on first use
// ═══════════════════════════════════════════════════
interface GestureHint {
  id: string;
  icon: any;
  label: string;
  labelHe: string;
  direction: string;
  color: string;
}

const gestureHints: GestureHint[] = [
  { id: 'swipe-up', icon: ArrowUp, label: 'Swipe up for search', labelHe: 'החלק למעלה לחיפוש', direction: 'up', color: '#2563EB' },
  { id: 'swipe-left', icon: ArrowLeft, label: 'Swipe left for panels', labelHe: 'החלק שמאלה לפאנלים', direction: 'left', color: '#00e88f' },
  { id: 'pinch', icon: Maximize2, label: 'Pinch to zoom', labelHe: 'צבוט לזום', direction: 'center', color: '#8855ff' },
  { id: 'rotate', icon: RotateCcw, label: '2-finger rotate', labelHe: 'סובב עם 2 אצבעות', direction: 'center', color: '#ffaa00' },
];

export function GestureHintOverlay({ show, onDismiss }: { show: boolean; onDismiss: () => void }) {
  const { t, dir, lang } = useLanguage();
  useEffect(() => {
    if (show) {
      const timer = setTimeout(onDismiss, 5000);
      return () => clearTimeout(timer);
    }
  }, [show, onDismiss]);

  return (
    <AnimatePresence>
      {show && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-50 flex items-center justify-center pointer-events-none"
          style={{ background: 'rgba(0,0,0,0.4)' }}
          onClick={onDismiss}
        >
          <motion.div
            initial={{ scale: 0.9, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 0.9, opacity: 0 }}
            className="relative w-64 h-64"
          >
            {/* Central hand icon */}
            <motion.div
              className="absolute inset-0 flex items-center justify-center"
              animate={{ scale: [1, 1.05, 1] }}
              transition={{ duration: 2, repeat: Infinity }}
            >
              <Hand className="w-10 h-10 text-white/20" />
            </motion.div>

            {/* Gesture arrows */}
            {gestureHints.map((hint, i) => {
              const angle = i * 90;
              const rad = (angle - 90) * Math.PI / 180;
              const x = Math.cos(rad) * 90;
              const y = Math.sin(rad) * 90;
              
              return (
                <motion.div
                  key={hint.id}
                  className="absolute flex flex-col items-center gap-1"
                  style={{
                    left: `calc(50% + ${x}px - 40px)`,
                    top: `calc(50% + ${y}px - 20px)`,
                    width: 80 }}
                  initial={{ opacity: 0, scale: 0.5 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: i * 0.15 + 0.3 }}
                >
                  <motion.div
                    animate={{ 
                      x: hint.direction === 'left' ? [-5, 5, -5] : hint.direction === 'right' ? [5, -5, 5] : 0,
                      y: hint.direction === 'up' ? [-5, 5, -5] : hint.direction === 'down' ? [5, -5, 5] : 0 }}
                    transition={{ duration: 1.5, repeat: Infinity }}
                  >
                    <hint.icon className="w-5 h-5" style={{ color: hint.color }} />
                  </motion.div>
                  <span className="text-[9px] text-center text-white/40" style={{ direction: dir }}>
                    {lang === 'he' ? hint.labelHe : hint.label}
                  </span>
                </motion.div>
              );
            })}
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}

// ═══════════════════════════════════════════════════
// Ambient Weather Overlay — subtle weather effects
// ═══════════════════════════════════════════════════
export function AmbientWeatherOverlay({ condition }: { condition?: string }) {
  if (!condition) return null;

  const isRain = /rain|drizzle|shower/i.test(condition);
  const isSnow = /snow|sleet/i.test(condition);
  const isFog = /fog|mist|haze/i.test(condition);

  if (!isRain && !isSnow && !isFog) return null;

  return (
    <div className="fixed inset-0 z-[5] pointer-events-none overflow-hidden">
      {isRain && (
        <>
          {Array.from({ length: 30 }).map((_, i) => (
            <motion.div
              key={`rain-${i}`}
              className="absolute w-[1px] rounded-full"
              style={{
                left: `${Math.random() * 100}%`,
                top: -20,
                height: 15 + Math.random() * 15,
                background: 'rgba(100,180,255,0.15)' }}
              animate={{ y: ['0vh', '105vh'] }}
              transition={{
                duration: 0.6 + Math.random() * 0.4,
                repeat: Infinity,
                delay: Math.random() * 2,
                ease: 'linear' }}
            />
          ))}
        </>
      )}
      {isSnow && (
        <>
          {Array.from({ length: 20 }).map((_, i) => (
            <motion.div
              key={`snow-${i}`}
              className="absolute w-1.5 h-1.5 rounded-full"
              style={{
                left: `${Math.random() * 100}%`,
                top: -10,
                background: 'rgba(209,213,219,0.8)' }}
              animate={{ 
                y: ['0vh', '105vh'],
                x: [0, Math.random() * 40 - 20] }}
              transition={{
                duration: 4 + Math.random() * 3,
                repeat: Infinity,
                delay: Math.random() * 5,
                ease: 'linear' }}
            />
          ))}
        </>
      )}
      {isFog && (
        <motion.div
          className="absolute inset-0"
          style={{ background: 'rgba(180,200,220,0.03)' }}
          animate={{ opacity: [0.02, 0.05, 0.02] }}
          transition={{ duration: 6, repeat: Infinity }}
        />
      )}
    </div>
  );
}

// ═══════════════════════════════════════════════════
// Speed-Adaptive UI — changes UI density based on speed
// ═══════════════════════════════════════════════════
export function useSpeedAdaptiveUI() {
  const [uiDensity, setUiDensity] = useState<'full' | 'reduced' | 'minimal'>('full');
  const [speed, setSpeed] = useState(0);

  useEffect(() => {
    // Simulate speed changes (in real app, this would come from GPS)
    const interval = setInterval(() => {
      const newSpeed = Math.random() * 120;
      setSpeed(newSpeed);
      if (newSpeed > 80) setUiDensity('minimal');
      else if (newSpeed > 40) setUiDensity('reduced');
      else setUiDensity('full');
    }, 5000);
    return () => clearInterval(interval);
  }, []);

  return { uiDensity, speed };
}

export default GestureHintOverlay;
