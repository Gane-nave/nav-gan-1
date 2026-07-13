/**
 * G.A.N.E — Open Navigation Map
 *
 * Fully open stack, zero API keys, zero cost:
 *   - MapLibre GL JS (BSD-3) renders the basemap
 *   - OpenFreeMap (https://openfreemap.org) serves vector tiles — free,
 *     no key, no registration, OSM data under ODbL
 *   - The imported OSM road graph renders as a native overlay, so the
 *     component stays fully functional offline / air-gapped: when tiles
 *     are unreachable it falls back to an inline dark style and the road
 *     network itself remains the basemap
 *   - Routing runs in the G.A.N.E Rust engine (WASM) on the same graph
 *
 * Interaction: first click sets origin, second sets destination and
 * computes a vehicle-aware route; the next click starts over.
 */
import { useCallback, useEffect, useRef, useState } from "react";
import maplibregl from "maplibre-gl";
import "maplibre-gl/dist/maplibre-gl.css";
import {
  loadGaneEngine,
  type GaneEngineBridge,
  type GaneVehicleEnvelope,
} from "@/engine/ganeWasmBridge";

const TILE_STYLE_URL = "https://tiles.openfreemap.org/styles/liberty";
const GRAPH_URL = "/engine/kouvola-graph.json";

/** Snap candidates per endpoint — mirrors gane-osm-import::route. */
const SNAP_CANDIDATES = 64;
const MAX_SNAP_DISTANCE_M = 1_500;

// ── Graph schema (as produced by gane-osm-import) ──────────────────────────

interface GraphPosition {
  latitude_deg: number;
  longitude_deg: number;
}

interface GraphNode {
  id: string;
  position: GraphPosition;
}

interface GraphSegment {
  id: string;
  from_node: string;
  to_node: string;
  geometry: GraphPosition[];
  road_class: string;
  one_way: boolean;
  tunnel: boolean;
  hazmat_restricted: boolean;
  weight_limit_kg: number | null;
  height_limit_m: number | null;
  length_m: number;
}

interface RoadGraph {
  nodes: GraphNode[];
  segments: GraphSegment[];
}

// ── Vehicle presets (mirrors gane-osm-import::route::envelope_by_name) ─────

