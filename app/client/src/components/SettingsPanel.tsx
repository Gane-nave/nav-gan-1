/**
 * G.A.N.E — Settings Panel v3.0
 * Premium settings with animated toggles, glassmorphism, and holographic design
 */
import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  ArrowLeft, Volume2, VolumeX, Globe, Car, Moon, Sun,
  Bell, ChevronRight, Palette, Gauge, Shield, Smartphone,
  Wifi, Zap, Sparkles, Eye, Languages
} from "lucide-react";
import { useNavigation } from "@/contexts/NavigationContext";
import { useLanguage } from "@/contexts/LanguageContext";
import { drivingProfiles, type DrivingProfile } from "@/lib/navStore";
import { useNotifications } from "@/hooks/useNotifications";
import { languages as i18nLanguages } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n";

// Use i18n languages list instead of local duplicate
const languages = i18nLanguages;

function AnimatedToggle({ enabled, onChange, color = '#2563EB' }: { enabled: boolean; onChange: () => void; color?: string }) {
  return (
    <motion.button
      onClick={onChange}
      className="relative w-12 h-7 rounded-full cursor-pointer flex-shrink-0"
      style={{
        background: enabled ? `${color}25` : 'rgba(229,231,235,0.6)',
        border: enabled ? `1px solid ${color}40` : '1px solid rgba(209,213,219,0.5)' }}
      whileTap={{ scale: 0.92 }}
    >
      <motion.div
        className="absolute top-0.5 w-6 h-6 rounded-full"
        animate={{ left: enabled ? 22 : 2 }}
        transition={{ type: 'spring', damping: 20, stiffness: 350 }}
        style={{
          background: enabled ? color : 'rgba(156,163,175,0.6)',
          boxShadow: enabled ? `0 0 12px ${color}40` : 'none' }}
      />
    </motion.button>
  );
}

function SettingRow({ icon: Icon, label, i18nKey, children, color = '#2563EB', delay = 0 }: {
  icon: any; label: string; i18nKey?: TranslationKey; children: React.ReactNode; color?: string; delay?: number;
}) {
  const { t, dir } = useLanguage();
  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay, type: 'spring', damping: 24 }}
      className="flex items-center justify-between px-4 py-3.5 transition-colors"
      style={{ borderBottom: '1px solid rgba(243,244,246,0.5)' }}
    >
      <div className="flex items-center gap-3" style={{ direction: dir }}>
        <div className="w-8 h-8 rounded-lg flex items-center justify-center" style={{ background: `${color}10` }}>
          <Icon className="w-4 h-4" style={{ color }} />
        </div>
        <div>
          <div className="text-sm text-white/70">{i18nKey ? t(i18nKey) : label}</div>
          <div className="text-[10px] text-white/20">{label}</div>
        </div>
      </div>
      {children}
    </motion.div>
  );
}

// ── Notification Preferences Sub-Section ──
function NotificationPreferencesSection() {
  const { preferences, updatePreferences } = useNotifications();

  const prefItems: { key: string; label: string; i18nKey: TranslationKey; color: string; icon: any }[] = [
    { key: 'enableInfo', label: 'Info', i18nKey: 'notif.info', color: '#3B82F6', icon: Bell },
    { key: 'enableSuccess', label: 'Success', i18nKey: 'notif.success', color: '#10B981', icon: Sparkles },
    { key: 'enableWarning', label: 'Warnings', i18nKey: 'notif.warning', color: '#F59E0B', icon: Zap },
    { key: 'enableError', label: 'Errors', i18nKey: 'notif.error', color: '#EF4444', icon: Shield },
    { key: 'enableSystem', label: 'System', i18nKey: 'notif.system', color: '#6366F1', icon: Gauge },
    { key: 'enableCollaboration', label: 'Collaboration', i18nKey: 'notif.collab', color: '#8B5CF6', icon: Eye },
    { key: 'enableAdmin', label: 'Admin', i18nKey: 'notif.admin', color: '#EC4899', icon: Shield },
    { key: 'enableSound', label: 'Sound', i18nKey: 'notif.sound', color: '#0EA5E9', icon: Volume2 },
    { key: 'enableToast', label: 'Toast Pop-ups', i18nKey: 'notif.toast', color: '#14B8A6', icon: Bell },
  ];

  return (
    <>
      {prefItems.map((item, i) => (
        <SettingRow
          key={item.key}
          icon={item.icon}
          label={item.label}
          i18nKey={item.i18nKey}
          color={item.color}
          delay={0.02 * i}
        >
          <AnimatedToggle
            enabled={preferences?.[item.key as keyof typeof preferences] as boolean ?? true}
            onChange={() => {
              const current = preferences?.[item.key as keyof typeof preferences] as boolean ?? true;
              updatePreferences({ [item.key]: !current });
            }}
            color={item.color}
          />
        </SettingRow>
      ))}
    </>
  );
}

