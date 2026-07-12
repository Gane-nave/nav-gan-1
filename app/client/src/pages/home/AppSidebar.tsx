/**
 * G.A.N.E — Application Sidebar (Responsive)
 * 
 * Adapts to device type:
 * - Phone: Horizontal bottom bar with group icons, swipe-up panel
 * - Tablet: Narrow collapsed sidebar (icons only)
 * - Desktop: Full sidebar with labels and groups
 * - Ultrawide: Expanded sidebar
 * 
 * Performance-aware: infinite animations gated by useAnimationPerformance.
 * i18n: Uses useLanguage() for all localized labels.
 */
import { useState, useCallback } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { Navigation, Settings, Signal, Shield, ChevronDown, Menu, X } from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";
import { useAdminMode } from "@/contexts/AdminModeContext";
import { useLanguage } from "@/contexts/LanguageContext";
import { useLocation } from "wouter";
import useAnimationPerformance from "@/hooks/useAnimationPerformance";
import useDeviceAdaptation from "@/hooks/useDeviceAdaptation";
import type { SmartPanel } from "@/lib/navStore";
import SidebarIcon from "./SidebarIcon";
import { COLORS, sidebarGroups } from "./homeConstants";

interface AppSidebarProps {
  activeMode: {
    id: string;
    color: string;
    gradient: string;
  };
  onTogglePanel: (panel: SmartPanel) => void;
}