const VEHICLES: Record<string, GaneVehicleEnvelope> = {
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

const VEHICLE_LABELS: Record<string, string> = {
  car: "🚗 Car",
  van: "🚐 Van",
  truck: "🚛 Truck",
  bus: "🚌 Bus",
  emergency: "🚑 Emergency",
};

/** TS port of VehicleEnvelope::permits — hard legality constraints only. */
function permits(env: GaneVehicleEnvelope, seg: GraphSegment): boolean {
  if (seg.height_limit_m != null && env.height_m > seg.height_limit_m)
    return false;
  if (seg.weight_limit_kg != null && env.weight_kg > seg.weight_limit_kg)
    return false;
  if (env.hazmat && (seg.hazmat_restricted || seg.tunnel)) return false;
  const motorized = env.class !== "bicycle" && env.class !== "pedestrian";
  switch (seg.road_class) {
    case "Pedestrian":
      return (
        env.class === "pedestrian" ||
        env.class === "bicycle" ||
        env.class === "emergency"
      );
    case "Cycleway":
      return env.class === "bicycle" || env.class === "pedestrian";
    case "Motorway":
    case "Trunk":
      return motorized;
    case "Track":
      return env.class !== "heavy_truck";
    default:
      return true;
  }
}

function haversineM(a: GraphPosition, b: GraphPosition): number {
  const R = 6_371_000;
  const la1 = (a.latitude_deg * Math.PI) / 180;
  const la2 = (b.latitude_deg * Math.PI) / 180;
  const dLa = la2 - la1;
  const dLo = ((b.longitude_deg - a.longitude_deg) * Math.PI) / 180;
  const h =
    Math.sin(dLa / 2) ** 2 +
    Math.cos(la1) * Math.cos(la2) * Math.sin(dLo / 2) ** 2;
  return 2 * R * Math.asin(Math.sqrt(h));
}

/** Road-class → overlay color (dark-theme friendly). */
const CLASS_COLORS: Record<string, string> = {
  Motorway: "#f59e0b",
  Trunk: "#f59e0b",
  Primary: "#facc15",
  Secondary: "#a3e635",
  Tertiary: "#86efac",
  Residential: "#64748b",
  Service: "#475569",
  Unclassified: "#475569",
  Track: "#57534e",
  Cycleway: "#22d3ee",
  Pedestrian: "#c084fc",
};

interface RouteResult {
  distanceM: number;
  timeS: number;
  polyline: [number, number][]; // [lon, lat]
}

interface OpenNavigationMapProps {
  onMapClick?: (lat: number, lon: number) => void;
}

export default function OpenNavigationMap({
  onMapClick,
}: OpenNavigationMapProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<maplibregl.Map | null>(null);
  const graphRef = useRef<RoadGraph | null>(null);
  const engineRef = useRef<GaneEngineBridge | null>(null);
  const markersRef = useRef<maplibregl.Marker[]>([]);
  const clicksRef = useRef<[number, number][]>([]); // [lat, lon]

  const [status, setStatus] = useState("loading engine…");
  const [vehicle, setVehicle] = useState("car");
  const vehicleRef = useRef(vehicle);
  vehicleRef.current = vehicle;
  const [route, setRoute] = useState<RouteResult | null>(null);
  const [routeError, setRouteError] = useState<string | null>(null);

  /** Vehicle-aware snap with multi-candidate fallback — mirrors the Rust CLI. */
  const computeRoute = useCallback(
    (from: [number, number], to: [number, number]): RouteResult => {
      const graph = graphRef.current;
      const engine = engineRef.current;
      if (!graph || !engine) throw new Error("engine not ready");
      const env = VEHICLES[vehicleRef.current];

      const usable = graph.nodes.filter(n =>
        graph.segments.some(
          s => (s.from_node === n.id || s.to_node === n.id) && permits(env, s)
        )
      );
      const candidates = (pt: [number, number]) =>
        usable
          .map(n => ({
            id: n.id,
            d: haversineM(n.position, {
              latitude_deg: pt[0],
              longitude_deg: pt[1],
            }),
          }))
          .filter(c => c.d <= MAX_SNAP_DISTANCE_M)
          .sort((a, b) => a.d - b.d)
          .slice(0, SNAP_CANDIDATES);

      const fromCands = candidates(from);
      const toCands = candidates(to);
      const pairs = fromCands
        .flatMap(f => toCands.map(t => ({ f, t, d: f.d + t.d })))
        .sort((a, b) => a.d - b.d);

      const segById = new Map(graph.segments.map(s => [s.id, s]));
      for (const { f, t } of pairs) {
        try {
          const res = engine.route({
            from_node: f.id,
            to_node: t.id,
            envelope: env,
          });
          if (!Number.isFinite(res.total_cost_s)) continue;
          let distanceM = 0;
          const polyline: [number, number][] = [];
          for (const segId of res.segments) {
            const seg = segById.get(segId);
            if (!seg) continue;
            distanceM += seg.length_m;
            for (const g of seg.geometry)
              polyline.push([g.longitude_deg, g.latitude_deg]);
          }
          return { distanceM, timeS: res.total_cost_s, polyline };
        } catch {
          // pair not connected for this vehicle — try the next one
        }
      }
      throw new Error("no legal route for the selected vehicle");
    },
    []
  );

  const drawRoute = useCallback((r: RouteResult | null) => {
    const map = mapRef.current;
    if (!map) return;
    const src = map.getSource("gane-route") as
      | maplibregl.GeoJSONSource
      | undefined;
    src?.setData({
      type: "FeatureCollection",
      features: r
        ? [
            {
              type: "Feature",
              geometry: { type: "LineString", coordinates: r.polyline },
              properties: {},
            },
          ]
        : [],
    });
  }, []);

  const handleClick = useCallback(
    (e: maplibregl.MapMouseEvent) => {
      onMapClick?.(e.lngLat.lat, e.lngLat.lng);
      const map = mapRef.current;
      if (!map || !engineRef.current) return;

      if (clicksRef.current.length >= 2) {
        clicksRef.current = [];
        markersRef.current.forEach(m => m.remove());
        markersRef.current = [];
        setRoute(null);
        setRouteError(null);
        drawRoute(null);
      }

      clicksRef.current.push([e.lngLat.lat, e.lngLat.lng]);
      const marker = new maplibregl.Marker({
        color: clicksRef.current.length === 1 ? "#22c55e" : "#ef4444",
      })
        .setLngLat(e.lngLat)
        .addTo(map);
      markersRef.current.push(marker);

      if (clicksRef.current.length === 2) {
        try {
          const r = computeRoute(clicksRef.current[0], clicksRef.current[1]);
          setRoute(r);
          setRouteError(null);
          drawRoute(r);
        } catch (err) {
          setRoute(null);
          setRouteError(err instanceof Error ? err.message : String(err));
        }
      }
    },
    [computeRoute, drawRoute, onMapClick]
  );

  useEffect(() => {
    if (!containerRef.current) return;

    /** Inline fallback style: works offline / air-gapped with zero fetches. */
    const fallbackStyle: maplibregl.StyleSpecification = {
      version: 8,
      sources: {},
      layers: [
        {
          id: "bg",
          type: "background",
          paint: { "background-color": "#0b1220" },
        },
      ],
    };

    // Offline-first: boot on the inline style (zero fetches, renders
    // instantly), then upgrade to OpenFreeMap tiles only once the style
    // is confirmed reachable. Overlays re-install on every style swap.
    const map = new maplibregl.Map({
      container: containerRef.current,
      style: fallbackStyle,
      center: [26.95, 60.53], // imported region (real OSM extract)
      zoom: 13,
      attributionControl: { compact: true },
    });
    mapRef.current = map;
    map.addControl(
      new maplibregl.NavigationControl({ showCompass: true }),
      "bottom-right"
    );
    map.addControl(new maplibregl.ScaleControl({ unit: "metric" }));

    fetch(TILE_STYLE_URL)
      .then(r =>
        r.ok ? r.json() : Promise.reject(new Error(`style ${r.status}`))
      )
      .then(style => map.setStyle(style as maplibregl.StyleSpecification))
      .catch(() =>
        console.info("[open-map] tiles unreachable — offline basemap")
      );

    // The container can be mid-layout at construction, leaving the canvas
    // at its 300 px default; track its real size for the map's lifetime.
    requestAnimationFrame(() => map.resize());
    const ro = new ResizeObserver(() => map.resize());
    ro.observe(containerRef.current);

    const addOverlays = async () => {
      try {
        const [graphResp, engine] = await Promise.all([
          fetch(GRAPH_URL),
          loadGaneEngine(),
        ]);
        const graph = (await graphResp.json()) as RoadGraph;
        graphRef.current = graph;
        engineRef.current = engine;
        if (engine) {
          engine.loadGraph(JSON.stringify(graph));
          setStatus(
            `engine v${engine.version()} · ${graph.nodes.length} nodes · ${graph.segments.length} roads`
          );
        } else {
          setStatus("engine unavailable — map only");
        }

        const roads: GeoJSON.FeatureCollection = {
          type: "FeatureCollection",
          features: graph.segments.map(s => ({
            type: "Feature",
            geometry: {
              type: "LineString",
              coordinates: s.geometry.map(g => [
                g.longitude_deg,
                g.latitude_deg,
              ]),
            },
            properties: { color: CLASS_COLORS[s.road_class] ?? "#475569" },
          })),
        };

        const install = () => {
          if (!map.getSource("gane-roads")) {
            map.addSource("gane-roads", { type: "geojson", data: roads });
            map.addLayer({
              id: "gane-roads",
              type: "line",
              source: "gane-roads",
              paint: {
                "line-color": ["get", "color"],
                "line-width": 2,
                "line-opacity": 0.85,
              },
            });
          }
          if (!map.getSource("gane-route")) {
            map.addSource("gane-route", {
              type: "geojson",
              data: { type: "FeatureCollection", features: [] },
            });
            map.addLayer({
              id: "gane-route",
              type: "line",
              source: "gane-route",
              layout: { "line-cap": "round", "line-join": "round" },
              paint: {
                "line-color": "#3b82f6",
                "line-width": 5,
                "line-opacity": 0.95,
              },
            });
          }
        };
        // (Re-)install overlays on every style swap, including the offline fallback.
        if (map.isStyleLoaded()) install();
        map.on("style.load", install);

        const lats = graph.nodes.map(n => n.position.latitude_deg);
        const lons = graph.nodes.map(n => n.position.longitude_deg);
        map.fitBounds(
          [
            [Math.min(...lons), Math.min(...lats)],
            [Math.max(...lons), Math.max(...lats)],
          ],
          { padding: 60, duration: 0 }
        );
      } catch (err) {
        console.warn("[open-map] overlay init failed", err);
        setStatus("road graph unavailable");
      }
    };
    void addOverlays();
    map.on("click", handleClick);

    return () => {
      ro.disconnect();
      map.off("click", handleClick);
      map.remove();
      mapRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Re-route on vehicle change when both endpoints are set.
  useEffect(() => {
    if (clicksRef.current.length !== 2) return;
    try {
      const r = computeRoute(clicksRef.current[0], clicksRef.current[1]);
      setRoute(r);
      setRouteError(null);
      drawRoute(r);
    } catch (err) {
      setRoute(null);
      setRouteError(err instanceof Error ? err.message : String(err));
      drawRoute(null);
    }
  }, [vehicle, computeRoute, drawRoute]);

  const minutes = route ? Math.max(1, Math.round(route.timeS / 60)) : 0;

  return (
    <div className="absolute inset-0" data-testid="open-navigation-map">
      {/* Inline position: maplibre-gl.css sets `.maplibregl-map{position:relative}`,
          which can out-cascade the utility class and collapse the height to 0. */}
      <div ref={containerRef} style={{ position: "absolute", inset: 0 }} />

      {/* Status chip */}
      <div className="pointer-events-none absolute left-1/2 top-16 z-20 -translate-x-1/2 rounded-lg bg-slate-900/80 px-3 py-1.5 font-mono text-xs text-emerald-300 backdrop-blur">
        G.A.N.E · {status}
      </div>

      {/* Vehicle picker */}
      <div className="absolute left-1/2 top-24 z-20 flex -translate-x-1/2 gap-1 rounded-lg bg-slate-900/80 p-1 backdrop-blur">
        {Object.keys(VEHICLES).map(v => (
          <button
            key={v}
            type="button"
            onClick={() => setVehicle(v)}
            className={`rounded-md px-2 py-1 text-xs transition-colors ${
              vehicle === v
                ? "bg-blue-600 text-white"
                : "text-slate-300 hover:bg-slate-700"
            }`}
          >
            {VEHICLE_LABELS[v]}
          </button>
        ))}
      </div>

      {/* Route result / hint */}
      <div className="absolute bottom-32 left-1/2 z-20 -translate-x-1/2 rounded-lg bg-slate-900/85 px-4 py-2 text-center text-sm text-slate-100 backdrop-blur">
        {routeError ? (
          <span className="text-amber-400">{routeError}</span>
        ) : route ? (
          <span>
            <span className="font-semibold text-blue-400">
              {(route.distanceM / 1000).toFixed(2)} km
            </span>
            {" · "}
            <span className="font-semibold text-emerald-400">
              ~{minutes} min
            </span>
            {" · "}
            <span className="text-slate-400">{VEHICLE_LABELS[vehicle]}</span>
          </span>
        ) : (
          <span className="text-slate-400">
            {clicksRef.current.length === 1
              ? "click destination…"
              : "click map to set origin"}
          </span>
        )}
      </div>
    </div>
  );
}
