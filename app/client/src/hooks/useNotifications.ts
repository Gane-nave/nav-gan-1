/**
 * G.A.N.E — useNotifications Hook
 * =================================
 * Manages notification state, real-time WebSocket delivery,
 * and tRPC queries/mutations for the notification system.
 *
 * Features:
 * - Fetches notifications via tRPC with pagination
 * - Listens for real-time notifications via global event bus
 *   (the server pushes "notification" events on the same WS
 *   connection managed by useCollaboration / wsClient — no
 *   duplicate WebSocket is opened)
 * - Shows toast notifications for new real-time alerts
 * - Provides mark-read, delete, clear-all mutations
 * - Tracks unread count with optimistic updates
 * - Notification sound engine (Web Audio API)
 */
import { useCallback, useEffect, useRef } from "react";
import { shouldShowNotification } from "@/lib/notificationThrottle";
import { trpc } from "@/lib/trpc";
import { toast } from "sonner";

export interface NotificationItem {
  id: number;
  notificationId: string;
  userId: number;
  type: "info" | "success" | "warning" | "error" | "system" | "collaboration" | "admin";
  title: string;
  message: string;
  isRead: boolean;
  metadata: unknown;
  createdAt: string | Date;
  readAt: string | Date | null;
  expiresAt: string | Date | null;
}

// ─── Toast Icon Map ───
const TOAST_ICONS: Record<string, string> = {
  info: "ℹ️",
  success: "✅",
  warning: "⚠️",
  error: "❌",
  system: "⚙️",
  collaboration: "👥",
  admin: "🔔",
};

// ─── Notification Sound Engine ───
const SOUND_FREQUENCIES: Record<string, { freq: number; duration: number; type: OscillatorType }> = {
  info: { freq: 800, duration: 150, type: "sine" },
  success: { freq: 1000, duration: 200, type: "sine" },
  warning: { freq: 600, duration: 250, type: "triangle" },
  error: { freq: 400, duration: 300, type: "sawtooth" },
  system: { freq: 700, duration: 180, type: "sine" },
  collaboration: { freq: 900, duration: 160, type: "sine" },
  admin: { freq: 850, duration: 220, type: "sine" },
};

let audioCtx: AudioContext | null = null;

function playNotificationSound(type: string): void {
  try {
    if (!audioCtx) {
      audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)();
    }
    if (audioCtx.state === "suspended") {
      audioCtx.resume();
    }

    const config = SOUND_FREQUENCIES[type] || SOUND_FREQUENCIES.info;
    const oscillator = audioCtx.createOscillator();
    const gainNode = audioCtx.createGain();

    oscillator.type = config.type;
    oscillator.frequency.setValueAtTime(config.freq, audioCtx.currentTime);

    // Gentle envelope: quick attack, smooth decay
    gainNode.gain.setValueAtTime(0, audioCtx.currentTime);
    gainNode.gain.linearRampToValueAtTime(0.15, audioCtx.currentTime + 0.02);
    gainNode.gain.exponentialRampToValueAtTime(0.001, audioCtx.currentTime + config.duration / 1000);

    oscillator.connect(gainNode);
    gainNode.connect(audioCtx.destination);

    oscillator.start(audioCtx.currentTime);
    oscillator.stop(audioCtx.currentTime + config.duration / 1000);
  } catch {
    // Audio not available (e.g., no user gesture yet)
  }
}

// ═══════════════════════════════════════════════════════════
// Global Notification Event Bus
// ═══════════════════════════════════════════════════════════
// Instead of opening a separate WebSocket, we expose a global
// event target. The existing WS connections (collabWsHandler on
// the server auto-subscribes each user to their notification
// channel) forward "notification" messages here. Any component
// can dispatch events on this bus.
// ═══════════════════════════════════════════════════════════
type NotificationEventDetail = {
  notificationType: string;
  title: string;
  message: string;
  notificationId?: string;
};

const NOTIFICATION_EVENT = "gane:notification";

export const notificationBus = new EventTarget();

/**
 * Call this from any WebSocket onmessage handler when a
 * `{ type: "notification", ... }` message arrives.
 */
export function dispatchNotificationEvent(data: {
  notificationType?: string;
  type?: string;
  title?: string;
  message?: string;
  notificationId?: string;
}) {
  notificationBus.dispatchEvent(
    new CustomEvent<NotificationEventDetail>(NOTIFICATION_EVENT, {
      detail: {
        notificationType: data.notificationType || data.type || "info",
        title: data.title || "Notification",
        message: data.message || "",
        notificationId: data.notificationId,
      },
    })
  );
}