export default function AppSidebar({ activeMode, onTogglePanel }: AppSidebarProps) {
  const { state, dispatch } = useNavigation();
  const { isAdmin } = useAdminMode();
  const { t } = useLanguage();
  const [, setLocation] = useLocation();
  const [openGroup, setOpenGroup] = useState<string>('NAV');
  const { shouldAnimateLight } = useAnimationPerformance();
  const device = useDeviceAdaptation();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  // ═══ PHONE: Horizontal bottom bar + expandable menu ═══
  if (device.isPhone) {
    return (
      <>
        {/* Bottom navigation bar */}
        <motion.div
          initial={{ y: 80, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          exit={{ y: 80, opacity: 0 }}
          transition={{ type: "spring", damping: 26, stiffness: 260 }}
          className="fixed left-0 right-0 bottom-0 z-40 flex items-center justify-around"
          style={{
            height: `calc(56px + env(safe-area-inset-bottom, 0px))`,
            paddingBottom: 'env(safe-area-inset-bottom, 0px)',
            background: '#FFFFFF',
            borderTop: '1px solid #E5E7EB',
            boxShadow: '0 -2px 12px rgba(0,0,0,0.06)',
          }}
        >
          {/* Quick access: first item from each group */}
          {sidebarGroups.slice(0, 4).map((group) => {
            const firstItem = group.items[0];
            const isActive = state.activePanel === firstItem.id;
            return (
              <motion.button
                key={group.label}
                whileTap={{ scale: 0.85 }}
                onClick={() => onTogglePanel(firstItem.id)}
                className="flex flex-col items-center gap-0.5 py-1 px-2 cursor-pointer"
                style={{ minWidth: 44, minHeight: 44 }}
              >
                <div className="w-6 h-6 flex items-center justify-center rounded-lg"
                  style={{
                    background: isActive ? `${group.color}15` : 'transparent',
                    color: isActive ? group.color : '#9CA3AF',
                  }}>
                  <firstItem.icon className="w-5 h-5" />
                </div>
                <span className="text-[9px] font-semibold" style={{ color: isActive ? group.color : '#9CA3AF' }}>
                  {t(group.i18nKey)}
                </span>
              </motion.button>
            );
          })}
          {/* Menu button to expand full panel list */}
          <motion.button
            whileTap={{ scale: 0.85 }}
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            className="flex flex-col items-center gap-0.5 py-1 px-2 cursor-pointer"
            style={{ minWidth: 44, minHeight: 44 }}
          >
            <div className="w-6 h-6 flex items-center justify-center rounded-lg"
              style={{ color: mobileMenuOpen ? activeMode.color : '#9CA3AF' }}>
              {mobileMenuOpen ? <X className="w-5 h-5" /> : <Menu className="w-5 h-5" />}
            </div>
            <span className="text-[9px] font-semibold" style={{ color: mobileMenuOpen ? activeMode.color : '#9CA3AF' }}>
              {t('dock.more') || 'More'}
            </span>
          </motion.button>
        </motion.div>

        {/* Expandable full menu — bottom sheet */}
        <AnimatePresence>
          {mobileMenuOpen && (
            <>
              {/* Backdrop */}
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="fixed inset-0 z-39 bg-black/20"
                onClick={() => setMobileMenuOpen(false)}
              />
              {/* Sheet */}
              <motion.div
                initial={{ y: '100%' }}
                animate={{ y: 0 }}
                exit={{ y: '100%' }}
                transition={{ type: "spring", damping: 28, stiffness: 300 }}
                className="fixed left-0 right-0 bottom-0 z-40 overflow-y-auto"
                style={{
                  maxHeight: '75vh',
                  paddingBottom: `calc(72px + env(safe-area-inset-bottom, 0px))`,
                  background: '#FFFFFF',
                  borderRadius: '24px 24px 0 0',
                  boxShadow: '0 -8px 32px rgba(0,0,0,0.12)',
                }}
              >
                {/* Handle */}
                <div className="flex justify-center pt-3 pb-2">
                  <div className="w-10 h-1 rounded-full bg-gray-300" />
                </div>
                {/* Groups */}
                <div className="px-4 pb-4">
                  {sidebarGroups.map((group) => (
                    <div key={group.label} className="mb-4">
                      <div className="flex items-center gap-2 mb-2 px-1">
                        <div className="w-2 h-2 rounded-full" style={{ background: group.color }} />
                        <span className="text-xs font-bold tracking-wider uppercase" style={{ color: group.color }}>
                          {t(group.i18nKey)}
                        </span>
                      </div>
                      <div className="grid grid-cols-4 gap-2">
                        {group.items.map((item) => {
                          const isActive = state.activePanel === item.id;
                          return (
                            <motion.button
                              key={item.id}
                              whileTap={{ scale: 0.9 }}
                              onClick={() => {
                                onTogglePanel(item.id);
                                setMobileMenuOpen(false);
                              }}
                              className="flex flex-col items-center gap-1 py-2 px-1 rounded-xl cursor-pointer"
                              style={{
                                background: isActive ? `${item.accent}12` : '#F9FAFB',
                                border: `1px solid ${isActive ? `${item.accent}30` : '#E5E7EB'}`,
                                minHeight: 44,
                              }}
                            >
                              <item.icon className="w-5 h-5" style={{ color: isActive ? item.accent : '#6B7280' }} />
                              <span className="text-[9px] font-medium text-center leading-tight"
                                style={{ color: isActive ? item.accent : '#6B7280' }}>
                                {t(item.i18nKey)}
                              </span>
                            </motion.button>
                          );
                        })}
                      </div>
                    </div>
                  ))}
                  {/* Bottom actions */}
                  <div className="flex gap-3 mt-4">
                    <motion.button
                      whileTap={{ scale: 0.9 }}
                      onClick={() => { dispatch({ type: 'SET_VIEW', view: 'traffic' }); setMobileMenuOpen(false); }}
                      className="flex-1 flex items-center justify-center gap-2 py-3 rounded-xl cursor-pointer"
                      style={{ background: '#FFF7ED', border: '1px solid #FDBA74', minHeight: 44 }}
                    >
                      <Signal className="w-4 h-4" style={{ color: COLORS.orange }} />
                      <span className="text-xs font-semibold" style={{ color: COLORS.orange }}>{t('dock.traffic')}</span>
                    </motion.button>
                    <motion.button
                      whileTap={{ scale: 0.9 }}
                      onClick={() => { dispatch({ type: 'SET_VIEW', view: 'settings' }); setMobileMenuOpen(false); }}
                      className="flex-1 flex items-center justify-center gap-2 py-3 rounded-xl cursor-pointer"
                      style={{ background: '#F3F4F6', border: '1px solid #D1D5DB', minHeight: 44 }}
                    >
                      <Settings className="w-4 h-4" style={{ color: '#6B7280' }} />
                      <span className="text-xs font-semibold" style={{ color: '#6B7280' }}>{t('dock.settings')}</span>
                    </motion.button>
                    {isAdmin && (
                      <motion.button
                        whileTap={{ scale: 0.9 }}
                        onClick={() => { setLocation('/admin'); setMobileMenuOpen(false); }}
                        className="flex-1 flex items-center justify-center gap-2 py-3 rounded-xl cursor-pointer"
                        style={{ background: '#F5F3FF', border: '1px solid #C4B5FD', minHeight: 44 }}
                      >
                        <Shield className="w-4 h-4" style={{ color: '#7C3AED' }} />
                        <span className="text-xs font-semibold" style={{ color: '#7C3AED' }}>{t('dock.admin')}</span>
                      </motion.button>
                    )}
                  </div>
                </div>
              </motion.div>
            </>
          )}
        </AnimatePresence>
      </>
    );
  }

  // ═══ TABLET: Narrow collapsed sidebar (icons only, 72px) ═══
  const sidebarWidth = device.isTablet ? 72 : 96;

  return (
    <motion.div
      initial={{ x: -sidebarWidth, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -sidebarWidth, opacity: 0 }}
      transition={{ type: "spring", damping: 26, stiffness: 260 }}
      className="fixed left-0 top-0 bottom-0 z-40 flex flex-col items-center"
      style={{
        width: `${sidebarWidth}px`,
        padding: device.isTablet ? '10px 0 8px' : '14px 0 10px',
        background: '#FFFFFF',
        borderRight: '1px solid #E5E7EB',
        boxShadow: '2px 0 8px rgba(0,0,0,0.04)',
      }}
    >
      {/* Top accent line — mode color */}
      <motion.div
        className="absolute top-0 left-0 right-0 h-[2px]"
        style={{ background: `linear-gradient(90deg, transparent, ${activeMode.color}, transparent)` }}
        animate={shouldAnimateLight ? { opacity: [0.4, 0.8, 0.4] } : { opacity: 0.6 }}
        transition={shouldAnimateLight ? { duration: 3, repeat: Infinity } : { duration: 0 }}
      />

      {/* ── Logo ── */}
      <motion.div className="flex flex-col items-center flex-shrink-0 pb-3 mb-2 w-full"
        style={{ borderBottom: '1px solid #E5E7EB' }}>
        <motion.button
          className={`${device.isTablet ? 'w-11 h-11 rounded-xl' : 'w-14 h-14 rounded-2xl'} flex items-center justify-center relative cursor-pointer`}
          style={{
            background: `${activeMode.color}08`,
            border: `1px solid ${activeMode.color}20`,
            boxShadow: `0 2px 8px ${activeMode.color}10`,
          }}
          whileHover={{ scale: 1.08, rotate: 5 }}
          whileTap={{ scale: 0.9 }}
          onClick={() => dispatch({ type: 'SET_VIEW', view: 'search' })}
          aria-label="G.A.N.E Navigation Home"
        >
          <Navigation className={device.isTablet ? "w-5 h-5" : "w-7 h-7"} style={{ color: activeMode.color }} />
          {shouldAnimateLight && (
            <motion.div
              className="absolute w-2 h-2 rounded-full"
              style={{ background: activeMode.color, boxShadow: `0 0 6px ${activeMode.color}40` }}
              animate={{ rotate: 360, x: [0, 22, 0, -22, 0], y: [-22, 0, 22, 0, -22] }}
              transition={{ duration: 5, repeat: Infinity, ease: 'linear' }}
            />
          )}
        </motion.button>
        {!device.isTablet && (
          <div className="text-[9px] font-black tracking-[0.4em] mt-2"
            style={{ color: activeMode.color, fontFamily: 'Syne, sans-serif', opacity: 0.8 }}>
            G.A.N.E
          </div>
        )}
      </motion.div>

      {/* ── Collapsible panel groups ── */}
      <div className="flex-1 flex flex-col items-center overflow-y-auto py-2 w-full gap-1"
        style={{ scrollbarWidth: 'none' }}>
        {sidebarGroups.map((group, gi) => {
          const isOpen = openGroup === group.label;
          const hasActiveItem = group.items.some(item => state.activePanel === item.id);
          return (
            <div key={group.label} className="w-full flex flex-col items-center">
              {/* Group header */}
              <motion.button
                className={`w-full flex items-center justify-center gap-${device.isTablet ? '1' : '2'} ${device.isTablet ? 'py-3' : 'py-4'} cursor-pointer relative`}
                onClick={() => {
                  if (!isOpen) dispatch({ type: 'SET_ACTIVE_PANEL', panel: null });
                  setOpenGroup(isOpen ? '' : group.label);
                }}
                whileHover={{ backgroundColor: '#F3F4F6' }}
                whileTap={{ scale: 0.97 }}
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: gi * 0.08 + 0.3 }}
                style={{ borderRadius: '8px' }}
              >
                {(isOpen || hasActiveItem) && (
                  <motion.div
                    className="absolute left-0 top-1/2 -translate-y-1/2 w-[3px] rounded-r-full"
                    style={{ height: '20px', background: group.color, boxShadow: `0 0 4px ${group.color}30` }}
                    layoutId={`group-indicator-${group.label}`}
                  />
                )}
                <div className={`${device.isTablet ? 'w-2 h-2' : 'w-2.5 h-2.5'} rounded-full flex-shrink-0`}
                  style={{ background: group.color, boxShadow: `0 0 4px ${group.color}30` }} />
                {!device.isTablet && (
                  <span className="text-[11px] font-bold tracking-[0.2em] uppercase"
                    style={{ fontFamily: 'Syne, sans-serif', color: isOpen ? group.color : '#9CA3AF' }}>
                    {t(group.i18nKey)}
                  </span>
                )}
                <motion.div
                  animate={{ rotate: isOpen ? 180 : 0 }}
                  transition={{ duration: 0.2 }}
                  className={device.isTablet ? '' : 'ml-auto mr-2'}
                >
                  <ChevronDown className="w-3 h-3" style={{ color: `${group.color}50` }} />
                </motion.div>
              </motion.button>

              {/* Group items */}
              <AnimatePresence initial={false}>
                {isOpen && (
                  <motion.div
                    initial={{ height: 0, opacity: 0 }}
                    animate={{ height: 'auto', opacity: 1 }}
                    exit={{ height: 0, opacity: 0 }}
                    transition={{ duration: 0.3, ease: [0.16, 1, 0.3, 1] }}
                    className="overflow-hidden w-full"
                  >
                    <div className={`flex flex-col items-center gap-${device.isTablet ? '2' : '3'} w-full py-2`}>
                      {group.items.map((item, ii) => (
                        <SidebarIcon
                          key={item.id}
                          item={item}
                          isActive={state.activePanel === item.id}
                          modeColor={activeMode.color}
                          onClick={() => onTogglePanel(item.id)}
                          delay={ii * 0.03}
                          compact={device.isTablet}
                        />
                      ))}
                    </div>
                  </motion.div>
                )}
              </AnimatePresence>

              {/* Group divider */}
              {gi < sidebarGroups.length - 1 && (
                <div className={`${device.isTablet ? 'w-8' : 'w-12'} my-1`}>
                  <div className="h-px" style={{ background: 'linear-gradient(90deg, transparent, #E5E7EB, transparent)' }} />
                </div>
              )}
            </div>
          );
        })}
      </div>

      {/* ── Bottom: Traffic + Settings ── */}
      <div className={`flex-shrink-0 flex flex-col items-center gap-${device.isTablet ? '2' : '3'} pt-3 mt-2 w-full`}
        style={{ borderTop: '1px solid #E5E7EB' }}>
        <motion.button
          whileHover={{ scale: 1.1 }}
          whileTap={{ scale: 0.85 }}
          onClick={() => dispatch({ type: 'SET_VIEW', view: 'traffic' })}
          className={`flex flex-col items-center gap-1 ${device.isTablet ? 'w-[44px] h-[44px]' : 'w-[56px] h-[56px]'} justify-center rounded-xl cursor-pointer`}
          style={{ color: COLORS.orange }}
          aria-label="Traffic"
        >
          <Signal className={device.isTablet ? "w-4 h-4" : "w-5 h-5"} />
          {!device.isTablet && <span className="text-[9px] font-bold tracking-wider opacity-75">{t('dock.traffic')}</span>}
        </motion.button>
        <motion.button
          whileHover={{ scale: 1.1, rotate: 60 }}
          whileTap={{ scale: 0.85 }}
          onClick={() => dispatch({ type: 'SET_VIEW', view: 'settings' })}
          className={`flex flex-col items-center gap-1 ${device.isTablet ? 'w-[44px] h-[44px]' : 'w-[56px] h-[56px]'} justify-center rounded-xl cursor-pointer`}
          style={{ color: '#9CA3AF' }}
          aria-label="Settings"
        >
          <Settings className={device.isTablet ? "w-4 h-4" : "w-5 h-5"} />
          {!device.isTablet && <span className="text-[9px] font-bold tracking-wider opacity-75">{t('dock.settings')}</span>}
        </motion.button>
        {isAdmin && (
          <motion.button
            whileHover={{ scale: 1.1 }}
            whileTap={{ scale: 0.9 }}
            onClick={() => setLocation('/admin')}
            className={`flex flex-col items-center gap-1 ${device.isTablet ? 'w-[44px] h-[44px]' : 'w-[56px] h-[56px]'} justify-center rounded-xl cursor-pointer`}
            style={{ color: '#7C3AED' }}
            aria-label="Admin Panel"
          >
            <Shield className={device.isTablet ? "w-4 h-4" : "w-5 h-5"} />
            {!device.isTablet && <span className="text-[9px] font-bold tracking-wider opacity-75">{t('dock.admin')}</span>}
          </motion.button>
        )}
      </div>
    </motion.div>
  );
}
