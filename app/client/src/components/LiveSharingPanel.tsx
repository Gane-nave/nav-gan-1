/**
 * G.A.N.E — Live Location Sharing Panel
 * =======================================
 * Create, manage, and revoke live location sharing links.
 * Connected to tRPC liveSharing router.
 */
import { useState, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  X, Share2, Link2, Copy, Check, Trash2, Clock, Shield,
  Eye, MapPin, Users, Zap, ChevronRight, ExternalLink
} from "lucide-react";
import { trpc } from "@/lib/trpc";
import { useAuth } from "@/_core/hooks/useAuth";
import { useLanguage } from "@/contexts/LanguageContext";

const ACCENT = 'oklch(0.55 0.22 264)';
const ACCENT_GREEN = 'oklch(0.75 0.18 150)';
const ACCENT_CYAN = 'oklch(0.82 0.15 192)';
const ACCENT_AMBER = 'oklch(0.80 0.16 75)';

function PanelWrapper({ title, titleHe, icon: Icon, onClose, accentColor, children }: {
  title: string; titleHe?: string; icon: any; onClose: () => void; accentColor: string; children: React.ReactNode;
}) {
  const { t, dir } = useLanguage();
  return (
    <motion.div
      initial={{ x: -380, opacity: 0 }}
      animate={{ x: 0, opacity: 1 }}
      exit={{ x: -380, opacity: 0 }}
      transition={{ type: "spring", damping: 28, stiffness: 300 }}
      className="expand-panel"
    >
      <div className="sticky top-0 z-10 px-5 pt-5 pb-3 relative overflow-hidden" style={{
        background: 'linear-gradient(180deg, oklch(0.07 0.015 264 / 99%) 0%, oklch(0.07 0.015 264 / 92%) 80%, transparent 100%)' }}>
        <motion.div className="absolute inset-0 pointer-events-none"
          style={{ background: `radial-gradient(ellipse at 20% 50%, color-mix(in oklch, ${accentColor}, transparent 92%), transparent 70%)` }}
          animate={{ opacity: [0.3, 0.6, 0.3] }}
          transition={{ duration: 4, repeat: Infinity, ease: 'easeInOut' }} />
        <motion.div className="absolute left-0 right-0 h-px pointer-events-none"
          style={{ background: `linear-gradient(90deg, transparent, ${accentColor}, transparent)` }}
          animate={{ top: ['0%', '100%'] }}
          transition={{ duration: 3, repeat: Infinity, ease: 'linear' }} />
        <div className="flex items-center gap-3 relative z-10">
          <motion.div
            initial={{ scale: 0.5, opacity: 0, rotate: -20 }}
            animate={{ scale: 1, opacity: 1, rotate: 0 }}
            transition={{ delay: 0.1, type: 'spring', stiffness: 350, damping: 15 }}
            className="w-10 h-10 rounded-xl flex items-center justify-center relative"
            style={{
              background: `color-mix(in oklch, ${accentColor}, transparent 82%)`,
              border: `1px solid color-mix(in oklch, ${accentColor}, transparent 60%)`,
              boxShadow: `0 0 20px color-mix(in oklch, ${accentColor}, transparent 75%)` }}>
            <Icon className="w-5 h-5" style={{ color: accentColor }} />
            <motion.div className="absolute inset-[-3px] rounded-[14px] pointer-events-none"
              style={{ border: `1px solid ${accentColor}` }}
              animate={{ scale: [1, 1.12, 1], opacity: [0.3, 0, 0.3] }}
              transition={{ duration: 2, repeat: Infinity }} />
          </motion.div>
          <div className="flex-1">
            <motion.h2 initial={{ opacity: 0, x: -12 }} animate={{ opacity: 1, x: 0 }}
              className="text-base font-bold text-white/90" style={{ fontFamily: 'Syne, sans-serif' }}>{title}</motion.h2>
            {titleHe && <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}
              className="text-[10px] text-white/25 mt-0.5" style={{ direction: dir }}>{titleHe}</motion.div>}
          </div>
          <motion.button onClick={onClose} whileHover={{ scale: 1.15, rotate: 90 }} whileTap={{ scale: 0.85 }}
            className="w-8 h-8 rounded-lg flex items-center justify-center cursor-pointer"
            style={{ color: 'oklch(0.40 0.01 264)' }}>
            <X className="w-4 h-4" />
          </motion.button>
        </div>
        <motion.div className="mt-3 h-px w-full relative overflow-hidden" style={{ background: 'oklch(1 0 0 / 3%)' }}>
          <motion.div className="absolute inset-0"
            style={{ background: `linear-gradient(90deg, transparent, ${accentColor}, transparent)`, backgroundSize: '200% 100%' }}
            animate={{ backgroundPosition: ['200% 0', '-200% 0'] }}
            transition={{ duration: 4, repeat: Infinity, ease: 'linear' }} />
        </motion.div>
      </div>
      <div className="absolute inset-0 pointer-events-none quantum-grid opacity-40" />
      <motion.div className="px-5 pb-6 futuristic-scroll relative z-[1]"
        initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }}>
        {children}
      </motion.div>
    </motion.div>
  );
}

