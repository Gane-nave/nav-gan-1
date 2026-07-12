/**
 * G.A.N.E — Smart Alert System
 * ==============================
 * Context-aware notification engine with priority queue.
 *
 * ALERT TYPES:
 *   - CRITICAL: Immediate danger (wrong-way, accident ahead)
 *   - WARNING: Upcoming issue (congestion, speed trap)
 *   - INFO: Helpful notification (ETA update, weather change)
 *   - SUGGESTION: Optional improvement (better route available)
 *
 * FEATURES:
 *   - Priority queue with deduplication
 *   - Context-aware suppression (don't alert during calls)
 *   - Distance-based triggering
 *   - Cooldown per alert type
 *   - Audio + haptic + visual channels
 *   - Hebrew + English bilingual alerts
 */

// ─── Types ───────────────────────────────────────────────

export type AlertPriority = 'critical' | 'warning' | 'info' | 'suggestion';
export type AlertChannel = 'visual' | 'audio' | 'haptic';

export interface Alert {
  id: string;
  type: string;
  priority: AlertPriority;
  title: string;
  titleHe: string;
  message: string;
  messageHe: string;
  icon: string;
  channels: AlertChannel[];
  lat?: number;
  lon?: number;
  triggerDistanceM?: number;       // Show when within this distance
  expiresAt: number;               // Unix ms
  cooldownMs: number;              // Min time before showing same type again
  isActionable: boolean;
  action?: {
    label: string;
    labelHe: string;
    callback: string;              // Action identifier
  };
  createdAt: number;
  shownAt: number;
  dismissedAt: number;
}

export interface AlertConfig {
  maxQueueSize: number;            // Max alerts in queue (default: 20)
  defaultCooldownMs: number;       // Default cooldown per type (default: 60000)
  criticalCooldownMs: number;      // Critical alert cooldown (default: 10000)
  suppressDuringCall: boolean;     // Suppress non-critical during calls
  suppressDuringNav: boolean;      // Suppress suggestions during navigation
  enableAudio: boolean;
  enableHaptic: boolean;
  enableVisual: boolean;
  language: 'en' | 'he' | 'auto';
  enabled: boolean;
}

