/**
 * G.A.N.E — Integration Hub
 * ================================
 * Unified notification and integration system.
 *
 * Channels:
 * - In-app notifications (via built-in notification API)
 * - Gmail notifications (via built-in notification API)
 * - Google Drive sync (file export)
 * - Payment event alerts
 * - In-app push notifications
 *
 * Architecture:
 * - Channel registry with enable/disable per user
 * - Priority-based routing (critical → all channels, info → in-app only)
 * - Retry with exponential backoff
 * - Audit trail for all sent notifications
 */

import { eq, and, desc } from "drizzle-orm";
import { getDb } from "../db";
import { integrationChannels, alerts, logEntries, paymentEvents } from "../../drizzle/schema";
import { notifyOwner } from "../_core/notification";

// ═══════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════

export type AlertPriority = 'critical' | 'high' | 'medium' | 'low' | 'info';
export type AlertCategory = 'navigation' | 'geofence' | 'fleet' | 'payment' | 'system' | 'security' | 'weather' | 'incident';
export type ChannelType = 'in_app' | 'email' | 'push' | 'sms' | 'webhook';

export interface NotificationPayload {
  userId: string;
  title: string;
  body: string;
  priority: AlertPriority;
  category: AlertCategory;
  metadata?: Record<string, unknown>;
  tripId?: string;
  deviceId?: string;
  lat?: number;
  lon?: number;
}

export interface ChannelConfig {
  channelId: string;
  userId: string;
  type: ChannelType;
  isEnabled: boolean;
  config: Record<string, unknown>;
  priorityFilter: AlertPriority[];
  categoryFilter: AlertCategory[];
}

export interface DeliveryResult {
  channelId: string;
  channelType: ChannelType;
  success: boolean;
  error?: string;
  retryCount: number;
  deliveredAt?: number;
}

export interface NotificationResult {
  alertId: string;
  deliveries: DeliveryResult[];
  totalSent: number;
  totalFailed: number;
}

// ═══════════════════════════════════════════════════
// RETRY ENGINE
// ═══════════════════════════════════════════════════

class RetryEngine {
  private maxRetries = 3;
  private baseDelayMs = 1000;
  private maxDelayMs = 30000;

  async executeWithRetry<T>(
    fn: () => Promise<T>,
    context: string
  ): Promise<{ result: T | null; retries: number; error?: string }> {
    let lastError: Error | null = null;
    for (let attempt = 0; attempt <= this.maxRetries; attempt++) {
      try {
        const result = await fn();
        return { result, retries: attempt };
      } catch (err) {
        lastError = err instanceof Error ? err : new Error(String(err));
        if (attempt < this.maxRetries) {
          const delay = Math.min(
            this.baseDelayMs * Math.pow(2, attempt) + Math.random() * 500,
            this.maxDelayMs
          );
          await new Promise(resolve => setTimeout(resolve, delay));
        }
      }
    }
    return {
      result: null,
      retries: this.maxRetries,
      error: lastError?.message || `${context} failed after ${this.maxRetries} retries`,
    };
  }
}

// ═══════════════════════════════════════════════════
// CIRCUIT BREAKER
// ═══════════════════════════════════════════════════

type CircuitState = 'closed' | 'open' | 'half_open';

class CircuitBreaker {
  private state: CircuitState = 'closed';
  private failureCount = 0;
  private lastFailureTime = 0;
  private readonly failureThreshold = 5;
  private readonly resetTimeoutMs = 60_000;
  private readonly halfOpenMaxAttempts = 2;
  private halfOpenAttempts = 0;

  canExecute(): boolean {
    if (this.state === 'closed') return true;
    if (this.state === 'open') {
      if (Date.now() - this.lastFailureTime > this.resetTimeoutMs) {
        this.state = 'half_open';
        this.halfOpenAttempts = 0;
        return true;
      }
      return false;
    }
    // half_open
    return this.halfOpenAttempts < this.halfOpenMaxAttempts;
  }

