/**
 * Notification Throttle System
 * Prevents notification spam by throttling duplicate messages
 */

interface ThrottledNotification {
  message: string;
  type: 'info' | 'success' | 'error' | 'warning';
  lastShown: number;
  count: number;
}

const notificationCache = new Map<string, ThrottledNotification>();
const THROTTLE_DURATION = 5000; // 5 seconds minimum between same notifications
const CACHE_CLEANUP_INTERVAL = 60000; // Clean up old entries every 60 seconds

// Clean up old cache entries periodically
setInterval(() => {
  const now = Date.now();
  notificationCache.forEach((notification, key) => {
    if (now - notification.lastShown > 300000) { // Remove entries older than 5 minutes
      notificationCache.delete(key);
    }
  });
}, CACHE_CLEANUP_INTERVAL);

/**
 * Check if a notification should be shown based on throttle rules
 */
export function shouldShowNotification(
  message: string,
  type: 'info' | 'success' | 'error' | 'warning' = 'info'
): boolean {
  const key = `${type}:${message}`;
  const now = Date.now();
  const cached = notificationCache.get(key);

  if (!cached) {
    // First time seeing this notification
    notificationCache.set(key, {
      message,
      type,
      lastShown: now,
      count: 1,
    });
    return true;
  }

  const timeSinceLastShown = now - cached.lastShown;

  if (timeSinceLastShown >= THROTTLE_DURATION) {
    // Enough time has passed, show it again
    cached.lastShown = now;
    cached.count++;
    return true;
  }

  // Too soon, skip this notification
  return false;
}

/**
 * Clear all throttled notifications
 */
export function clearNotificationCache(): void {
  notificationCache.clear();
}

/**
 * Get throttle stats for debugging
 */
export function getThrottleStats(): Record<string, { count: number; lastShown: number }> {
  const stats: Record<string, { count: number; lastShown: number }> = {};
  notificationCache.forEach((notification, key) => {
    stats[key] = {
      count: notification.count,
      lastShown: notification.lastShown,
    };
  });
  return stats;
}
