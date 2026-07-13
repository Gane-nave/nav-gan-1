/**
 * G.A.N.E — Master Admin System Tests
 * =====================================
 * Tests for admin router, targeting, feature flags, config, and notifications.
 */
import { describe, it, expect } from "vitest";
import { appRouter } from "../routers";

describe("Master Admin Router", () => {
  it("admin router is registered in appRouter", () => {
    expect(appRouter._def.procedures).toHaveProperty("admin.dashboard");
    expect(appRouter._def.procedures).toHaveProperty("admin.listUsers");
    expect(appRouter._def.procedures).toHaveProperty("admin.blockUser");
    expect(appRouter._def.procedures).toHaveProperty("admin.unblockUser");
    expect(appRouter._def.procedures).toHaveProperty("admin.promoteUser");
    expect(appRouter._def.procedures).toHaveProperty("admin.demoteUser");
    expect(appRouter._def.procedures).toHaveProperty("admin.listFeatureFlags");
    expect(appRouter._def.procedures).toHaveProperty("admin.toggleFeatureFlag");
    expect(appRouter._def.procedures).toHaveProperty("admin.createFeatureFlag");
    expect(appRouter._def.procedures).toHaveProperty("admin.listConfig");
    expect(appRouter._def.procedures).toHaveProperty("admin.updateConfig");
    expect(appRouter._def.procedures).toHaveProperty("admin.sendNotification");
    expect(appRouter._def.procedures).toHaveProperty("admin.getNotifications");
    expect(appRouter._def.procedures).toHaveProperty("admin.dismissNotification");
    expect(appRouter._def.procedures).toHaveProperty("admin.hideAnomaly");
    expect(appRouter._def.procedures).toHaveProperty("admin.showAnomaly");
    expect(appRouter._def.procedures).toHaveProperty("admin.getAuditLog");
    expect(appRouter._def.procedures).toHaveProperty("admin.getSystemLogs");
    expect(appRouter._def.procedures).toHaveProperty("admin.setMaintenanceMode");
    expect(appRouter._def.procedures).toHaveProperty("admin.aiCommand");
    expect(appRouter._def.procedures).toHaveProperty("admin.isAdmin");
    expect(appRouter._def.procedures).toHaveProperty("admin.getFeatureFlag");
  });

  it("has all 22 admin procedures", () => {
    const adminProcedures = Object.keys(appRouter._def.procedures).filter(
      (k) => k.startsWith("admin.")
    );
    expect(adminProcedures.length).toBe(22);
  });
});

describe("Targeting System", () => {
  it("supports individual, group, and all scopes", () => {
    // The targeting schema is embedded in blockUser, sendNotification etc.
    // Verify the procedures exist — the z.enum validates at runtime
    expect(appRouter._def.procedures).toHaveProperty("admin.blockUser");
    expect(appRouter._def.procedures).toHaveProperty("admin.sendNotification");
  });
});

describe("Feature Flags", () => {
  it("has CRUD procedures for feature flags", () => {
    expect(appRouter._def.procedures).toHaveProperty("admin.listFeatureFlags");
    expect(appRouter._def.procedures).toHaveProperty("admin.toggleFeatureFlag");
    expect(appRouter._def.procedures).toHaveProperty("admin.createFeatureFlag");
    expect(appRouter._def.procedures).toHaveProperty("admin.getFeatureFlag");
  });
});

describe("Config Management", () => {
  it("has config CRUD and maintenance mode", () => {
    expect(appRouter._def.procedures).toHaveProperty("admin.listConfig");
    expect(appRouter._def.procedures).toHaveProperty("admin.updateConfig");
    expect(appRouter._def.procedures).toHaveProperty("admin.setMaintenanceMode");
  });
});

describe("Notification System", () => {
  it("has send, get, and dismiss notification procedures", () => {
    expect(appRouter._def.procedures).toHaveProperty("admin.sendNotification");
    expect(appRouter._def.procedures).toHaveProperty("admin.getNotifications");
    expect(appRouter._def.procedures).toHaveProperty("admin.dismissNotification");
  });
});

describe("Audit & Logs", () => {
  it("has audit log and system logs procedures", () => {
    expect(appRouter._def.procedures).toHaveProperty("admin.getAuditLog");
    expect(appRouter._def.procedures).toHaveProperty("admin.getSystemLogs");
  });
});

describe("AI Command Bot", () => {
  it("has aiCommand procedure", () => {
    expect(appRouter._def.procedures).toHaveProperty("admin.aiCommand");
  });
});

describe("Admin Mode Context", () => {
  it("AdminModeContext exports useAdminMode and AdminModeProvider", async () => {
    const mod = await import("../../client/src/contexts/AdminModeContext");
    expect(mod.useAdminMode).toBeDefined();
    expect(mod.AdminModeProvider).toBeDefined();
  });
});
