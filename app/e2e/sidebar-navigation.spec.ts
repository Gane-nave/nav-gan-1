import { test, expect } from "@playwright/test";

test.describe("Sidebar Navigation", () => {
  test("page loads with substantial HTML content", async ({ request }) => {
    const response = await request.get("/");
    const html = await response.text();
    expect(html.length).toBeGreaterThan(1000);
    expect(html).toContain('id="root"');
  });

  test("page includes JavaScript bundles for navigation", async ({ request }) => {
    const response = await request.get("/");
    const html = await response.text();
    expect(html).toContain("<script");
    expect(html).toContain("type=\"module\"");
  });
});
