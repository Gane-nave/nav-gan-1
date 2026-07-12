/**
 * G.A.N.E — Bottom Navigation Dock (Responsive)
 * 
 * Adapts to device type:
 * - Phone: Compact mode selector + central nav button, no right side actions (sidebar handles those)
 * - Tablet: Standard dock with slightly smaller elements
 * - Desktop: Full dock with all elements
 * 
 * Performance-aware: infinite animations gated by useAnimationPerformance.
 * i18n: Uses useLanguage() for all localized labels.
 */
import { AnimatePresence, motion } from "framer-motion";
import { Navigation, Settings, Signal, ChevronRight } from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";
import { useLanguage } from "@/contexts/LanguageContext";
import useAnimationPerformance from "@/hooks/useAnimationPerformance";
import useDeviceAdaptation from "@/hooks/useDeviceAdaptation";
import { COLORS, navModes } from "./homeConstants";

interface BottomDockProps {
  activeMode: {
    id: string;
    color: string;
    gradient: string;
  };
  showSidebar: boolean;
}

export default function BottomDock({ activeMode, showSidebar }: BottomDockProps) {
  const { state, dispatch } = useNavigation();
  const { shouldAnimateLight } = useAnimationPerformance();
  const { t } = useLanguage();
  const device = useDeviceAdaptation();

  // Phone: position above the bottom bar (56px + safe area)
  const bottomOffset = device.isPhone ? 'calc(60px + env(safe-area-inset-bottom, 0px))' : '14px';
  const leftOffset = device.isPhone ? '8px' : showSidebar ? (device.isTablet ? '84px' : '108px') : '16px';
  const rightOffset = device.isPhone ? '8px' : '16px';

  return (
    <motion.div
      initial={{ y: 50, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ type: 'spring', damping: 24, stiffness: 260, delay: 0.3 }}
      className="fixed z-20"
      style={{ left: leftOffset, right: rightOffset, bottom: bottomOffset }}
    >
      {/* Quick favorites — hidden on phone to save space */}
      {!device.isPhone && state.favorites.length > 0 && (
        <div className="flex items-center gap-2 mb-2.5 overflow-x-auto px-1"
          style={{ scrollbarWidth: 'none' }}>
          {state.favorites.map((fav, i) => (
            <motion.button
              key={fav.id}
              initial={{ opacity: 0, y: 8 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: i * 0.08 + 0.4 }}
              whileHover={{ scale: 1.04, y: -2 }}
              whileTap={{ scale: 0.96 }}
              onClick={() => {
                dispatch({ type: 'SET_DESTINATION', place: fav });
                dispatch({ type: 'SET_VIEW', view: 'route-plan' });
              }}
              className="flex items-center gap-2.5 px-4 py-2.5 rounded-xl transition-all duration-200 flex-shrink-0 cursor-pointer group"
              style={{
                background: '#FFFFFF',
                border: '1px solid #E5E7EB',
                boxShadow: '0 1px 3px rgba(0,0,0,0.06)',
              }}
            >
              <span className="text-base">{fav.icon || '📍'}</span>
              <span className="text-xs text-gray-500 font-semibold group-hover:text-gray-700 transition-colors">{fav.name}</span>
              <ChevronRight className="w-3.5 h-3.5 text-gray-400 group-hover:text-gray-600 transition-all" />
            </motion.button>
          ))}
        </div>
      )}

      {/* Main dock */}
      <div className="rounded-2xl overflow-hidden relative"
        style={{
          background: '#FFFFFF',
          border: '1px solid #E5E7EB',
          boxShadow: '0 -2px 12px rgba(0,0,0,0.06)',
        }}>
        {/* Top accent line */}
        <motion.div
          className="absolute top-0 left-0 right-0 h-[2px]"
          style={{ background: `linear-gradient(90deg, transparent 10%, ${activeMode.color}50 50%, transparent 90%)` }}
          animate={shouldAnimateLight ? { opacity: [0.4, 0.7, 0.4] } : { opacity: 0.55 }}
          transition={shouldAnimateLight ? { duration: 3, repeat: Infinity } : { duration: 0 }}
        />

        <div className={`flex items-center justify-between ${device.isPhone ? 'px-3 py-3' : device.isTablet ? 'px-4 py-4' : 'px-6 py-5'}`}>
          {/* Mode selector — LEFT */}
          <div className={`flex items-center ${device.isPhone ? 'gap-1.5' : 'gap-3'}`}>
            {navModes.map((mode) => {
              const isActiveMode = state.navMode === mode.id;
              return (
                <motion.button
                  key={mode.id}
                  whileHover={device.isPhone ? undefined : { scale: 1.06 }}
                  whileTap={{ scale: 0.9 }}
                  onClick={() => dispatch({ type: 'SET_NAV_MODE', mode: mode.id })}
                  className="flex items-center gap-1.5 rounded-xl text-xs font-bold transition-all duration-300 cursor-pointer relative"
                  style={{
                    padding: device.isPhone
                      ? (isActiveMode ? '8px 12px' : '8px 10px')
                      : (isActiveMode ? '12px 18px' : '12px 14px'),
                    background: isActiveMode ? `${mode.color}12` : '#F3F4F6',
                    border: isActiveMode ? `1px solid ${mode.color}30` : '1px solid #E5E7EB',
                    color: isActiveMode ? mode.color : '#6B7280',
                    boxShadow: isActiveMode ? `0 2px 8px ${mode.color}15` : 'none',
                    minHeight: device.isPhone ? '40px' : undefined,
                  }}
                  aria-label={`${mode.label} mode`}
                >
                  <mode.icon className={device.isPhone ? "w-4 h-4" : "w-5 h-5"} />
                  <AnimatePresence mode="wait">
                    {isActiveMode && !device.isPhone && (
                      <motion.span
                        key={mode.id}
                        initial={{ width: 0, opacity: 0 }}
                        animate={{ width: 'auto', opacity: 1 }}
                        exit={{ width: 0, opacity: 0 }}
                        transition={{ duration: 0.2 }}
                        className="overflow-hidden whitespace-nowrap tracking-wide text-[12px]"
                      >
                        {t(mode.i18nKey)}
                      </motion.span>
                    )}
                  </AnimatePresence>
                </motion.button>
              );
            })}
          </div>

          {/* Central navigate button */}
          <motion.button
            whileHover={device.isPhone ? undefined : { scale: 1.1 }}
            whileTap={{ scale: 0.88 }}
            onClick={() => dispatch({ type: 'SET_VIEW', view: 'search' })}
            className="relative cursor-pointer mx-2"
            aria-label="Navigate"
            style={{ minWidth: 44, minHeight: 44 }}
          >
            {shouldAnimateLight && (
              <motion.div
                className="absolute inset-[-10px] rounded-full pointer-events-none"
                style={{ background: `radial-gradient(circle, ${activeMode.color}25, transparent 70%)` }}
                animate={{ opacity: [0.3, 0.6, 0.3], scale: [1, 1.15, 1] }}
                transition={{ duration: 2.5, repeat: Infinity }}
              />
            )}
            <div className={`${device.isPhone ? 'w-12 h-12' : 'w-16 h-16'} rounded-full flex items-center justify-center relative z-10`}
              style={{
                background: activeMode.gradient,
                boxShadow: `0 4px 16px ${activeMode.color}30`,
              }}>
              <Navigation className={device.isPhone ? "w-5 h-5 text-black" : "w-7 h-7 text-black"} />
            </div>
            {shouldAnimateLight && (
              <motion.div
                className="absolute inset-[-5px] rounded-full pointer-events-none"
                style={{ border: `1.5px solid ${activeMode.color}35` }}
                animate={{ scale: [1, 1.25, 1], opacity: [0.4, 0, 0.4] }}
                transition={{ duration: 2, repeat: Infinity }}
              />
            )}
          </motion.button>

          {/* Right side — Traffic + Settings (hidden on phone, handled by sidebar bottom bar) */}
          {!device.isPhone && (
            <div className={`flex items-center ${device.isTablet ? 'gap-2' : 'gap-3'}`}>
              <motion.button
                whileHover={{ scale: 1.08 }}
                whileTap={{ scale: 0.9 }}
                onClick={() => dispatch({ type: 'SET_VIEW', view: 'traffic' })}
                className={`flex flex-col items-center gap-1.5 ${device.isTablet ? 'px-3 py-2' : 'px-4 py-3'} rounded-xl transition-all duration-300 cursor-pointer`}
                style={{ background: `${COLORS.orange}08`, border: `1px solid ${COLORS.orange}20` }}
                aria-label="Traffic"
              >
                <Signal className={device.isTablet ? "w-4 h-4" : "w-5 h-5"} style={{ color: COLORS.orange }} />
                <span className="text-[10px] font-bold" style={{ color: `${COLORS.orange}90` }}>{t('dock.traffic')}</span>
              </motion.button>
              <motion.button
                whileHover={{ scale: 1.08, rotate: 45 }}
                whileTap={{ scale: 0.9 }}
                onClick={() => dispatch({ type: 'SET_VIEW', view: 'settings' })}
                className={`flex flex-col items-center gap-1.5 ${device.isTablet ? 'px-3 py-2' : 'px-4 py-3'} rounded-xl transition-all duration-300 cursor-pointer`}
                style={{ background: '#F3F4F6', border: '1px solid #E5E7EB' }}
                aria-label="Settings"
              >
                <Settings className={device.isTablet ? "w-4 h-4" : "w-5 h-5"} style={{ color: '#9CA3AF' }} />
                <span className="text-[10px] font-bold text-gray-400">{t('dock.settings')}</span>
              </motion.button>
            </div>
          )}
        </div>
      </div>
    </motion.div>
  );
}
