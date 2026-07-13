/**
 * G.A.N.E — Notification Bell
 * =============================
 * Top-right notification bell icon with:
 * - Animated unread count badge
 * - Dropdown panel with notification list
 * - Mark read/unread, delete, clear all actions
 * - Real-time updates via useNotifications hook
 */
import { useState, useRef, useEffect, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Bell, Check, CheckCheck, Trash2, X, Settings,
  Info, CheckCircle, AlertTriangle, AlertCircle,
  Cog, Users, Shield, ExternalLink
} from "lucide-react";
import { useLocation } from "wouter";
import { useNotifications, type NotificationItem } from "@/hooks/useNotifications";
import { useLanguage } from "@/contexts/LanguageContext";

// ─── Notification Type Config ───
const TYPE_CONFIG: Record<string, { icon: typeof Info; color: string; bgColor: string; label: string; i18nKey?: string }> = {
  info: { icon: Info, color: "#3B82F6", bgColor: "#EFF6FF", label: "Info", i18nKey: 'notif.info' },
  success: { icon: CheckCircle, color: "#10B981", bgColor: "#ECFDF5", label: "Success", i18nKey: 'notif.success' },
  warning: { icon: AlertTriangle, color: "#F59E0B", bgColor: "#FFFBEB", label: "Warning", i18nKey: 'notif.warning' },
  error: { icon: AlertCircle, color: "#EF4444", bgColor: "#FEF2F2", label: "Error", i18nKey: 'notif.error' },
  system: { icon: Cog, color: "#6366F1", bgColor: "#EEF2FF", label: "System", i18nKey: 'group.sys' },
  collaboration: { icon: Users, color: "#8B5CF6", bgColor: "#F5F3FF", label: "Collaboration", i18nKey: 'sidebar.liveShare' },
  admin: { icon: Shield, color: "#EC4899", bgColor: "#FDF2F8", label: "Admin", i18nKey: 'dock.admin' },
};

// ─── Time Ago Helper ───
function timeAgo(date: string | Date): string {
  const now = Date.now();
  const then = new Date(date).getTime();
  const diffMs = now - then;
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHr = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHr / 24);

  if (diffSec < 60) return "עכשיו";
  if (diffMin < 60) return `לפני ${diffMin} דק'`;
  if (diffHr < 24) return `לפני ${diffHr} שע'`;
  if (diffDay < 7) return `לפני ${diffDay} ימים`;
  return new Date(date).toLocaleDateString("he-IL", { day: "numeric", month: "short" });
}

// ─── Single Notification Item ───
function NotificationItemRow({
  notification,
  onMarkRead,
  onDelete,
}: {
  notification: NotificationItem;
  onMarkRead: (id: string) => void;
  onDelete: (id: string) => void;
}) {
  const config = TYPE_CONFIG[notification.type] || TYPE_CONFIG.info;
  const Icon = config.icon;

  return (
    <motion.div
      initial={{ opacity: 0, y: -8 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, x: 50 }}
      transition={{ duration: 0.2 }}
      className={`group relative px-4 py-3 border-b border-gray-100 transition-colors duration-200 cursor-pointer ${
        notification.isRead ? "bg-white" : "bg-blue-50/40"
      } hover:bg-gray-50`}
      onClick={() => {
        if (!notification.isRead) {
          onMarkRead(notification.notificationId);
        }
      }}
    >
      <div className="flex items-start gap-3">
        {/* Type Icon */}
        <div
          className="flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center mt-0.5"
          style={{ background: config.bgColor }}
        >
          <Icon className="w-4 h-4" style={{ color: config.color }} />
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0" style={{ direction: "rtl" }}>
          <div className="flex items-center gap-2">
            <h4
              className={`text-sm truncate ${
                notification.isRead ? "font-normal text-gray-600" : "font-semibold text-gray-900"
              }`}
            >
              {notification.title}
            </h4>
            {!notification.isRead && (
              <div className="flex-shrink-0 w-2 h-2 rounded-full bg-blue-500" />
            )}
          </div>
          <p className="text-xs text-gray-500 mt-0.5 line-clamp-2">{notification.message}</p>
          <div className="flex items-center gap-2 mt-1.5">
            <span
              className="text-[10px] font-medium px-1.5 py-0.5 rounded-full"
              style={{ color: config.color, background: config.bgColor }}
            >
              {config.i18nKey}
            </span>
            <span className="text-[10px] text-gray-400">{timeAgo(notification.createdAt)}</span>
          </div>
        </div>

        {/* Actions (visible on hover) */}
        <div className="flex-shrink-0 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
          {!notification.isRead && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onMarkRead(notification.notificationId);
              }}
              className="p-1 rounded hover:bg-gray-200 transition-colors"
              title="סמן כנקרא"
            >
              <Check className="w-3.5 h-3.5 text-gray-400" />
            </button>
          )}
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDelete(notification.notificationId);
            }}
            className="p-1 rounded hover:bg-red-100 transition-colors"
            title="מחק"
          >
            <Trash2 className="w-3.5 h-3.5 text-gray-400 hover:text-red-500" />
          </button>
        </div>
      </div>
    </motion.div>
  );
}

