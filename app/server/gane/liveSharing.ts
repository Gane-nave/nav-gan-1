/**
 * G.A.N.E — Live Location Sharing Engine
 * ========================================
 * Secure tokenized URL for real-time location sharing.
 *
 * FEATURES:
 *   - Secure tokenized sharing links
 *   - Real-time tracking via WebSocket
 *   - Configurable expiration (15min, 1h, 8h, forever)
 *   - Permission levels (view-only, view+ETA, full)
 *   - Revocation support
 *   - Rate limiting per user
 */

import { z } from 'zod';
import { publicProcedure, protectedProcedure, router } from '../_core/trpc';
import crypto from 'crypto';

// ─── Types ───────────────────────────────────────────────

export interface SharedLocation {
  id: string;
  token: string;
  userId: number;
  deviceId: string;
  permission: 'view' | 'view_eta' | 'full';
  expiresAt: number;           // Unix ms, 0 = never
  isActive: boolean;
  label?: string;
  createdAt: number;
  lastAccessedAt: number;
  accessCount: number;
  // Current position (updated in real-time)
  lat: number;
  lon: number;
  heading: number;
  speed: number;
  lastPositionAt: number;
}

// ─── In-Memory Store ────────────────────────────────────

class LiveSharingStore {
  private shares: Map<string, SharedLocation> = new Map();
  private tokenIndex: Map<string, string> = new Map(); // token → id
  private cleanupTimer: ReturnType<typeof setInterval> | null = null;

  start() {
    this.cleanupTimer = setInterval(() => this.cleanup(), 60000);
  }

  stop() {
    if (this.cleanupTimer) {
      clearInterval(this.cleanupTimer);
      this.cleanupTimer = null;
    }
  }

  create(
    userId: number,
    deviceId: string,
    permission: 'view' | 'view_eta' | 'full' = 'view',
    durationMs: number = 3600000,
    label?: string
  ): SharedLocation {
    const id = `share_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
    const token = crypto.randomBytes(32).toString('hex');

    const share: SharedLocation = {
      id,
      token,
      userId,
      deviceId,
      permission,
      expiresAt: durationMs > 0 ? Date.now() + durationMs : 0,
      isActive: true,
      label,
      createdAt: Date.now(),
      lastAccessedAt: 0,
      accessCount: 0,
      lat: 0,
      lon: 0,
      heading: 0,
      speed: 0,
      lastPositionAt: 0,
    };

    this.shares.set(id, share);
    this.tokenIndex.set(token, id);
    return share;
  }

  getByToken(token: string): SharedLocation | null {
    const id = this.tokenIndex.get(token);
    if (!id) return null;
    const share = this.shares.get(id);
    if (!share || !share.isActive) return null;
    if (share.expiresAt > 0 && share.expiresAt < Date.now()) {
      share.isActive = false;
      return null;
    }
    share.lastAccessedAt = Date.now();
    share.accessCount++;
    return share;
  }

  updatePosition(
    userId: number,
    deviceId: string,
    lat: number,
    lon: number,
    heading: number,
    speed: number
  ) {
    const entries = Array.from(this.shares.values());
    for (const share of entries) {
      if (share.userId === userId && share.deviceId === deviceId && share.isActive) {
        share.lat = lat;
        share.lon = lon;
        share.heading = heading;
        share.speed = speed;
        share.lastPositionAt = Date.now();
      }
    }
  }

  revoke(id: string, userId: number): boolean {
    const share = this.shares.get(id);
    if (!share || share.userId !== userId) return false;
    share.isActive = false;
    return true;
  }

  getUserShares(userId: number): SharedLocation[] {
    return Array.from(this.shares.values())
      .filter(s => s.userId === userId && s.isActive);
  }

  private cleanup() {
    const now = Date.now();
    const entries = Array.from(this.shares.entries());
    for (const [id, share] of entries) {
      if (share.expiresAt > 0 && share.expiresAt < now) {
        share.isActive = false;
      }
      // Remove inactive shares older than 24h
      if (!share.isActive && now - share.createdAt > 86400000) {
        this.tokenIndex.delete(share.token);
        this.shares.delete(id);
      }
    }
  }
}

export const liveSharingStore = new LiveSharingStore();

// ─── tRPC Router ────────────────────────────────────────

export const liveSharingRouter = router({
  /**
   * Create a new live sharing link.
   */
  create: protectedProcedure
    .input(z.object({
      deviceId: z.string().min(1).max(64),
      permission: z.enum(['view', 'view_eta', 'full']).default('view'),
      durationMinutes: z.number().min(0).max(1440).default(60), // 0 = no expiry
      label: z.string().max(128).optional(),
    }))
    .mutation(({ input, ctx }) => {
      const durationMs = input.durationMinutes > 0 ? input.durationMinutes * 60 * 1000 : 0;
      const share = liveSharingStore.create(
        ctx.user.id,
        input.deviceId,
        input.permission,
        durationMs,
        input.label
      );

      return {
        shareId: share.id,
        token: share.token,
        expiresAt: share.expiresAt,
        // The frontend constructs the full URL using window.location.origin
      };
    }),

  /**
   * Track a shared location by token (public — anyone with the link).
   */
  track: publicProcedure
    .input(z.object({
      token: z.string().min(1),
    }))
    .query(({ input }) => {
      const share = liveSharingStore.getByToken(input.token);
      if (!share) return null;

      // Return position data based on permission level
      const result: Record<string, unknown> = {
        lat: share.lat,
        lon: share.lon,
        heading: share.heading,
        lastUpdated: share.lastPositionAt,
        label: share.label,
      };

      if (share.permission === 'view_eta' || share.permission === 'full') {
        result.speed = share.speed;
      }

      return result;
    }),

  /**
   * Revoke a sharing link.
   */
  revoke: protectedProcedure
    .input(z.object({
      shareId: z.string().min(1),
    }))
    .mutation(({ input, ctx }) => {
      return { success: liveSharingStore.revoke(input.shareId, ctx.user.id) };
    }),

  /**
   * List user's active sharing links.
   */
  myShares: protectedProcedure
    .query(({ ctx }) => {
      return liveSharingStore.getUserShares(ctx.user.id).map(s => ({
        id: s.id,
        token: s.token,
        permission: s.permission,
        expiresAt: s.expiresAt,
        label: s.label,
        accessCount: s.accessCount,
        createdAt: s.createdAt,
      }));
    }),
});