export default function SettingsPanel() {
  const { state, dispatch } = useNavigation();
  const { t, dir } = useLanguage();
  const [expandedSection, setExpandedSection] = useState<string | null>(null);
  const [showLanguages, setShowLanguages] = useState(false);

  const toggleSection = (id: string) => setExpandedSection(prev => prev === id ? null : id);

  return (
    <motion.div
      initial={{ opacity: 0, x: 300 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: 300 }}
      transition={{ type: "spring", damping: 28, stiffness: 300 }}
      className="fixed inset-0 z-40 overflow-y-auto"
      style={{ background: 'rgba(2,4,8,0.98)', scrollbarWidth: 'none' }}
    >
      {/* ── Header ── */}
      <motion.div
        initial={{ y: -10, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        className="flex items-center gap-3 px-4 pt-4 pb-2"
      >
        <motion.button
          whileHover={{ scale: 1.1 }}
          whileTap={{ scale: 0.85 }}
          onClick={() => dispatch({ type: 'SET_VIEW', view: 'map' })}
          className="w-10 h-10 rounded-xl flex items-center justify-center cursor-pointer"
          style={{ background: 'rgba(229,231,235,0.4)', border: '1px solid rgba(229,231,235,0.6)' }}
          aria-label="Close settings"
        >
          <ArrowLeft className="w-5 h-5 text-white/40" />
        </motion.button>
        <div style={{ direction: dir }}>
          <h1 className="text-lg font-bold text-white/85" style={{ fontFamily: 'Syne, sans-serif' }}>{t('common.settings')}</h1>
          <p className="text-[10px] text-white/20">Settings & Preferences</p>
        </div>
      </motion.div>

      <div className="px-4 space-y-4 pb-8 mt-2">
        {/* ── Voice & Sound ── */}
        <motion.section
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.05 }}
        >
          <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
            style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif', direction: dir }}>
            <Volume2 className="w-3 h-3" style={{ color: '#2563EB' }} />
            {t('settings.voiceGuidance')}
          </div>
          <div className="rounded-xl overflow-hidden" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}>
            <SettingRow icon={state.voiceEnabled ? Volume2 : VolumeX} label="Voice Guidance" i18nKey="settings.voiceGuidance" color="#2563EB" delay={0.08}>
              <AnimatedToggle enabled={state.voiceEnabled} onChange={() => dispatch({ type: 'SET_VOICE_ENABLED', enabled: !state.voiceEnabled })} />
            </SettingRow>
            <SettingRow icon={Bell} label="Alert Sounds" i18nKey="settings.alertSounds" color="#ffaa00" delay={0.1}>
              <AnimatedToggle enabled={true} onChange={() => {}} color="#ffaa00" />
            </SettingRow>
          </div>
        </motion.section>

        {/* ── Language ── */}
        <motion.section
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.12 }}
        >
          <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
            style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif', direction: dir }}>
            <Languages className="w-3 h-3" style={{ color: '#7C3AED' }} />
            {t('settings.language')}
          </div>
          <motion.button
            onClick={() => setShowLanguages(!showLanguages)}
            className="w-full flex items-center justify-between px-4 py-3.5 rounded-xl cursor-pointer transition-all"
            style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}
            whileTap={{ scale: 0.98 }}
          >
            <div className="flex items-center gap-3" style={{ direction: dir }}>
              <span className="text-lg">{languages.find(l => l.code === state.voiceLanguage)?.flag || '🌐'}</span>
              <span className="text-sm text-white/70">{languages.find(l => l.code === state.voiceLanguage)?.name || 'English'}</span>
            </div>
            <motion.div animate={{ rotate: showLanguages ? 90 : 0 }}>
              <ChevronRight className="w-4 h-4 text-white/20" />
            </motion.div>
          </motion.button>

          <AnimatePresence>
            {showLanguages && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                transition={{ type: 'spring', damping: 24 }}
                className="overflow-hidden"
              >
                <div className="mt-2 rounded-xl overflow-hidden max-h-[280px] overflow-y-auto"
                  style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)', scrollbarWidth: 'none' }}>
                  {languages.map(lang => (
                    <motion.button
                      key={lang.code}
                      onClick={() => { dispatch({ type: 'SET_VOICE_LANGUAGE', language: lang.code }); setShowLanguages(false); }}
                      className="w-full flex items-center gap-3 px-4 py-2.5 transition-all text-left cursor-pointer"
                      style={{
                        background: state.voiceLanguage === lang.code ? 'rgba(136,85,255,0.06)' : 'transparent',
                        borderBottom: '1px solid rgba(243,244,246,0.4)' }}
                      whileTap={{ scale: 0.98 }}
                    >
                      <span className="text-base">{lang.flag}</span>
                      <span className={`text-sm ${state.voiceLanguage === lang.code ? 'font-medium' : ''}`}
                        style={{ color: state.voiceLanguage === lang.code ? '#7C3AED' : 'rgba(156,163,175,0.9)' }}>
                        {lang.name}
                      </span>
                      {state.voiceLanguage === lang.code && (
                        <motion.div
                          layoutId="langIndicator"
                          className="ml-auto w-2 h-2 rounded-full"
                          style={{ background: '#7C3AED', boxShadow: '0 0 8px #7C3AED60' }}
                        />
                      )}
                    </motion.button>
                  ))}
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </motion.section>

        {/* ── Driving Profile ── */}
        <motion.section
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.18 }}
        >
          <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
            style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif', direction: dir }}>
            <Car className="w-3 h-3" style={{ color: '#00e88f' }} />
            {t('mode.drive')}
          </div>
          <div className="grid grid-cols-2 gap-2">
            {(Object.entries(drivingProfiles) as [DrivingProfile, typeof drivingProfiles.standard][]).map(([key, profile], i) => (
              <motion.button
                key={key}
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ delay: i * 0.05 + 0.2 }}
                whileHover={{ scale: 1.02, y: -2 }}
                whileTap={{ scale: 0.97 }}
                onClick={() => dispatch({ type: 'SET_DRIVING_PROFILE', profile: key })}
                className="flex flex-col items-center gap-2 px-3 py-4 rounded-xl transition-all duration-300 cursor-pointer"
                style={{
                  background: state.drivingProfile === key ? 'rgba(0,232,143,0.06)' : 'rgba(243,244,246,0.4)',
                  border: state.drivingProfile === key ? '1px solid rgba(0,232,143,0.25)' : '1px solid rgba(229,231,235,0.4)',
                  boxShadow: state.drivingProfile === key ? '0 0 16px rgba(0,232,143,0.08)' : 'none' }}
              >
                <span className="text-2xl">{profile.icon}</span>
                <span className="text-xs font-medium" style={{ color: state.drivingProfile === key ? '#00e88f' : 'rgba(107,114,128,0.9)' }}>
                  {profile.label}
                </span>
                <span className="text-[9px] text-center leading-tight" style={{ color: 'rgba(156,163,175,0.6)' }}>
                  {profile.description}
                </span>
              </motion.button>
            ))}
          </div>
        </motion.section>

        {/* ── Display ── */}
        <motion.section
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.24 }}
        >
          <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
            style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif', direction: dir }}>
            <Palette className="w-3 h-3" style={{ color: '#ff8800' }} />
            {t('settings.hudDisplay')}
          </div>
          <div className="rounded-xl overflow-hidden" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}>
            <SettingRow icon={Eye} label="HUD Display" i18nKey="settings.hudDisplay" color="#2563EB" delay={0.26}>
              <AnimatedToggle enabled={true} onChange={() => {}} />
            </SettingRow>
            <SettingRow icon={Gauge} label="Speed Display" i18nKey="settings.speedDisplay" color="#00e88f" delay={0.28}>
              <AnimatedToggle enabled={true} onChange={() => {}} color="#00e88f" />
            </SettingRow>
            <SettingRow icon={Sparkles} label="Animations" i18nKey="settings.animations" color="#7C3AED" delay={0.3}>
              <AnimatedToggle enabled={true} onChange={() => {}} color="#7C3AED" />
            </SettingRow>
          </div>
        </motion.section>

        {/* ── Privacy & Security ── */}
        <motion.section
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.32 }}
        >
          <div className="text-[10px] font-semibold tracking-wider uppercase mb-2 px-1 flex items-center gap-1.5"
            style={{ color: 'rgba(156,163,175,0.6)', fontFamily: 'Syne, sans-serif', direction: dir }}>
            <Shield className="w-3 h-3" style={{ color: '#ff4466' }} />
            {t('settings.locationSharing')}
          </div>
          <div className="rounded-xl overflow-hidden" style={{ background: 'rgba(243,244,246,0.4)', border: '1px solid rgba(229,231,235,0.4)' }}>
            <SettingRow icon={Wifi} label="Offline Maps" i18nKey="settings.offlineMaps" color="#2563EB" delay={0.34}>
              <AnimatedToggle enabled={false} onChange={() => {}} />
            </SettingRow>
            <SettingRow icon={Shield} label="Location Sharing" i18nKey="settings.locationSharing" color="#ff4466" delay={0.36}>
              <AnimatedToggle enabled={false} onChange={() => {}} color="#ff4466" />
            </SettingRow>
          </div>
        </motion.section>

        {/* ── Notifications ── */}
        <motion.section
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.36 }}
        >
          <h3 className="text-[10px] font-bold tracking-widest mb-2 px-1" style={{ color: '#6366F1', direction: dir }}>{t('settings.notifications')}</h3>
          <div className="rounded-xl overflow-hidden" style={{ background: 'rgba(99,102,241,0.03)', border: '1px solid rgba(99,102,241,0.08)' }}>
            <NotificationPreferencesSection />
          </div>
        </motion.section>

        {/* ── About ── */}
        <motion.section
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.38 }}
        >
          <div className="rounded-xl p-4 relative overflow-hidden"
            style={{ background: 'rgba(0,212,255,0.03)', border: '1px solid rgba(0,212,255,0.08)' }}>
            {/* Ambient glow */}
            <div className="absolute -top-12 -right-12 w-32 h-32 rounded-full pointer-events-none"
              style={{ background: 'radial-gradient(circle, rgba(0,212,255,0.06), transparent 70%)' }} />

            <div className="flex items-center gap-3 mb-3 relative" style={{ direction: dir }}>
              <div className="w-12 h-12 rounded-xl flex items-center justify-center relative"
                style={{ background: 'rgba(0,212,255,0.08)', border: '1px solid rgba(0,212,255,0.15)' }}>
                <Globe className="w-6 h-6" style={{ color: '#2563EB' }} />
                <motion.div
                  className="absolute inset-0 rounded-xl"
                  style={{ border: '1px solid rgba(0,212,255,0.2)' }}
                  animate={{ opacity: [0.3, 0.8, 0.3] }}
                  transition={{ duration: 2, repeat: Infinity }}
                />
              </div>
              <div>
                <div className="text-base font-bold" style={{ fontFamily: 'Syne, sans-serif', color: '#2563EB' }}>G.A.N.E</div>
                <div className="text-[10px] text-white/25">v3.0.0 — Global Autonomous Navigation Ecosystem</div>
              </div>
            </div>
            <p className="text-xs text-white/20 leading-relaxed relative" style={{ direction: dir }}>
              G.A.N.E — Global Autonomous Navigation Ecosystem. Multi-layered navigation platform with GNSS fusion, real-time telemetry, and multi-channel integrations.
              Includes probabilistic routing, real-time traffic intelligence, multi-modal transport, and 43+ language support.
            </p>
            <div className="flex items-center gap-4 mt-3 relative" style={{ direction: dir }}>
              <div className="text-[9px] text-white/15">2,246 Crates</div>
              <div className="text-[9px] text-white/15">24 Panels</div>
              <div className="text-[9px] text-white/15">43+ Languages</div>
            </div>
          </div>
        </motion.section>
      </div>
    </motion.div>
  );
}
