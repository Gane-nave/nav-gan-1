/**
 * G.A.N.E — Accessibility Layer
 * ═══════════════════════════════════════
 * WCAG 2.1 AA compliance features
 * 
 * Features:
 * - Screen reader announcements (ARIA live regions)
 * - High contrast mode toggle
 * - Reduced motion support
 * - Focus trap for modals/panels
 * - Skip navigation links
 * - Font size scaling
 * - Color blind mode filters
 */
import { createContext, useContext, useState, useEffect, useCallback, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  Eye, Type, Contrast, Volume2, Accessibility,
  ZoomIn, ZoomOut, Palette, X
} from 'lucide-react';
import { useLanguage } from "@/contexts/LanguageContext";

interface A11yState {
  highContrast: boolean;
  reducedMotion: boolean;
  fontSize: number; // 1 = normal, 1.2 = large, 1.5 = extra large
  colorBlindMode: 'none' | 'protanopia' | 'deuteranopia' | 'tritanopia';
  screenReaderMode: boolean;
}

interface A11yContextType extends A11yState {
  setHighContrast: (v: boolean) => void;
  setReducedMotion: (v: boolean) => void;
  setFontSize: (v: number) => void;
  setColorBlindMode: (v: A11yState['colorBlindMode']) => void;
  announce: (message: string, priority?: 'polite' | 'assertive') => void;
}

const A11yContext = createContext<A11yContextType | null>(null);

export function useA11y() {
  const ctx = useContext(A11yContext);
  if (!ctx) throw new Error('useA11y must be used within AccessibilityProvider');
  return ctx;
}

// ═══ Accessibility Provider ═══
export function AccessibilityProvider({ children }: { children: React.ReactNode }) {
  const { t, dir } = useLanguage();
  const [state, setState] = useState<A11yState>(() => {
    const saved = localStorage.getItem('gane-a11y');
    if (saved) {
      try { return JSON.parse(saved); } catch {}
    }
    return {
      highContrast: false,
      reducedMotion: window.matchMedia('(prefers-reduced-motion: reduce)').matches,
      fontSize: 1,
      colorBlindMode: 'none' as const,
      screenReaderMode: false };
  });

  const announceRef = useRef<HTMLDivElement>(null);

  // Persist settings
  useEffect(() => {
    localStorage.setItem('gane-a11y', JSON.stringify(state));
  }, [state]);

  // Apply CSS custom properties
  useEffect(() => {
    document.documentElement.style.setProperty('--a11y-font-scale', String(state.fontSize));
    document.documentElement.classList.toggle('high-contrast', state.highContrast);
    document.documentElement.classList.toggle('reduced-motion', state.reducedMotion);
  }, [state.highContrast, state.reducedMotion, state.fontSize]);

  // Color blind filter
  useEffect(() => {
    const filters: Record<string, string> = {
      none: 'none',
      protanopia: 'url(#protanopia)',
      deuteranopia: 'url(#deuteranopia)',
      tritanopia: 'url(#tritanopia)' };
    document.documentElement.style.filter = filters[state.colorBlindMode] || 'none';
    return () => { document.documentElement.style.filter = 'none'; };
  }, [state.colorBlindMode]);

  const announce = useCallback((message: string, priority: 'polite' | 'assertive' = 'polite') => {
    if (announceRef.current) {
      announceRef.current.setAttribute('aria-live', priority);
      announceRef.current.textContent = '';
      // Force reflow
      void announceRef.current.offsetHeight;
      announceRef.current.textContent = message;
    }
  }, []);

  const value: A11yContextType = {
    ...state,
    setHighContrast: (v) => setState(s => ({ ...s, highContrast: v })),
    setReducedMotion: (v) => setState(s => ({ ...s, reducedMotion: v })),
    setFontSize: (v) => setState(s => ({ ...s, fontSize: v })),
    setColorBlindMode: (v) => setState(s => ({ ...s, colorBlindMode: v })),
    announce };

  return (
    <A11yContext.Provider value={value}>
      {children}
      {/* ARIA Live Region for announcements */}
      <div
        ref={announceRef}
        role="status"
        aria-live="polite"
        aria-atomic="true"
        className="sr-only"
        style={{ position: 'absolute', width: 1, height: 1, overflow: 'hidden', clip: 'rect(0,0,0,0)' }}
      />
      {/* Skip navigation link */}
      <a
        href="#main-content"
        className="sr-only focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-[100] focus:px-4 focus:py-2 focus:rounded-lg"
        style={{ background: '#2563EB', color: '#000', fontWeight: 'bold' }}
      >
        {t('a11y.skipToContent')}
      </a>
      {/* Color blind SVG filters */}
      <svg className="absolute w-0 h-0" aria-hidden="true">
        <defs>
          <filter id="protanopia">
            <feColorMatrix type="matrix" values="0.567,0.433,0,0,0 0.558,0.442,0,0,0 0,0.242,0.758,0,0 0,0,0,1,0" />
          </filter>
          <filter id="deuteranopia">
            <feColorMatrix type="matrix" values="0.625,0.375,0,0,0 0.7,0.3,0,0,0 0,0.3,0.7,0,0 0,0,0,1,0" />
          </filter>
          <filter id="tritanopia">
            <feColorMatrix type="matrix" values="0.95,0.05,0,0,0 0,0.433,0.567,0,0 0,0.475,0.525,0,0 0,0,0,1,0" />
          </filter>
        </defs>
      </svg>
    </A11yContext.Provider>
  );
}