export function useNotifications() {
  const utils = trpc.useUtils();
  const preferencesRef = useRef<{ enableSound?: boolean; enableToast?: boolean } | null>(null);

  // ─── tRPC Queries ───
  const notificationsQuery = trpc.notifications.list.useQuery(
    { limit: 30 },
    {
      refetchOnWindowFocus: false,
      staleTime: 30_000,
    }
  );

  const unreadCountQuery = trpc.notifications.getUnreadCount.useQuery(undefined, {
    refetchInterval: 60_000, // Poll every 60s as fallback
    staleTime: 10_000,
  });

  const preferencesQuery = trpc.notifications.getPreferences.useQuery(undefined, {
    staleTime: 300_000, // 5 min cache
  });

  // ─── tRPC Mutations ───
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
    },
  });

  const deleteMutation = trpc.notifications.delete.useMutation({
    onSuccess: () => {
      utils.notifications.list.invalidate();
      utils.notifications.getUnreadCount.invalidate();
    },
  });

  const clearAllMutation = trpc.notifications.clearAll.useMutation({
    onSuccess: () => {
      utils.notifications.list.invalidate();
      utils.notifications.getUnreadCount.invalidate();
    },
  });

  const updatePreferencesMutation = trpc.notifications.updatePreferences.useMutation({
    onSuccess: () => {
      utils.notifications.getPreferences.invalidate();
    },
  });

  // Keep preferences in a ref so the event handler always reads the latest
  useEffect(() => {
    preferencesRef.current = preferencesQuery.data ?? null;
  }, [preferencesQuery.data]);

  // ─── Listen for notification events from the global bus ───
  useEffect(() => {
    function handleNotification(e: Event) {
      const { detail } = e as CustomEvent<NotificationEventDetail>;
      const prefs = preferencesRef.current;

      // Throttle repeats before anything user-visible happens. Without this a
      // stuck upstream condition re-fires the same alert every tick — the
      // "warnings appear every second" defect. Per-message cooldown lives in
      // lib/notificationThrottle so sound and toast stay in lockstep.
      const throttleType =
        detail.notificationType === "error" ||
        detail.notificationType === "warning" ||
        detail.notificationType === "success"
          ? detail.notificationType
          : "info";
      if (
        !shouldShowNotification(
          `${detail.title}|${detail.message ?? ""}`,
          throttleType
        )
      ) {
        return;
      }

      // Play sound if enabled
      if (prefs?.enableSound !== false) {
        playNotificationSound(detail.notificationType || "info");
      }

      // Show toast if enabled
      if (prefs?.enableToast !== false) {
        const icon = TOAST_ICONS[detail.notificationType] || "🔔";
        const nType = detail.notificationType;
        if (nType === "error") {
          toast.error(`${icon} ${detail.title}`, { description: detail.message, duration: 6000 });
        } else if (nType === "warning") {
          toast.warning(`${icon} ${detail.title}`, { description: detail.message, duration: 5000 });
        } else if (nType === "success") {
          toast.success(`${icon} ${detail.title}`, { description: detail.message, duration: 4000 });
        } else {
          toast.info(`${icon} ${detail.title}`, { description: detail.message, duration: 4000 });
        }
      }

      // Invalidate queries to refresh data
      utils.notifications.list.invalidate();
      utils.notifications.getUnreadCount.invalidate();
    }

    notificationBus.addEventListener(NOTIFICATION_EVENT, handleNotification);
    return () => {
      notificationBus.removeEventListener(NOTIFICATION_EVENT, handleNotification);
    };
  }, [utils]);

  // ─── Actions ───
  const markRead = useCallback(
    (notificationId: string) => {
      markReadMutation.mutate({ notificationId });
    },
    [markReadMutation]
  );

  const markAllRead = useCallback(() => {
    markAllReadMutation.mutate();
  }, [markAllReadMutation]);

  const deleteNotification = useCallback(
    (notificationId: string) => {
      deleteMutation.mutate({ notificationId });
    },
    [deleteMutation]
  );

  const clearAll = useCallback(() => {
    clearAllMutation.mutate();
  }, [clearAllMutation]);

  const updatePreferences = useCallback(
    (prefs: Record<string, boolean>) => {
      updatePreferencesMutation.mutate(prefs);
    },
    [updatePreferencesMutation]
  );

  return {
    // Data
    notifications: notificationsQuery.data?.items ?? [],
    unreadCount: unreadCountQuery.data?.count ?? 0,
    preferences: preferencesQuery.data ?? null,
    hasMore: notificationsQuery.data?.hasMore ?? false,
    isLoading: notificationsQuery.isLoading,

    // Actions
    markRead,
    markAllRead,
    deleteNotification,
    clearAll,
    updatePreferences,

    // Refresh
    refresh: () => {
      utils.notifications.list.invalidate();
      utils.notifications.getUnreadCount.invalidate();
    },
  };
}
