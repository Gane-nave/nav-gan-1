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
        const glueUrl = `${import.meta.env.BASE_URL}engine/gane_wasm.js`;
        const mod = await import(/* @vite-ignore */ glueUrl);
        await mod.default({
          module_or_path: `${import.meta.env.BASE_URL}engine/gane_wasm_bg.wasm`,
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
