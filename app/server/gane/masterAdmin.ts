/**
 * G.A.N.E — Master Admin System
 * ===============================
 * Full admin control with 3-level targeting:
 *   - individual: single user by ID
 *   - group: selected set of users
 *   - all: every user in the system
 *
 * Includes: user management, content control, feature flags,
 * app config, notifications, analytics, and AI command bot.
 */
import { z } from "zod";
import { eq, desc, sql, and, inArray, like } from "drizzle-orm";
import { adminProcedure, publicProcedure, router } from "../_core/trpc";
import { getDb } from "../db";
import { invokeLLM } from "../_core/llm";
import {
  users, User,
  adminActions, InsertAdminAction,
  featureFlags, InsertFeatureFlag,
  appConfig,
  userBlocks, InsertUserBlock,
  adminNotifications, InsertAdminNotification,
  trips, devices, alerts, mapAnomalies,
  logEntries,
} from "../../drizzle/schema";
import { createNotification, createBulkNotifications } from "./notificationRouter";

// ─── Helpers ──────────────────────────────────────
function genId(prefix: string): string {
  return `${prefix}_${Date.now().toString(36)}_${Math.random().toString(36).slice(2, 8)}`;
}

async function logAdminAction(data: Omit<InsertAdminAction, "actionId">) {
  const db = await getDb();
  if (!db) return;
  await db.insert(adminActions).values({ ...data, actionId: genId("aa") });
}

// Target scope schema — reused across all targeted operations
const targetSchema = z.object({
  scope: z.enum(["individual", "group", "all"]),
  userIds: z.array(z.number()).optional(), // required for individual/group
});

