/**
 * G.A.N.E — Real-Time Collaboration Router (Scalability-Hardened)
 * =================================================================
 * Multi-user map editing with production-grade scalability:
 * - SSE-based event streaming with connection limits + cleanup
 * - Rate limiting on all procedures (token bucket)
 * - Cursor update throttling (server-side 5Hz dedup)
 * - Session capacity enforcement (maxParticipants)
 * - Pagination on all list queries
 * - Presence system with colored cursors
 * - Shared markers, annotations, and drawings
 * - Activity feed with bounded history
 * - Conflict resolution (last-write-wins with optimistic updates)
 */
import { z } from "zod";
import { protectedProcedure, publicProcedure, router } from "../_core/trpc";
import { TRPCError } from "@trpc/server";
import { getDb } from "../db";
import { eq, and, desc, sql, lt } from "drizzle-orm";
import {
  collaborationSessions,
  collaborationParticipants,
  sharedMarkers,
  sharedAnnotations,
  collaborationEvents,
  collaborationInvites,
} from "../../drizzle/schema";
import { randomUUID } from "crypto";
import { pubsub, sessionChannel } from "./redisPubSub";
import { createNotification } from "./notificationRouter";
import {
  enforceRateLimit,
  RATE_LIMITS,
  shouldThrottleCursor,
  canOpenSSEConnection,
  registerSSEConnection,
  unregisterSSEConnection,
  updateSSEActivity,
  getSSEStats,
  getRateLimiterStats,
  PAGINATION,
  MAX_PARTICIPANTS_DEFAULT,
  MAX_PARTICIPANTS_HARD_LIMIT,
} from "./rateLimiter";

// ─── Event Hub ───
// Uses Redis Pub/Sub for multi-server broadcasting.
// Falls back to in-memory EventEmitter when Redis is unavailable.
// Initialize Redis connection on module load (async, non-blocking).
pubsub.connect(process.env.REDIS_URL).catch(() => {
  console.log("[Collab] Redis not available — using in-memory EventEmitter fallback");
});

// Participant colors — assigned round-robin on join
const PARTICIPANT_COLORS = [
  "#FF5733", "#33FF57", "#3357FF", "#FF33F5", "#F5FF33",
  "#33FFF5", "#FF8C33", "#8C33FF", "#33FF8C", "#FF3333",
  "#33FFCC", "#CC33FF", "#FFCC33", "#3399FF", "#FF3399",
  "#99FF33", "#6633FF", "#FF6633", "#33FF66", "#FF33CC",
];

// ─── Helper: Generate unique ID ───
function genId(prefix: string) {
  return `${prefix}_${randomUUID().replace(/-/g, "").slice(0, 16)}`;
}

// ─── Helper: Broadcast event to session ───
function broadcastToSession(sid: string, event: CollabEvent) {
  pubsub.publish(sessionChannel(sid), event);
}

// ─── Event Types ───
export type CollabEvent = {
  type: string;
  sessionId: string;
  userId: number;
  userName?: string;
  userColor?: string;
  payload: Record<string, unknown>;
  timestamp: number;
};

// ─── Pagination Schema ───
const paginationInput = z.object({
  cursor: z.number().min(0).optional(), // last seen ID
  limit: z.number().min(1).max(PAGINATION.MAX_PAGE_SIZE).optional(),
});

