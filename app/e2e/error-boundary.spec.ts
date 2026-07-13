import { test, expect } from "@playwright/test";

test.describe("Error Boundary & Resilience", () => {
  test("app loads without critical JavaScript errors", async ({ page }) => {
    const errors: string[] = [];
    page.on("pageerror", (err) => errors.push(err.message));
    await page.goto("/", { waitUntil: "load", timeout: 30000 });
    await page.waitForTimeout(5000);
    const criticalErrors = errors.filter(
      (e) =>
        !e.includes("Google Maps") &&
        !e.includes("maps.googleapis") &&
        !e.includes("ResizeObserver") &&
        !e.includes("ChunkLoadError") &&
        !e.includes("Failed to fetch")
    );
    expect(criticalErrors).toHaveLength(0);
  });

  test("invalid route returns page without server crash", async ({ request }) => {
    const response = await request.get("/nonexistent-route-12345");
    // SPA returns 200 for all routes (client-side routing)
    expect(response.status()).toBe(200);
  });

  test("metrics endpoint returns valid Prometheus format", async ({ request }) => {
    const response = await request.get("/metrics");
    expect(response.status()).toBe(200);
    const text = await response.text();
    expect(text).toContain("# HELP");
    expect(text).toContain("# TYPE");
    expect(text).toContain("process_cpu_");
    expect(text).toContain("nodejs_");
  });

  test("tRPC endpoint returns proper error for invalid procedure", async ({ request }) => {
    const response = await request.get("/api/trpc/nonexistent.procedure");
    expect(response.status()).toBeLessThan(500);
  });

  test("health endpoint confirms system status", async ({ request }) => {
    const response = await request.get("/api/health");
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.status).toBe("ok");
    expect(body).toHaveProperty("uptime");
    expect(body).toHaveProperty("memory");
    expect(body).toHaveProperty("services");
  });
});
