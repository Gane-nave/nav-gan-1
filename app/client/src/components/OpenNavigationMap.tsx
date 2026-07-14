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
 *   - Routing runs in the G.A.N.E Rust engine (WASM) via route_geo — the
 *     exact snap+route pipeline the CLI and worker use. No TS fork.
 *
 * Interaction: first click sets origin, second sets destination and
 * computes a vehicle-aware route; the next click starts over.
 */
import { useCallback, useEffect, useRef, useState } from "react";
import maplibregl from "maplibre-gl";
import "maplibre-gl/dist/maplibre-gl.css";
import {
  engineAssetUrl,
  loadGaneEngine,
  VEHICLE_PRESETS,
  type GaneEngineBridge,
} from "@/engine/ganeWasmBridge";

const TILE_STYLE_URL = "https://tiles.openfreemap.org/styles/liberty";
const GRAPH_ASSET = "kouvola-graph.json";

// ── Graph overlay schema (subset of the importer's output we render) ───────

interface GraphPosition {
  latitude_deg: number;
  longitude_deg: number;
}

interface OverlayGraph {
  nodes: { position: GraphPosition }[];
  segments: { geometry: GraphPosition[]; road_class: string }[];
}

const VEHICLE_LABELS: Record<string, string> = {
  car: "🚗 Car",
  van: "🚐 Van",
  truck: "🚛 Truck",
  bus: "🚌 Bus",
  emergency: "🚑 Emergency",
};

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
  polyline: [number, number][]; // [lon, lat] — MapLibre order
}

interface OpenNavigationMapProps {
  onMapClick?: (lat: number, lon: number) => void;
}

export default function OpenNavigationMap({
  onMapClick,
}: OpenNavigationMapProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<maplibregl.Map | null>(null);
  const engineRef = useRef<GaneEngineBridge | null>(null);
  const markersRef = useRef<maplibregl.Marker[]>([]);
  // The drawn route survives style swaps via this ref (style.load re-applies).
  const routeRef = useRef<RouteResult | null>(null);

  const [clicks, setClicks] = useState<[number, number][]>([]); // [lat, lon]
  const [status, setStatus] = useState("loading engine…");
  const [vehicle, setVehicle] = useState("car");
  const [route, setRoute] = useState<RouteResult | null>(null);
  const [routeError, setRouteError] = useState<string | null>(null);

  const drawRoute = useCallback((r: RouteResult | null) => {
    routeRef.current = r;
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

  /** One shared compute path for clicks and vehicle switches. */
  const recompute = useCallback(
    (from: [number, number], to: [number, number], vehicleKey: string) => {
      const engine = engineRef.current;
      if (!engine) return;
      try {
        const geo = engine.routeGeo(
          from[0],
          from[1],
          to[0],
          to[1],
          VEHICLE_PRESETS[vehicleKey]
        );
        const r: RouteResult = {
          distanceM: geo.total_length_m,
          timeS: geo.total_time_s,
          // Engine polyline is (lat, lon); MapLibre wants (lon, lat).
          polyline: geo.polyline.map(([lat, lon]) => [lon, lat]),
        };
        setRoute(r);
        setRouteError(null);
        drawRoute(r);
      } catch (err) {
        setRoute(null);
        setRouteError(err instanceof Error ? err.message : String(err));
        drawRoute(null);
      }
    },
    [drawRoute]
  );

  const handleClick = useCallback(
    (e: maplibregl.MapMouseEvent) => {
      onMapClick?.(e.lngLat.lat, e.lngLat.lng);
      const map = mapRef.current;
      if (!map || !engineRef.current) return;

      let base = clicks;
      if (clicks.length >= 2) {
        base = [];
        markersRef.current.forEach(m => m.remove());
        markersRef.current = [];
        setRoute(null);
        setRouteError(null);
        drawRoute(null);
      }
      const next: [number, number][] = [...base, [e.lngLat.lat, e.lngLat.lng]];

      const marker = new maplibregl.Marker({
        color: next.length === 1 ? "#22c55e" : "#ef4444",
      })
        .setLngLat(e.lngLat)
        .addTo(map);
      markersRef.current.push(marker);

      setClicks(next);
    },
    [clicks, drawRoute, onMapClick]
  );
  // The map binds one listener at mount; this ref always points at the
  // freshest closure so late prop changes (collab onMapClick) are honored.
  const handleClickRef = useRef(handleClick);
  handleClickRef.current = handleClick;

  // Compute on destination click and recompute on vehicle switch —
  // one effect, one code path.
  useEffect(() => {
    if (clicks.length === 2) recompute(clicks[0], clicks[1], vehicle);
  }, [clicks, vehicle, recompute]);

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

    let disposed = false;
    const addOverlays = async () => {
      try {
        const [graphResp, engine] = await Promise.all([
          fetch(engineAssetUrl(GRAPH_ASSET)),
          loadGaneEngine(),
        ]);
        // One fetch, one parse: raw text feeds the engine (which parses in
        // Rust); the overlay parses the same text once for rendering.
        const graphText = await graphResp.text();
        const graph = JSON.parse(graphText) as OverlayGraph;
        if (disposed) return;
        engineRef.current = engine;
        if (engine) {
          engine.loadGraph(graphText);
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
            // A style swap wiped the sources; restore the active route.
            drawRoute(routeRef.current);
          }
        };
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
    const onClick = (e: maplibregl.MapMouseEvent) => handleClickRef.current(e);
    map.on("click", onClick);

    return () => {
      disposed = true;
      ro.disconnect();
      map.off("click", onClick);
      map.remove();
      mapRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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
        {Object.keys(VEHICLE_PRESETS).map(v => (
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
            {VEHICLE_LABELS[v] ?? v}
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
            {clicks.length === 1
              ? "click destination…"
              : "click map to set origin"}
          </span>
        )}
      </div>
    </div>
  );
}