// ═══════════════════════════════════════════════════
// COLLABORATION ROUTER
// ═══════════════════════════════════════════════════
export const collaborationRouter = router({
  // ─── Create Session ───
  createSession: protectedProcedure
    .input(z.object({
      name: z.string().min(1).max(256),
      centerLat: z.number().optional(),
      centerLon: z.number().optional(),
      zoomLevel: z.number().min(1).max(22).optional(),
      maxParticipants: z.number().min(2).max(MAX_PARTICIPANTS_HARD_LIMIT).optional(),
    }))
    .mutation(async ({ input, ctx }) => {
      // Rate limit: session creation is expensive
      enforceRateLimit(ctx.user.id, "createSession", RATE_LIMITS.create);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      const sessionId = genId("collab");
      await db.insert(collaborationSessions).values({
        sessionId,
        name: input.name,
        createdBy: ctx.user.id,
        centerLat: input.centerLat ?? 32.0853,
        centerLon: input.centerLon ?? 34.7818,
        zoomLevel: input.zoomLevel ?? 12,
        maxParticipants: input.maxParticipants ?? MAX_PARTICIPANTS_DEFAULT,
      });

      // Log creation event
      await db.insert(collaborationEvents).values({
        sessionId,
        userId: ctx.user.id,
        type: "session_created",
        payload: { name: input.name },
      });

      return { sessionId, name: input.name };
    }),

  // ─── List Active Sessions (Paginated) ───
  listSessions: protectedProcedure
    .input(paginationInput.optional())
    .query(async ({ ctx, input }) => {
      enforceRateLimit(ctx.user.id, "listSessions", RATE_LIMITS.query);

      const db = await getDb();
      if (!db) return { sessions: [], nextCursor: null };

      const limit = input?.limit ?? PAGINATION.SESSIONS_DEFAULT;
      const cursor = input?.cursor;

      let query = db
        .select()
        .from(collaborationSessions)
        .where(eq(collaborationSessions.isActive, true))
        .orderBy(desc(collaborationSessions.updatedAt))
        .limit(limit + 1); // Fetch one extra to detect next page

      if (cursor) {
        query = db
          .select()
          .from(collaborationSessions)
          .where(
            and(
              eq(collaborationSessions.isActive, true),
              lt(collaborationSessions.id, cursor)
            )
          )
          .orderBy(desc(collaborationSessions.updatedAt))
          .limit(limit + 1);
      }

      const sessions = await query;
      const hasMore = sessions.length > limit;
      const page = hasMore ? sessions.slice(0, limit) : sessions;

      // Get participant counts efficiently with a single query per session
      // (Use COUNT instead of fetching all participants)
      const results = await Promise.all(
        page.map(async (session) => {
          const countResult = await db
            .select({ count: sql<number>`COUNT(*)` })
            .from(collaborationParticipants)
            .where(
              and(
                eq(collaborationParticipants.sessionId, session.sessionId),
                eq(collaborationParticipants.isOnline, true)
              )
            );

          // Only fetch participant details (limited to 5 for preview)
          const participants = await db
            .select({
              userId: collaborationParticipants.userId,
              displayName: collaborationParticipants.displayName,
              color: collaborationParticipants.color,
            })
            .from(collaborationParticipants)
            .where(
              and(
                eq(collaborationParticipants.sessionId, session.sessionId),
                eq(collaborationParticipants.isOnline, true)
              )
            )
            .limit(5);

          return {
            ...session,
            participantCount: countResult[0]?.count ?? 0,
            participants,
          };
        })
      );

      return {
        sessions: results,
        nextCursor: hasMore ? page[page.length - 1].id : null,
      };
    }),

  // ─── Join Session (with capacity check) ───
  joinSession: protectedProcedure
    .input(z.object({ sessionId: z.string() }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "joinSession", RATE_LIMITS.mutation);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      // Check session exists and is active
      const [session] = await db
        .select()
        .from(collaborationSessions)
        .where(eq(collaborationSessions.sessionId, input.sessionId));

      if (!session || !session.isActive) {
        throw new TRPCError({ code: "NOT_FOUND", message: "Session not found or inactive" });
      }

      // Check if already a participant (re-join is always allowed)
      const [existing] = await db
        .select()
        .from(collaborationParticipants)
        .where(
          and(
            eq(collaborationParticipants.sessionId, input.sessionId),
            eq(collaborationParticipants.userId, ctx.user.id)
          )
        );

      // ─── CAPACITY CHECK (only for new participants) ───
      if (!existing) {
        const countResult = await db
          .select({ count: sql<number>`COUNT(*)` })
          .from(collaborationParticipants)
          .where(
            and(
              eq(collaborationParticipants.sessionId, input.sessionId),
              eq(collaborationParticipants.isOnline, true)
            )
          );

        const currentCount = countResult[0]?.count ?? 0;
        const maxParticipants = session.maxParticipants ?? MAX_PARTICIPANTS_DEFAULT;

        if (currentCount >= maxParticipants) {
          throw new TRPCError({
            code: "PRECONDITION_FAILED",
            message: `Session is at capacity (${maxParticipants} participants). Try again later.`,
          });
        }
      }

      // Assign a color
      const onlineParticipants = await db
        .select({ color: collaborationParticipants.color })
        .from(collaborationParticipants)
        .where(
          and(
            eq(collaborationParticipants.sessionId, input.sessionId),
            eq(collaborationParticipants.isOnline, true)
          )
        );

      const usedColors = new Set(onlineParticipants.map((p) => p.color));
      const color = PARTICIPANT_COLORS.find((c) => !usedColors.has(c)) || PARTICIPANT_COLORS[onlineParticipants.length % PARTICIPANT_COLORS.length];

      if (existing) {
        // Re-join: update status
        await db
          .update(collaborationParticipants)
          .set({ isOnline: true, color, lastHeartbeat: new Date() })
          .where(eq(collaborationParticipants.id, existing.id));
      } else {
        // New participant
        await db.insert(collaborationParticipants).values({
          sessionId: input.sessionId,
          userId: ctx.user.id,
          displayName: ctx.user.name || `User ${ctx.user.id}`,
          color,
          isOnline: true,
        });
      }

      // Log join event
      await db.insert(collaborationEvents).values({
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: "user_joined",
        payload: { displayName: ctx.user.name, color },
      });

      // Broadcast join to other participants
      broadcastToSession(input.sessionId, {
        type: "user_joined",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        userName: ctx.user.name || undefined,
        userColor: color,
        payload: { displayName: ctx.user.name, color },
        timestamp: Date.now(),
      });

      // Notify session creator about new participant (if not self)
      if (session.createdBy !== ctx.user.id) {
        createNotification({
          userId: session.createdBy,
          type: "collaboration",
          title: "משתתף חדש הצטרף",
          message: `${ctx.user.name || "User"} הצטרף לסשן שיתוף ${session.name}`,
          metadata: { sessionId: input.sessionId, sourceUserId: ctx.user.id },
        }).catch(() => {});
      }

      return { sessionId: input.sessionId, color, session };
    }),

  // ─── Leave Session ───
  leaveSession: protectedProcedure
    .input(z.object({ sessionId: z.string() }))
    .mutation(async ({ input, ctx }) => {
      const db = await getDb();
      if (!db) return { success: true };

      await db
        .update(collaborationParticipants)
        .set({ isOnline: false })
        .where(
          and(
            eq(collaborationParticipants.sessionId, input.sessionId),
            eq(collaborationParticipants.userId, ctx.user.id)
          )
        );

      await db.insert(collaborationEvents).values({
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: "user_left",
        payload: {},
      });

      broadcastToSession(input.sessionId, {
        type: "user_left",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        payload: {},
        timestamp: Date.now(),
      });

      return { success: true };
    }),

  // ─── Update Cursor Position (Throttled) ───
  updateCursor: protectedProcedure
    .input(z.object({
      sessionId: z.string(),
      lat: z.number(),
      lon: z.number(),
    }))
    .mutation(async ({ input, ctx }) => {
      // Server-side cursor throttling: max 5Hz per user per session
      if (shouldThrottleCursor(ctx.user.id, input.sessionId)) {
        return { success: true, throttled: true };
      }

      enforceRateLimit(ctx.user.id, "updateCursor", RATE_LIMITS.realtime);

      const db = await getDb();
      if (!db) return { success: true };

      // Update cursor in DB (for persistence)
      await db
        .update(collaborationParticipants)
        .set({
          cursorLat: input.lat,
          cursorLon: input.lon,
          lastHeartbeat: new Date(),
        })
        .where(
          and(
            eq(collaborationParticipants.sessionId, input.sessionId),
            eq(collaborationParticipants.userId, ctx.user.id)
          )
        );

      // Broadcast cursor update (no DB log for cursor moves — too frequent)
      broadcastToSession(input.sessionId, {
        type: "cursor_moved",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        payload: { lat: input.lat, lon: input.lon },
        timestamp: Date.now(),
      });

      return { success: true, throttled: false };
    }),

  // ─── Heartbeat ───
  heartbeat: protectedProcedure
    .input(z.object({ sessionId: z.string() }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "heartbeat", RATE_LIMITS.realtime);

      const db = await getDb();
      if (!db) return { success: true };

      await db
        .update(collaborationParticipants)
        .set({ lastHeartbeat: new Date(), isOnline: true })
        .where(
          and(
            eq(collaborationParticipants.sessionId, input.sessionId),
            eq(collaborationParticipants.userId, ctx.user.id)
          )
        );

      return { success: true };
    }),

  // ─── Add Shared Marker ───
  addMarker: protectedProcedure
    .input(z.object({
      sessionId: z.string(),
      lat: z.number(),
      lon: z.number(),
      label: z.string().max(256).optional(),
      description: z.string().max(2000).optional(),
      icon: z.string().max(64).optional(),
      color: z.string().max(7).optional(),
    }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "addMarker", RATE_LIMITS.mutation);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      const markerId = genId("marker");
      await db.insert(sharedMarkers).values({
        markerId,
        sessionId: input.sessionId,
        userId: ctx.user.id,
        lat: input.lat,
        lon: input.lon,
        label: input.label,
        description: input.description,
        icon: input.icon || "pin",
        color: input.color || "#00e5ff",
      });

      await db.insert(collaborationEvents).values({
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: "marker_added",
        payload: { markerId, lat: input.lat, lon: input.lon, label: input.label },
      });

      broadcastToSession(input.sessionId, {
        type: "marker_added",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        payload: {
          markerId,
          lat: input.lat,
          lon: input.lon,
          label: input.label,
          description: input.description,
          icon: input.icon || "pin",
          color: input.color || "#00e5ff",
        },
        timestamp: Date.now(),
      });

      // Notify other participants about new marker
      const participants = await db
        .select({ userId: collaborationParticipants.userId })
        .from(collaborationParticipants)
        .where(
          and(
            eq(collaborationParticipants.sessionId, input.sessionId),
            eq(collaborationParticipants.isOnline, true)
          )
        );
      for (const p of participants) {
        if (p.userId !== ctx.user.id) {
          createNotification({
            userId: p.userId,
            type: "collaboration",
            title: "סמן חדש נוסף",
            message: `${ctx.user.name || "User"} הוסיף סמן: ${input.label || "ללא שם"}`,
            metadata: { sessionId: input.sessionId, markerId, sourceUserId: ctx.user.id },
          }).catch(() => {});
        }
      }

      return { markerId };
    }),

  // ─── Move Shared Marker ───
  moveMarker: protectedProcedure
    .input(z.object({
      markerId: z.string(),
      sessionId: z.string(),
      lat: z.number(),
      lon: z.number(),
    }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "moveMarker", RATE_LIMITS.mutation);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      await db
        .update(sharedMarkers)
        .set({ lat: input.lat, lon: input.lon })
        .where(eq(sharedMarkers.markerId, input.markerId));

      await db.insert(collaborationEvents).values({
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: "marker_moved",
        payload: { markerId: input.markerId, lat: input.lat, lon: input.lon },
      });

      broadcastToSession(input.sessionId, {
        type: "marker_moved",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        payload: { markerId: input.markerId, lat: input.lat, lon: input.lon },
        timestamp: Date.now(),
      });

      return { success: true };
    }),

  // ─── Delete Shared Marker ───
  deleteMarker: protectedProcedure
    .input(z.object({
      markerId: z.string(),
      sessionId: z.string(),
    }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "deleteMarker", RATE_LIMITS.mutation);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      await db
        .update(sharedMarkers)
        .set({ isActive: false })
        .where(eq(sharedMarkers.markerId, input.markerId));

      await db.insert(collaborationEvents).values({
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: "marker_deleted",
        payload: { markerId: input.markerId },
      });

      broadcastToSession(input.sessionId, {
        type: "marker_deleted",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        payload: { markerId: input.markerId },
        timestamp: Date.now(),
      });

      return { success: true };
    }),

  // ─── Get Session Markers (Paginated) ───
  getMarkers: protectedProcedure
    .input(z.object({
      sessionId: z.string(),
      cursor: z.number().min(0).optional(),
      limit: z.number().min(1).max(PAGINATION.MAX_PAGE_SIZE).optional(),
    }))
    .query(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "getMarkers", RATE_LIMITS.query);

      const db = await getDb();
      if (!db) return { markers: [], nextCursor: null };

      const limit = input.limit ?? PAGINATION.MARKERS_DEFAULT;

      const conditions = [
        eq(sharedMarkers.sessionId, input.sessionId),
        eq(sharedMarkers.isActive, true),
      ];

      if (input.cursor) {
        conditions.push(lt(sharedMarkers.id, input.cursor));
      }

      const markers = await db
        .select()
        .from(sharedMarkers)
        .where(and(...conditions))
        .orderBy(desc(sharedMarkers.createdAt))
        .limit(limit + 1);

      const hasMore = markers.length > limit;
      const page = hasMore ? markers.slice(0, limit) : markers;

      return {
        markers: page,
        nextCursor: hasMore ? page[page.length - 1].id : null,
      };
    }),

  // ─── Add Annotation ───
  addAnnotation: protectedProcedure
    .input(z.object({
      sessionId: z.string(),
      type: z.enum(["text", "route", "area", "measurement", "arrow"]),
      data: z.record(z.string(), z.unknown()),
      color: z.string().max(7).optional(),
    }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "addAnnotation", RATE_LIMITS.mutation);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      const annotationId = genId("annot");
      await db.insert(sharedAnnotations).values({
        annotationId,
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: input.type,
        data: input.data,
        color: input.color || "#00e5ff",
      });

      await db.insert(collaborationEvents).values({
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: "annotation_added",
        payload: { annotationId, type: input.type },
      });

      broadcastToSession(input.sessionId, {
        type: "annotation_added",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        payload: {
          annotationId,
          type: input.type,
          data: input.data,
          color: input.color || "#00e5ff",
        },
        timestamp: Date.now(),
      });

      return { annotationId };
    }),

  // ─── Delete Annotation ───
  deleteAnnotation: protectedProcedure
    .input(z.object({
      annotationId: z.string(),
      sessionId: z.string(),
    }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "deleteAnnotation", RATE_LIMITS.mutation);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      await db
        .update(sharedAnnotations)
        .set({ isActive: false })
        .where(eq(sharedAnnotations.annotationId, input.annotationId));

      await db.insert(collaborationEvents).values({
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: "annotation_deleted",
        payload: { annotationId: input.annotationId },
      });

      broadcastToSession(input.sessionId, {
        type: "annotation_deleted",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        payload: { annotationId: input.annotationId },
        timestamp: Date.now(),
      });

      return { success: true };
    }),

  // ─── Get Session Annotations (Paginated) ───
  getAnnotations: protectedProcedure
    .input(z.object({
      sessionId: z.string(),
      cursor: z.number().min(0).optional(),
      limit: z.number().min(1).max(PAGINATION.MAX_PAGE_SIZE).optional(),
    }))
    .query(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "getAnnotations", RATE_LIMITS.query);

      const db = await getDb();
      if (!db) return { annotations: [], nextCursor: null };

      const limit = input.limit ?? PAGINATION.ANNOTATIONS_DEFAULT;
      const conditions = [
        eq(sharedAnnotations.sessionId, input.sessionId),
        eq(sharedAnnotations.isActive, true),
      ];

      if (input.cursor) {
        conditions.push(lt(sharedAnnotations.id, input.cursor));
      }

      const annotations = await db
        .select()
        .from(sharedAnnotations)
        .where(and(...conditions))
        .orderBy(desc(sharedAnnotations.createdAt))
        .limit(limit + 1);

      const hasMore = annotations.length > limit;
      const page = hasMore ? annotations.slice(0, limit) : annotations;

      return {
        annotations: page,
        nextCursor: hasMore ? page[page.length - 1].id : null,
      };
    }),

  // ─── Get Session Participants ───
  getParticipants: protectedProcedure
    .input(z.object({
      sessionId: z.string(),
      limit: z.number().min(1).max(PAGINATION.MAX_PAGE_SIZE).optional(),
    }))
    .query(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "getParticipants", RATE_LIMITS.query);

      const db = await getDb();
      if (!db) return [];

      return db
        .select()
        .from(collaborationParticipants)
        .where(
          and(
            eq(collaborationParticipants.sessionId, input.sessionId),
            eq(collaborationParticipants.isOnline, true)
          )
        )
        .limit(input.limit ?? PAGINATION.PARTICIPANTS_DEFAULT);
    }),

  // ─── Get Activity Feed (Paginated) ───
  getActivityFeed: protectedProcedure
    .input(z.object({
      sessionId: z.string(),
      cursor: z.number().min(0).optional(),
      limit: z.number().min(1).max(PAGINATION.MAX_PAGE_SIZE).optional(),
    }))
    .query(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "getActivityFeed", RATE_LIMITS.query);

      const db = await getDb();
      if (!db) return { events: [], nextCursor: null };

      const limit = input.limit ?? PAGINATION.EVENTS_DEFAULT;
      const conditions = [eq(collaborationEvents.sessionId, input.sessionId)];

      if (input.cursor) {
        conditions.push(lt(collaborationEvents.id, input.cursor));
      }

      const events = await db
        .select()
        .from(collaborationEvents)
        .where(and(...conditions))
        .orderBy(desc(collaborationEvents.createdAt))
        .limit(limit + 1);

      const hasMore = events.length > limit;
      const page = hasMore ? events.slice(0, limit) : events;

      return {
        events: page,
        nextCursor: hasMore ? page[page.length - 1].id : null,
      };
    }),

  // ─── Generate Invite Link ───
  generateInvite: protectedProcedure
    .input(z.object({
      sessionId: z.string(),
      maxUses: z.number().min(0).max(1000).optional(), // 0 = unlimited
      expiresInHours: z.number().min(1).max(168).optional(), // default 24h, max 7 days
    }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "generateInvite", RATE_LIMITS.invite);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      // Verify session exists and is active
      const [session] = await db
        .select()
        .from(collaborationSessions)
        .where(eq(collaborationSessions.sessionId, input.sessionId));

      if (!session || !session.isActive) {
        throw new TRPCError({ code: "NOT_FOUND", message: "Session not found or inactive" });
      }

      const inviteToken = randomUUID().replace(/-/g, "").slice(0, 24);
      const expiresAt = new Date(Date.now() + (input.expiresInHours ?? 24) * 60 * 60 * 1000);

      await db.insert(collaborationInvites).values({
        inviteToken,
        sessionId: input.sessionId,
        createdBy: ctx.user.id,
        maxUses: input.maxUses ?? 0,
        expiresAt,
      });

      return { inviteToken, expiresAt: expiresAt.toISOString() };
    }),

  // ─── Validate Invite Token ───
  validateInvite: publicProcedure
    .input(z.object({ token: z.string() }))
    .query(async ({ input }) => {
      const db = await getDb();
      if (!db) return { valid: false, sessionId: null, sessionName: null };

      const [invite] = await db
        .select()
        .from(collaborationInvites)
        .where(eq(collaborationInvites.inviteToken, input.token));

      if (!invite || !invite.isActive) {
        return { valid: false, sessionId: null, sessionName: null };
      }

      // Check expiry
      if (new Date() > invite.expiresAt) {
        return { valid: false, sessionId: null, sessionName: null };
      }

      // Check usage limit
      if (invite.maxUses && invite.maxUses > 0 && invite.usedCount >= invite.maxUses) {
        return { valid: false, sessionId: null, sessionName: null };
      }

      // Get session info
      const [session] = await db
        .select()
        .from(collaborationSessions)
        .where(eq(collaborationSessions.sessionId, invite.sessionId));

      if (!session || !session.isActive) {
        return { valid: false, sessionId: null, sessionName: null };
      }

      return {
        valid: true,
        sessionId: invite.sessionId,
        sessionName: session.name,
      };
    }),

  // ─── Redeem Invite Token ───
  redeemInvite: protectedProcedure
    .input(z.object({ token: z.string() }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "redeemInvite", RATE_LIMITS.mutation);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      const [invite] = await db
        .select()
        .from(collaborationInvites)
        .where(eq(collaborationInvites.inviteToken, input.token));

      if (!invite || !invite.isActive) {
        throw new TRPCError({ code: "NOT_FOUND", message: "Invalid invite link" });
      }

      if (new Date() > invite.expiresAt) {
        throw new TRPCError({ code: "BAD_REQUEST", message: "Invite link has expired" });
      }

      if (invite.maxUses && invite.maxUses > 0 && invite.usedCount >= invite.maxUses) {
        throw new TRPCError({ code: "BAD_REQUEST", message: "Invite link has reached its usage limit" });
      }

      // Increment usage count
      await db
        .update(collaborationInvites)
        .set({ usedCount: (invite.usedCount ?? 0) + 1 })
        .where(eq(collaborationInvites.id, invite.id));

      return { sessionId: invite.sessionId };
    }),

  // ─── End Session ───
  endSession: protectedProcedure
    .input(z.object({ sessionId: z.string() }))
    .mutation(async ({ input, ctx }) => {
      enforceRateLimit(ctx.user.id, "endSession", RATE_LIMITS.mutation);

      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "Database unavailable" });

      // Only creator can end session
      const [session] = await db
        .select()
        .from(collaborationSessions)
        .where(eq(collaborationSessions.sessionId, input.sessionId));

      if (!session) {
        throw new TRPCError({ code: "NOT_FOUND", message: "Session not found" });
      }

      if (session.createdBy !== ctx.user.id && ctx.user.role !== "admin") {
        throw new TRPCError({ code: "FORBIDDEN", message: "Only the creator or admin can end a session" });
      }

      await db
        .update(collaborationSessions)
        .set({ isActive: false })
        .where(eq(collaborationSessions.sessionId, input.sessionId));

      // Set all participants offline
      await db
        .update(collaborationParticipants)
        .set({ isOnline: false })
        .where(eq(collaborationParticipants.sessionId, input.sessionId));

      await db.insert(collaborationEvents).values({
        sessionId: input.sessionId,
        userId: ctx.user.id,
        type: "session_ended",
        payload: {},
      });

      broadcastToSession(input.sessionId, {
        type: "session_ended",
        sessionId: input.sessionId,
        userId: ctx.user.id,
        payload: {},
        timestamp: Date.now(),
      });

      return { success: true };
    }),

  // ─── Scalability Stats (Admin Only) ───
  getScalabilityStats: protectedProcedure
    .query(async ({ ctx }) => {
      if (ctx.user.role !== "admin") {
        throw new TRPCError({ code: "FORBIDDEN", message: "Admin access required" });
      }

      const db = await getDb();
      const dbStats = db ? {
        activeSessions: (await db
          .select({ count: sql<number>`COUNT(*)` })
          .from(collaborationSessions)
          .where(eq(collaborationSessions.isActive, true))
        )[0]?.count ?? 0,
        onlineParticipants: (await db
          .select({ count: sql<number>`COUNT(*)` })
          .from(collaborationParticipants)
          .where(eq(collaborationParticipants.isOnline, true))
        )[0]?.count ?? 0,
        totalMarkers: (await db
          .select({ count: sql<number>`COUNT(*)` })
          .from(sharedMarkers)
          .where(eq(sharedMarkers.isActive, true))
        )[0]?.count ?? 0,
      } : null;

      return {
        rateLimiter: getRateLimiterStats(),
        sse: getSSEStats(),
        database: dbStats,
        limits: {
          maxParticipantsPerSession: MAX_PARTICIPANTS_DEFAULT,
          maxParticipantsHardLimit: MAX_PARTICIPANTS_HARD_LIMIT,
          pagination: PAGINATION,
          rateLimits: RATE_LIMITS,
        },
      };
    }),
});