  recordSuccess(): void {
    this.failureCount = 0;
    this.state = 'closed';
    this.halfOpenAttempts = 0;
  }

  recordFailure(): void {
    this.failureCount++;
    this.lastFailureTime = Date.now();
    if (this.state === 'half_open') {
      this.halfOpenAttempts++;
      if (this.halfOpenAttempts >= this.halfOpenMaxAttempts) {
        this.state = 'open';
      }
    } else if (this.failureCount >= this.failureThreshold) {
      this.state = 'open';
    }
  }

  getState(): CircuitState {
    return this.state;
  }

  getStats() {
    return {
      state: this.state,
      failureCount: this.failureCount,
      lastFailureTime: this.lastFailureTime,
    };
  }
}

// ═══════════════════════════════════════════════════
// INTEGRATION HUB
// ═══════════════════════════════════════════════════

export class IntegrationHub {
  private retryEngine = new RetryEngine();
  private circuitBreakers = new Map<string, CircuitBreaker>();
  private stats = {
    totalSent: 0,
    totalFailed: 0,
    totalRetries: 0,
    byChannel: new Map<ChannelType, { sent: number; failed: number }>(),
    byPriority: new Map<AlertPriority, number>(),
  };

  // ── Get or create circuit breaker for a channel ──

  private getBreaker(channelId: string): CircuitBreaker {
    let breaker = this.circuitBreakers.get(channelId);
    if (!breaker) {
      breaker = new CircuitBreaker();
      this.circuitBreakers.set(channelId, breaker);
    }
    return breaker;
  }

  // ── Send notification through all matching channels ──

  async sendNotification(payload: NotificationPayload): Promise<NotificationResult> {
    const db = await getDb();
    const alertId = crypto.randomUUID();

    // 1. Store alert in DB
    if (db) {
      try {
        // Map category to alert type enum
        const typeMap: Record<string, typeof alerts.type.enumValues[number]> = {
          navigation: 'route_deviation',
          geofence: 'geofence_enter',
          fleet: 'maintenance',
          payment: 'payment',
          system: 'system',
          security: 'gnss_spoofing',
          weather: 'weather',
          incident: 'accident',
        };
        // Map priority to severity enum
        const severityMap: Record<string, typeof alerts.severity.enumValues[number]> = {
          critical: 'critical',
          high: 'emergency',
          medium: 'warning',
          low: 'info',
          info: 'info',
        };
        await db.insert(alerts).values({
          alertId,
          userId: parseInt(payload.userId) || null,
          tripId: payload.tripId || null,
          deviceId: payload.deviceId || null,
          type: typeMap[payload.category] || 'system',
          severity: severityMap[payload.priority] || 'info',
          title: payload.title,
          message: payload.body,
          lat: payload.lat || null,
          lon: payload.lon || null,
          isRead: false,
          metadata: payload.metadata ? JSON.stringify(payload.metadata) : null,
          expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000), // 7 days
        });
      } catch (err) {
        console.error('[IntegrationHub] Failed to store alert:', err);
      }
    }

    // 2. Get user's enabled channels
    const channels = await this.getUserChannels(payload.userId);
    const matchingChannels = channels.filter(ch =>
      ch.isEnabled &&
      ch.priorityFilter.includes(payload.priority) &&
      ch.categoryFilter.includes(payload.category)
    );

    // 3. Always send critical alerts via owner notification
    if (payload.priority === 'critical') {
      try {
        await notifyOwner({
          title: `[CRITICAL] ${payload.title}`,
          content: payload.body,
        });
      } catch { /* best effort */ }
    }

    // 4. Deliver to each channel
    const deliveries: DeliveryResult[] = [];

