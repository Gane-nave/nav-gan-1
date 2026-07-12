/**
 * G.A.N.E — Stripe Payment Integration
 * ======================================
 * Server-side Stripe SDK wrapper providing:
 *   - Checkout session creation for subscriptions and one-time payments
 *   - Payment intent management
 *   - Webhook event verification
 *   - Customer management
 *
 * Products are defined centrally for consistency.
 */

import Stripe from "stripe";
import { ENV } from "../_core/env";

// ─── Stripe Client (lazy init) ───

let _stripe: Stripe | null = null;

export function getStripe(): Stripe | null {
  if (!ENV.stripeSecretKey) return null;
  if (!_stripe) {
    _stripe = new Stripe(ENV.stripeSecretKey, {
      apiVersion: "2026-03-25.dahlia",
      typescript: true,
    });
  }
  return _stripe;
}

export function isStripeConfigured(): boolean {
  return !!ENV.stripeSecretKey;
}

// ─── Product Definitions ───

export interface GANEProduct {
  id: string;
  name: string;
  nameHe: string;
  description: string;
  descriptionHe: string;
  priceAmount: number; // in cents
  currency: string;
  interval?: "month" | "year";
  features: string[];
}

export const GANE_PRODUCTS: GANEProduct[] = [
  {
    id: "gane_pro_monthly",
    name: "G.A.N.E Pro Monthly",
    nameHe: "G.A.N.E פרו חודשי",
    description: "Full access to all navigation features, AR HUD, AI routing, and real-time collaboration",
    descriptionHe: "גישה מלאה לכל תכונות הניווט, AR HUD, ניתוב AI, ושיתוף פעולה בזמן אמת",
    priceAmount: 2990, // $29.90
    currency: "usd",
    interval: "month",
    features: [
      "AI-powered route optimization",
      "AR Head-Up Display",
      "Real-time collaboration",
      "Multi-provider routing (Mapbox/HERE/TomTom)",
      "Offline maps & navigation",
      "Priority support",
    ],
  },
  {
    id: "gane_pro_yearly",
    name: "G.A.N.E Pro Yearly",
    nameHe: "G.A.N.E פרו שנתי",
    description: "Annual subscription with 2 months free — all Pro features included",
    descriptionHe: "מנוי שנתי עם 2 חודשים חינם — כל תכונות הפרו כלולות",
    priceAmount: 29900, // $299.00
    currency: "usd",
    interval: "year",
    features: [
      "Everything in Pro Monthly",
      "2 months free (save $59.80)",
      "Fleet management access",
      "Advanced analytics dashboard",
      "Custom route profiles",
    ],
  },
  {
    id: "gane_enterprise",
    name: "G.A.N.E Enterprise",
    nameHe: "G.A.N.E ארגוני",
    description: "Enterprise fleet management with dedicated support and SLA",
    descriptionHe: "ניהול צי ארגוני עם תמיכה ייעודית ו-SLA",
    priceAmount: 9900, // $99.00
    currency: "usd",
    interval: "month",
    features: [
      "Everything in Pro",
      "Fleet management (unlimited vehicles)",
      "Command center access",
      "Custom integrations",
      "Dedicated support & SLA",
      "On-premise deployment option",
    ],
  },
  {
    id: "gane_toll_pass",
    name: "Toll Pass Top-Up",
    nameHe: "טעינת מעבר אגרה",
    description: "One-time toll pass credit for highway and bridge tolls",
    descriptionHe: "טעינת קרדיט חד-פעמית לאגרות כבישים וגשרים",
    priceAmount: 5000, // $50.00
    currency: "usd",
    features: [
      "Automatic toll payment",
      "Works on all supported highways",
      "Real-time balance tracking",
    ],
  },
];

// ─── Checkout Session Creation ───

export interface CreateCheckoutParams {
  productId: string;
  userId: number;
  userEmail: string;
  userName: string;
  origin: string;
  successPath?: string;
  cancelPath?: string;
}

export async function createCheckoutSession(params: CreateCheckoutParams): Promise<{ url: string; sessionId: string } | null> {
  const stripe = getStripe();
  if (!stripe) return null;

  const product = GANE_PRODUCTS.find((p) => p.id === params.productId);
  if (!product) return null;

  const isSubscription = !!product.interval;

  const session = await stripe.checkout.sessions.create({
    mode: isSubscription ? "subscription" : "payment",
    client_reference_id: params.userId.toString(),
    customer_email: params.userEmail,
    allow_promotion_codes: true,
    metadata: {
      user_id: params.userId.toString(),
      customer_email: params.userEmail,
      customer_name: params.userName,
      product_id: product.id,
    },
    line_items: [
      {
        price_data: {
          currency: product.currency,
          product_data: {
            name: product.name,
            description: product.description,
          },
          unit_amount: product.priceAmount,
          ...(isSubscription
            ? { recurring: { interval: product.interval! } }
            : {}),
        },
        quantity: 1,
      },
    ],
    success_url: `${params.origin}${params.successPath || "/payments?status=success&session_id={CHECKOUT_SESSION_ID}"}`,
    cancel_url: `${params.origin}${params.cancelPath || "/payments?status=cancelled"}`,
  });

  return session.url ? { url: session.url, sessionId: session.id } : null;
}

// ─── Webhook Verification ───

export function verifyWebhookEvent(
  payload: Buffer | string,
  signature: string
): Stripe.Event | null {
  const stripe = getStripe();
  if (!stripe || !ENV.stripeWebhookSecret) return null;

  try {
    return stripe.webhooks.constructEvent(
      payload,
      signature,
      ENV.stripeWebhookSecret
    );
  } catch (err) {
    console.error("[Stripe] Webhook verification failed:", err);
    return null;
  }
}

// ─── Customer Management ───

export async function getOrCreateCustomer(
  email: string,
  name: string,
  userId: number
): Promise<string | null> {
  const stripe = getStripe();
  if (!stripe) return null;

  // Search for existing customer
  const existing = await stripe.customers.list({
    email,
    limit: 1,
  });

  if (existing.data.length > 0) {
    return existing.data[0].id;
  }

  // Create new customer
  const customer = await stripe.customers.create({
    email,
    name,
    metadata: {
      gane_user_id: userId.toString(),
    },
  });

  return customer.id;
}

// ─── Payment Intent (for direct payments) ───

export async function createPaymentIntent(
  amount: number,
  currency: string,
  userId: number,
  metadata?: Record<string, string>
): Promise<{ clientSecret: string; intentId: string } | null> {
  const stripe = getStripe();
  if (!stripe) return null;

  const intent = await stripe.paymentIntents.create({
    amount,
    currency,
    metadata: {
      gane_user_id: userId.toString(),
      ...metadata,
    },
  });

  return intent.client_secret
    ? { clientSecret: intent.client_secret, intentId: intent.id }
    : null;
}
