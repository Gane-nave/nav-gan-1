import { test, expect } from "@playwright/test";

test.describe("Settings Panel", () => {
  test("page loads and serves valid HTML", async ({ request }) => {
    const response = await request.get("/");
    expect(response.status()).toBe(200);
    const html = await response.text();
    expect(html).toContain('id="root"');
  });

  test("page includes CSS stylesheets", async ({ request }) => {
    const response = await request.get("/");
    const html = await response.text();
    // Vite injects CSS links
    expect(html).toContain("stylesheet");
  });
});
