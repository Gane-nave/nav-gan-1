import { test, expect } from "@playwright/test";

test.describe("Collaboration Invite Join Flow", () => {
  test("join page serves valid HTML for a token URL", async ({ request }) => {
    const response = await request.get("/collab/join/test-token-12345");
    expect(response.status()).toBe(200);
    const html = await response.text();
    expect(html).toContain('id="root"');
  });

  test("join page handles invalid token gracefully", async ({ page }) => {
    const errors: string[] = [];
    page.on("pageerror", (err) => errors.push(err.message));
    await page.goto("/collab/join/invalid-token-xyz", {
      waitUntil: "load",
      timeout: 30000,
    });
    await page.waitForTimeout(3000);
    const criticalErrors = errors.filter(
      (e) =>
        !e.includes("Google Maps") &&
        !e.includes("maps.googleapis") &&
        !e.includes("ResizeObserver") &&
        !e.includes("Failed to fetch")
    );
    expect(criticalErrors).toHaveLength(0);
  });
});
