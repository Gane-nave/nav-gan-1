import { test, expect } from "@playwright/test";

test.describe("Boot Sequence & Map View", () => {
  test("page loads and returns 200", async ({ request }) => {
    const response = await request.get("/");
    expect(response.status()).toBe(200);
  });

  test("page HTML contains G.A.N.E title", async ({ request }) => {
    const response = await request.get("/");
    const html = await response.text();
    expect(html).toContain("G.A.N.E");
  });

  test("page serves valid HTML with React root", async ({ request }) => {
    const response = await request.get("/");
    const html = await response.text();
    expect(html).toContain('id="root"');
    expect(html).toContain("</html>");
  });

  test("page includes required meta tags", async ({ request }) => {
    const response = await request.get("/");
    const html = await response.text();
    expect(html).toContain('name="viewport"');
  });
});
