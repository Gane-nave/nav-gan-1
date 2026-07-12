/**
 * Validates the GOOGLE_API_KEY by calling the Google Maps Geocoding API
 */
import { describe, it, expect } from "vitest";

describe.skipIf(!process.env.GOOGLE_API_KEY)("Google API Key Validation", () => {
  it("should have GOOGLE_API_KEY set in environment", () => {
    // The key is injected by the platform, check it exists
    const key = process.env.GOOGLE_API_KEY;
    expect(key).toBeDefined();
    expect(key!.length).toBeGreaterThan(10);
    expect(key).toMatch(/^AIza/); // Google API keys start with AIza
  });

  it("should successfully call Google Geocoding API", async () => {
    const key = process.env.GOOGLE_API_KEY;
    if (!key) {
      console.log("Skipping: GOOGLE_API_KEY not set");
      return;
    }

    const response = await fetch(
      `https://maps.googleapis.com/maps/api/geocode/json?address=Tel+Aviv&key=${key}`
    );
    const data = await response.json();
    
    // If key is valid, status should be OK or ZERO_RESULTS
    // If key is invalid, status would be REQUEST_DENIED
    expect(response.status).toBe(200);
    expect(["OK", "ZERO_RESULTS"]).toContain(data.status);
  }, 10000);
});
