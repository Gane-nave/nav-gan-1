/**
 * Tests for shared/contracts/fallbackSeverity.ts
 * Pure functions — no DOM / React dependencies.
 */
import { describe, it, expect } from "vitest";
import {
  classifyTransition,
  SEVERITY_LABEL,
  tierToSeverity,
  type ProviderTier,
} from "../../shared/contracts/fallbackSeverity";

describe("tierToSeverity", () => {
  it("maps tier 1-2 (GNSS / SBAS) to green / OPTIMAL", () => {
    expect(tierToSeverity(1)).toBe("green");
    expect(tierToSeverity(2)).toBe("green");
    expect(SEVERITY_LABEL[tierToSeverity(1)]).toBe("OPTIMAL");
  });

  it("maps tier 3 (WiFi / Cell) to yellow / DEGRADED", () => {
    expect(tierToSeverity(3)).toBe("yellow");
    expect(SEVERITY_LABEL[tierToSeverity(3)]).toBe("DEGRADED");
  });

  it("maps tier 4 (IMU / PDR / Visual) to orange / FALLBACK", () => {
    expect(tierToSeverity(4)).toBe("orange");
    expect(SEVERITY_LABEL[tierToSeverity(4)]).toBe("FALLBACK");
  });

  it("maps tier 5 (cached / IP) to red / LAST_RESORT", () => {
    expect(tierToSeverity(5)).toBe("red");
    expect(SEVERITY_LABEL[tierToSeverity(5)]).toBe("LAST_RESORT");
  });
});

describe("classifyTransition", () => {
  it("returns unchanged when tiers are equal", () => {
    expect(classifyTransition(1, 1)).toBe("unchanged");
    expect(classifyTransition(5, 5)).toBe("unchanged");
  });

  it("classifies GNSS -> network correctly", () => {
    expect(classifyTransition(1, 3)).toBe("gnss-to-network");
    expect(classifyTransition(2, 3)).toBe("gnss-to-network");
  });

  it("classifies network -> dead-reckoning correctly", () => {
    expect(classifyTransition(3, 4)).toBe("network-to-dead-reckoning");
  });

  it("classifies dead-reckoning -> cached correctly", () => {
    expect(classifyTransition(4, 5)).toBe("dead-reckoning-to-cached");
  });

  it("classifies any return to tier 1-2 as recovered-to-gnss", () => {
    expect(classifyTransition(5, 1)).toBe("recovered-to-gnss");
    expect(classifyTransition(4, 2)).toBe("recovered-to-gnss");
    expect(classifyTransition(3, 1)).toBe("recovered-to-gnss");
  });

  it("classifies generic improvements as improved", () => {
    expect(classifyTransition(5, 4)).toBe("improved");
    expect(classifyTransition(5, 3)).toBe("improved");
    expect(classifyTransition(4, 3)).toBe("improved");
  });

  it("classifies generic degrades as degraded", () => {
    expect(classifyTransition(1, 5)).toBe("degraded");
    expect(classifyTransition(2, 4)).toBe("degraded");
    expect(classifyTransition(2, 5)).toBe("degraded");
  });

  it("covers every ordered pair exhaustively", () => {
    const tiers: ProviderTier[] = [1, 2, 3, 4, 5];
    for (const from of tiers) {
      for (const to of tiers) {
        expect(classifyTransition(from, to)).toBeTypeOf("string");
      }
    }
  });
});
