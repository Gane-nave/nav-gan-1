/**
 * G.A.N.E — Notification Router
 * ===============================
 * In-app notification system with real-time delivery via WebSocket.
 *
 * Procedures:
 *   notifications.list         — Paginated list of user notifications
 *   notifications.getUnreadCount — Count of unread notifications
 *   notifications.markRead     — Mark a single notification as read
 *   notifications.markAllRead  — Mark all notifications as read
 *   notifications.delete       — Delete a single notification
 *   notifications.clearAll     — Delete all notifications for user
 *   notifications.getPreferences — Get notification preferences
 *   notifications.updatePreferences — Update notification preferences
 *   notifications.create       — (Admin) Create notification for user(s)
 *
 * Server-side helper:
 *   createNotification()       — Used by other modules to create notifications
 *   broadcastNotification()    — Push via WebSocket to connected user
 */
import { z } from "zod";
import { protectedProcedure, adminProcedure, router } from "../_core/trpc";
import { TRPCError } from "@trpc/server";
import { getDb } from "../db";
import { eq, and, desc, sql, lt, inArray } from "drizzle-orm";
import {
  userNotifications,
  notificationPreferences,
  type InsertUserNotification,
} from "../../drizzle/schema";
import { randomUUID } from "crypto";
import { pubsub } from "./redisPubSub";
import { notifyOwner } from "../_core/notification";
import { ENV } from "../_core/env";

// ─── Constants ───
const MAX_PAGE_SIZE = 50;
const DEFAULT_PAGE_SIZE = 20;

// ─── PubSub Channel Helper ───
export function notificationChannel(userId: number): string {
  return `notify:user:${userId}`;
}

// ─── Notification Types ───
export type NotificationType = "info" | "success" | "warning" | "error" | "system" | "collaboration" | "admin";

export interface CreateNotificationInput {
  userId: number;
  type: NotificationType;
  title: string;
  message: string;
  metadata?: Record<string, unknown>;
  expiresAt?: Date;
}

// ─── Server-Side Helper: Create & Broadcast Notification ───
export async function createNotification(input: CreateNotificationInput): Promise<string | null> {
  const db = await getDb();
  if (!db) {
    console.warn("[Notifications] Database not available");
    return null;
  }

  const notificationId = `notif_${randomUUID().replace(/-/g, "").slice(0, 16)}`;

  try {
    // Check user preferences before creating
    const prefs = await db
      .select()
      .from(notificationPreferences)
      .where(eq(notificationPreferences.userId, input.userId))
      .limit(1);

    if (prefs.length > 0) {
      const p = prefs[0];
      const typeKey = `enable${input.type.charAt(0).toUpperCase() + input.type.slice(1)}` as keyof typeof p;
      if (typeKey in p && p[typeKey] === false) {
        // User has disabled this notification type
        return null;
      }
    }

    await db.insert(userNotifications).values({
      notificationId,
      userId: input.userId,
      type: input.type,
      title: input.title,
      message: input.message,
      metadata: input.metadata ?? null,
      expiresAt: input.expiresAt ?? null,
    });

    // Broadcast via WebSocket/PubSub
    broadcastNotification(input.userId, {
      notificationId,
      type: input.type,
      title: input.title,
      message: input.message,
      metadata: input.metadata ?? null,
      createdAt: new Date().toISOString(),
    });

    // Email/Push channel: forward critical notifications to project owner via Manus Notification Service
    // This ensures the owner is notified even when offline (delivered as push/email by the platform)
    if (input.type === "error" || input.type === "system" || input.type === "admin") {
      try {
        // Check if this notification targets the owner
        const { users } = await import("../../drizzle/schema");
        const ownerRows = await db
          .select({ id: users.id, openId: users.openId })
          .from(users)
          .where(eq(users.openId, ENV.ownerOpenId))
          .limit(1);

        const isOwnerNotification = ownerRows.length > 0 && ownerRows[0].id === input.userId;
        if (isOwnerNotification) {
          // Fire-and-forget: don't block the main notification flow
          notifyOwner({
            title: `[${input.type.toUpperCase()}] ${input.title}`,
            content: input.message,
          }).catch(() => {
            // Silently fail — in-app notification already created
          });
        }
      } catch {
        // Owner lookup failed — continue without external notification
      }
    }

    return notificationId;
  } catch (err) {
    console.error("[Notifications] Failed to create notification:", err);
    return null;
  }
}

