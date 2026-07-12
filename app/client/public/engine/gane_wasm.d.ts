/* tslint:disable */
/* eslint-disable */

/**
 * The navigation engine instance exposed to JavaScript.
 */
export class GaneEngine {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Number of nodes in the loaded graph (0 if none).
     */
    graph_nodes(): number;
    /**
     * Load a road graph (JSON per contracts RoadGraph schema).
     */
    load_graph(graph_json: string): void;
    constructor();
    /**
     * Fused state as JSON: position/velocity/heading/uncertainty.
     */
    position(): string;
    /**
     * Advance the filter by `dt` seconds.
     */
    predict(dt_s: number): void;
    /**
     * Reset the filter (e.g. after teleport/replay restart).
     */
    reset(): void;
    /**
     * Compute a route. Request/response are JSON strings.
     *
     * With an `envelope` in the request, hard vehicle constraints are
     * enforced: an illegal route is never returned.
     */
    route(request_json: string): string;
    /**
     * Route between two geographic coordinates (nearest-node snap).
     *
     * `envelope_json` optionally carries a VehicleEnvelope; hard constraints
     * are enforced — an illegal route is never returned. The response is a
     * GeoRoute JSON with polyline, length, and drive time.
     */
    route_geo(from_lat: number, from_lon: number, to_lat: number, to_lon: number, envelope_json?: string | null): string;
    /**
     * Feed a heading measurement (radians).
     */
    update_heading(heading_rad: number, sigma_rad: number): void;
    /**
     * Feed a position fix in local ENU metres with 1-sigma accuracy.
     */
    update_position(east_m: number, north_m: number, up_m: number, sigma_m: number): void;
    /**
     * Feed a velocity measurement (ENU, m/s).
     */
    update_velocity(ve: number, vn: number, vu: number, sigma_mps: number): void;
    /**
     * Engine version (single source: the workspace version).
     */
    version(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_ganeengine_free: (a: number, b: number) => void;
    readonly ganeengine_graph_nodes: (a: number) => number;
    readonly ganeengine_load_graph: (a: number, b: number, c: number, d: number) => void;
    readonly ganeengine_new: () => number;
    readonly ganeengine_position: (a: number, b: number) => void;
    readonly ganeengine_predict: (a: number, b: number) => void;
    readonly ganeengine_reset: (a: number) => void;
    readonly ganeengine_route: (a: number, b: number, c: number, d: number) => void;
    readonly ganeengine_route_geo: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly ganeengine_update_heading: (a: number, b: number, c: number) => void;
    readonly ganeengine_update_position: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly ganeengine_update_velocity: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly ganeengine_version: (a: number, b: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
