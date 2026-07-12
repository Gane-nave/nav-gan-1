import { test, expect } from "@playwright/test";

test.describe("Admin Panel Access Control", () => {
  test("/admin route returns valid page", async ({ request }) => {
    const response = await request.get("/admin");
    expect(response.status()).toBe(200);
    const html = await response.text();
    expect(html).toContain('id="root"');
  });

  test("/admin route does not expose admin API without auth", async ({ request }) => {
    const response = await request.get("/api/trpc/admin.getUsers");
    expect(response.status()).toBeGreaterThanOrEqual(400);
  });
});
