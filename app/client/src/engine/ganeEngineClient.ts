/**
 * G.A.N.E NAV — typed client for the engine Web Worker.
 *
 * Runs the Rust WASM engine off the main thread (worker file served from
 * /engine/gane.worker.js). Promise-per-call RPC with request ids; graceful
 * null when Workers/WASM are unavailable so callers keep fallback paths.
 */

import type {
  GaneFusedState,
  GaneVehicleEnvelope,
} from "./ganeWasmBridge";

export interface GaneGeoRoute {
  from_node: string;
  to_node: string;
  node_count: number;
  segment_ids: string[];
  total_time_s: number;
  total_length_m: number;
  constrained: boolean;
  polyline: [number, number][];
}

interface Pending {
  resolve: (v: unknown) => void;
  reject: (e: Error) => void;
}

export class GaneEngineWorkerClient {
  private next = 1;
  private readonly pending = new Map<number, Pending>();

  private constructor(private readonly worker: Worker) {
    worker.onmessage = (ev: MessageEvent) => {
      const { id, ok, result, error } = ev.data ?? {};
      const p = this.pending.get(id);
      if (!p) return;
      this.pending.delete(id);
      if (ok) p.resolve(result);
      else p.reject(new Error(error));
    };
  }

  private call<T>(cmd: string, ...args: unknown[]): Promise<T> {
    const id = this.next++;
    return new Promise<T>((resolve, reject) => {
      this.pending.set(id, { resolve: resolve as (v: unknown) => void, reject });
      this.worker.postMessage({ id, cmd, args });
    });
  }

  version(): Promise<string> {
    return this.call("version");
  }

  /** Load a RoadGraph JSON; resolves with the node count. */
  loadGraph(graphJson: string): Promise<number> {
    return this.call("loadGraph", graphJson);
  }

  /** Vehicle-aware geographic routing; rejects with "no legal route" when constrained out. */
  routeGeo(
    fromLat: number,
    fromLon: number,
    toLat: number,
    toLon: number,
    envelope?: GaneVehicleEnvelope,
  ): Promise<GaneGeoRoute> {
    return this.call(
      "routeGeo",
      fromLat,
      fromLon,
      toLat,
      toLon,
      envelope ? JSON.stringify(envelope) : undefined,
    );
  }

  predict(dtS: number): Promise<void> {
    return this.call("predict", dtS);
  }

  updatePosition(eastM: number, northM: number, upM: number, sigmaM: number): Promise<void> {
    return this.call("updatePosition", eastM, northM, upM, sigmaM);
  }

  fusedState(): Promise<GaneFusedState> {
    return this.call("fusedState");
  }

  reset(): Promise<void> {
    return this.call("reset");
  }

  terminate(): void {
    this.worker.terminate();
  }

  /**
   * Spawn the engine worker and initialize WASM inside it.
   * Resolves to null when unavailable — callers must keep a fallback.
   */
  static async spawn(): Promise<GaneEngineWorkerClient | null> {
    try {
      if (typeof Worker === "undefined") return null;
      const worker = new Worker("/engine/gane.worker.js", { type: "module" });
      const client = new GaneEngineWorkerClient(worker);
      const version = await client.call<string>("init");
      console.info(`[gane-engine-worker] engine v${version} ready off-main-thread`);
      return client;
    } catch (err) {
      console.warn("[gane-engine-worker] unavailable, using fallback paths", err);
      return null;
    }
  }
}
