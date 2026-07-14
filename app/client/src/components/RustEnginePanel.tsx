/**
 * Rust Engine Panel — the canonical G.A.N.E engine, live in the product UI.
 *
 * Runs the Rust WASM engine in a Web Worker (off the main thread) and shows:
 * - engine version + worker status
 * - canonical 15-state ESKF fused state (fed a deterministic demo sequence)
 * - vehicle-aware routing on the imported demo graph (car vs. heavy truck)
 *
 * Everything shown here is computed by the same Rust code that runs on the
 * server and in the CLI — proven bit-identical by gane-replay-verify.
 */

import { useEffect, useRef, useState } from "react";
import { X, Cpu, Route, Gauge, ShieldCheck, TriangleAlert } from "lucide-react";
import {
  GaneEngineWorkerClient,
  type GaneGeoRoute,
} from "@/engine/ganeEngineClient";
import { engineAssetUrl, VEHICLE_PRESETS } from "@/engine/ganeWasmBridge";
import type { GaneFusedState } from "@/engine/ganeWasmBridge";

const ORIGIN: [number, number] = [32.08, 34.78];
const DEST: [number, number] = [32.09, 34.78];

const HEAVY_TRUCK = VEHICLE_PRESETS.truck;

interface EngineDemoState {
  status: "loading" | "ready" | "unavailable";
  version?: string;
  graphNodes?: number;
  fused?: GaneFusedState;
  car?: GaneGeoRoute;
  truck?: GaneGeoRoute;
  error?: string;
}

function Row({
  label,
  value,
  accent,
}: {
  label: string;
  value: string;
  accent?: string;
}) {
  return (
    <div className="flex items-center justify-between py-1 text-sm">
      <span className="text-white/60">{label}</span>
      <span
        className="font-mono tabular-nums"
        style={{ color: accent ?? "#e8eefc" }}
      >
        {value}
      </span>
    </div>
  );
}

export default function RustEnginePanel({ onClose }: { onClose: () => void }) {
  const [state, setState] = useState<EngineDemoState>({ status: "loading" });
  const clientRef = useRef<GaneEngineWorkerClient | null>(null);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      const client = await GaneEngineWorkerClient.spawn();
      if (!client) {
        if (!cancelled) setState({ status: "unavailable" });
        return;
      }
      clientRef.current = client;
      try {
        const version = await client.version();

        // Deterministic ESKF demo: 5 s stationary at a fixed ENU point,
        // 20 Hz IMU + 1 Hz GNSS — same shape as the golden trace.
        for (let i = 0; i < 100; i++) {
          await client.predict(0.05);
          if (i % 20 === 19) await client.updatePosition(120, -40, 8, 2.0);
        }
        const fused = await client.fusedState();

        // Vehicle-aware routing on the imported demo graph.
        const graphJson = await (
          await fetch(engineAssetUrl("demo-graph.json"))
        ).text();
        const graphNodes = await client.loadGraph(graphJson);
        const car = await client.routeGeo(...ORIGIN, ...DEST);
        const truck = await client.routeGeo(...ORIGIN, ...DEST, HEAVY_TRUCK);

        if (!cancelled) {
          setState({ status: "ready", version, graphNodes, fused, car, truck });
        }
      } catch (err) {
        if (!cancelled) {
          setState({
            status: "unavailable",
            error: err instanceof Error ? err.message : String(err),
          });
        }
      }
    })();
    return () => {
      cancelled = true;
      clientRef.current?.terminate();
    };
  }, []);

  return (
    <div
      dir="rtl"
      className="w-[340px] max-h-[70vh] overflow-y-auto rounded-2xl border border-white/10 bg-[#0d1526]/95 p-4 text-white shadow-2xl backdrop-blur"
      data-testid="rust-engine-panel"
    >
      <div className="mb-3 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Cpu size={18} className="text-sky-400" />
          <h2 className="text-sm font-semibold">
            מנוע Rust — ליבה קנונית (WASM)
          </h2>
        </div>
        <button
          onClick={onClose}
          aria-label="סגירת פאנל המנוע"
          className="rounded-lg p-1 text-white/60 hover:bg-white/10 hover:text-white"
        >
          <X size={16} />
        </button>
      </div>

      {state.status === "loading" && (
        <div className="py-6 text-center text-sm text-white/60">
          טוען את המנוע ב-Web Worker…
        </div>
      )}

      {state.status === "unavailable" && (
        <div className="flex items-start gap-2 rounded-xl border border-amber-400/30 bg-amber-400/10 p-3 text-sm">
          <TriangleAlert size={16} className="mt-0.5 shrink-0 text-amber-400" />
          <div>
            מנוע ה-WASM אינו זמין בדפדפן זה — המערכת ממשיכה במסלולי הגיבוי
            (tRPC/מנועי TS).
            {state.error ? (
              <div className="mt-1 font-mono text-xs text-white/50">
                {state.error}
              </div>
            ) : null}
          </div>
        </div>
      )}

      {state.status === "ready" && state.fused && state.car && state.truck && (
        <div className="space-y-3">
          <section className="rounded-xl border border-white/10 bg-white/5 p-3">
            <div className="mb-1 flex items-center gap-2 text-xs font-semibold text-emerald-400">
              <ShieldCheck size={14} /> מנוע v{state.version} · Worker פעיל ·{" "}
              {state.graphNodes} צמתים
            </div>
          </section>

          <section className="rounded-xl border border-white/10 bg-white/5 p-3">
            <div className="mb-1 flex items-center gap-2 text-xs font-semibold text-sky-300">
              <Gauge size={14} /> מסנן ESKF (15 מצבים) — מצב היתוך חי
            </div>
            <Row
              label="מיקום ENU"
              value={`${state.fused.east_m.toFixed(1)}, ${state.fused.north_m.toFixed(1)} מ'`}
            />
            <Row
              label="אי-ודאות אופקית"
              value={`±${state.fused.horizontal_uncertainty_m.toFixed(2)} מ'`}
              accent={
                state.fused.horizontal_uncertainty_m < 5 ? "#34d399" : "#fbbf24"
              }
            />
          </section>

          <section className="rounded-xl border border-white/10 bg-white/5 p-3">
            <div className="mb-1 flex items-center gap-2 text-xs font-semibold text-orange-300">
              <Route size={14} /> ניתוב מודע-רכב (גשר 4.0 מ' בדרך)
            </div>
            <Row
              label="🚗 רכב — ישיר"
              value={`${Math.round(state.car.total_length_m)} מ' · ${Math.round(state.car.total_time_s)} שנ'`}
              accent="#4da3ff"
            />
            <Row
              label="🚚 משאית — עקיפה"
              value={`${Math.round(state.truck.total_length_m)} מ' · ${Math.round(state.truck.total_time_s)} שנ'`}
              accent="#ffb454"
            />
            <div className="mt-1 text-[11px] leading-relaxed text-white/50">
              המגבלה נאכפת בתוך המנוע: מסלול לא-חוקי לרכב הנתון לעולם לא יוחזר.
            </div>
          </section>
        </div>
      )}
    </div>
  );
}