export interface AlertState {
  isActive: boolean;
  queueSize: number;
  totalShown: number;
  totalSuppressed: number;
  totalDismissed: number;
  lastAlertAt: number;
  activeAlert: Alert | null;
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: AlertConfig = {
  maxQueueSize: 20,
  defaultCooldownMs: 60000,
  criticalCooldownMs: 10000,
  suppressDuringCall: true,
  suppressDuringNav: false,
  enableAudio: true,
  enableHaptic: true,
  enableVisual: true,
  language: 'auto',
  enabled: true,
};

const PRIORITY_ORDER: Record<AlertPriority, number> = {
  critical: 0,
  warning: 1,
  info: 2,
  suggestion: 3,
};

// ─── Pre-defined Alert Templates ────────────────────────

export const ALERT_TEMPLATES = {
  // Critical
  wrongWay: {
    type: 'wrong_way',
    priority: 'critical' as AlertPriority,
    title: 'Wrong Way!',
    titleHe: 'כיוון שגוי!',
    message: 'You are driving against traffic. Turn around immediately.',
    messageHe: 'אתה נוסע נגד התנועה. פנה מיד.',
    icon: '🚨',
    channels: ['visual', 'audio', 'haptic'] as AlertChannel[],
    cooldownMs: 10000,
    isActionable: false,
  },
  accidentAhead: {
    type: 'accident_ahead',
    priority: 'critical' as AlertPriority,
    title: 'Accident Ahead',
    titleHe: 'תאונה לפנים',
    message: 'Accident reported ahead on your route.',
    messageHe: 'דווחה תאונה בהמשך המסלול.',
    icon: '⚠️',
    channels: ['visual', 'audio'] as AlertChannel[],
    cooldownMs: 30000,
    isActionable: true,
    action: { label: 'Reroute', labelHe: 'נתב מחדש', callback: 'reroute' },
  },
  // Warning
  speedTrap: {
    type: 'speed_trap',
    priority: 'warning' as AlertPriority,
    title: 'Speed Camera Ahead',
    titleHe: 'מצלמת מהירות לפנים',
    message: 'Speed enforcement camera detected ahead.',
    messageHe: 'זוהתה מצלמת אכיפת מהירות.',
    icon: '📷',
    channels: ['visual', 'audio'] as AlertChannel[],
    cooldownMs: 120000,
    isActionable: false,
  },
  congestionAhead: {
    type: 'congestion_ahead',
    priority: 'warning' as AlertPriority,
    title: 'Heavy Traffic Ahead',
    titleHe: 'עומס תנועה לפנים',
    message: 'Heavy congestion detected on your route.',
    messageHe: 'זוהה עומס כבד במסלול.',
    icon: '🚗',
    channels: ['visual'] as AlertChannel[],
    cooldownMs: 300000,
    isActionable: true,
    action: { label: 'Find Alternative', labelHe: 'חפש חלופה', callback: 'find_alternative' },
  },
  hazard: {
    type: 'hazard',
    priority: 'warning' as AlertPriority,
    title: 'Road Hazard',
    titleHe: 'מפגע בכביש',
    message: 'Hazard reported on the road ahead.',
    messageHe: 'דווח מפגע בכביש.',
    icon: '⚡',
    channels: ['visual', 'audio'] as AlertChannel[],
    cooldownMs: 60000,
    isActionable: false,
  },
  // Info
  etaUpdate: {
    type: 'eta_update',
    priority: 'info' as AlertPriority,
    title: 'ETA Updated',
    titleHe: 'זמן הגעה עודכן',
    message: 'Your estimated arrival time has changed.',
    messageHe: 'זמן ההגעה המשוער השתנה.',
    icon: '⏱️',
    channels: ['visual'] as AlertChannel[],
    cooldownMs: 300000,
    isActionable: false,
  },
  weatherChange: {
    type: 'weather_change',
    priority: 'info' as AlertPriority,
    title: 'Weather Alert',
    titleHe: 'התראת מזג אוויר',
    message: 'Weather conditions changing along your route.',
    messageHe: 'תנאי מזג האוויר משתנים במסלול.',
    icon: '🌧️',
    channels: ['visual'] as AlertChannel[],
    cooldownMs: 600000,
    isActionable: false,
  },
  // Suggestion
  betterRoute: {
    type: 'better_route',
    priority: 'suggestion' as AlertPriority,
    title: 'Faster Route Available',
    titleHe: 'מסלול מהיר יותר זמין',
    message: 'A faster route has been found.',
    messageHe: 'נמצא מסלול מהיר יותר.',
    icon: '🛤️',
    channels: ['visual'] as AlertChannel[],
    cooldownMs: 300000,
    isActionable: true,
    action: { label: 'Switch Route', labelHe: 'החלף מסלול', callback: 'switch_route' },
  },
  lowBattery: {
    type: 'low_battery',
    priority: 'suggestion' as AlertPriority,
    title: 'Low Battery',
    titleHe: 'סוללה חלשה',
    message: 'Switching to power-saving mode.',
    messageHe: 'עובר למצב חיסכון בחשמל.',
    icon: '🔋',
    channels: ['visual'] as AlertChannel[],
    cooldownMs: 600000,
    isActionable: false,
  },
} as const;

// ─── Alert System ───────────────────────────────────────

export class AlertSystem {
  private config: AlertConfig;
  private state: AlertState;
  private queue: Alert[] = [];
  private cooldowns: Map<string, number> = new Map(); // type → lastShownAt
  private currentPosition: { lat: number; lon: number } | null = null;

  // Callbacks
  private onAlert: ((alert: Alert) => void) | null = null;
  private onDismiss: ((alertId: string) => void) | null = null;

