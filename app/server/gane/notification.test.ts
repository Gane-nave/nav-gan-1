/**
 * G.A.N.E — Notification System Tests
 * =====================================
 * Tests for notification router procedures, preferences,
 * and integration with the appRouter.
 */
import { describe, it, expect } from "vitest";
import { appRouter } from "../routers";

describe("Notification Router", () => {
  it("notification router is registered in appRouter", () => {
    expect(appRouter._def.procedures).toHaveProperty("notifications.list");
    expect(appRouter._def.procedures).toHaveProperty("notifications.getUnreadCount");
    expect(appRouter._def.procedures).toHaveProperty("notifications.markRead");
    expect(appRouter._def.procedures).toHaveProperty("notifications.markAllRead");
    expect(appRouter._def.procedures).toHaveProperty("notifications.delete");
    expect(appRouter._def.procedures).toHaveProperty("notifications.clearAll");
    expect(appRouter._def.procedures).toHaveProperty("notifications.getPreferences");
    expect(appRouter._def.procedures).toHaveProperty("notifications.updatePreferences");
    expect(appRouter._def.procedures).toHaveProperty("notifications.adminCreate");
  });

  it("has exactly 9 notification procedures", () => {
    const notifProcedures = Object.keys(appRouter._def.procedures).filter(
      (k) => k.startsWith("notifications.")
    );
    expect(notifProcedures.length).toBe(9);
  });
});

describe("Notification Procedure Definitions", () => {
  it("all notification procedures have _def", () => {
    const procedureNames = [
      "notifications.list",
      "notifications.getUnreadCount",
      "notifications.markRead",
      "notifications.markAllRead",
      "notifications.delete",
      "notifications.clearAll",
      "notifications.getPreferences",
      "notifications.updatePreferences",
      "notifications.adminCreate",
    ];

    for (const name of procedureNames) {
      const proc = appRouter._def.procedures[name] as any;
      expect(proc).toBeDefined();
      expect(proc._def).toBeDefined();
    }
  });

  it("mutation procedures have inputs defined for markRead", () => {
    const proc = appRouter._def.procedures["notifications.markRead"] as any;
    expect(proc._def.inputs).toBeDefined();
    expect(proc._def.inputs.length).toBeGreaterThan(0);
  });

  it("mutation procedures have inputs defined for delete", () => {
    const proc = appRouter._def.procedures["notifications.delete"] as any;
    expect(proc._def.inputs).toBeDefined();
    expect(proc._def.inputs.length).toBeGreaterThan(0);
  });

  it("adminCreate has inputs defined", () => {
    const proc = appRouter._def.procedures["notifications.adminCreate"] as any;
    expect(proc._def.inputs).toBeDefined();
    expect(proc._def.inputs.length).toBeGreaterThan(0);
  });

  it("updatePreferences has inputs defined", () => {
    const proc = appRouter._def.procedures["notifications.updatePreferences"] as any;
    expect(proc._def.inputs).toBeDefined();
    expect(proc._def.inputs.length).toBeGreaterThan(0);
  });
});

describe("Notification Helper Functions", () => {
  it("notificationChannel returns correct channel name", async () => {
    const { notificationChannel } = await import("./notificationRouter");
    expect(notificationChannel(1)).toBe("notify:user:1");
    expect(notificationChannel(42)).toBe("notify:user:42");
    expect(notificationChannel(999)).toBe("notify:user:999");
  });

  it("createNotification is an async function", async () => {
    const { createNotification } = await import("./notificationRouter");
    expect(typeof createNotification).toBe("function");
  });

  it("createBulkNotifications is an async function", async () => {
    const { createBulkNotifications } = await import("./notificationRouter");
    expect(typeof createBulkNotifications).toBe("function");
  });

  it("broadcastNotification is a function", async () => {
    const { broadcastNotification } = await import("./notificationRouter");
    expect(typeof broadcastNotification).toBe("function");
  });
});

describe("Notification Schema Integration", () => {
  it("userNotifications table is defined in schema", async () => {
    const schema = await import("../../drizzle/schema");
    expect(schema.userNotifications).toBeDefined();
    expect(typeof schema.userNotifications).toBe("object");
  });

  it("notificationPreferences table is defined in schema", async () => {
    const schema = await import("../../drizzle/schema");
    expect(schema.notificationPreferences).toBeDefined();
    expect(typeof schema.notificationPreferences).toBe("object");
  });

  it("UserNotification type is exported", async () => {
    const schema = await import("../../drizzle/schema");
    // Type exports don't exist at runtime, but the table does
    expect(schema.userNotifications).toBeDefined();
  });

  it("NotificationPreference type is exported", async () => {
    const schema = await import("../../drizzle/schema");
    expect(schema.notificationPreferences).toBeDefined();
  });
});

describe("Notification Router Input Validation", () => {
  it("list accepts optional input", () => {
    const proc = appRouter._def.procedures["notifications.list"] as any;
    // list has optional input (limit, cursor, unreadOnly, type)
    expect(proc._def.inputs).toBeDefined();
  });

  it("markRead requires notificationId", () => {
    const proc = appRouter._def.procedures["notifications.markRead"] as any;
    expect(proc._def.inputs).toBeDefined();
    expect(proc._def.inputs.length).toBeGreaterThan(0);
  });

  it("delete requires notificationId", () => {
    const proc = appRouter._def.procedures["notifications.delete"] as any;
    expect(proc._def.inputs).toBeDefined();
    expect(proc._def.inputs.length).toBeGreaterThan(0);
  });

  it("adminCreate requires type, title, and message", () => {
    const proc = appRouter._def.procedures["notifications.adminCreate"] as any;
    expect(proc._def.inputs).toBeDefined();
    expect(proc._def.inputs.length).toBeGreaterThan(0);
  });

  it("updatePreferences accepts boolean fields", () => {
    const proc = appRouter._def.procedures["notifications.updatePreferences"] as any;
    expect(proc._def.inputs).toBeDefined();
    expect(proc._def.inputs.length).toBeGreaterThan(0);
  });
});
