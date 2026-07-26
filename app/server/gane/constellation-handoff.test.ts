/**
 * Tests for shared/contracts/constellationHandoff.ts
 */
import { describe, it, expect } from "vitest";
import {
  MIN_SATELLITES_FOR_3D_FIX,
  SPOOFING_ALERT_THRESHOLD,
  deltaToAlerts,
  diffSnapshots,
  type ConstellationId,
  type ConstellationSnapshot,
} from "../../shared/contracts/constellationHandoff";

function snap(
  available: ConstellationId[],
  best: ConstellationId,
  opts: Partial<Pick<ConstellationSnapshot, "spoofingRisk" | "totalUsed" | "pdop">> = {},
): ConstellationSnapshot {
  return {
    available: new Set(available),
    best,
    spoofingRisk: opts.spoofingRisk ?? 0,
    totalUsed: opts.totalUsed ?? 8,
    pdop: opts.pdop ?? 2.0,
  };
}

describe("diffSnapshots", () => {
  it("detects lost and gained constellations", () => {
    const prev = snap(["GPS", "GLONASS", "GALILEO"], "GPS");
    const next = snap(["GPS", "BEIDOU"], "GPS");
    const d = diffSnapshots(prev, next);
    expect(d.lost).toEqual(["GALILEO", "GLONASS"]);
    expect(d.gained).toEqual(["BEIDOU"]);
    expect(d.bestChanged).toBe(false);
  });

  it("detects best-constellation change", () => {
    const prev = snap(["GPS", "GALILEO"], "GPS");
    const next = snap(["GPS", "GALILEO"], "GALILEO");
    const d = diffSnapshots(prev, next);
    expect(d.bestChanged).toBe(true);
    expect(d.fromBest).toBe("GPS");
    expect(d.toBest).toBe("GALILEO");
  });

  it("flags spoofing risk crossing the threshold in both directions", () => {
    const low = snap(["GPS"], "GPS", { spoofingRisk: 0.1 });
    const high = snap(["GPS"], "GPS", { spoofingRisk: SPOOFING_ALERT_THRESHOLD });
    expect(diffSnapshots(low, high).spoofingRoseAboveThreshold).toBe(true);
    expect(diffSnapshots(high, low).spoofingFellBelowThreshold).toBe(true);
  });

  it("flags crossing the 3D-fix floor in both directions", () => {
    const ok = snap(["GPS"], "GPS", { totalUsed: MIN_SATELLITES_FOR_3D_FIX });
    const low = snap(["GPS"], "GPS", { totalUsed: 3 });
    expect(diffSnapshots(ok, low).crossedMinUsedFloor).toBe(true);
    expect(diffSnapshots(low, ok).restoredMinUsedFloor).toBe(true);
  });

  it("returns empty lost/gained when nothing changed", () => {
    const s = snap(["GPS", "GALILEO"], "GPS");
    const d = diffSnapshots(s, s);
    expect(d.lost).toEqual([]);
    expect(d.gained).toEqual([]);
    expect(d.bestChanged).toBe(false);
  });
});

describe("deltaToAlerts", () => {
  it("returns empty list for a no-op delta", () => {
    const s = snap(["GPS"], "GPS");
    expect(deltaToAlerts(diffSnapshots(s, s))).toEqual([]);
  });

  it("orders critical alerts first (fix-loss before best-changed)", () => {
    const prev = snap(["GPS", "GALILEO"], "GPS", { totalUsed: 8 });
    const next = snap(["GALILEO"], "GALILEO", { totalUsed: 3 });
    const alerts = deltaToAlerts(diffSnapshots(prev, next));
    expect(alerts[0].kind).toBe("below-3d-fix-floor");
    const kinds = alerts.map((a) => a.kind);
    expect(kinds).toContain("constellation-lost");
    expect(kinds).toContain("best-changed");
    // below-3d-fix-floor must come before best-changed
    expect(kinds.indexOf("below-3d-fix-floor")).toBeLessThan(
      kinds.indexOf("best-changed"),
    );
  });

  it("emits spoofing-risk-rose when threshold is crossed upward", () => {
    const prev = snap(["GPS"], "GPS", { spoofingRisk: 0.0 });
    const next = snap(["GPS"], "GPS", { spoofingRisk: 0.9 });
    const alerts = deltaToAlerts(diffSnapshots(prev, next));
    expect(alerts.map((a) => a.kind)).toContain("spoofing-risk-rose");
  });

  it("emits constellation-regained with the correct ids", () => {
    const prev = snap(["GPS"], "GPS");
    const next = snap(["GPS", "NAVIC", "QZSS"], "GPS");
    const alerts = deltaToAlerts(diffSnapshots(prev, next));
    const regained = alerts.find((a) => a.kind === "constellation-regained");
    expect(regained).toBeDefined();
    if (regained && regained.kind === "constellation-regained") {
      expect(regained.ids).toEqual(["NAVIC", "QZSS"]);
    }
  });
});