// ─── Main NotificationBell Component ───
export default function NotificationBell() {
  const { t, dir } = useLanguage();
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);
  const [, setLocation] = useLocation();
  const {
    notifications,
    unreadCount,
    isLoading,
    markRead,
    markAllRead,
    deleteNotification,
    clearAll,
    refresh,
  } = useNotifications();

  // Close dropdown on outside click
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    }
    if (isOpen) {
      document.addEventListener("mousedown", handleClickOutside);
    }
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [isOpen]);

  // Close on Escape
  useEffect(() => {
    function handleEscape(e: KeyboardEvent) {
      if (e.key === "Escape") setIsOpen(false);
    }
    if (isOpen) {
      document.addEventListener("keydown", handleEscape);
    }
    return () => document.removeEventListener("keydown", handleEscape);
  }, [isOpen]);

  const toggleDropdown = useCallback(() => {
    setIsOpen((prev) => !prev);
    if (!isOpen) {
      refresh();
    }
  }, [isOpen, refresh]);

  return (
    <div ref={dropdownRef} className="relative">
      {/* Bell Button */}
      <motion.button
        whileHover={{ scale: 1.08 }}
        whileTap={{ scale: 0.92 }}
        onClick={toggleDropdown}
        className="relative w-11 h-11 rounded-2xl flex items-center justify-center transition-all duration-300 cursor-pointer"
        style={{
          background: isOpen ? "#F0F4FF" : "#FFFFFF",
          border: isOpen ? "1px solid #C7D2FE" : "1px solid #E5E7EB",
          boxShadow: isOpen ? "0 0 12px rgba(99,102,241,0.15)" : "0 1px 3px rgba(0,0,0,0.06)",
        }}
        aria-label={`Notifications${unreadCount > 0 ? ` (${unreadCount} unread)` : ""}`}
        aria-expanded={isOpen}
      >
        <Bell className={`w-5 h-5 transition-colors ${isOpen ? "text-indigo-600" : "text-gray-500"}`} />

        {/* Unread Badge */}
        <AnimatePresence>
          {unreadCount > 0 && (
            <motion.div
              initial={{ scale: 0 }}
              animate={{ scale: 1 }}
              exit={{ scale: 0 }}
              className="absolute -top-1 -right-1 min-w-[18px] h-[18px] rounded-full bg-red-500 text-white text-[10px] font-bold flex items-center justify-center px-1 shadow-sm"
            >
              {unreadCount > 99 ? "99+" : unreadCount}
            </motion.div>
          )}
        </AnimatePresence>
      </motion.button>

      {/* Dropdown Panel */}
      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, y: -8, scale: 0.95 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -8, scale: 0.95 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
            className="absolute top-full mt-2 w-[380px] max-h-[520px] rounded-2xl overflow-hidden z-50"
            style={{
              right: 0,
              background: "#FFFFFF",
              border: "1px solid #E5E7EB",
              boxShadow: "0 8px 30px rgba(0,0,0,0.12), 0 2px 8px rgba(0,0,0,0.06)",
            }}
          >
            {/* Header */}
            <div className="px-4 py-3 border-b border-gray-100 flex items-center justify-between">
              <div className="flex items-center gap-2" style={{ direction: "rtl" }}>
                <h3 className="text-sm font-bold text-gray-900">{t('sidebar.alerts')}</h3>
                {unreadCount > 0 && (
                  <span className="text-[10px] font-bold text-white bg-red-500 rounded-full px-1.5 py-0.5 min-w-[20px] text-center">
                    {unreadCount}
                  </span>
                )}
              </div>
              <div className="flex items-center gap-1">
                {unreadCount > 0 && (
                  <button
                    onClick={markAllRead}
                    className="p-1.5 rounded-lg hover:bg-gray-100 transition-colors text-gray-400 hover:text-indigo-600"
                    title="סמן הכל כנקרא"
                  >
                    <CheckCheck className="w-4 h-4" />
                  </button>
                )}
                {notifications.length > 0 && (
                  <button
                    onClick={clearAll}
                    className="p-1.5 rounded-lg hover:bg-red-50 transition-colors text-gray-400 hover:text-red-500"
                    title="נקה הכל"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                )}
                <button
                  onClick={() => setIsOpen(false)}
                  className="p-1.5 rounded-lg hover:bg-gray-100 transition-colors text-gray-400"
                >
                  <X className="w-4 h-4" />
                </button>
              </div>
            </div>

            {/* Notification List */}
            <div className="overflow-y-auto max-h-[420px] overscroll-contain">
              {isLoading ? (
                <div className="py-12 flex flex-col items-center gap-3">
                  <div className="w-8 h-8 rounded-full border-2 border-indigo-200 border-t-indigo-600 animate-spin" />
                  <p className="text-xs text-gray-400">טוען התראות...</p>
                </div>
              ) : notifications.length === 0 ? (
                <div className="py-12 flex flex-col items-center gap-3">
                  <div className="w-14 h-14 rounded-2xl bg-gray-50 flex items-center justify-center">
                    <Bell className="w-7 h-7 text-gray-300" />
                  </div>
                  <div className="text-center" style={{ direction: "rtl" }}>
                    <p className="text-sm font-medium text-gray-500">אין התראות</p>
                    <p className="text-xs text-gray-400 mt-1">כשיהיו התראות חדשות, הן יופיעו כאן</p>
                  </div>
                </div>
              ) : (
                <AnimatePresence initial={false}>
                  {notifications.map((notif: NotificationItem) => (
                    <NotificationItemRow
                      key={notif.notificationId}
                      notification={notif}
                      onMarkRead={markRead}
                      onDelete={deleteNotification}
                    />
                  ))}
                </AnimatePresence>
              )}
            </div>

            {/* Footer — View All */}
            <div className="px-4 py-2.5 border-t border-gray-100 flex items-center justify-center">
              <button
                onClick={() => {
                  setIsOpen(false);
                  setLocation("/notifications");
                }}
                className="text-xs font-medium text-indigo-600 hover:text-indigo-700 flex items-center gap-1 transition-colors"
              >
                View All Notifications
                <ExternalLink className="w-3 h-3" />
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