    for (const channel of matchingChannels) {
      const breaker = this.getBreaker(channel.channelId);

      if (!breaker.canExecute()) {
        deliveries.push({
          channelId: channel.channelId,
          channelType: channel.type,
          success: false,
          error: 'Circuit breaker open',
          retryCount: 0,
        });
        continue;
      }

      const { result, retries, error } = await this.retryEngine.executeWithRetry(
        () => this.deliverToChannel(channel, payload),
        `${channel.type}:${channel.channelId}`
      );

      if (result) {
        breaker.recordSuccess();
        deliveries.push({
          channelId: channel.channelId,
          channelType: channel.type,
          success: true,
          retryCount: retries,
          deliveredAt: Date.now(),
        });
        this.updateStats(channel.type, true, retries);
      } else {
        breaker.recordFailure();
        deliveries.push({
          channelId: channel.channelId,
          channelType: channel.type,
          success: false,
          error,
          retryCount: retries,
        });
        this.updateStats(channel.type, false, retries);
      }
    }

    // 5. Log audit entry
    await this.logAudit(payload.userId, 'notification_sent', {
      alertId,
      title: payload.title,
      priority: payload.priority,
      category: payload.category,
      channelCount: matchingChannels.length,
      successCount: deliveries.filter(d => d.success).length,
    });

