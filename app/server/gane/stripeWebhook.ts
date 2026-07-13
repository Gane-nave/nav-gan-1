/**
 * G.A.N.E — Stripe Webhook Handler
 * ==================================
 * Express route handler for /api/stripe/webhook
 * Verifies Stripe webhook signatures and processes payment events.
 *
 * MUST be registered BEFORE express.json() middleware to receive raw body.
 */

import type { Request, Response } from "express";
import { verifyWebhookEvent, isStripeConfigured } from "./stripe";
import { getDb } from "../db";
import { paymentEvents } from "../../drizzle/schema";
import { eq } from "drizzle-orm";

export async function stripeWebhookHandler(req: Request, res: Response): Promise<void> {
  if (!isStripeConfigured()) {
    res.status(503).json({ error: "Stripe not configured" });
    return;
  }

  const signature = req.headers["stripe-signature"] as string;
  if (!signature) {
    res.status(400).json({ error: "Missing stripe-signature header" });
    return;
  }

  // req.body is raw Buffer because we use express.raw() for this route
  const event = verifyWebhookEvent(req.body, signature);
  if (!event) {
    res.status(400).json({ error: "Webhook signature verification failed" });
    return;
  }

  // Handle test events
  if (event.id.startsWith("evt_test_")) {
    console.log("[Stripe Webhook] Test event detected, returning verification response");
    res.json({ verified: true });
    return;
  }

  console.log(`[Stripe Webhook] Processing event: ${event.type} (${event.id})`);

  try {
    const db = await getDb();

    switch (event.type) {
      case "checkout.session.completed": {
        const session = event.data.object as unknown as Record<string, unknown>;
        const sessionId = session.id as string;
        const userId = parseInt((session.metadata as Record<string, string>)?.user_id || "0", 10);
        const amountTotal = (session.amount_total as number) || 0;
        const currency = (session.currency as string) || "usd";

        if (db && userId) {
          // Update existing pending event or create new one
          const existing = await db
            .select()
            .from(paymentEvents)
            .where(eq(paymentEvents.externalRef, sessionId))
            .limit(1);

          if (existing.length > 0) {
            await db
              .update(paymentEvents)
              .set({
                status: "completed",
                amount: amountTotal / 100, // Convert from cents
                currency: currency.toUpperCase(),
              })
              .where(eq(paymentEvents.externalRef, sessionId));
          } else {
            await db.insert(paymentEvents).values({
              eventId: event.id,
              userId,
              type: "subscription",
              amount: amountTotal / 100,
              currency: currency.toUpperCase(),
              provider: "stripe",
              status: "completed",
              externalRef: sessionId,
              metadata: JSON.stringify({
                stripeEventId: event.id,
                productId: (session.metadata as Record<string, string>)?.product_id,
              }),
            });
          }
        }
        break;
      }

      case "payment_intent.succeeded": {
        const intent = event.data.object as unknown as Record<string, unknown>;
        const userId = parseInt((intent.metadata as Record<string, string>)?.gane_user_id || "0", 10);
        const amount = (intent.amount as number) || 0;
        const currency = (intent.currency as string) || "usd";

        if (db && userId) {
          await db.insert(paymentEvents).values({
            eventId: event.id,
            userId,
            type: "toll", // Default type for direct payments
            amount: amount / 100,
            currency: currency.toUpperCase(),
            provider: "stripe",
            status: "completed",
            externalRef: intent.id as string,
            metadata: JSON.stringify({ stripeEventId: event.id }),
          });
        }
        break;
      }

      case "charge.refunded": {
        const charge = event.data.object as unknown as Record<string, unknown>;
        const userId = parseInt((charge.metadata as Record<string, string>)?.gane_user_id || "0", 10);
        const amountRefunded = (charge.amount_refunded as number) || 0;
        const currency = (charge.currency as string) || "usd";

        if (db && userId) {
          await db.insert(paymentEvents).values({
            eventId: event.id,
            userId,
            type: "refund",
            amount: amountRefunded / 100,
            currency: currency.toUpperCase(),
            provider: "stripe",
            status: "completed",
            externalRef: charge.id as string,
            metadata: JSON.stringify({ stripeEventId: event.id }),
          });
        }
        break;
      }

      case "invoice.paid": {
        const invoice = event.data.object as unknown as Record<string, unknown>;
        const customerId = invoice.customer as string;
        const amountPaid = (invoice.amount_paid as number) || 0;
        const currency = (invoice.currency as string) || "usd";

        // For subscription renewals
        if (db) {
          await db.insert(paymentEvents).values({
            eventId: event.id,
            userId: 0, // Will be matched via customer ID
            type: "subscription",
            amount: amountPaid / 100,
            currency: currency.toUpperCase(),
            provider: "stripe",
            status: "completed",
            externalRef: invoice.id as string,
            metadata: JSON.stringify({
              stripeEventId: event.id,
              stripeCustomerId: customerId,
            }),
          });
        }
        break;
      }

      default:
        console.log(`[Stripe Webhook] Unhandled event type: ${event.type}`);
    }

    res.json({ received: true, eventId: event.id });
  } catch (err) {
    console.error("[Stripe Webhook] Processing error:", err);
    res.status(500).json({ error: "Webhook processing failed" });
  }
}