// ─── SSE Stream Handler (with connection limits) ───
// This is registered as a raw Express route in the server setup.
// Clients connect via EventSource to receive real-time updates.
export function createSSEHandler() {
  return (req: any, res: any) => {
    const sessionId = req.query.sessionId as string;
    const userId = req.query.userId as string || "anonymous";

    if (!sessionId) {
      res.status(400).json({ error: "sessionId is required" });
      return;
    }

    // ─── SSE Connection Limit Check ───
    const connectionCheck = canOpenSSEConnection(userId, sessionId);
    if (!connectionCheck.allowed) {
      res.status(429).json({
        error: "Connection limit reached",
        reason: connectionCheck.reason,
        userConnections: connectionCheck.userConnections,
        sessionConnections: connectionCheck.sessionConnections,
      });
      return;
    }

    // Generate unique connection ID
    const connectionId = `sse_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;

    // Register connection
    registerSSEConnection(connectionId, userId, sessionId);

    // Set SSE headers
    res.writeHead(200, {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      Connection: "keep-alive",
      "X-Accel-Buffering": "no", // Disable nginx buffering
    });

    // Send initial connection event with stats
    res.write(`data: ${JSON.stringify({
      type: "connected",
      sessionId,
      connectionId,
      timestamp: Date.now(),
      limits: {
        maxParticipantsPerSession: MAX_PARTICIPANTS_DEFAULT,
      },
    })}\n\n`);

    // Heartbeat to keep connection alive
    const heartbeatInterval = setInterval(() => {
      updateSSEActivity(connectionId);
      res.write(`:heartbeat\n\n`);
    }, 15000);

    // Listen for session events
    const handler = (data: unknown) => {
      updateSSEActivity(connectionId);
      res.write(`data: ${JSON.stringify(data)}\n\n`);
    };

    pubsub.subscribe(sessionChannel(sessionId), handler);

    // Cleanup on disconnect
    req.on("close", () => {
      clearInterval(heartbeatInterval);
      pubsub.unsubscribe(sessionChannel(sessionId), handler);
      unregisterSSEConnection(connectionId);
    });
  };
}
