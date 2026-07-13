/**
 * G.A.N.E — Payment Router
 * ========================
 * tRPC procedures for wallet, payment events, and Stripe integration.
 * Provides:
 *   - Wallet balance & transaction history
 *   - Stripe checkout session creation
 *   - Product catalog
 *   - Payment recording
 *   - Stripe configuration status
 */

import { z } from "zod";
import { eq, desc, and, sql, gte, lte } from "drizzle-orm";
import { protectedProcedure, publicProcedure, router } from "../_core/trpc";
import { getDb } from "../db";
import { paymentEvents } from "../../drizzle/schema";
import { TRPCError } from "@trpc/server";
import {
  isStripeConfigured,
  createCheckoutSession,
  GANE_PRODUCTS,
} from "./stripe";

export const paymentRouter = router({
  /** Check if Stripe is configured */
  stripeStatus: publicProcedure.query(() => {
    return {
      configured: isStripeConfigured(),
      publishableKey: process.env.VITE_STRIPE_PUBLISHABLE_KEY || process.env.VITE_STRIPE_PK || null,
    };
  }),

  /** Get product catalog */
  products: publicProcedure.query(() => {
    return GANE_PRODUCTS.map((p) => ({
      id: p.id,
      name: p.name,
      nameHe: p.nameHe,
      description: p.description,
      descriptionHe: p.descriptionHe,
      priceAmount: p.priceAmount,
      currency: p.currency,
      interval: p.interval || null,
      features: p.features,
    }));
  }),

  /** Create Stripe checkout session */
  createCheckout: protectedProcedure
    .input(
      z.object({
        productId: z.string(),
        origin: z.string().url(),
      })
    )
    .mutation(async ({ ctx, input }) => {
      if (!isStripeConfigured()) {
        throw new TRPCError({
          code: "PRECONDITION_FAILED",
          message: "Stripe is not configured. Please add STRIPE_SECRET_KEY in Settings → Payment.",
        });
      }

      const result = await createCheckoutSession({
        productId: input.productId,
        userId: ctx.user.id,
        userEmail: ctx.user.email || "",
        userName: ctx.user.name || "",
        origin: input.origin,
      });

      if (!result) {
        throw new TRPCError({
          code: "BAD_REQUEST",
          message: "Invalid product or checkout creation failed",
        });
      }

      // Record pending payment event
      const db = await getDb();
      if (db) {
        await db.insert(paymentEvents).values({
          eventId: result.sessionId,
          userId: ctx.user.id,
          type: "subscription",
          amount: 0,
          currency: "USD",
          provider: "stripe",
          status: "pending",
          externalRef: result.sessionId,
          metadata: JSON.stringify({ productId: input.productId }),
        });
      }

      return { url: result.url, sessionId: result.sessionId };
    }),

  /** Get wallet summary — balance computed from all transactions */
  getWallet: protectedProcedure.query(async ({ ctx }) => {
    const db = await getDb();
    if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "DB unavailable" });

    const userId = ctx.user.id;

    const [result] = await db
      .select({
        totalSpent: sql<number>`COALESCE(SUM(CASE WHEN ${paymentEvents.type} != 'refund' AND ${paymentEvents.status} = 'completed' THEN ${paymentEvents.amount} ELSE 0 END), 0)`,
        totalRefunded: sql<number>`COALESCE(SUM(CASE WHEN ${paymentEvents.type} = 'refund' AND ${paymentEvents.status} = 'completed' THEN ${paymentEvents.amount} ELSE 0 END), 0)`,
        transactionCount: sql<number>`COUNT(*)`,
      })
      .from(paymentEvents)
      .where(eq(paymentEvents.userId, userId));

    return {
      balance: Number(result.totalRefunded) - Number(result.totalSpent),
      totalSpent: Number(result.totalSpent),
      totalRefunded: Number(result.totalRefunded),
      transactionCount: Number(result.transactionCount),
      currency: "₪",
      stripeConfigured: isStripeConfigured(),
    };
  }),

  /** List payment events with optional filters */
  list: protectedProcedure
    .input(
      z.object({
        limit: z.number().min(1).max(100).default(20),
        offset: z.number().min(0).default(0),
        type: z.enum(["toll", "parking", "fuel", "charging", "subscription", "fine", "refund"]).optional(),
        status: z.enum(["pending", "completed", "failed", "refunded"]).optional(),
        from: z.number().optional(),
        to: z.number().optional(),
      }).optional()
    )
    .query(async ({ ctx, input }) => {
      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "DB unavailable" });

      const userId = ctx.user.id;
      const limit = input?.limit ?? 20;
      const offset = input?.offset ?? 0;

      const conditions = [eq(paymentEvents.userId, userId)];
      if (input?.type) conditions.push(eq(paymentEvents.type, input.type));
      if (input?.status) conditions.push(eq(paymentEvents.status, input.status));
      if (input?.from) conditions.push(gte(paymentEvents.createdAt, new Date(input.from)));
      if (input?.to) conditions.push(lte(paymentEvents.createdAt, new Date(input.to)));

      const rows = await db
        .select()
        .from(paymentEvents)
        .where(and(...conditions))
        .orderBy(desc(paymentEvents.createdAt))
        .limit(limit)
        .offset(offset);

      const [countResult] = await db
        .select({ count: sql<number>`COUNT(*)` })
        .from(paymentEvents)
        .where(and(...conditions));

      return {
        items: rows.map((r) => ({
          id: r.id,
          eventId: r.eventId,
          type: r.type,
          amount: Number(r.amount),
          currency: r.currency || "₪",
          provider: r.provider || "system",
          status: r.status,
          tripId: r.tripId,
          externalRef: r.externalRef,
          createdAt: r.createdAt?.getTime() ?? Date.now(),
        })),
        total: Number(countResult.count),
        hasMore: offset + limit < Number(countResult.count),
      };
    }),

  /** Record a new payment event */
  record: protectedProcedure
    .input(
      z.object({
        type: z.enum(["toll", "parking", "fuel", "charging", "subscription", "fine", "refund"]),
        amount: z.number().min(0),
        currency: z.string().default("₪"),
        provider: z.string().default("system"),
        status: z.enum(["pending", "completed", "failed", "refunded"]).default("completed"),
        tripId: z.string().optional(),
      })
    )
    .mutation(async ({ ctx, input }) => {
      const db = await getDb();
      if (!db) throw new TRPCError({ code: "INTERNAL_SERVER_ERROR", message: "DB unavailable" });

      const eventId = crypto.randomUUID();

      await db.insert(paymentEvents).values({
        eventId,
        userId: ctx.user.id,
        type: input.type,
        amount: input.amount,
        currency: input.currency,
        provider: input.provider,
        status: input.status,
        tripId: input.tripId || null,
        metadata: null,
      });

      return { eventId, success: true };
    }),
});
