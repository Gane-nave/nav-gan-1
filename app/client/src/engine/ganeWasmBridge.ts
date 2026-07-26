/**
 * G.A.N.E WASM engine bridge.
 *
 * Loads the Rust navigation engine (gane-wasm crate) compiled to WebAssembly
 * and served from /engine/. The same EKF fusion and vehicle-aware routing
 * code runs here, on the server, and in the CLI — one engine, zero drift.
 *
 * Graceful degradation: if WASM is unavailable (old WebView, blocked fetch),
 * loadGaneEngine() resolves to null and callers keep their existing tRPC /
 * TS-engine paths.
 */

export interface GaneVehicleEnvelope {
  class:
    | "car"
    | "motorcycle"
    | "van"
    | "light_truck"
    | "medium_truck"
    | "heavy_truck"
    | "bus"
    | "emergency"
    | "military"
    | "bicycle"
    | "pedestrian";
  height_m: number;
  width_m: number;
  length_m: number;
  weight_kg: number;
  axle_count: number;
  hazmat: boolean;
}

export interface GaneRouteRequest {
  from_node: string;
  to_node: string;
  envelope?: GaneVehicleEnvelope;
}

export interface GaneRouteResponse {
  nodes: string[];
  segments: string[];
  total_cost_s: number;
  constrained: boolean;
}

/** Geographic route as returned by the engine's route_geo. */
export interface GaneGeoRoute {
  from_node: string;
  to_node: string;
  node_count: number;
  segment_ids: string[];
  total_time_s: number;
  total_length_m: number;
  constrained: boolean;
  /** (lat, lon) pairs — note: NOT MapLibre order. */
  polyline: [number, number][];
}

/**
 * Canonical vehicle presets for UI surfaces — single TS source, mirroring
 * gane-osm-import::route::envelope_by_name. Add presets here, not per-panel.
 */
export const VEHICLE_PRESETS: Record<string, GaneVehicleEnvelope> = {
  car: {
    class: "car",
    height_m: 1.6,
    width_m: 1.8,
    length_m: 4.5,
    weight_kg: 1_800,
    axle_count: 2,
    hazmat: false,
  },
  van: {
    class: "van",
    height_m: 2.5,
    width_m: 1.9,
    length_m: 5.5,
    weight_kg: 3_200,
    axle_count: 2,
    hazmat: false,
  },
  truck: {
    class: "heavy_truck",
    height_m: 4.2,
    width_m: 2.55,
    length_m: 16.5,
    weight_kg: 26_000,
    axle_count: 5,
    hazmat: false,
  },
  bus: {
    class: "bus",
    height_m: 3.4,
    width_m: 2.55,
    length_m: 12,
    weight_kg: 18_000,
    axle_count: 3,
    hazmat: false,
  },
  emergency: {
    class: "emergency",
    height_m: 2.8,
    width_m: 2.0,
    length_m: 6.0,
    weight_kg: 4_500,
    axle_count: 2,
    hazmat: false,
  },
};

/** Resolve an engine asset under the deploy base — the one sanctioned way. */
export function engineAssetUrl(name: string): string {
  return `${import.meta.env.BASE_URL}engine/${name}`;
}

/**
 * Canonical 15-state ESKF snapshot — the filter the product is specified
 * around (position, velocity, heading, estimated IMU biases, uncertainty).
 */
export interface GaneEskfState {
  /** ENU position, metres. */
  p: [number, number, number];
  /** ENU velocity, m/s. */
  v: [number, number, number];
  heading_rad: number;
  /** Estimated accelerometer bias, m/s². */
  accel_bias: [number, number, number];
  /** Estimated gyroscope bias, rad/s. */
  gyro_bias: [number, number, number];
  horizontal_uncertainty_m: number;
}

export interface GaneFusedState {
  east_m: number;
  north_m: number;
  up_m: number;
  ve_mps: number;
  vn_mps: number;
  vu_mps: number;
  heading_rad: number;
  horizontal_uncertainty_m: number;
  vertical_uncertainty_m: number;
}

/** Minimal surface of the wasm-bindgen GaneEngine class we consume. */
interface WasmEngine {
  version(): string;
  predict(dtS: number): void;
  update_position(e: number, n: number, u: number, sigmaM: number): void;
  update_velocity(ve: number, vn: number, vu: number, sigma: number): void;
  update_heading(headingRad: number, sigmaRad: number): void;
  position(): string;
  reset(): void;
  load_graph(graphJson: string): void;
  graph_nodes(): number;
  route(requestJson: string): string;
  route_geo(
    fromLat: number,
    fromLon: number,
    toLat: number,
    toLon: number,
    envelopeJson?: string | null
  ): string;
  eskf_imu(
    ax: number,
    ay: number,
    az: number,
    gx: number,
    gy: number,
    gz: number,
    dtS: number
  ): void;
  eskf_update_position(e: number, n: number, u: number, sigmaM: number): number;
  eskf_zupt(sigmaMps: number): number;
  eskf_state(): string;
}

