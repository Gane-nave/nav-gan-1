/**
 * Tests for shared/contracts/vehicleRouting.ts
 * Verifies profile selection, avoid accumulation, and constraints pass-through.
 */
import { describe, it, expect } from "vitest";
import {
  adaptRouteRequestForVehicle,
  avoidFromRestrictions,
  VEHICLE_TO_PROFILE,
  type VehicleProfileLike,
  type RouteRequestLike,
} from "../../shared/contracts/vehicleRouting";

const CAR: VehicleProfileLike = {
  type: "car",
  dimensions: { length: 4.5, width: 1.8, height: 1.5, wheelbase: 2.7 },
  weight: { empty: 1200, loaded: 1500, axles: 2 },
  restrictions: {
    maxGrade: 25,
    minTurningRadius: 5.5,
    avoidTunnels: false,
    avoidBridges: false,
    hazmatAllowed: false,
    restrictions: [],
  },
};

const HEAVY_TRUCK: VehicleProfileLike = {
  type: "truck_heavy",
  dimensions: { length: 16.5, width: 2.55, height: 4.0, wheelbase: 6.5 },
  weight: { empty: 12000, loaded: 44000, axles: 5 },
  restrictions: {
    maxGrade: 8,
    minTurningRadius: 14,
    avoidTunnels: true,
    avoidBridges: true,
    hazmatAllowed: false,
    restrictions: ["low_bridges", "weight_limits"],
  },
};

const BICYCLE: VehicleProfileLike = {
  type: "bicycle",
  dimensions: { length: 1.8, width: 0.5, height: 1.1, wheelbase: 1.0 },
  weight: { empty: 12, loaded: 110, axles: 2 },
  restrictions: {
    maxGrade: 15,
    minTurningRadius: 1.5,
    avoidTunnels: false,
    avoidBridges: false,
    hazmatAllowed: false,
    restrictions: [],
  },
};

const PEDESTRIAN: VehicleProfileLike = {
  ...BICYCLE,
  type: "pedestrian",
};

function baseRequest(): RouteRequestLike {
  return { profile: "driving" };
}

describe("VEHICLE_TO_PROFILE", () => {
  it("maps every VehicleType to a RouteProfile", () => {
    for (const [vehicle, profile] of Object.entries(VEHICLE_TO_PROFILE)) {
      expect(["driving", "walking", "cycling", "trucking"]).toContain(profile);
      expect(typeof vehicle).toBe("string");
    }
  });

  it("routes heavy trucks, medium trucks, buses and military through trucking", () => {
    expect(VEHICLE_TO_PROFILE.truck_heavy).toBe("trucking");
    expect(VEHICLE_TO_PROFILE.truck_medium).toBe("trucking");
    expect(VEHICLE_TO_PROFILE.bus).toBe("trucking");
    expect(VEHICLE_TO_PROFILE.military).toBe("trucking");
  });

  it("routes bicycles via cycling and pedestrians via walking", () => {
    expect(VEHICLE_TO_PROFILE.bicycle).toBe("cycling");
    expect(VEHICLE_TO_PROFILE.pedestrian).toBe("walking");
  });
});

describe("avoidFromRestrictions", () => {
  it("returns tunnels when avoidTunnels is true", () => {
    expect(avoidFromRestrictions(HEAVY_TRUCK.restrictions)).toContain("tunnels");
  });
  it("returns an empty list for an unrestricted car", () => {
    expect(avoidFromRestrictions(CAR.restrictions)).toEqual([]);
  });
});

describe("adaptRouteRequestForVehicle", () => {
  it("leaves a driving request alone for a car", () => {
    const out = adaptRouteRequestForVehicle(baseRequest(), CAR);
    expect(out.request.profile).toBe("driving");
    expect(out.request.avoid).toEqual([]);
    expect(out.notes).toEqual([]);
  });

  it("rewrites profile and adds tunnels avoid for a heavy truck", () => {
    const out = adaptRouteRequestForVehicle(baseRequest(), HEAVY_TRUCK);
    expect(out.request.profile).toBe("trucking");
    expect(out.request.avoid).toContain("tunnels");
    expect(out.notes.some((n) => n.includes("trucking"))).toBe(true);
    expect(out.notes.some((n) => n.includes("avoidBridges"))).toBe(true);
    expect(out.notes.some((n) => n.includes("hazmat"))).toBe(true);
  });

  it("preserves pre-existing avoid entries and deduplicates", () => {
    const out = adaptRouteRequestForVehicle(
      { ...baseRequest(), avoid: ["tolls", "tunnels"] },
      HEAVY_TRUCK,
    );
    expect(out.request.avoid!.sort()).toEqual(["tolls", "tunnels"]);
  });

  it("switches profile to cycling for a bicycle and walking for a pedestrian", () => {
    expect(adaptRouteRequestForVehicle(baseRequest(), BICYCLE).request.profile).toBe("cycling");
    expect(adaptRouteRequestForVehicle(baseRequest(), PEDESTRIAN).request.profile).toBe("walking");
  });

  it("emits a constraints envelope with loaded weight + dimensions", () => {
    const out = adaptRouteRequestForVehicle(baseRequest(), HEAVY_TRUCK);
    expect(out.constraints).toEqual({
      type: "truck_heavy",
      heightM: 4.0,
      widthM: 2.55,
      lengthM: 16.5,
      weightKg: 44000,
      axles: 5,
      minTurningRadiusM: 14,
      hazmatAllowed: false,
    });
  });
});
