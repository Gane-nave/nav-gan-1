import { test as base, expect, type Page } from "@playwright/test";

/**
 * Shared E2E test fixtures for G.A.N.E
 *
 * The app has a QuantumBoot animation (~3.3s) that uses canvas rendering.
 * In headless Chromium, canvas animations may not render properly,
 * leaving the page blank. These fixtures handle boot gracefully.
 */

/**
 * Wait for the app to be ready after navigation.
 * Handles the boot sequence by waiting for content to appear.
 */
export async function waitForAppReady(page: Page, timeout = 15000) {
  await page.waitForSelector("#root", { state: "attached", timeout });
  // Wait for React to hydrate and boot to complete
  // The boot takes ~3.3s, add buffer for headless rendering
  await page.waitForTimeout(5000);
  // Verify the app has rendered meaningful content
  const rootHTML = await page.evaluate(() => document.getElementById("root")?.innerHTML || "");
  return rootHTML;
}

/**
 * Navigate to a page and wait for it to be ready.
 * Returns the page response.
 */
export async function navigateAndWait(page: Page, path: string) {
  const response = await page.goto(path, { waitUntil: "commit", timeout: 30000 });
  return response;
}

export { base as test, expect };