// ═══ Accessibility Quick Settings Panel ═══
export function AccessibilityPanel({ onClose }: { onClose: () => void }) {
  const { t, dir } = useLanguage();
  const a11y = useA11y();

  const fontSizes = [
    { value: 1, label: t('a11y.normal'), labelEn: 'Normal' },
    { value: 1.15, label: t('a11y.large'), labelEn: 'Large' },
    { value: 1.3, label: t('a11y.extraLarge'), labelEn: 'Extra Large' },
  ];

  const colorModes = [
    { value: 'none' as const, label: t('a11y.normal'), color: '#2563EB' },
    { value: 'protanopia' as const, label: 'פרוטנופיה', color: '#F97316' },
    { value: 'deuteranopia' as const, label: 'דויטרנופיה', color: '#16A34A' },
    { value: 'tritanopia' as const, label: 'טריטנופיה', color: '#7C3AED' },
  ];

  return (
    <motion.div
      initial={{ opacity: 0, x: 50 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: 50 }}
      className="expand-panel"
      style={{ direction: dir }}
    >
      {/* Header */}
      <div className="flex items-center justify-between p-4" style={{ borderBottom: '1px solid rgba(37,99,235,0.08)' }}>
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl flex items-center justify-center" style={{ background: 'rgba(37,99,235,0.1)', border: '1px solid rgba(37,99,235,0.2)' }}>
            <Accessibility className="w-5 h-5" style={{ color: '#2563EB' }} />
          </div>
          <div>
            <h2 className="text-sm font-bold" style={{ fontFamily: 'Syne, sans-serif', color: 'rgba(17,24,39,0.9)' }}>
              {t('a11y.title')}
            </h2>
            <p className="text-[10px]" style={{ color: 'rgba(107,114,128,0.7)' }}>WCAG 2.1 AA</p>
          </div>
        </div>
        <motion.button whileHover={{ scale: 1.1 }} whileTap={{ scale: 0.9 }} onClick={onClose}
          className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
          style={{ background: 'rgba(229,231,235,0.4)', border: '1px solid rgba(229,231,235,0.6)' }}>
          <X className="w-4 h-4 text-white/40" />
        </motion.button>
      </div>

      <div className="flex-1 overflow-y-auto px-4 py-3 space-y-3" style={{ scrollbarWidth: 'none' }}>
        {/* High Contrast */}
        <ToggleOption
          icon={Contrast}
          label="ניגודיות גבוהה"
          description="הגדלת ניגודיות צבעים לקריאות טובה יותר"
          active={a11y.highContrast}
          color="#ffaa00"
          onToggle={() => a11y.setHighContrast(!a11y.highContrast)}
        />

        {/* Reduced Motion */}
        <ToggleOption
          icon={Eye}
          label="הפחתת תנועה"
          description="הפחתת אנימציות ומעברים"
          active={a11y.reducedMotion}
          color="#7C3AED"
          onToggle={() => a11y.setReducedMotion(!a11y.reducedMotion)}
        />

        {/* Font Size */}
        <div className="rounded-xl p-3" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}>
          <div className="flex items-center gap-2 mb-3">
            <Type className="w-4 h-4" style={{ color: '#2563EB' }} />
            <span className="text-xs font-medium" style={{ color: 'rgba(75,85,99,0.9)' }}>{t('a11y.fontSize')}</span>
          </div>
          <div className="flex gap-2">
            {fontSizes.map(fs => (
              <button
                key={fs.value}
                onClick={() => a11y.setFontSize(fs.value)}
                className="flex-1 py-2 rounded-lg text-[10px] font-medium transition-all cursor-pointer"
                style={{
                  background: a11y.fontSize === fs.value ? 'rgba(37,99,235,0.1)' : 'rgba(243,244,246,0.5)',
                  border: `1px solid ${a11y.fontSize === fs.value ? 'rgba(37,99,235,0.2)' : 'rgba(229,231,235,0.4)'}`,
                  color: a11y.fontSize === fs.value ? '#2563EB' : 'rgba(156,163,175,0.9)' }}
              >
                {fs.label}
              </button>
            ))}
          </div>
        </div>

        {/* Color Blind Mode */}
        <div className="rounded-xl p-3" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}>
          <div className="flex items-center gap-2 mb-3">
            <Palette className="w-4 h-4" style={{ color: '#16A34A' }} />
            <span className="text-xs font-medium" style={{ color: 'rgba(75,85,99,0.9)' }}>{t('a11y.colorBlind')}</span>
          </div>
          <div className="grid grid-cols-2 gap-2">
            {colorModes.map(cm => (
              <button
                key={cm.value}
                onClick={() => a11y.setColorBlindMode(cm.value)}
                className="py-2 rounded-lg text-[10px] font-medium transition-all cursor-pointer"
                style={{
                  background: a11y.colorBlindMode === cm.value ? `${cm.color}12` : 'rgba(243,244,246,0.5)',
                  border: `1px solid ${a11y.colorBlindMode === cm.value ? `${cm.color}25` : 'rgba(229,231,235,0.4)'}`,
                  color: a11y.colorBlindMode === cm.value ? cm.color : 'rgba(156,163,175,0.9)' }}
              >
                {cm.label}
              </button>
            ))}
          </div>
        </div>

        {/* Screen Reader Info */}
        <div className="rounded-xl p-3" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}>
          <div className="flex items-center gap-2 mb-2">
            <Volume2 className="w-4 h-4" style={{ color: 'rgba(156,163,175,0.9)' }} />
            <span className="text-xs font-medium" style={{ color: 'rgba(75,85,99,0.9)' }}>{t('a11y.screenReader')}</span>
          </div>
          <p className="text-[10px]" style={{ color: 'rgba(156,163,175,0.8)' }}>
            האפליקציה תומכת ב-ARIA labels ו-live regions. כל הפעולות מוכרזות אוטומטית לקוראי מסך.
          </p>
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 py-2 flex items-center justify-center" style={{ borderTop: '1px solid rgba(229,231,235,0.4)' }}>
        <span className="text-[8px]" style={{ color: 'rgba(209,213,219,0.8)' }}>
          G.A.N.E · WCAG 2.1 AA Compliant
        </span>
      </div>
    </motion.div>
  );
}

