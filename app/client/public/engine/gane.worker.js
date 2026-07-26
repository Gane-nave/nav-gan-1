/**
 * G.A.N.E NAV — engine worker (ES module worker).
 *
 * Hosts the Rust WASM engine off the main thread so fusion and routing never
 * block rendering (60 FPS budget, Blueprint §13). Message protocol:
 *   in : { id, cmd, args }
 *   out: { id, ok: true, result } | { id, ok: false, error }
 *
 * Commands: init, version, loadGraph(graphJson), graphNodes,
 *           routeGeo(fromLat, fromLon, toLat, toLon, envelopeJson?),
 *           predict(dtS), updatePosition(e,n,u,sigma), fusedState, reset.
 */

let engine = null;

async function ensureEngine() {
  if (engine) return engine;
  const mod = await import("./gane_wasm.js");
  await mod.default({ module_or_path: new URL("./gane_wasm_bg.wasm", import.meta.url) });
  engine = new mod.GaneEngine();
  return engine;
}

const handlers = {
  init: async () => (await ensureEngine()).version(),
  version: async () => (await ensureEngine()).version(),
  loadGraph: async (graphJson) => {
    (await ensureEngine()).load_graph(graphJson);
    return engine.graph_nodes();
  },
  graphNodes: async () => (await ensureEngine()).graph_nodes(),
  routeGeo: async (fromLat, fromLon, toLat, toLon, envelopeJson) =>
    JSON.parse(
      (await ensureEngine()).route_geo(fromLat, fromLon, toLat, toLon, envelopeJson ?? undefined),
    ),
  predict: async (dtS) => (await ensureEngine()).predict(dtS),
  updatePosition: async (e, n, u, sigma) =>
    (await ensureEngine()).update_position(e, n, u, sigma),
  fusedState: async () => JSON.parse((await ensureEngine()).position()),
  // Canonical 15-state ESKF — same filter as native/CLI, off the main thread.
  eskfImu: async (ax, ay, az, gx, gy, gz, dtS) =>
    (await ensureEngine()).eskf_imu(ax, ay, az, gx, gy, gz, dtS),
  eskfUpdatePosition: async (e, n, u, sigmaM) =>
    (await ensureEngine()).eskf_update_position(e, n, u, sigmaM),
  eskfZupt: async sigmaMps => (await ensureEngine()).eskf_zupt(sigmaMps),
  eskfState: async () => JSON.parse((await ensureEngine()).eskf_state()),
  reset: async () => (await ensureEngine()).reset(),
};

self.onmessage = async (ev) => {
  const { id, cmd, args = [] } = ev.data ?? {};
  try {
    const fn = handlers[cmd];
    if (!fn) throw new Error(`unknown command: ${cmd}`);
    const result = await fn(...args);
    self.postMessage({ id, ok: true, result });
  } catch (err) {
    self.postMessage({ id, ok: false, error: String(err?.message ?? err) });
  }
};
