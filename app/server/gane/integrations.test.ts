/**
 * G.A.N.E — Integration Tests
 * ============================
 * Tests for:
 *   - Payment router (products, wallet, stripe status)
 *   - Map keys router (status, key retrieval)
 *   - Stripe module (product definitions, configuration check)
 */

import { describe, expect, it } from "vitest";
import { appRouter } from "../routers";
import type { TrpcContext } from "../_core/context";
import { GANE_PRODUCTS, isStripeConfigured } from "./stripe";

// ─── Test Context Helpers ───

type AuthenticatedUser = NonNullable<TrpcContext["user"]>;

function createAuthContext(): TrpcContext {
  const user: AuthenticatedUser = {
    id: 1,
    openId: "test-user-001",
    email: "test@gane.dev",
    name: "Test Navigator",
    loginMethod: "manus",
    role: "user",
    createdAt: new Date(),
    updatedAt: new Date(),
    lastSignedIn: new Date(),
  };

  return {
    user,
    req: {
      protocol: "https",
      headers: { origin: "https://test.gane.dev" },
    } as TrpcContext["req"],
    res: {
      clearCookie: () => {},
    } as TrpcContext["res"],
  };
}

function createPublicContext(): TrpcContext {
  return {
    user: null,
    req: {
      protocol: "https",
      headers: {},
    } as TrpcContext["req"],
    res: {
      clearCookie: () => {},
    } as TrpcContext["res"],
  };
}

// ─── Stripe Module Tests ───

describe("Stripe Module", () => {
  it("has valid product definitions", () => {
    expect(GANE_PRODUCTS.length).toBeGreaterThanOrEqual(3);

    for (const product of GANE_PRODUCTS) {
      expect(product.id).toBeTruthy();
      expect(product.name).toBeTruthy();
      expect(product.nameHe).toBeTruthy();
      expect(product.description).toBeTruthy();
      expect(product.priceAmount).toBeGreaterThan(0);
      expect(product.currency).toBe("usd");
      expect(product.features.length).toBeGreaterThan(0);
    }
  });

  it("has subscription products with intervals", () => {
    const subscriptions = GANE_PRODUCTS.filter((p) => p.interval);
    expect(subscriptions.length).toBeGreaterThanOrEqual(2);

    for (const sub of subscriptions) {
      expect(["month", "year"]).toContain(sub.interval);
    }
  });

  it("has one-time purchase products", () => {
    const oneTime = GANE_PRODUCTS.filter((p) => !p.interval);
    expect(oneTime.length).toBeGreaterThanOrEqual(1);
  });

  it("reports stripe configuration status correctly", () => {
    // In test env, STRIPE_SECRET_KEY is not set
    const configured = isStripeConfigured();
    expect(typeof configured).toBe("boolean");
  });
});

// ─── Payment Router Tests ───

describe("Payment Router", () => {
  it("returns stripe status (public)", async () => {
    const ctx = createPublicContext();
    const caller = appRouter.createCaller(ctx);

    const status = await caller.payments.stripeStatus();
    expect(status).toHaveProperty("configured");
    expect(typeof status.configured).toBe("boolean");
    expect(status).toHaveProperty("publishableKey");
  });

  it("returns product catalog (public)", async () => {
    const ctx = createPublicContext();
    const caller = appRouter.createCaller(ctx);

    const products = await caller.payments.products();
    expect(Array.isArray(products)).toBe(true);
    expect(products.length).toBeGreaterThanOrEqual(3);

    const firstProduct = products[0];
    expect(firstProduct).toHaveProperty("id");
    expect(firstProduct).toHaveProperty("name");
    expect(firstProduct).toHaveProperty("nameHe");
    expect(firstProduct).toHaveProperty("priceAmount");
    expect(firstProduct).toHaveProperty("currency");
    expect(firstProduct).toHaveProperty("features");
  });

  it("product prices are in cents", async () => {
    const ctx = createPublicContext();
    const caller = appRouter.createCaller(ctx);

    const products = await caller.payments.products();
    for (const product of products) {
      expect(product.priceAmount).toBeGreaterThanOrEqual(100); // At least $1.00
      expect(Number.isInteger(product.priceAmount)).toBe(true);
    }
  });
});

// ─── Map Keys Router Tests ───

describe("Map Keys Router", () => {
  it("returns provider status (public)", async () => {
    const ctx = createPublicContext();
    const caller = appRouter.createCaller(ctx);

    const status = await caller.mapKeys.status();
    expect(status).toHaveProperty("providers");
    expect(status).toHaveProperty("configuredCount");
    expect(status).toHaveProperty("totalCount");
    expect(status.totalCount).toBe(3); // Mapbox, HERE, TomTom
    expect(Array.isArray(status.providers)).toBe(true);

    for (const provider of status.providers) {
      expect(provider).toHaveProperty("id");
      expect(provider).toHaveProperty("name");
      expect(provider).toHaveProperty("configured");
      expect(typeof provider.configured).toBe("boolean");
    }
  });

  it("provider IDs match expected providers", async () => {
    const ctx = createPublicContext();
    const caller = appRouter.createCaller(ctx);

    const status = await caller.mapKeys.status();
    const ids = status.providers.map((p: { id: string }) => p.id);
    expect(ids).toContain("mapbox");
    expect(ids).toContain("here");
    expect(ids).toContain("tomtom");
  });

  it("returns keys for authenticated users", async () => {
    const ctx = createAuthContext();
    const caller = appRouter.createCaller(ctx);

    const result = await caller.mapKeys.getKeys();
    expect(result).toHaveProperty("keys");
    expect(result).toHaveProperty("configuredProviders");
    expect(typeof result.keys).toBe("object");
    expect(Array.isArray(result.configuredProviders)).toBe(true);
  });
});
