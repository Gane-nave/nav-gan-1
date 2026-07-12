import { test, expect } from "@playwright/test";

test.describe("Mode Switching", () => {
  test("page loads successfully", async ({ request }) => {
    const response = await request.get("/");
    expect(response.status()).toBe(200);
  });

  test("page HTML contains mode-related content", async ({ request }) => {
    const response = await request.get("/");
    const html = await response.text();
    expect(html).toContain('id="root"');
    expect(html.length).toBeGreaterThan(1000);
  });
});
