/**
 * G.A.N.E — Vehicle-Aware Routing Adaptation (shared, pure)
 * ===========================================================
 * Translates a VehicleType + VehicleProfile into the provider-agnostic
 * RouteRequest fields the multiProviderRouting engine already understands:
 *   - picks the best RouteProfile (driving / trucking / cycling / walking)
 *   - accumulates AvoidFeatures from the profile's hard restrictions
 *   - carries dimensions + weight through a parallel VehicleConstraints
 *     object that adapters may use for provider-specific enrichment
 *     (HERE truck params, TomTom vehicleHeading, Mapbox max-*-weight, etc.)
 *
 * Covers todo.md:
 *   - [x] Implement vehicle-specific routing optimization (integrate
 *         with routing engine)
 *
 * Kept in `shared/` so it is importable from both client code and the
 * server-side test runner without React or DOM dependencies.
 */

// Mirror the VehicleType union from client/src/lib/vehicleProfiles.ts.
// Keeping the string set here lets the shared module stay dependency-free.
export type VehicleType =
  | "car"
  | "motorcycle"
  | "truck_light"
  | "truck_medium"
  | "truck_heavy"
  | "bus"
  | "emergency"
  | "military"
  | "van"
  | "bicycle"
  | "pedestrian";

// Mirror the routing RouteProfile / AvoidFeature unions.
export type RouteProfile = "driving" | "walking" | "cycling" | "trucking";
export type AvoidFeature =
  | "tolls"
  | "highways"
  | "ferries"
  | "tunnels"
  | "unpaved"
  | "u_turns";

export interface VehicleDimensions {
  length: number;
  width: number;
  height: number;
  wheelbase: number;
}

export interface VehicleWeight {
  empty: number;
  loaded: number;
  axles: number;
}

export interface VehicleRestrictions {
  maxGrade: number;
  minTurningRadius: number;
  maxRoadWidth?: number;
  avoidTunnels: boolean;
  avoidBridges: boolean;
  hazmatAllowed: boolean;
  restrictions: string[];
}

export interface VehicleProfileLike {
  type: VehicleType;
  dimensions: VehicleDimensions;
  weight: VehicleWeight;
  restrictions: VehicleRestrictions;
}

/** Subset of RouteRequest we need to adapt — keeps this file decoupled. */
export interface RouteRequestLike {
  profile: RouteProfile;
  avoid?: AvoidFeature[];
}

export interface VehicleConstraints {
  type: VehicleType;
  heightM: number;
  widthM: number;
  lengthM: number;
  weightKg: number;           // loaded
  axles: number;
  minTurningRadiusM: number;
  hazmatAllowed: boolean;
}

// ── Mapping tables ────────────────────────────────────────────────────────

export const VEHICLE_TO_PROFILE: Record<VehicleType, RouteProfile> = {
  car: "driving",
  motorcycle: "driving",
  van: "driving",
  emergency: "driving",
  military: "trucking",
  truck_light: "trucking",
  truck_medium: "trucking",
  truck_heavy: "trucking",
  bus: "trucking",
  bicycle: "cycling",
  pedestrian: "walking",
};

/** Bridges → trucking profile AvoidFeature is not directly supported. */
export function avoidFromRestrictions(r: VehicleRestrictions): AvoidFeature[] {
  const out: AvoidFeature[] = [];
  if (r.avoidTunnels) out.push("tunnels");
  // "avoidBridges" has no direct AvoidFeature; callers can enforce via
  // adapter-specific flags or by avoiding ferries-adjacent routes.
  // Hazmat + unpaved are separate concerns handled by providers.
  return out;
}

// ── Main adaptation ────────────────────────────────────────────────────────

export interface AdaptationResult<R extends RouteRequestLike> {
  request: R;
  constraints: VehicleConstraints;
  notes: string[];
}

/**
 * Produce a new request with profile + avoid filled in from a vehicle
 * profile, plus the parallel constraints object for provider enrichment.
 * Existing avoid entries are preserved; duplicates are removed.
 */
export function adaptRouteRequestForVehicle<R extends RouteRequestLike>(
  request: R,
  vehicle: VehicleProfileLike,
): AdaptationResult<R> {
  const notes: string[] = [];
  const profile = VEHICLE_TO_PROFILE[vehicle.type];
  if (request.profile !== profile) {
    notes.push(
      `profile=${request.profile} overridden to ${profile} for vehicle ${vehicle.type}`,
    );
  }

  const avoids = new Set<AvoidFeature>(request.avoid ?? []);
  for (const a of avoidFromRestrictions(vehicle.restrictions)) avoids.add(a);

  if (vehicle.restrictions.avoidBridges) {
    notes.push(
      "avoidBridges requested but no direct AvoidFeature — adapters should enforce via provider-specific parameters",
    );
  }
  if (!vehicle.restrictions.hazmatAllowed && vehicle.type === "truck_heavy") {
    notes.push("non-hazmat heavy truck — pass hazmat=false to provider");
  }

  const adapted: R = {
    ...request,
    profile,
    avoid: Array.from(avoids),
  };

  const constraints: VehicleConstraints = {
    type: vehicle.type,
    heightM: vehicle.dimensions.height,
    widthM: vehicle.dimensions.width,
    lengthM: vehicle.dimensions.length,
    weightKg: vehicle.weight.loaded,
    axles: vehicle.weight.axles,
    minTurningRadiusM: vehicle.restrictions.minTurningRadius,
    hazmatAllowed: vehicle.restrictions.hazmatAllowed,
  };

  return { request: adapted, constraints, notes };
}