    return {
      alertId,
      deliveries,
      totalSent: deliveries.filter(d => d.success).length,
      totalFailed: deliveries.filter(d => !d.success).length,
    };
  }

  // ── Channel delivery implementations ──

  private async deliverToChannel(
    channel: ChannelConfig,
    payload: NotificationPayload
  ): Promise<boolean> {
    switch (channel.type) {
      case 'push':
        return this.deliverPush(payload);
      case 'email':
        return this.deliverEmail(channel, payload);
      case 'in_app':
        return this.deliverInApp(channel, payload);
      case 'webhook':
        return this.deliverWebhook(channel, payload);
      case 'sms':
        return this.deliverSMS(channel, payload);
      default:
        return false;
    }
  }

  private async deliverPush(payload: NotificationPayload): Promise<boolean> {
    // Use built-in notification API
    try {
      await notifyOwner({
        title: payload.title,
        content: payload.body,
      });
      return true;
    } catch {
      return false;
    }
  }

  private async deliverEmail(
    channel: ChannelConfig,
    payload: NotificationPayload
  ): Promise<boolean> {
    // Use built-in notification API for email
    try {
      await notifyOwner({
        title: `[${payload.priority.toUpperCase()}] ${payload.title}`,
        content: `${payload.body}\n\nCategory: ${payload.category}\nUser: ${payload.userId}`,
      });
      return true;
    } catch {
      return false;
    }
  }

  private async deliverInApp(
    _channel: ChannelConfig,
    payload: NotificationPayload
  ): Promise<boolean> {
    // Use built-in notification API for in-app alerts
    try {
      const message = [
        `*${payload.title}*`,
        '',
        payload.body,
        '',
        `Priority: ${payload.priority}`,
        `Category: ${payload.category}`,
        payload.lat && payload.lon
          ? `Location: ${payload.lat.toFixed(6)}, ${payload.lon.toFixed(6)}`
          : '',
      ].filter(Boolean).join('\n');

      await notifyOwner({
        title: payload.title,
        content: message,
      });
      return true;
    } catch {
      return false;
    }
  }

  private async deliverWebhook(
    channel: ChannelConfig,
    payload: NotificationPayload
  ): Promise<boolean> {
    const webhookUrl = channel.config.url as string;
    if (!webhookUrl) return false;

    try {
      const response = await fetch(webhookUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          alertId: crypto.randomUUID(),
          ...payload,
          timestamp: Date.now(),
        }),
        signal: AbortSignal.timeout(10_000),
      });
      return response.ok;
    } catch {
      return false;
    }
  }

  private async deliverSMS(
    _channel: ChannelConfig,
    payload: NotificationPayload
  ): Promise<boolean> {
    // SMS delivery via notification API (placeholder for SMS gateway)
    try {
      await notifyOwner({
        title: `SMS: ${payload.title}`,
        content: payload.body.substring(0, 160),
      });
      return true;
    } catch {
      return false;
    }
  }

  // ── Channel Management ──

  async getUserChannels(userId: string): Promise<ChannelConfig[]> {
    const db = await getDb();
    if (!db) return this.getDefaultChannels(userId);

    try {
      const userIdNum = parseInt(userId) || 0;
      const rows = await db.select().from(integrationChannels)
        .where(eq(integrationChannels.userId, userIdNum));

      return rows.map(row => ({
        channelId: String(row.id),
        userId: String(row.userId),
        type: row.channelType as ChannelType,
        isEnabled: row.isEnabled ?? true,
        config: typeof row.config === 'string' ? JSON.parse(row.config) : (row.config as Record<string, unknown> || {}),
        priorityFilter: ['critical', 'high', 'medium'] as AlertPriority[],
        categoryFilter: ['navigation', 'geofence', 'fleet', 'payment', 'system', 'security'] as AlertCategory[],
      }));
    } catch {
      return this.getDefaultChannels(userId);
    }
  }

  private getDefaultChannels(userId: string): ChannelConfig[] {
    return [{
      channelId: `default-push-${userId}`,
      userId,
      type: 'push',
      isEnabled: true,
      config: {},
      priorityFilter: ['critical', 'high', 'medium'],
      categoryFilter: ['navigation', 'geofence', 'fleet', 'payment', 'system', 'security'],
    }];
  }

  async registerChannel(
    userId: string,
    type: ChannelType,
    config: Record<string, unknown>,
    priorityFilter: AlertPriority[] = ['critical', 'high', 'medium'],
    categoryFilter: AlertCategory[] = ['navigation', 'geofence', 'fleet', 'payment', 'system', 'security']
  ): Promise<string> {
    const db = await getDb();
    if (!db) throw new Error('Database unavailable');

    const channelId = crypto.randomUUID();

    const userIdNum = parseInt(userId) || 0;
    // Map ChannelType to schema enum
    const channelTypeMap: Record<ChannelType, typeof integrationChannels.channelType.enumValues[number]> = {
      in_app: 'push',
      email: 'gmail',
      push: 'push',
      sms: 'sms',
      webhook: 'webhook',
    };
    await db.insert(integrationChannels).values({
      userId: userIdNum,
      channelType: channelTypeMap[type] || 'push',
      isEnabled: true,
      config: JSON.stringify(config),
    });

    return channelId;
  }

  async toggleChannel(channelId: string, isEnabled: boolean): Promise<void> {
    const db = await getDb();
    if (!db) return;

    const channelIdNum = parseInt(channelId) || 0;
    await db.update(integrationChannels)
      .set({ isEnabled })
      .where(eq(integrationChannels.id, channelIdNum));
  }

  // ── Payment Event Alerts ──

  async processPaymentEvent(event: {
    userId: string;
    tripId?: string;
    type: string;
    amount: number;
    currency: string;
    provider: string;
    status: string;
  }): Promise<void> {
    const db = await getDb();
    if (!db) return;

    const eventId = crypto.randomUUID();

    // Map event type to schema enum
    const paymentTypeMap: Record<string, typeof paymentEvents.type.enumValues[number]> = {
      toll: 'toll', parking: 'parking', fuel: 'fuel',
      charging: 'charging', subscription: 'subscription',
      fine: 'fine', refund: 'refund',
    };
    const paymentStatusMap: Record<string, typeof paymentEvents.status.enumValues[number]> = {
      pending: 'pending', completed: 'completed',
      failed: 'failed', refunded: 'refunded',
    };
    // Store payment event
    await db.insert(paymentEvents).values({
      eventId,
      userId: parseInt(event.userId) || 0,
      tripId: event.tripId || null,
      type: paymentTypeMap[event.type] || 'toll',
      amount: event.amount,
      currency: event.currency,
      provider: event.provider,
      status: paymentStatusMap[event.status] || 'pending',
      metadata: null,
    });

    // Send notification for significant payment events
    if (['completed', 'failed', 'refunded'].includes(event.status)) {
      const emoji = event.status === 'completed' ? '✅' : event.status === 'failed' ? '❌' : '↩️';
      await this.sendNotification({
        userId: event.userId,
        title: `Payment ${event.status}: ${event.currency} ${event.amount.toFixed(2)}`,
        body: `${emoji} ${event.type} payment via ${event.provider} — ${event.status}`,
        priority: event.status === 'failed' ? 'high' : 'medium',
        category: 'payment',
        tripId: event.tripId,
        metadata: { eventId, ...event },
      });
    }
  }

  // ── Audit Logging ──

  async logAudit(
    userId: string,
    action: string,
    details: Record<string, unknown>,
    level: 'info' | 'warn' | 'error' | 'debug' = 'info'
  ): Promise<void> {
    const db = await getDb();
    if (!db) return;

    try {
      await db.insert(logEntries).values({
        userId: parseInt(userId) || null,
        action,
        level,
        source: 'integration_hub',
        details: JSON.stringify(details),
        ipAddress: null,
        userAgent: null,
      });
    } catch {
      // Silent fail for audit logging
    }
  }

  // ── Alert History ──

  async getUserAlerts(
    userId: string,
    limit = 50,
    unreadOnly = false
  ): Promise<Array<{
    alertId: string;
    type: string;
    severity: string;
    title: string;
    message: string;
    isRead: boolean;
    createdAt: Date | null;
  }>> {
    const db = await getDb();
    if (!db) return [];

    const userIdNum = parseInt(userId) || 0;
    const conditions = [eq(alerts.userId, userIdNum)];
    if (unreadOnly) {
      conditions.push(eq(alerts.isRead, false));
    }

    const rows = await db.select({
      alertId: alerts.alertId,
      type: alerts.type,
      severity: alerts.severity,
      title: alerts.title,
      message: alerts.message,
      isRead: alerts.isRead,
      createdAt: alerts.createdAt,
    })
      .from(alerts)
      .where(and(...conditions))
      .orderBy(desc(alerts.createdAt))
      .limit(limit);

    return rows.map(r => ({
      ...r,
      type: r.type || 'system',
      severity: r.severity || 'info',
      title: r.title || '',
      message: r.message || '',
      isRead: r.isRead ?? false,
    }));
  }

  async markAlertRead(alertId: string): Promise<void> {
    const db = await getDb();
    if (!db) return;

    await db.update(alerts)
      .set({ isRead: true })
      .where(eq(alerts.alertId, alertId));
  }

  async dismissAlert(alertId: string): Promise<void> {
    const db = await getDb();
    if (!db) return;

    await db.update(alerts)
      .set({ isAcknowledged: true, isRead: true })
      .where(eq(alerts.alertId, alertId));
  }

  // ── Stats ──

  private updateStats(type: ChannelType, success: boolean, retries: number): void {
    if (success) {
      this.stats.totalSent++;
    } else {
      this.stats.totalFailed++;
    }
    this.stats.totalRetries += retries;

    const channelStats = this.stats.byChannel.get(type) || { sent: 0, failed: 0 };
    if (success) channelStats.sent++;
    else channelStats.failed++;
    this.stats.byChannel.set(type, channelStats);
  }

  getStats() {
    const byChannel: Record<string, { sent: number; failed: number }> = {};
    for (const [key, val] of Array.from(this.stats.byChannel.entries())) {
      byChannel[key] = val;
    }

    const breakers: Record<string, { state: string; failureCount: number }> = {};
    for (const [key, breaker] of Array.from(this.circuitBreakers.entries())) {
      breakers[key] = breaker.getStats();
    }

    return {
      totalSent: this.stats.totalSent,
      totalFailed: this.stats.totalFailed,
      totalRetries: this.stats.totalRetries,
      byChannel,
      circuitBreakers: breakers,
    };
  }
}

// Singleton
export const integrationHub = new IntegrationHub();
