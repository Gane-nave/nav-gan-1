import { test, expect } from "@playwright/test";

test.describe("Collaboration Panel", () => {
  test("main page serves valid HTML", async ({ request }) => {
    const response = await request.get("/");
    expect(response.status()).toBe(200);
    const html = await response.text();
    expect(html).toContain('id="root"');
    expect(html.length).toBeGreaterThan(1000);
  });

  test("API health endpoint returns ok status", async ({ request }) => {
    const response = await request.get("/api/health");
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body).toHaveProperty("status");
    expect(body.status).toBe("ok");
    expect(body).toHaveProperty("uptime");
    expect(body).toHaveProperty("memory");
  });

  test("Prometheus metrics endpoint is accessible", async ({ request }) => {
    const response = await request.get("/metrics");
    expect(response.status()).toBe(200);
    const text = await response.text();
    expect(text).toContain("# HELP");
    expect(text).toContain("# TYPE");
  });

  test("tRPC endpoint is reachable", async ({ request }) => {
    // Calling a non-existent procedure should return error, not crash
    const response = await request.get("/api/trpc/nonexistent.procedure");
    expect(response.status()).toBeGreaterThanOrEqual(400);
    expect(response.status()).toBeLessThan(500);
  });
});