export class GaneEngineBridge {
  private constructor(private readonly engine: WasmEngine) {}

  static wrap(engine: WasmEngine): GaneEngineBridge {
    return new GaneEngineBridge(engine);
  }

  version(): string {
    return this.engine.version();
  }

  predict(dtS: number): void {
    this.engine.predict(dtS);
  }

  updatePosition(
    eastM: number,
    northM: number,
    upM: number,
    sigmaM: number
  ): void {
    this.engine.update_position(eastM, northM, upM, sigmaM);
  }

  updateVelocity(ve: number, vn: number, vu: number, sigma: number): void {
    this.engine.update_velocity(ve, vn, vu, sigma);
  }

  updateHeading(headingRad: number, sigmaRad: number): void {
    this.engine.update_heading(headingRad, sigmaRad);
  }

  fusedState(): GaneFusedState {
    return JSON.parse(this.engine.position()) as GaneFusedState;
  }

  reset(): void {
    this.engine.reset();
  }

  loadGraph(graphJson: string): void {
    this.engine.load_graph(graphJson);
  }

  graphNodeCount(): number {
    return this.engine.graph_nodes();
  }

  /** Vehicle-aware routing: throws "no legal route" when constraints exclude all paths. */
  route(request: GaneRouteRequest): GaneRouteResponse {
    return JSON.parse(
      this.engine.route(JSON.stringify(request))
    ) as GaneRouteResponse;
  }

  /**
   * Geographic routing: snap coordinates (vehicle-aware, multi-candidate
   * fallback) and route — the exact pipeline the CLI and worker use.
   * Throws "no legal route" when constraints exclude all paths.
   */
  routeGeo(
    fromLat: number,
    fromLon: number,
    toLat: number,
    toLon: number,
    envelope?: GaneVehicleEnvelope
  ): GaneGeoRoute {
    return JSON.parse(
      this.engine.route_geo(
        fromLat,
        fromLon,
        toLat,
        toLon,
        envelope ? JSON.stringify(envelope) : null
      )
    ) as GaneGeoRoute;
  }

  // ── Canonical 15-state ESKF ──────────────────────────────────────────────
  // The same filter the native stack and the replay harness run. Wrapped here
  // so the browser can use the canonical path instead of only the legacy
  // predict/updatePosition EKF shims.

  /** Strapdown IMU propagation: accel (m/s²) and gyro (rad/s), body frame. */
  eskfImu(
    ax: number,
    ay: number,
    az: number,
    gx: number,
    gy: number,
    gz: number,
    dtS: number
  ): void {
    this.engine.eskf_imu(ax, ay, az, gx, gy, gz, dtS);
  }

  /** GNSS position update (ENU metres). Returns the NIS gate value. */
  eskfUpdatePosition(e: number, n: number, u: number, sigmaM: number): number {
    return this.engine.eskf_update_position(e, n, u, sigmaM);
  }

  /** Zero-velocity update (stationary). Returns the NIS gate value. */
  eskfZupt(sigmaMps: number): number {
    return this.engine.eskf_zupt(sigmaMps);
  }

  /** Full canonical filter state, including estimated IMU biases. */
  eskfState(): GaneEskfState {
    return JSON.parse(this.engine.eskf_state()) as GaneEskfState;
  }
}

let loader: Promise<GaneEngineBridge | null> | null = null;

/**
 * Load (once) the WASM engine from /engine/. Returns null when unavailable —
 * callers must keep a fallback path.
 */
export function loadGaneEngine(): Promise<GaneEngineBridge | null> {
  if (!loader) {
    loader = (async () => {
      try {
        const glueUrl = engineAssetUrl("gane_wasm.js");
        const mod = await import(/* @vite-ignore */ glueUrl);
        await mod.default({
          module_or_path: engineAssetUrl("gane_wasm_bg.wasm"),
        });
        const engine: WasmEngine = new mod.GaneEngine();
        console.info(`[gane-wasm] engine v${engine.version()} ready`);
        return GaneEngineBridge.wrap(engine);
      } catch (err) {
        console.warn(
          "[gane-wasm] engine unavailable, using fallback paths",
          err
        );
        return null;
      }
    })();
  }
  return loader;
}
