/**
 * Stripe API Key Validation Test
 * Verifies that Stripe keys are set and the API connection works.
 */
import { describe, it, expect } from "vitest";

describe.skipIf(!(process.env.STRIPE_SECRET_KEY || process.env.STRIPE_SK))("Stripe Key Validation", () => {
  const sk = process.env.STRIPE_SECRET_KEY || process.env.STRIPE_SK;
  const pk = process.env.VITE_STRIPE_PUBLISHABLE_KEY || process.env.VITE_STRIPE_PK;

  it("should have Stripe Secret Key set", () => {
    expect(sk).toBeTruthy();
    expect(sk!.startsWith("sk_test_") || sk!.startsWith("sk_live_")).toBe(true);
  });

  it("should have Stripe Publishable Key set", () => {
    expect(pk).toBeTruthy();
    expect(pk!.startsWith("pk_test_") || pk!.startsWith("pk_live_")).toBe(true);
  });

  it("should connect to Stripe API successfully", async () => {
    // Use a lightweight API call to verify the key works
    const response = await fetch("https://api.stripe.com/v1/balance", {
      headers: {
        Authorization: `Bearer ${sk}`,
      },
    });
    // 200 = valid key, 401 = invalid key
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.object).toBe("balance");
  });
});
