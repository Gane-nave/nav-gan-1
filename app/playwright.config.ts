import { defineConfig } from "@playwright/test";

/**
 * G.A.N.E — Playwright E2E Test Configuration
 * =============================================
 * Runs against the dev server on port 3000 (already running via webdev).
 * For CI, build and start a production server on port 4173.
 * Uses Chromium only for speed.
 */
export default defineConfig({
  testDir: "./e2e",
  timeout: 60_000,
  retries: process.env.CI ? 2 : 1,
  reporter: process.env.CI ? "html" : "list",
  fullyParallel: true,
  workers: process.env.CI ? 1 : undefined,
  use: {
    baseURL: process.env.CI ? "http://localhost:4173" : "http://localhost:3000",
    headless: true,
    viewport: { width: 1280, height: 720 },
    screenshot: "only-on-failure",
    video: "retain-on-failure",
    trace: "on-first-retry",
    navigationTimeout: 30_000,
  },
  // In CI: pnpm build && PORT=4173 node dist/index.js &
  // Locally: dev server is already running via webdev
});
