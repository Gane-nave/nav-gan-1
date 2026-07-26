import { describe, it, expect } from "vitest";
import { GeospatialIndex } from "../../client/src/engine/geospatialIndex";
import { PolicyEngine } from "../../client/src/engine/policyEngine";

describe("regressions found by adversarial review", () => {
  it("findNearby returns a point 10.7 km away at default precision", () => {
    const idx = new GeospatialIndex();
    idx.insert({ id: "tlv", lat: 32.0853, lon: 34.7818 });
    idx.insert({ id: "herzliya", lat: 32.1624, lon: 34.8443 });
    const ids = idx.findNearby(32.0853, 34.7818, 25000).map((p: any) => p.id ?? p.point?.id);
    expect(ids).toContain("herzliya");
  });

  it("policy cache does not replay a verdict across different destinations", () => {
    const mk = (dest: { lat: number; lon: number }, hasTunnels: boolean) => ({
      vehicle: { class: "heavy_truck" as any, height: 4.2, weight: 26000, hasHazmat: true },
      route: { origin: { lat: 32.0, lon: 34.7 }, destination: dest, hasTunnels },
      driver: {},
      time: { hour: 10, dayOfWeek: 2 },
      environment: {},
    });
    const engine = new PolicyEngine();
    const clean = engine.evaluate(mk({ lat: 32.1, lon: 34.8 }, false) as any);
    const tunnels = engine.evaluate(mk({ lat: 31.5, lon: 35.2 }, true) as any);
    expect(tunnels).not.toBe(clean);
    const fresh = new PolicyEngine().evaluate(mk({ lat: 31.5, lon: 35.2 }, true) as any);
    expect(tunnels.allowed).toBe(fresh.allowed);
  });
});