// ─── Server-Side Helper: Broadcast to WebSocket ───
export function broadcastNotification(userId: number, data: Record<string, unknown>): void {
  pubsub.publish(notificationChannel(userId), {
    type: "notification",
    ...data,
  });
}

// ─── Server-Side Helper: Create Notifications for Multiple Users ───
export async function createBulkNotifications(
  userIds: number[],
  type: NotificationType,
  title: string,
  message: string,
  metadata?: Record<string, unknown>,
): Promise<number> {
  let created = 0;
  for (const userId of userIds) {
    const id = await createNotification({ userId, type, title, message, metadata });
    if (id) created++;
  }
  return created;
}

// ─── tRPC Router ───
export const notificationRouter = router({
  /**
   * List notifications for the current user with pagination.
   */
  list: protectedProcedure
    .input(
      z.object({
        limit: z.number().min(1).max(MAX_PAGE_SIZE).default(DEFAULT_PAGE_SIZE).optional(),
        cursor: z.number().optional(), // Last notification ID for cursor pagination
        unreadOnly: z.boolean().default(false).optional(),
        type: z.enum(["info", "success", "warning", "error", "system", "collaboration", "admin"]).optional(),
      }).optional()
    )
    .query(async ({ ctx, input }) => {
      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      const limit = input?.limit ?? DEFAULT_PAGE_SIZE;
      const conditions = [eq(userNotifications.userId, ctx.user.id)];

      if (input?.unreadOnly) {
        conditions.push(eq(userNotifications.isRead, false));
      }

      if (input?.type) {
        conditions.push(eq(userNotifications.type, input.type));
      }

      if (input?.cursor) {
        conditions.push(lt(userNotifications.id, input.cursor));
      }

      const rows = await db
        .select()
        .from(userNotifications)
        .where(and(...conditions))
        .orderBy(desc(userNotifications.createdAt))
        .limit(limit + 1); // Fetch one extra to determine hasMore

      const hasMore = rows.length > limit;
      const items = hasMore ? rows.slice(0, limit) : rows;
      const nextCursor = hasMore ? items[items.length - 1].id : undefined;

      return {
        items,
        nextCursor,
        hasMore,
      };
    }),

  /**
   * Get count of unread notifications.
   */
  getUnreadCount: protectedProcedure.query(async ({ ctx }) => {
    const db = await getDb();
    if (!db) return { count: 0 };

    const result = await db
      .select({ count: sql<number>`COUNT(*)` })
      .from(userNotifications)
      .where(
        and(
          eq(userNotifications.userId, ctx.user.id),
          eq(userNotifications.isRead, false)
        )
      );

    return { count: Number(result[0]?.count ?? 0) };
  }),

  /**
   * Mark a single notification as read.
   */
  markRead: protectedProcedure
    .input(z.object({ notificationId: z.string() }))
    .mutation(async ({ ctx, input }) => {
      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      const result = await db
        .update(userNotifications)
        .set({ isRead: true, readAt: new Date() })
        .where(
          and(
            eq(userNotifications.notificationId, input.notificationId),
            eq(userNotifications.userId, ctx.user.id)
          )
        );

      return { success: true };
    }),

  /**
   * Mark all notifications as read.
   */
  markAllRead: protectedProcedure.mutation(async ({ ctx }) => {
    const db = await getDb();
    if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

    await db
      .update(userNotifications)
      .set({ isRead: true, readAt: new Date() })
      .where(
        and(
          eq(userNotifications.userId, ctx.user.id),
          eq(userNotifications.isRead, false)
        )
      );

    return { success: true };
  }),

  /**
   * Delete a single notification.
   */
  delete: protectedProcedure
    .input(z.object({ notificationId: z.string() }))
    .mutation(async ({ ctx, input }) => {
      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      await db
        .delete(userNotifications)
        .where(
          and(
            eq(userNotifications.notificationId, input.notificationId),
            eq(userNotifications.userId, ctx.user.id)
          )
        );

      return { success: true };
    }),

  /**
   * Clear all notifications for the current user.
   */
  clearAll: protectedProcedure.mutation(async ({ ctx }) => {
    const db = await getDb();
    if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

    await db
      .delete(userNotifications)
      .where(eq(userNotifications.userId, ctx.user.id));

    return { success: true };
  }),

  /**
   * Get notification preferences for the current user.
   */
  getPreferences: protectedProcedure.query(async ({ ctx }) => {
    const db = await getDb();
    if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

    const rows = await db
      .select()
      .from(notificationPreferences)
      .where(eq(notificationPreferences.userId, ctx.user.id))
      .limit(1);

    if (rows.length === 0) {
      // Return defaults
      return {
        enableInfo: true,
        enableSuccess: true,
        enableWarning: true,
        enableError: true,
        enableSystem: true,
        enableCollaboration: true,
        enableAdmin: true,
        enableSound: true,
        enableToast: true,
      };
    }

    const { id, userId, updatedAt, ...prefs } = rows[0];
    return prefs;
  }),

  /**
   * Update notification preferences.
   */
  updatePreferences: protectedProcedure
    .input(
      z.object({
        enableInfo: z.boolean().optional(),
        enableSuccess: z.boolean().optional(),
        enableWarning: z.boolean().optional(),
        enableError: z.boolean().optional(),
        enableSystem: z.boolean().optional(),
        enableCollaboration: z.boolean().optional(),
        enableAdmin: z.boolean().optional(),
        enableSound: z.boolean().optional(),
        enableToast: z.boolean().optional(),
      })
    )
    .mutation(async ({ ctx, input }) => {
      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      // Upsert: insert if not exists, update if exists
      const existing = await db
        .select()
        .from(notificationPreferences)
        .where(eq(notificationPreferences.userId, ctx.user.id))
        .limit(1);

      if (existing.length === 0) {
        await db.insert(notificationPreferences).values({
          userId: ctx.user.id,
          ...input,
        });
      } else {
        await db
          .update(notificationPreferences)
          .set(input)
          .where(eq(notificationPreferences.userId, ctx.user.id));
      }

      return { success: true };
    }),

  /**
   * (Admin) Create a notification for specific user(s) or all users.
   */
  adminCreate: adminProcedure
    .input(
      z.object({
        type: z.enum(["info", "success", "warning", "error", "system", "collaboration", "admin"]),
        title: z.string().min(1).max(256),
        message: z.string().min(1).max(2000),
        targetUserIds: z.array(z.number()).optional(), // If empty/undefined, targets all users
        metadata: z.record(z.string(), z.unknown()).optional(),
      })
    )
    .mutation(async ({ ctx, input }) => {
      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      if (input.targetUserIds && input.targetUserIds.length > 0) {
        // Target specific users
        const created = await createBulkNotifications(
          input.targetUserIds,
          input.type,
          input.title,
          input.message,
          input.metadata,
        );
        return { success: true, created };
      } else {
        // Target all users — fetch all user IDs
        const { users } = await import("../../drizzle/schema");
        const allUsers = await db.select({ id: users.id }).from(users);
        const created = await createBulkNotifications(
          allUsers.map(u => u.id),
          input.type,
          input.title,
          input.message,
          input.metadata,
        );
        return { success: true, created };
      }
    }),
});
