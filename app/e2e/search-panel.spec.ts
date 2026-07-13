import { test, expect } from "@playwright/test";

test.describe("Search Panel", () => {
  test("page loads and serves valid HTML", async ({ request }) => {
    const response = await request.get("/");
    expect(response.status()).toBe(200);
    const html = await response.text();
    expect(html).toContain('id="root"');
  });

  test("page includes JavaScript bundles", async ({ request }) => {
    const response = await request.get("/");
    const html = await response.text();
    // Vite injects script tags for the app
    expect(html).toContain("<script");
    expect(html).toContain("src=");
  });
});