// ─── Router ───────────────────────────────────────
export const masterAdminRouter = router({

  // ═══════════════════════════════════════════
  // DASHBOARD — Real-time system overview
  // ═══════════════════════════════════════════
  dashboard: adminProcedure.query(async ({ ctx }) => {
    const db = await getDb();
    if (!db) return { totalUsers: 0, activeUsers: 0, totalTrips: 0, activeTrips: 0, totalDevices: 0, totalAlerts: 0, totalAnomalies: 0, recentActions: [] };

    const [userStats] = await db.select({
      total: sql<number>`COUNT(*)`,
      active: sql<number>`SUM(CASE WHEN lastSignedIn > DATE_SUB(NOW(), INTERVAL 7 DAY) THEN 1 ELSE 0 END)`,
    }).from(users);

    const [tripStats] = await db.select({
      total: sql<number>`COUNT(*)`,
      active: sql<number>`SUM(CASE WHEN status = 'active' THEN 1 ELSE 0 END)`,
    }).from(trips);

    const [deviceStats] = await db.select({
      total: sql<number>`COUNT(*)`,
    }).from(devices);

    const [alertStats] = await db.select({
      total: sql<number>`COUNT(*)`,
    }).from(alerts);

    const [anomalyStats] = await db.select({
      total: sql<number>`COUNT(*)`,
    }).from(mapAnomalies);

    const recentActions = await db.select().from(adminActions)
      .orderBy(desc(adminActions.createdAt))
      .limit(20);

    return {
      totalUsers: userStats?.total ?? 0,
      activeUsers: userStats?.active ?? 0,
      totalTrips: tripStats?.total ?? 0,
      activeTrips: tripStats?.active ?? 0,
      totalDevices: deviceStats?.total ?? 0,
      totalAlerts: alertStats?.total ?? 0,
      totalAnomalies: anomalyStats?.total ?? 0,
      recentActions,
    };
  }),

  // ═══════════════════════════════════════════
  // USER MANAGEMENT
  // ═══════════════════════════════════════════
  listUsers: adminProcedure.input(z.object({
    page: z.number().min(1).default(1),
    limit: z.number().min(1).max(100).default(20),
    search: z.string().optional(),
    role: z.enum(["user", "admin", "dispatcher", "driver", "ems", "sports"]).optional(),
  })).query(async ({ input }) => {
    const db = await getDb();
    if (!db) return { users: [], total: 0 };

    const offset = (input.page - 1) * input.limit;
    const conditions = [];
    if (input.role) conditions.push(eq(users.role, input.role));
    if (input.search) conditions.push(like(users.name, `%${input.search}%`));

    const where = conditions.length > 0 ? and(...conditions) : undefined;

    const [countResult] = await db.select({ count: sql<number>`COUNT(*)` })
      .from(users).where(where);

    const userList = await db.select().from(users)
      .where(where)
      .orderBy(desc(users.lastSignedIn))
      .limit(input.limit)
      .offset(offset);

    return { users: userList, total: countResult?.count ?? 0 };
  }),

  blockUser: adminProcedure.input(z.object({
    target: targetSchema,
    reason: z.string().optional(),
    expiresAt: z.string().optional(), // ISO date
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    const targetIds = input.target.scope === "all"
      ? (await db.select({ id: users.id }).from(users)).map(u => u.id)
      : input.target.userIds ?? [];

    // Don't block yourself
    const safeIds = targetIds.filter(id => id !== ctx.user.id);

    for (const uid of safeIds) {
      await db.insert(userBlocks).values({
        userId: uid,
        blockedBy: ctx.user.id,
        reason: input.reason ?? null,
        expiresAt: input.expiresAt ? new Date(input.expiresAt) : null,
        isActive: true,
      }).onDuplicateKeyUpdate({ set: { isActive: true, reason: input.reason ?? null } });
    }

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "block_user",
      targetScope: input.target.scope,
      targetUserIds: safeIds,
      description: input.reason ?? `Blocked ${safeIds.length} user(s)`,
      status: "completed",
    });

    return { blocked: safeIds.length };
  }),

  unblockUser: adminProcedure.input(z.object({
    target: targetSchema,
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    const targetIds = input.target.scope === "all"
      ? (await db.select({ id: users.id }).from(users)).map(u => u.id)
      : input.target.userIds ?? [];

    for (const uid of targetIds) {
      await db.update(userBlocks)
        .set({ isActive: false })
        .where(and(eq(userBlocks.userId, uid), eq(userBlocks.isActive, true)));
    }

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "unblock_user",
      targetScope: input.target.scope,
      targetUserIds: targetIds,
      description: `Unblocked ${targetIds.length} user(s)`,
      status: "completed",
    });

    return { unblocked: targetIds.length };
  }),

  promoteUser: adminProcedure.input(z.object({
    target: targetSchema,
    role: z.enum(["admin", "dispatcher", "driver", "ems", "sports"]),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    const targetIds = input.target.scope === "all"
      ? (await db.select({ id: users.id }).from(users)).map(u => u.id)
      : input.target.userIds ?? [];

    for (const uid of targetIds) {
      await db.update(users).set({ role: input.role }).where(eq(users.id, uid));
    }

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "promote_user",
      targetScope: input.target.scope,
      targetUserIds: targetIds,
      newValue: { role: input.role },
      description: `Promoted ${targetIds.length} user(s) to ${input.role}`,
      status: "completed",
    });

    return { promoted: targetIds.length, role: input.role };
  }),

  demoteUser: adminProcedure.input(z.object({
    target: targetSchema,
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    const targetIds = (input.target.userIds ?? []).filter(id => id !== ctx.user.id);

    for (const uid of targetIds) {
      await db.update(users).set({ role: "user" }).where(eq(users.id, uid));
    }

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "demote_user",
      targetScope: input.target.scope,
      targetUserIds: targetIds,
      description: `Demoted ${targetIds.length} user(s) to regular user`,
      status: "completed",
    });

    return { demoted: targetIds.length };
  }),

  // ═══════════════════════════════════════════
  // FEATURE FLAGS
  // ═══════════════════════════════════════════
  listFeatureFlags: adminProcedure.query(async () => {
    const db = await getDb();
    if (!db) return [];
    return db.select().from(featureFlags).orderBy(featureFlags.key);
  }),

  toggleFeatureFlag: adminProcedure.input(z.object({
    key: z.string(),
    isEnabled: z.boolean(),
    scope: z.enum(["global", "group", "individual"]).optional(),
    targetUserIds: z.array(z.number()).optional(),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    await db.update(featureFlags)
      .set({
        isEnabled: input.isEnabled,
        scope: input.scope ?? "global",
        targetUserIds: input.targetUserIds ?? null,
        updatedBy: ctx.user.id,
      })
      .where(eq(featureFlags.key, input.key));

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "toggle_feature",
      targetScope: input.scope === "individual" ? "individual" : input.scope === "group" ? "group" : "all",
      description: `${input.isEnabled ? "Enabled" : "Disabled"} feature: ${input.key}`,
      newValue: { key: input.key, isEnabled: input.isEnabled },
      status: "completed",
    });

    return { key: input.key, isEnabled: input.isEnabled };
  }),

  createFeatureFlag: adminProcedure.input(z.object({
    key: z.string().min(1),
    label: z.string().min(1),
    description: z.string().optional(),
    isEnabled: z.boolean().default(true),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    await db.insert(featureFlags).values({
      key: input.key,
      label: input.label,
      description: input.description ?? null,
      isEnabled: input.isEnabled,
      updatedBy: ctx.user.id,
    });

    return { key: input.key, created: true };
  }),

  // ═══════════════════════════════════════════
  // APP CONFIG
  // ═══════════════════════════════════════════
  listConfig: adminProcedure.query(async () => {
    const db = await getDb();
    if (!db) return [];
    return db.select().from(appConfig).orderBy(appConfig.category, appConfig.key);
  }),

  updateConfig: adminProcedure.input(z.object({
    key: z.string(),
    value: z.any(),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    const existing = await db.select().from(appConfig).where(eq(appConfig.key, input.key)).limit(1);
    const previousValue = existing[0]?.value ?? null;

    await db.insert(appConfig).values({
      key: input.key,
      value: input.value,
      updatedBy: ctx.user.id,
    }).onDuplicateKeyUpdate({
      set: { value: input.value, updatedBy: ctx.user.id },
    });

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "update_config",
      targetScope: "all",
      description: `Updated config: ${input.key}`,
      previousValue: previousValue as Record<string, unknown> | null,
      newValue: input.value as Record<string, unknown>,
      status: "completed",
    });

    return { key: input.key, updated: true };
  }),

  // ═══════════════════════════════════════════
  // NOTIFICATIONS (Targeted)
  // ═══════════════════════════════════════════
  sendNotification: adminProcedure.input(z.object({
    title: z.string().min(1),
    message: z.string().min(1),
    type: z.enum(["info", "warning", "success", "error", "announcement"]).default("info"),
    target: targetSchema,
    expiresAt: z.string().optional(),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    const targetIds = input.target.scope === "all"
      ? null
      : input.target.userIds ?? null;

    await db.insert(adminNotifications).values({
      notificationId: genId("an"),
      title: input.title,
      message: input.message,
      type: input.type,
      targetScope: input.target.scope,
      targetUserIds: targetIds,
      sentBy: ctx.user.id,
      expiresAt: input.expiresAt ? new Date(input.expiresAt) : null,
    });

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "send_notification",
      targetScope: input.target.scope,
      targetUserIds: targetIds,
      description: `Sent notification: ${input.title}`,
      status: "completed",
    });

    // Also push to user notification system for real-time delivery
    const notifType = (input.type === "announcement" ? "admin" : input.type) as "info" | "success" | "warning" | "error" | "admin";
    if (input.target.scope === "all") {
      const allUsers = await db.select({ id: users.id }).from(users);
      createBulkNotifications(
        allUsers.map(u => u.id),
        notifType,
        input.title,
        input.message,
        { source: "admin", adminUserId: ctx.user.id },
      ).catch(() => {});
    } else if (targetIds && targetIds.length > 0) {
      createBulkNotifications(
        targetIds,
        notifType,
        input.title,
        input.message,
        { source: "admin", adminUserId: ctx.user.id },
      ).catch(() => {});
    }

    return { sent: true, scope: input.target.scope };
  }),

  getNotifications: publicProcedure.query(async ({ ctx }) => {
    const db = await getDb();
    if (!db || !ctx.user) return [];

    const notifs = await db.select().from(adminNotifications)
      .where(eq(adminNotifications.isActive, true))
      .orderBy(desc(adminNotifications.createdAt))
      .limit(50);

    // Filter by targeting
    return notifs.filter(n => {
      if (n.targetScope === "all") return true;
      const targets = (n.targetUserIds as number[] | null) ?? [];
      return targets.includes(ctx.user!.id);
    });
  }),

  dismissNotification: publicProcedure.input(z.object({
    notificationId: z.string(),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db || !ctx.user) return { dismissed: false };

    const [notif] = await db.select().from(adminNotifications)
      .where(eq(adminNotifications.notificationId, input.notificationId))
      .limit(1);

    if (!notif) return { dismissed: false };

    const readBy = (notif.readBy as number[] | null) ?? [];
    if (!readBy.includes(ctx.user.id)) {
      readBy.push(ctx.user.id);
      await db.update(adminNotifications)
        .set({ readBy })
        .where(eq(adminNotifications.notificationId, input.notificationId));
    }

    return { dismissed: true };
  }),

  // ═══════════════════════════════════════════
  // CONTENT MANAGEMENT
  // ═══════════════════════════════════════════
  hideAnomaly: adminProcedure.input(z.object({
    anomalyId: z.string(),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    await db.update(mapAnomalies)
      .set({ isActive: false })
      .where(eq(mapAnomalies.anomalyId, input.anomalyId));

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "hide_content",
      targetScope: "all",
      description: `Hidden anomaly: ${input.anomalyId}`,
      status: "completed",
    });

    return { hidden: true };
  }),

  showAnomaly: adminProcedure.input(z.object({
    anomalyId: z.string(),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    await db.update(mapAnomalies)
      .set({ isActive: true })
      .where(eq(mapAnomalies.anomalyId, input.anomalyId));

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "show_content",
      targetScope: "all",
      description: `Restored anomaly: ${input.anomalyId}`,
      status: "completed",
    });

    return { shown: true };
  }),

  // ═══════════════════════════════════════════
  // AUDIT LOG
  // ═══════════════════════════════════════════
  getAuditLog: adminProcedure.input(z.object({
    page: z.number().min(1).default(1),
    limit: z.number().min(1).max(100).default(50),
    actionType: z.string().optional(),
  })).query(async ({ input }) => {
    const db = await getDb();
    if (!db) return { actions: [], total: 0 };

    const where = input.actionType
      ? eq(adminActions.actionType, input.actionType as any)
      : undefined;

    const [countResult] = await db.select({ count: sql<number>`COUNT(*)` })
      .from(adminActions).where(where);

    const actions = await db.select().from(adminActions)
      .where(where)
      .orderBy(desc(adminActions.createdAt))
      .limit(input.limit)
      .offset((input.page - 1) * input.limit);

    return { actions, total: countResult?.count ?? 0 };
  }),

  // ═══════════════════════════════════════════
  // SYSTEM LOGS
  // ═══════════════════════════════════════════
  getSystemLogs: adminProcedure.input(z.object({
    page: z.number().min(1).default(1),
    limit: z.number().min(1).max(100).default(50),
    level: z.enum(["debug", "info", "warn", "error", "critical"]).optional(),
    source: z.string().optional(),
  })).query(async ({ input }) => {
    const db = await getDb();
    if (!db) return { logs: [], total: 0 };

    const conditions = [];
    if (input.level) conditions.push(eq(logEntries.level, input.level));
    if (input.source) conditions.push(eq(logEntries.source, input.source));
    const where = conditions.length > 0 ? and(...conditions) : undefined;

    const [countResult] = await db.select({ count: sql<number>`COUNT(*)` })
      .from(logEntries).where(where);

    const logs = await db.select().from(logEntries)
      .where(where)
      .orderBy(desc(logEntries.createdAt))
      .limit(input.limit)
      .offset((input.page - 1) * input.limit);

    return { logs, total: countResult?.count ?? 0 };
  }),

  // ═══════════════════════════════════════════
  // MAINTENANCE MODE
  // ═══════════════════════════════════════════
  setMaintenanceMode: adminProcedure.input(z.object({
    enabled: z.boolean(),
    message: z.string().optional(),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) throw new Error("DB unavailable");

    await db.insert(appConfig).values({
      key: "maintenance_mode",
      value: { enabled: input.enabled, message: input.message ?? "System maintenance in progress" },
      category: "general",
      updatedBy: ctx.user.id,
    }).onDuplicateKeyUpdate({
      set: {
        value: { enabled: input.enabled, message: input.message ?? "System maintenance in progress" },
        updatedBy: ctx.user.id,
      },
    });

    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "maintenance_mode",
      targetScope: "all",
      description: `Maintenance mode ${input.enabled ? "ENABLED" : "DISABLED"}`,
      newValue: { enabled: input.enabled },
      status: "completed",
    });

    return { maintenanceMode: input.enabled };
  }),

  // ═══════════════════════════════════════════
  // AI COMMAND BOT
  // ═══════════════════════════════════════════
  aiCommand: adminProcedure.input(z.object({
    prompt: z.string().min(1),
    conversationHistory: z.array(z.object({
      role: z.enum(["user", "assistant"]),
      content: z.string(),
    })).optional(),
  })).mutation(async ({ ctx, input }) => {
    const db = await getDb();

    // Build system prompt with available actions
    const systemPrompt = `You are the G.A.N.E Master Admin AI Assistant. You help the Master Admin manage the system.

AVAILABLE ACTIONS (return as JSON):
- { "action": "block_user", "userIds": [1,2], "reason": "..." }
- { "action": "unblock_user", "userIds": [1,2] }
- { "action": "promote_user", "userIds": [1], "role": "admin|dispatcher|driver|ems|sports" }
- { "action": "demote_user", "userIds": [1] }
- { "action": "send_notification", "title": "...", "message": "...", "scope": "all|group|individual", "userIds": [1,2], "type": "info|warning|success|error|announcement" }
- { "action": "toggle_feature", "key": "feature_name", "enabled": true/false }
- { "action": "update_config", "key": "config_key", "value": "..." }
- { "action": "maintenance_mode", "enabled": true/false, "message": "..." }
- { "action": "hide_anomaly", "anomalyId": "..." }
- { "action": "show_anomaly", "anomalyId": "..." }
- { "action": "query", "description": "..." } — for read-only queries about system state

RULES:
1. If the user asks to perform an action, return ONLY a JSON object with the action.
2. If the user asks a question, answer it and optionally suggest actions.
3. Always confirm destructive actions before executing.
4. Never block the Master Admin (user ID: ${ctx.user.id}).
5. For "all users" scope, set scope to "all" and omit userIds.
6. Respond in the same language the user uses.

CURRENT SYSTEM STATE:
- Master Admin: ${ctx.user.name} (ID: ${ctx.user.id})
- Current time: ${new Date().toISOString()}`;

    const messages = [
      { role: "system" as const, content: systemPrompt },
      ...(input.conversationHistory ?? []).map(m => ({
        role: m.role as "user" | "assistant",
        content: m.content,
      })),
      { role: "user" as const, content: input.prompt },
    ];

    const response = await invokeLLM({ messages });
    const rawContent = response.choices?.[0]?.message?.content;
    const aiResponse: string = typeof rawContent === "string" ? rawContent : "I couldn't process that request.";

    // Try to parse action from response
    let executedAction: Record<string, unknown> | null = null;
    try {
      const jsonMatch = aiResponse.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        const parsed = JSON.parse(jsonMatch[0]);
        if (parsed.action && db) {
          executedAction = await executeAiAction(parsed, ctx.user, db);
        }
      }
    } catch {
      // Not an action response, just a text reply
    }

    // Log the AI command
    await logAdminAction({
      adminUserId: ctx.user.id,
      actionType: "ai_command",
      targetScope: "all",
      aiPrompt: input.prompt,
      description: executedAction
        ? `AI executed: ${JSON.stringify(executedAction)}`
        : `AI response (no action)`,
      newValue: executedAction,
      status: executedAction ? "completed" : "completed",
    });

    return {
      response: aiResponse,
      executedAction,
    };
  }),

  // ═══════════════════════════════════════════
  // CHECK ADMIN STATUS (public — used by frontend)
  // ═══════════════════════════════════════════
  isAdmin: publicProcedure.query(async ({ ctx }) => {
    if (!ctx.user) return { isAdmin: false, isMasterAdmin: false };
    const isMasterAdmin = ctx.user.openId === (process.env.OWNER_OPEN_ID ?? "");
    return {
      isAdmin: ctx.user.role === "admin" || isMasterAdmin,
      isMasterAdmin,
      role: ctx.user.role,
    };
  }),

  // ═══════════════════════════════════════════
  // FEATURE FLAG CHECK (public — used by frontend)
  // ═══════════════════════════════════════════
  getFeatureFlag: publicProcedure.input(z.object({
    key: z.string(),
  })).query(async ({ ctx, input }) => {
    const db = await getDb();
    if (!db) return { enabled: true }; // default to enabled if DB unavailable

    const [flag] = await db.select().from(featureFlags)
      .where(eq(featureFlags.key, input.key))
      .limit(1);

    if (!flag) return { enabled: true }; // unknown flags default to enabled

    if (flag.scope === "global") return { enabled: flag.isEnabled };

    // Check targeting
    if (!ctx.user) return { enabled: flag.isEnabled };
    const targets = (flag.targetUserIds as number[] | null) ?? [];
    if (flag.scope === "individual" || flag.scope === "group") {
      return { enabled: targets.includes(ctx.user.id) ? flag.isEnabled : !flag.isEnabled };
    }

    return { enabled: flag.isEnabled };
  }),
});

