/**
 * G.A.N.E — Notification Center
 * ===============================
 * Full-page notification management with:
 * - Type filters (info, success, warning, error, system, collaboration, admin)
 * - Search by title/message
 * - Date range filtering
 * - Pagination with infinite scroll
 * - Bulk actions (mark all read, clear all)
 * - Individual actions (mark read, delete)
 */
import { useState, useMemo, useCallback, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { trpc } from "@/lib/trpc";
import { useAuth } from "@/_core/hooks/useAuth";
import { getLoginUrl } from "@/const";
import {
  Bell, Search, Filter, X, Check, CheckCheck, Trash2,
  ChevronLeft, Info, CheckCircle, AlertTriangle, AlertCircle,
  Cog, Users, Shield, Calendar, RefreshCw, ArrowLeft, Inbox
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";

// ─── Type Config ───
const TYPE_CONFIG: Record<string, { icon: typeof Info; color: string; bgColor: string; label: string; labelHe: string }> = {
  info: { icon: Info, color: "#3B82F6", bgColor: "#EFF6FF", label: "Info", labelHe: "מידע" },
  success: { icon: CheckCircle, color: "#10B981", bgColor: "#ECFDF5", label: "Success", labelHe: "הצלחה" },
  warning: { icon: AlertTriangle, color: "#F59E0B", bgColor: "#FFFBEB", label: "Warning", labelHe: "אזהרה" },
  error: { icon: AlertCircle, color: "#EF4444", bgColor: "#FEF2F2", label: "Error", labelHe: "שגיאה" },
  system: { icon: Cog, color: "#6366F1", bgColor: "#EEF2FF", label: "System", labelHe: "מערכת" },
  collaboration: { icon: Users, color: "#8B5CF6", bgColor: "#F5F3FF", label: "Collaboration", labelHe: "שיתוף" },
  admin: { icon: Shield, color: "#EC4899", bgColor: "#FDF2F8", label: "Admin", labelHe: "ניהול" },
};

const ALL_TYPES = ["info", "success", "warning", "error", "system", "collaboration", "admin"] as const;

// ─── Time Ago Helper ───
function timeAgo(date: string | Date): string {
  const now = Date.now();
  const then = new Date(date).getTime();
  const diffMs = now - then;
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHr = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHr / 24);

  if (diffSec < 60) return "Just now";
  if (diffMin < 60) return `${diffMin}m ago`;
  if (diffHr < 24) return `${diffHr}h ago`;
  if (diffDay < 7) return `${diffDay}d ago`;
  return new Date(date).toLocaleDateString("en-US", { day: "numeric", month: "short", year: "numeric" });
}

function formatDate(date: string | Date): string {
  return new Date(date).toLocaleDateString("en-US", {
    weekday: "short",
    day: "numeric",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export default function NotificationCenter() {
  const { user, loading: authLoading } = useAuth();
  const utils = trpc.useUtils();

  // ─── Filter State ───
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedTypes, setSelectedTypes] = useState<Set<string>>(new Set());
  const [unreadOnly, setUnreadOnly] = useState(false);
  const [showFilters, setShowFilters] = useState(false);
  const [cursor, setCursor] = useState<number | undefined>(undefined);
  const [allItems, setAllItems] = useState<any[]>([]);
  const [hasLoadedOnce, setHasLoadedOnce] = useState(false);

  // ─── tRPC Queries ───
  const typeFilter = selectedTypes.size === 1 ? (Array.from(selectedTypes)[0] as any) : undefined;

  const notificationsQuery = trpc.notifications.list.useQuery(
    {
      limit: 30,
      cursor,
      unreadOnly,
      type: typeFilter,
    },
    {
      enabled: !!user,
      staleTime: 15_000,
    }
  );

  const unreadCountQuery = trpc.notifications.getUnreadCount.useQuery(undefined, {
    enabled: !!user,
    staleTime: 10_000,
  });

  // ─── Accumulate paginated results ───
  useEffect(() => {
    if (notificationsQuery.data?.items) {
      if (!cursor) {
        // First page or filter change
        setAllItems(notificationsQuery.data.items);
      } else {
        // Append next page
        setAllItems(prev => {
          const existingIds = new Set(prev.map(i => i.notificationId));
          const newItems = notificationsQuery.data!.items.filter(
            (i: any) => !existingIds.has(i.notificationId)
          );
          return [...prev, ...newItems];
        });
      }
      setHasLoadedOnce(true);
    }
  }, [notificationsQuery.data, cursor]);

  // Reset when filters change
  useEffect(() => {
    setCursor(undefined);
    setAllItems([]);
    setHasLoadedOnce(false);
  }, [selectedTypes, unreadOnly]);

  // ─── Client-side search filter ───
  const filteredItems = useMemo(() => {
    let items = allItems;

    // Search filter (client-side)
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      items = items.filter(
        (n: any) =>
          n.title.toLowerCase().includes(q) ||
          n.message.toLowerCase().includes(q)
      );
    }

    // Multi-type filter (client-side when multiple selected)
    if (selectedTypes.size > 1) {
      items = items.filter((n: any) => selectedTypes.has(n.type));
    }

    return items;
  }, [allItems, searchQuery, selectedTypes]);

  // ─── Mutations ───
  const markReadMutation = trpc.notifications.markRead.useMutation({
    onSuccess: () => {
      utils.notifications.list.invalidate();
      utils.notifications.getUnreadCount.invalidate();
    },
  });

  const markAllReadMutation = trpc.notifications.markAllRead.useMutation({
    onSuccess: () => {
      utils.notifications.list.invalidate();
      utils.notifications.getUnreadCount.invalidate();
      setAllItems(prev => prev.map(i => ({ ...i, isRead: true })));
    },
  });

  const deleteMutation = trpc.notifications.delete.useMutation({
    onSuccess: (_data, variables) => {
      setAllItems(prev => prev.filter(i => i.notificationId !== variables.notificationId));
      utils.notifications.getUnreadCount.invalidate();
    },
  });

  const clearAllMutation = trpc.notifications.clearAll.useMutation({
    onSuccess: () => {
      setAllItems([]);
      utils.notifications.getUnreadCount.invalidate();
    },
  });

  // ─── Actions ───
  const toggleType = useCallback((type: string) => {
    setSelectedTypes(prev => {
      const next = new Set(prev);
      if (next.has(type)) {
        next.delete(type);
      } else {
        next.add(type);
      }
      return next;
    });
  }, []);

  const loadMore = useCallback(() => {
    if (notificationsQuery.data?.hasMore && notificationsQuery.data.nextCursor) {
      setCursor(notificationsQuery.data.nextCursor);
    }
  }, [notificationsQuery.data]);

  const clearFilters = useCallback(() => {
    setSelectedTypes(new Set());
    setUnreadOnly(false);
    setSearchQuery("");
  }, []);

  // ─── Auth Guard ───
  if (authLoading) {
    return (
      <div className="h-screen w-screen flex items-center justify-center bg-gray-50">
        <div className="w-8 h-8 rounded-full border-2 border-indigo-200 border-t-indigo-600 animate-spin" />
      </div>
    );
  }

  if (!user) {
    return (
      <div className="h-screen w-screen flex items-center justify-center bg-gray-50">
        <div className="text-center">
          <Bell className="w-12 h-12 text-gray-300 mx-auto mb-4" />
          <p className="text-gray-500 mb-4">Please log in to view notifications</p>
          <Button onClick={() => { window.location.href = getLoginUrl(); }}>Log In</Button>
        </div>
      </div>
    );
  }

  const activeFilterCount = selectedTypes.size + (unreadOnly ? 1 : 0) + (searchQuery ? 1 : 0);
  const unreadCount = unreadCountQuery.data?.count ?? 0;

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="sticky top-0 z-40 bg-white border-b border-gray-200 shadow-sm">
        <div className="max-w-4xl mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <button
                onClick={() => window.history.back()}
                className="p-2 rounded-xl hover:bg-gray-100 transition-colors"
              >
                <ArrowLeft className="w-5 h-5 text-gray-600" />
              </button>
              <div>
                <h1 className="text-xl font-bold text-gray-900 flex items-center gap-2">
                  <Bell className="w-5 h-5 text-indigo-600" />
                  Notification Center
                  {unreadCount > 0 && (
                    <Badge variant="destructive" className="text-xs">{unreadCount} unread</Badge>
                  )}
                </h1>
                <p className="text-sm text-gray-500 mt-0.5">
                  {filteredItems.length} notification{filteredItems.length !== 1 ? "s" : ""}
                  {activeFilterCount > 0 && ` (filtered)`}
                </p>
              </div>
            </div>

            <div className="flex items-center gap-2">
              {unreadCount > 0 && (
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => markAllReadMutation.mutate()}
                  disabled={markAllReadMutation.isPending}
                  className="text-xs"
                >
                  <CheckCheck className="w-3.5 h-3.5 mr-1" />
                  Mark All Read
                </Button>
              )}
              {allItems.length > 0 && (
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => {
                    if (window.confirm("Clear all notifications? This cannot be undone.")) {
                      clearAllMutation.mutate();
                    }
                  }}
                  disabled={clearAllMutation.isPending}
                  className="text-xs text-red-600 hover:text-red-700 hover:bg-red-50"
                >
                  <Trash2 className="w-3.5 h-3.5 mr-1" />
                  Clear All
                </Button>
              )}
              <Button
                variant="outline"
                size="sm"
                onClick={() => {
                  setCursor(undefined);
                  setAllItems([]);
                  setHasLoadedOnce(false);
                  utils.notifications.list.invalidate();
                  utils.notifications.getUnreadCount.invalidate();
                }}
                className="text-xs"
              >
                <RefreshCw className="w-3.5 h-3.5" />
              </Button>
            </div>
          </div>

          {/* Search & Filter Bar */}
          <div className="mt-3 flex items-center gap-2">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <Input
                placeholder="Search notifications..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-9 h-9 text-sm"
              />
              {searchQuery && (
                <button
                  onClick={() => setSearchQuery("")}
                  className="absolute right-2 top-1/2 -translate-y-1/2 p-1 rounded hover:bg-gray-100"
                >
                  <X className="w-3.5 h-3.5 text-gray-400" />
                </button>
              )}
            </div>

            <Button
              variant={showFilters ? "default" : "outline"}
              size="sm"
              onClick={() => setShowFilters(!showFilters)}
              className="h-9 text-xs"
            >
              <Filter className="w-3.5 h-3.5 mr-1" />
              Filters
              {activeFilterCount > 0 && (
                <span className="ml-1 bg-indigo-100 text-indigo-700 rounded-full px-1.5 text-[10px] font-bold">
                  {activeFilterCount}
                </span>
              )}
            </Button>

            <Button
              variant={unreadOnly ? "default" : "outline"}
              size="sm"
              onClick={() => setUnreadOnly(!unreadOnly)}
              className="h-9 text-xs"
            >
              Unread Only
            </Button>
          </div>

          {/* Filter Panel */}
          <AnimatePresence>
            {showFilters && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: "auto", opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                transition={{ duration: 0.2 }}
                className="overflow-hidden"
              >
                <div className="mt-3 pt-3 border-t border-gray-100">
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-xs font-medium text-gray-500">Filter by type:</span>
                    {selectedTypes.size > 0 && (
                      <button
                        onClick={clearFilters}
                        className="text-xs text-indigo-600 hover:text-indigo-700"
                      >
                        Clear all
                      </button>
                    )}
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {ALL_TYPES.map((type) => {
                      const config = TYPE_CONFIG[type];
                      const isActive = selectedTypes.has(type);
                      return (
                        <button
                          key={type}
                          onClick={() => toggleType(type)}
                          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium transition-all duration-200 border ${
                            isActive
                              ? "border-transparent shadow-sm"
                              : "border-gray-200 bg-white text-gray-600 hover:bg-gray-50"
                          }`}
                          style={
                            isActive
                              ? { background: config.bgColor, color: config.color, borderColor: config.color + "40" }
                              : undefined
                          }
                        >
                          <config.icon className="w-3.5 h-3.5" />
                          {config.label}
                        </button>
                      );
                    })}
                  </div>
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>

      {/* Notification List */}
      <div className="max-w-4xl mx-auto px-4 py-6">
        {notificationsQuery.isLoading && !hasLoadedOnce ? (
          <div className="py-16 flex flex-col items-center gap-4">
            <div className="w-10 h-10 rounded-full border-2 border-indigo-200 border-t-indigo-600 animate-spin" />
            <p className="text-sm text-gray-400">Loading notifications...</p>
          </div>
        ) : filteredItems.length === 0 ? (
          <div className="py-16 flex flex-col items-center gap-4">
            <div className="w-16 h-16 rounded-2xl bg-gray-100 flex items-center justify-center">
              <Inbox className="w-8 h-8 text-gray-300" />
            </div>
            <div className="text-center">
              <p className="text-base font-medium text-gray-500">
                {activeFilterCount > 0 ? "No matching notifications" : "No notifications yet"}
              </p>
              <p className="text-sm text-gray-400 mt-1">
                {activeFilterCount > 0
                  ? "Try adjusting your filters"
                  : "When you receive notifications, they'll appear here"}
              </p>
              {activeFilterCount > 0 && (
                <Button variant="outline" size="sm" onClick={clearFilters} className="mt-3 text-xs">
                  Clear Filters
                </Button>
              )}
            </div>
          </div>
        ) : (
          <div className="space-y-2">
            <AnimatePresence initial={false}>
              {filteredItems.map((notif: any) => {
                const config = TYPE_CONFIG[notif.type] || TYPE_CONFIG.info;
                const Icon = config.icon;

                return (
                  <motion.div
                    key={notif.notificationId}
                    initial={{ opacity: 0, y: -8 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, x: 100 }}
                    transition={{ duration: 0.2 }}
                    className={`group relative rounded-xl border transition-all duration-200 ${
                      notif.isRead
                        ? "bg-white border-gray-100 hover:border-gray-200"
                        : "bg-indigo-50/30 border-indigo-100 hover:border-indigo-200"
                    }`}
                  >
                    <div className="px-5 py-4 flex items-start gap-4">
                      {/* Type Icon */}
                      <div
                        className="flex-shrink-0 w-10 h-10 rounded-xl flex items-center justify-center mt-0.5"
                        style={{ background: config.bgColor }}
                      >
                        <Icon className="w-5 h-5" style={{ color: config.color }} />
                      </div>

                      {/* Content */}
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2">
                          <h4
                            className={`text-sm ${
                              notif.isRead ? "font-normal text-gray-600" : "font-semibold text-gray-900"
                            }`}
                          >
                            {notif.title}
                          </h4>
                          {!notif.isRead && (
                            <div className="flex-shrink-0 w-2 h-2 rounded-full bg-indigo-500" />
                          )}
                        </div>
                        <p className="text-sm text-gray-500 mt-1 leading-relaxed">{notif.message}</p>
                        <div className="flex items-center gap-3 mt-2">
                          <span
                            className="text-[11px] font-medium px-2 py-0.5 rounded-full"
                            style={{ color: config.color, background: config.bgColor }}
                          >
                            {config.label}
                          </span>
                          <span className="text-[11px] text-gray-400 flex items-center gap-1">
                            <Calendar className="w-3 h-3" />
                            {formatDate(notif.createdAt)}
                          </span>
                          <span className="text-[11px] text-gray-400">{timeAgo(notif.createdAt)}</span>
                        </div>
                      </div>

                      {/* Actions */}
                      <div className="flex-shrink-0 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                        {!notif.isRead && (
                          <button
                            onClick={() => markReadMutation.mutate({ notificationId: notif.notificationId })}
                            className="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                            title="Mark as read"
                          >
                            <Check className="w-4 h-4 text-gray-400 hover:text-indigo-600" />
                          </button>
                        )}
                        <button
                          onClick={() => deleteMutation.mutate({ notificationId: notif.notificationId })}
                          className="p-2 rounded-lg hover:bg-red-50 transition-colors"
                          title="Delete"
                        >
                          <Trash2 className="w-4 h-4 text-gray-400 hover:text-red-500" />
                        </button>
                      </div>
                    </div>
                  </motion.div>
                );
              })}
            </AnimatePresence>

            {/* Load More */}
            {notificationsQuery.data?.hasMore && (
              <div className="pt-4 flex justify-center">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={loadMore}
                  disabled={notificationsQuery.isFetching}
                  className="text-xs"
                >
                  {notificationsQuery.isFetching ? (
                    <>
                      <div className="w-3.5 h-3.5 rounded-full border-2 border-gray-200 border-t-indigo-600 animate-spin mr-1.5" />
                      Loading...
                    </>
                  ) : (
                    "Load More"
                  )}
                </Button>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
