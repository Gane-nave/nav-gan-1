/**
 * G.A.N.E — Search Bar (Responsive)
 * 
 * Adapts to device type:
 * - Phone: Full width, compact height, no mode pill, no layer count
 * - Tablet: Adjusted left offset for collapsed sidebar
 * - Desktop: Full search bar with all elements
 * 
 * i18n: Uses useLanguage() for all localized labels.
 */
import { useState, useCallback } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { Search, Mic, X, Layers } from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";
import { useLanguage } from "@/contexts/LanguageContext";
import { NightModeToggle } from "@/components/NightMode";
import useDeviceAdaptation from "@/hooks/useDeviceAdaptation";

interface SearchBarProps {
  activeMode: {
    id: string;
    icon: React.ComponentType<{ className?: string }>;
    label: string;
    labelHe: string;
    i18nKey: import("@/lib/i18n").TranslationKey;
    color: string;
  };
  showSidebar: boolean;
}

export default function SearchBar({ activeMode, showSidebar }: SearchBarProps) {
  const { state, dispatch } = useNavigation();
  const { t, dir } = useLanguage();
  const [isListening, setIsListening] = useState(false);
  const [isNightMode, setIsNightMode] = useState(false);
  const device = useDeviceAdaptation();

  const toggleVoiceSearch = useCallback(() => {
    if (isListening) {
      setIsListening(false);
    } else {
      setIsListening(true);
      setTimeout(() => {
        setIsListening(false);
        dispatch({ type: 'SET_VIEW', view: 'search' });
      }, 2000);
    }
  }, [isListening, dispatch]);

  const leftOffset = device.isPhone
    ? '8px'
    : showSidebar
      ? (device.isTablet ? '84px' : '108px')
      : '16px';
  const rightOffset = device.isPhone ? '8px' : '130px';

  return (
    <motion.div
      initial={{ y: -30, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ type: 'spring', damping: 22, stiffness: 240, delay: 0.2 }}
      className="fixed z-20"
      style={{
        top: device.isPhone ? 'calc(8px + env(safe-area-inset-top, 0px))' : '12px',
        left: leftOffset,
        right: rightOffset,
      }}
    >
      <div className={`flex items-center ${device.isPhone ? 'gap-2' : 'gap-3'}`}>
        {/* Main search button */}
        <button
          onClick={() => dispatch({ type: 'SET_VIEW', view: 'search' })}
          className={`flex-1 flex items-center gap-2 ${device.isPhone ? 'px-3 py-2.5' : 'px-5 py-3.5'} rounded-2xl text-left transition-all duration-300 group cursor-pointer relative overflow-hidden`}
          style={{
            background: '#FFFFFF',
            border: '1px solid #E5E7EB',
            boxShadow: '0 2px 8px rgba(0,0,0,0.06)',
            minHeight: device.isPhone ? '44px' : undefined,
          }}
          aria-label="Open search"
        >
          <Search className={`${device.isPhone ? 'w-4 h-4' : 'w-5 h-5'} text-gray-400 group-hover:text-gray-600 transition-colors duration-300`} />
          <span className={`${device.isPhone ? 'text-xs' : 'text-sm'} text-gray-400 group-hover:text-gray-600 transition-colors duration-300 font-medium`} style={{ direction: dir }}>
            {t('search.whereTo')}
          </span>
          {/* Mode pill + Layer count — hidden on phone */}
          {!device.isPhone && (
            <div className="ml-auto flex items-center gap-2">
              <motion.div
                key={activeMode.id}
                initial={{ scale: 0.8, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
                className="flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[11px] font-bold"
                style={{
                  background: `${activeMode.color}20`,
                  color: activeMode.color,
                  border: `1px solid ${activeMode.color}40`,
                }}>
                <activeMode.icon className="w-3.5 h-3.5" />
                {t(activeMode.i18nKey)}
              </motion.div>
              <div className="flex items-center gap-1 px-2.5 py-1 rounded-full text-[10px] font-bold text-gray-500"
                style={{ background: '#F3F4F6', border: '1px solid #E5E7EB' }}>
                <Layers className="w-3 h-3 text-gray-500" /> {state.activeLayers.length}
              </div>
            </div>
          )}
        </button>

        {/* Night mode toggle — hidden on phone */}
        {!device.isPhone && (
          <NightModeToggle isNight={isNightMode} onToggle={() => setIsNightMode(!isNightMode)} />
        )}

        {/* Voice search */}
        <motion.button
          whileHover={device.isPhone ? undefined : { scale: 1.08 }}
          whileTap={{ scale: 0.9 }}
          onClick={toggleVoiceSearch}
          className={`flex-shrink-0 ${device.isPhone ? 'w-10 h-10 rounded-xl' : 'w-12 h-12 rounded-2xl'} flex items-center justify-center transition-all duration-300 cursor-pointer relative`}
          style={{
            background: isListening ? `${activeMode.color}12` : '#FFFFFF',
            border: isListening ? `1px solid ${activeMode.color}40` : '1px solid #E5E7EB',
            boxShadow: isListening ? `0 0 12px ${activeMode.color}20` : '0 1px 3px rgba(0,0,0,0.06)',
            minHeight: '44px',
            minWidth: '44px',
          }}
          aria-label="Voice search"
        >
          {isListening ? (
            <motion.div animate={{ scale: [1, 1.2, 1] }} transition={{ duration: 0.5, repeat: Infinity }}>
              <Mic className={`${device.isPhone ? 'w-4 h-4' : 'w-5 h-5'}`} style={{ color: activeMode.color }} />
            </motion.div>
          ) : (
            <Mic className={`${device.isPhone ? 'w-4 h-4' : 'w-5 h-5'} text-gray-400`} />
          )}
        </motion.button>
      </div>

      {/* Voice listening overlay */}
      <AnimatePresence>
        {isListening && (
          <motion.div
            initial={{ opacity: 0, y: -6, height: 0 }}
            animate={{ opacity: 1, y: 0, height: 'auto' }}
            exit={{ opacity: 0, y: -6, height: 0 }}
            className="mt-2 rounded-2xl overflow-hidden"
            style={{
              background: '#FFFFFF',
              border: '1px solid #E5E7EB',
              boxShadow: '0 4px 12px rgba(0,0,0,0.08)',
            }}
          >
            <div className={`${device.isPhone ? 'px-3 py-3' : 'px-5 py-4'} flex items-center gap-4`}>
              <motion.div className="flex gap-1">
                {[...Array(7)].map((_, i) => (
                  <motion.div
                    key={i}
                    className="w-[3px] rounded-full"
                    style={{ background: activeMode.color }}
                    animate={{ height: [6, 20 + Math.random() * 14, 6] }}
                    transition={{ duration: 0.4 + Math.random() * 0.3, repeat: Infinity, delay: i * 0.07 }}
                  />
                ))}
              </motion.div>
              <span className="text-sm text-gray-400 font-medium" style={{ direction: dir }}>{t('search.listening')}</span>
              <button
                onClick={() => setIsListening(false)}
                className="ml-auto p-2 rounded-lg hover:bg-gray-100 transition-colors cursor-pointer"
                style={{ minHeight: '44px', minWidth: '44px' }}
              >
                <X className="w-4 h-4 text-gray-400" />
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  );
}
