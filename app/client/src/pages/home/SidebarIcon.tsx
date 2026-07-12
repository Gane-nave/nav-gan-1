/**
 * G.A.N.E — Sidebar Icon Component
 * 
 * Individual sidebar navigation icon with tooltip,
 * active state indicator, and hover animations.
 * i18n: Uses useLanguage() for localized tooltip text.
 */
import { motion } from "framer-motion";
import { Hexagon } from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";
import type { SmartPanel } from "@/lib/navStore";
import type { TranslationKey } from "@/lib/i18n";

export type SidebarIconItem = {
  id: SmartPanel;
  icon: typeof Hexagon;
  label: string;
  labelHe: string;
  i18nKey: TranslationKey;
  accent: string;
};

interface SidebarIconProps {
  item: SidebarIconItem;
  isActive: boolean;
  modeColor: string;
  onClick: () => void;
  delay: number;
  compact?: boolean;
}

export default function SidebarIcon({ item, isActive, onClick, delay, compact }: SidebarIconProps) {
  const Icon = item.icon;
  const color = isActive ? item.accent : '#6B7280';
  const { t } = useLanguage();

  return (
    <motion.div
      className="relative group w-full flex justify-center"
      initial={{ opacity: 0, x: -20 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay, type: 'spring', damping: 20 }}
    >
      <motion.button
        whileHover={{ scale: 1.12 }}
        whileTap={{ scale: 0.88 }}
        onClick={onClick}
        className="relative flex items-center justify-center cursor-pointer"
        style={{
          width: compact ? '40px' : '52px',
          height: compact ? '40px' : '52px',
          borderRadius: compact ? '10px' : '14px',
          background: isActive ? `${item.accent}12` : '#FFFFFF',
          border: isActive ? `1px solid ${item.accent}30` : '1px solid #E5E7EB',
          boxShadow: isActive ? `0 2px 8px ${item.accent}15` : '0 1px 3px rgba(0,0,0,0.06)',
          transition: 'all 0.25s cubic-bezier(0.16, 1, 0.3, 1)',
        }}
        aria-label={item.label}
      >
        {/* Active indicator bar */}
        {isActive && (
          <motion.div
            className="absolute left-0 top-1/2 -translate-y-1/2 w-[3px] rounded-r-full"
            style={{ height: '24px', background: item.accent, boxShadow: `0 0 6px ${item.accent}40` }}
            layoutId="sidebar-indicator"
            transition={{ type: 'spring', damping: 25, stiffness: 300 }}
          />
        )}

        <Icon
          className="relative z-10 transition-all duration-200"
          style={{
            width: '22px',
            height: '22px',
            color,
            filter: isActive ? `drop-shadow(0 0 4px ${item.accent}40)` : 'none',
          }}
        />
      </motion.button>

      {/* Tooltip — appears on hover */}
      <div
        className="absolute left-full ml-3 top-1/2 -translate-y-1/2 pointer-events-none opacity-0 group-hover:opacity-100 transition-all duration-200 z-[100]"
        style={{ transform: 'translateY(-50%) translateX(0px)' }}
      >
        <div className="relative px-3 py-2 rounded-lg whitespace-nowrap"
          style={{
            background: '#FFFFFF',
            border: '1px solid #E5E7EB',
            boxShadow: '0 4px 12px rgba(0,0,0,0.08)',
          }}>
          <div className="text-xs font-bold" style={{ color: item.accent }}>{item.label}</div>
          <div className="text-[10px] mt-0.5" style={{ color: '#9CA3AF' }}>{t(item.i18nKey)}</div>
          {/* Arrow */}
          <div className="absolute left-0 top-1/2 -translate-x-1 -translate-y-1/2 w-2 h-2 rotate-45"
            style={{ background: '#FFFFFF', borderLeft: '1px solid #E5E7EB', borderBottom: '1px solid #E5E7EB' }} />
        </div>
      </div>
    </motion.div>
  );
}