const PERMISSIONS = [
  { id: 'view' as const, label: 'View Only', i18nKey: 'share.viewOnly', icon: Eye, desc: 'See location on map' },
  { id: 'view_eta' as const, label: 'View + ETA', i18nKey: 'share.viewEta', icon: Clock, desc: 'See location + arrival time' },
  { id: 'full' as const, label: 'Full Access', i18nKey: 'share.fullAccess', icon: Shield, desc: 'Location, ETA, speed, heading' },
];

const DURATIONS = [
  { minutes: 15, label: '15 min' },
  { minutes: 30, label: '30 min' },
  { minutes: 60, label: '1 hour' },
  { minutes: 120, label: '2 hours' },
  { minutes: 480, label: '8 hours' },
  { minutes: 0, label: 'No expiry' },
];

export default function LiveSharingPanel({ onClose }: { onClose: () => void }) {
  const { t, dir } = useLanguage();
  const { isAuthenticated } = useAuth();
  const [tab, setTab] = useState<'create' | 'active'>('active');
  const [permission, setPermission] = useState<'view' | 'view_eta' | 'full'>('view');
  const [duration, setDuration] = useState(60);
  const [label, setLabel] = useState('');
  const [creating, setCreating] = useState(false);
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [deviceId] = useState(() => `web-${Math.random().toString(36).slice(2, 10)}`);

  const mySharesQuery = trpc.liveSharing.myShares.useQuery(
    undefined,
    { refetchInterval: 15_000, enabled: isAuthenticated }
  );

  const createMutation = trpc.liveSharing.create.useMutation({
    onSuccess: () => {
      setTab('active');
      setLabel('');
      mySharesQuery.refetch();
    },
    onSettled: () => setCreating(false),
  });

  const revokeMutation = trpc.liveSharing.revoke.useMutation({
    onSuccess: () => mySharesQuery.refetch(),
  });

  const handleCreate = useCallback(() => {
    setCreating(true);
    createMutation.mutate({
      deviceId,
      permission,
      durationMinutes: duration,
      label: label || undefined,
    });
  }, [deviceId, permission, duration, label, createMutation]);

  const handleRevoke = useCallback((shareId: string) => {
    revokeMutation.mutate({ shareId });
  }, [revokeMutation]);

  const handleCopy = useCallback((shareId: string, token: string) => {
    const url = `${window.location.origin}/track/${token}`;
    navigator.clipboard.writeText(url).then(() => {
      setCopiedId(shareId);
      setTimeout(() => setCopiedId(null), 2000);
    });
  }, []);

  const shares = mySharesQuery.data ?? [];

  if (!isAuthenticated) {
    return (
      <PanelWrapper title="Live Sharing" titleHe="שיתוף מיקום חי" icon={Share2} onClose={onClose} accentColor={ACCENT}>
        <div className="feature-card flex flex-col items-center py-8 gap-3">
          <Shield className="w-10 h-10" style={{ color: ACCENT }} />
          <div className="text-xs text-white/50 text-center">Sign in to share your live location</div>
          <div className="text-[10px] text-white/25 text-center" style={{ direction: dir }}>התחבר כדי לשתף מיקום חי</div>
        </div>
      </PanelWrapper>
    );
  }

  return (
    <PanelWrapper title="Live Sharing" titleHe="שיתוף מיקום חי" icon={Share2} onClose={onClose} accentColor={ACCENT}>
      {/* Tab selector */}
      <div className="flex gap-1.5 mb-4 mt-2">
        {[
          { id: 'active' as const, label: 'Active Links', icon: Link2 },
          { id: 'create' as const, label: 'New Share', icon: Share2 },
        ].map(t => (
          <motion.button
            key={t.id}
            onClick={() => setTab(t.id)}
            whileHover={{ scale: 1.03 }}
            whileTap={{ scale: 0.97 }}
            className="flex-1 flex items-center justify-center gap-1.5 py-2.5 rounded-xl text-xs font-semibold transition-all cursor-pointer"
            style={{
              background: tab === t.id ? `color-mix(in oklch, ${ACCENT}, transparent 80%)` : 'oklch(1 0 0 / 3%)',
              border: tab === t.id ? `1px solid color-mix(in oklch, ${ACCENT}, transparent 50%)` : '1px solid oklch(1 0 0 / 6%)',
              color: tab === t.id ? ACCENT : 'oklch(1 0 0 / 40%)',
            }}
          >
            <t.icon className="w-3.5 h-3.5" />
            {t.label}
          </motion.button>
        ))}
      </div>

      <AnimatePresence mode="wait">
        {/* ═══ CREATE TAB ═══ */}
        {tab === 'create' && (
          <motion.div key="create" initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -8 }}>
            {/* Permission selector */}
            <div className="text-xs text-white/30 uppercase tracking-wider mb-3">Permission Level</div>
            <div className="space-y-2 mb-4">
              {PERMISSIONS.map(p => {
                const isSelected = permission === p.id;
                return (
                  <motion.button
                    key={p.id}
                    onClick={() => setPermission(p.id)}
                    whileHover={{ scale: 1.02 }}
                    whileTap={{ scale: 0.98 }}
                    className="w-full feature-card flex items-center gap-3 py-3 cursor-pointer transition-all"
                    style={{
                      borderColor: isSelected ? `color-mix(in oklch, ${ACCENT}, transparent 40%)` : undefined,
                      background: isSelected ? `color-mix(in oklch, ${ACCENT}, transparent 92%)` : undefined,
                    }}
                  >
                    <div className="w-8 h-8 rounded-lg flex items-center justify-center"
                      style={{
                        background: isSelected ? `color-mix(in oklch, ${ACCENT}, transparent 80%)` : 'oklch(1 0 0 / 5%)',
                      }}>
                      <p.icon className="w-4 h-4" style={{ color: isSelected ? ACCENT : 'oklch(1 0 0 / 30%)' }} />
                    </div>
                    <div className="flex-1 text-left">
                      <div className="text-xs font-semibold" style={{ color: isSelected ? 'oklch(1 0 0 / 80%)' : 'oklch(1 0 0 / 50%)' }}>{p.label}</div>
                      <div className="text-[10px]" style={{ color: 'oklch(1 0 0 / 25%)' }}>{p.desc}</div>
                    </div>
                    {isSelected && <Check className="w-4 h-4" style={{ color: ACCENT }} />}
                  </motion.button>
                );
              })}
            </div>

            {/* Duration selector */}
            <div className="text-xs text-white/30 uppercase tracking-wider mb-3">Duration</div>
            <div className="grid grid-cols-3 gap-2 mb-4">
              {DURATIONS.map(d => (
                <motion.button
                  key={d.minutes}
                  onClick={() => setDuration(d.minutes)}
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.97 }}
                  className="py-2.5 rounded-xl text-xs font-semibold cursor-pointer transition-all"
                  style={{
                    background: duration === d.minutes ? `color-mix(in oklch, ${ACCENT}, transparent 80%)` : 'oklch(1 0 0 / 3%)',
                    border: duration === d.minutes ? `1px solid color-mix(in oklch, ${ACCENT}, transparent 50%)` : '1px solid oklch(1 0 0 / 6%)',
                    color: duration === d.minutes ? ACCENT : 'oklch(1 0 0 / 35%)',
                  }}
                >
                  {d.label}
                </motion.button>
              ))}
            </div>

            {/* Label input */}
            <div className="feature-card mb-4">
              <div className="text-xs text-white/30 uppercase tracking-wider mb-2">Label (optional)</div>
              <input
                value={label}
                onChange={(e) => setLabel(e.target.value)}
                placeholder="e.g., Family trip, Meeting..."
                className="w-full bg-transparent text-xs text-white/70 placeholder:text-white/15 outline-none"
                maxLength={128}
              />
            </div>

            {/* Create button */}
            <motion.button
              onClick={handleCreate}
              disabled={creating}
              whileHover={{ scale: 1.02 }}
              whileTap={{ scale: 0.98 }}
              className="w-full py-3.5 rounded-xl text-sm font-bold flex items-center justify-center gap-2 cursor-pointer transition-all disabled:opacity-30"
              style={{
                background: `linear-gradient(135deg, ${ACCENT}, oklch(0.45 0.22 264))`,
                color: 'white',
                boxShadow: `0 4px 20px color-mix(in oklch, ${ACCENT}, transparent 60%)`,
              }}
            >
              {creating ? (
                <motion.div animate={{ rotate: 360 }} transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}>
                  <Zap className="w-4 h-4" />
                </motion.div>
              ) : (
                <Share2 className="w-4 h-4" />
              )}
              {creating ? 'Creating...' : 'Create Share Link'}
            </motion.button>
          </motion.div>
        )}

        {/* ═══ ACTIVE LINKS TAB ═══ */}
        {tab === 'active' && (
          <motion.div key="active" initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -8 }}>
            <div className="flex items-center justify-between mb-3">
              <div className="text-xs text-white/30 uppercase tracking-wider">
                Active Shares ({shares.length})
              </div>
            </div>

            {shares.length === 0 && (
              <div className="feature-card flex flex-col items-center py-8 gap-2">
                <Link2 className="w-8 h-8" style={{ color: 'oklch(1 0 0 / 15%)' }} />
                <div className="text-xs text-white/30">No active sharing links</div>
                <div className="text-[10px] text-white/15" style={{ direction: dir }}>אין קישורי שיתוף פעילים</div>
                <motion.button
                  onClick={() => setTab('create')}
                  whileHover={{ scale: 1.03 }}
                  whileTap={{ scale: 0.97 }}
                  className="mt-2 px-4 py-2 rounded-lg text-xs font-semibold cursor-pointer"
                  style={{
                    background: `color-mix(in oklch, ${ACCENT}, transparent 85%)`,
                    border: `1px solid color-mix(in oklch, ${ACCENT}, transparent 60%)`,
                    color: ACCENT,
                  }}
                >
                  Create First Link
                </motion.button>
              </div>
            )}

            <div className="space-y-2">
              {shares.map((share: any, i: number) => {
                const permInfo = PERMISSIONS.find(p => p.id === share.permission) ?? PERMISSIONS[0];
                const PermIcon = permInfo.icon;
                const expiresAt = share.expiresAt;
                const expiresIn = expiresAt > 0 ? Math.max(0, Math.floor((expiresAt - Date.now()) / 60000)) : -1;
                const isCopied = copiedId === share.id;

                return (
                  <motion.div
                    key={share.id}
                    initial={{ opacity: 0, x: -20 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: i * 0.05 }}
                    className="feature-card"
                  >
                    <div className="flex items-start gap-3">
                      <div className="w-9 h-9 rounded-lg flex items-center justify-center flex-shrink-0"
                        style={{
                          background: `color-mix(in oklch, ${ACCENT}, transparent 85%)`,
                          border: `1px solid color-mix(in oklch, ${ACCENT}, transparent 65%)`,
                        }}>
                        <PermIcon className="w-4.5 h-4.5" style={{ color: ACCENT }} />
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="text-xs font-semibold text-white/80">
                          {share.label || permInfo.label}
                        </div>
                        <div className="flex items-center gap-2 mt-1">
                          <span className="text-[10px] text-white/20 flex items-center gap-1">
                            <Clock className="w-3 h-3" />
                            {expiresIn < 0 ? 'No expiry' : `${expiresIn}m left`}
                          </span>
                          <span className="text-[10px] text-white/15 flex items-center gap-1">
                            <Shield className="w-3 h-3" /> {permInfo.label}
                          </span>
                        </div>
                      </div>
                    </div>

                    {/* Action buttons */}
                    <div className="flex gap-2 mt-3">
                      <motion.button
                        onClick={() => handleCopy(share.id, share.token)}
                        whileHover={{ scale: 1.03 }}
                        whileTap={{ scale: 0.97 }}
                        className="flex-1 py-2 rounded-lg text-[11px] font-semibold flex items-center justify-center gap-1.5 cursor-pointer transition-all"
                        style={{
                          background: isCopied
                            ? `color-mix(in oklch, ${ACCENT_GREEN}, transparent 85%)`
                            : `color-mix(in oklch, ${ACCENT_CYAN}, transparent 88%)`,
                          border: isCopied
                            ? `1px solid color-mix(in oklch, ${ACCENT_GREEN}, transparent 60%)`
                            : `1px solid color-mix(in oklch, ${ACCENT_CYAN}, transparent 65%)`,
                          color: isCopied ? ACCENT_GREEN : ACCENT_CYAN,
                        }}
                      >
                        {isCopied ? <Check className="w-3.5 h-3.5" /> : <Copy className="w-3.5 h-3.5" />}
                        {isCopied ? 'Copied!' : 'Copy Link'}
                      </motion.button>
                      <motion.button
                        onClick={() => handleRevoke(share.id)}
                        whileHover={{ scale: 1.03 }}
                        whileTap={{ scale: 0.97 }}
                        className="py-2 px-3 rounded-lg text-[11px] font-semibold flex items-center justify-center gap-1.5 cursor-pointer transition-all"
                        style={{
                          background: 'oklch(1 0 0 / 3%)',
                          border: '1px solid oklch(1 0 0 / 8%)',
                          color: 'oklch(0.65 0.22 25)',
                        }}
                      >
                        <Trash2 className="w-3.5 h-3.5" />
                      </motion.button>
                    </div>
                  </motion.div>
                );
              })}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </PanelWrapper>
  );
}