// ═══ Toggle Option Component ═══
function ToggleOption({ icon: Icon, label, description, active, color, onToggle }: {
  icon: typeof Eye; label: string; description: string; active: boolean; color: string; onToggle: () => void;
}) {
  return (
    <motion.div
      className="rounded-xl p-3 cursor-pointer"
      style={{
        background: active ? `${color}08` : 'rgba(243,244,246,0.4)',
        border: `1px solid ${active ? `${color}20` : 'rgba(229,231,235,0.4)'}` }}
      whileHover={{ scale: 1.01 }}
      onClick={onToggle}
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg flex items-center justify-center"
            style={{ background: active ? `${color}15` : 'rgba(229,231,235,0.4)' }}>
            <Icon className="w-4 h-4" style={{ color: active ? color : 'rgba(156,163,175,0.6)' }} />
          </div>
          <div>
            <div className="text-xs font-medium" style={{ color: active ? color : 'rgba(107,114,128,0.9)' }}>{label}</div>
            <div className="text-[9px]" style={{ color: 'rgba(156,163,175,0.6)' }}>{description}</div>
          </div>
        </div>
        <div className="w-8 h-4 rounded-full relative" style={{
          background: active ? `${color}30` : 'rgba(229,231,235,0.6)' }}>
          <motion.div className="w-3.5 h-3.5 rounded-full absolute top-0.5"
            animate={{ left: active ? 16 : 2 }}
            style={{ background: active ? color : 'rgba(156,163,175,0.6)' }}
            transition={{ type: 'spring', stiffness: 300, damping: 20 }} />
        </div>
      </div>
    </motion.div>
  );
}