  constructor(config: Partial<AlertConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.state = {
      isActive: false,
      queueSize: 0,
      totalShown: 0,
      totalSuppressed: 0,
      totalDismissed: 0,
      lastAlertAt: 0,
      activeAlert: null,
    };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  start() {
    this.state.isActive = true;
  }

  stop() {
    this.state.isActive = false;
    this.queue = [];
    this.state.activeAlert = null;
  }

  // ─── Alert Creation ───────────────────────────────────

  /**
   * Push a new alert from a template.
   */
  pushFromTemplate(
    templateKey: keyof typeof ALERT_TEMPLATES,
    overrides: Partial<Alert> = {}
  ): Alert | null {
    const template = ALERT_TEMPLATES[templateKey];
    return this.push({
      type: template.type,
      priority: template.priority,
      title: template.title,
      titleHe: template.titleHe,
      message: overrides.message || template.message,
      messageHe: overrides.messageHe || template.messageHe,
      icon: template.icon,
      channels: template.channels as AlertChannel[],
      cooldownMs: template.cooldownMs,
      isActionable: template.isActionable,
      action: 'action' in template ? template.action as Alert['action'] : undefined,
      ...overrides,
    });
  }

  /**
   * Push a custom alert.
   */
  push(partial: Partial<Alert> & { type: string; priority: AlertPriority }): Alert | null {
    if (!this.config.enabled || !this.state.isActive) return null;

    // Check cooldown
    const lastShown = this.cooldowns.get(partial.type) || 0;
    const cooldown = partial.cooldownMs || this.config.defaultCooldownMs;
    if (Date.now() - lastShown < cooldown) {
      this.state.totalSuppressed++;
      return null;
    }

    // Check distance trigger
    if (partial.lat && partial.lon && partial.triggerDistanceM && this.currentPosition) {
      const dist = this.haversine(
        this.currentPosition.lat, this.currentPosition.lon,
        partial.lat, partial.lon
      );
      if (dist > partial.triggerDistanceM) {
        return null; // Not close enough yet
      }
    }

    const alert: Alert = {
      id: `alert_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`,
      type: partial.type,
      priority: partial.priority,
      title: partial.title || '',
      titleHe: partial.titleHe || '',
      message: partial.message || '',
      messageHe: partial.messageHe || '',
      icon: partial.icon || '📢',
      channels: partial.channels || ['visual'],
      lat: partial.lat,
      lon: partial.lon,
      triggerDistanceM: partial.triggerDistanceM,
      expiresAt: partial.expiresAt || Date.now() + 30000,
      cooldownMs: cooldown,
      isActionable: partial.isActionable || false,
      action: partial.action,
      createdAt: Date.now(),
      shownAt: 0,
      dismissedAt: 0,
    };

    // Add to queue (sorted by priority)
    this.queue.push(alert);
    this.queue.sort((a, b) => PRIORITY_ORDER[a.priority] - PRIORITY_ORDER[b.priority]);

    // Cap queue size
    if (this.queue.length > this.config.maxQueueSize) {
      this.queue = this.queue.slice(0, this.config.maxQueueSize);
    }

    this.state.queueSize = this.queue.length;

    // If no active alert, show immediately
    if (!this.state.activeAlert) {
      this.showNext();
    }

    return alert;
  }

  // ─── Alert Display ────────────────────────────────────

  private showNext() {
    // Remove expired alerts
    const now = Date.now();
    this.queue = this.queue.filter(a => a.expiresAt > now);

    if (this.queue.length === 0) {
      this.state.activeAlert = null;
      this.state.queueSize = 0;
      return;
    }

    const alert = this.queue.shift()!;
    alert.shownAt = now;
    this.state.activeAlert = alert;
    this.state.queueSize = this.queue.length;
    this.state.totalShown++;
    this.state.lastAlertAt = now;

    // Record cooldown
    this.cooldowns.set(alert.type, now);

    // Notify
    if (this.onAlert) {
      this.onAlert(alert);
    }
  }

  /**
   * Dismiss the current alert.
   */
  dismiss(alertId?: string) {
    if (this.state.activeAlert) {
      const id = alertId || this.state.activeAlert.id;
      this.state.activeAlert.dismissedAt = Date.now();
      this.state.totalDismissed++;

      if (this.onDismiss) {
        this.onDismiss(id);
      }

      this.state.activeAlert = null;

      // Show next in queue
      this.showNext();
    }
  }

  // ─── Position Updates ─────────────────────────────────

  updatePosition(lat: number, lon: number) {
    this.currentPosition = { lat, lon };

    // Check distance-triggered alerts in queue
    for (const alert of this.queue) {
      if (alert.lat && alert.lon && alert.triggerDistanceM) {
        const dist = this.haversine(lat, lon, alert.lat, alert.lon);
        if (dist <= alert.triggerDistanceM && !this.state.activeAlert) {
          this.showNext();
          break;
        }
      }
    }
  }

  // ─── Callbacks ────────────────────────────────────────

  setOnAlert(callback: (alert: Alert) => void) {
    this.onAlert = callback;
  }

  setOnDismiss(callback: (alertId: string) => void) {
    this.onDismiss = callback;
  }

  // ─── Helpers ──────────────────────────────────────────

  private haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const R = 6371000;
    const dLat = (lat2 - lat1) * Math.PI / 180;
    const dLon = (lon2 - lon1) * Math.PI / 180;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * Math.PI / 180) * Math.cos(lat2 * Math.PI / 180) *
      Math.sin(dLon / 2) ** 2;
    return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  // ─── Public API ───────────────────────────────────────

  getState(): AlertState {
    return { ...this.state };
  }

  getConfig(): AlertConfig {
    return { ...this.config };
  }

  updateConfig(partial: Partial<AlertConfig>) {
    this.config = { ...this.config, ...partial };
  }

  getQueue(): Alert[] {
    return [...this.queue];
  }

  clearQueue() {
    this.queue = [];
    this.state.queueSize = 0;
  }

  destroy() {
    this.stop();
    this.onAlert = null;
    this.onDismiss = null;
    this.cooldowns.clear();
  }
}