// ─── AI Action Executor ───────────────────────────
async function executeAiAction(
  parsed: Record<string, unknown>,
  admin: User,
  db: NonNullable<Awaited<ReturnType<typeof getDb>>>
): Promise<Record<string, unknown> | null> {
  const action = parsed.action as string;
  const userIds = (parsed.userIds as number[] | undefined) ?? [];

  switch (action) {
    case "block_user": {
      const safeIds = userIds.filter(id => id !== admin.id);
      for (const uid of safeIds) {
        await db.insert(userBlocks).values({
          userId: uid,
          blockedBy: admin.id,
          reason: (parsed.reason as string) ?? "Blocked by AI command",
          isActive: true,
        }).onDuplicateKeyUpdate({ set: { isActive: true } });
      }
      return { action, blocked: safeIds.length };
    }

    case "unblock_user": {
      for (const uid of userIds) {
        await db.update(userBlocks)
          .set({ isActive: false })
          .where(and(eq(userBlocks.userId, uid), eq(userBlocks.isActive, true)));
      }
      return { action, unblocked: userIds.length };
    }

    case "promote_user": {
      const role = parsed.role as string;
      if (!["admin", "dispatcher", "driver", "ems", "sports"].includes(role)) return null;
      for (const uid of userIds) {
        await db.update(users).set({ role: role as any }).where(eq(users.id, uid));
      }
      return { action, promoted: userIds.length, role };
    }

    case "demote_user": {
      const safeIds = userIds.filter(id => id !== admin.id);
      for (const uid of safeIds) {
        await db.update(users).set({ role: "user" }).where(eq(users.id, uid));
      }
      return { action, demoted: safeIds.length };
    }

    case "send_notification": {
      const scope = (parsed.scope as string) ?? "all";
      await db.insert(adminNotifications).values({
        notificationId: genId("an"),
        title: (parsed.title as string) ?? "Notification",
        message: (parsed.message as string) ?? "",
        type: (parsed.type as any) ?? "info",
        targetScope: scope as any,
        targetUserIds: scope !== "all" ? userIds : null,
        sentBy: admin.id,
      });
      return { action, sent: true, scope };
    }

    case "toggle_feature": {
      const key = parsed.key as string;
      const enabled = parsed.enabled as boolean;
      if (!key) return null;
      await db.update(featureFlags)
        .set({ isEnabled: enabled, updatedBy: admin.id })
        .where(eq(featureFlags.key, key));
      return { action, key, enabled };
    }

    case "update_config": {
      const key = parsed.key as string;
      if (!key) return null;
      await db.insert(appConfig).values({
        key,
        value: parsed.value ?? null,
        updatedBy: admin.id,
      }).onDuplicateKeyUpdate({
        set: { value: parsed.value ?? null, updatedBy: admin.id },
      });
      return { action, key, updated: true };
    }

    case "maintenance_mode": {
      const enabled = parsed.enabled as boolean;
      await db.insert(appConfig).values({
        key: "maintenance_mode",
        value: { enabled, message: (parsed.message as string) ?? "Maintenance" },
        category: "general",
        updatedBy: admin.id,
      }).onDuplicateKeyUpdate({
        set: {
          value: { enabled, message: (parsed.message as string) ?? "Maintenance" },
          updatedBy: admin.id,
        },
      });
      return { action, enabled };
    }

    case "hide_anomaly": {
      const anomalyId = parsed.anomalyId as string;
      if (!anomalyId) return null;
      await db.update(mapAnomalies)
        .set({ isActive: false })
        .where(eq(mapAnomalies.anomalyId, anomalyId));
      return { action, anomalyId, hidden: true };
    }

    case "show_anomaly": {
      const anomalyId = parsed.anomalyId as string;
      if (!anomalyId) return null;
      await db.update(mapAnomalies)
        .set({ isActive: true })
        .where(eq(mapAnomalies.anomalyId, anomalyId));
      return { action, anomalyId, shown: true };
    }

    default:
      return null;
  }
}
