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
import { useLanguage } from "@/contexts/LanguageContext";

const TILE_STYLE_URL = "https://tiles.openfreemap.org/styles/liberty";

/** Region registry: add a city by dropping a graph JSON + one entry in
 *  public/engine/regions.json — no code changes. */
interface Region {
  id: string;
  name: string;
  graph: string;
  center: [number, number]; // [lon, lat]
  zoom: number;
}
const DEFAULT_REGION: Region = {
  id: "kouvola",
  name: "Kouvola · Finland",
  graph: "kouvola-graph.json",
  center: [26.95, 60.53],
  zoom: 13,
};

// ── Graph overlay schema (subset of the importer's output we render) ───────

interface GraphPosition {
  latitude_deg: number;
  longitude_deg: number;
}

interface OverlayGraph {
  nodes: { position: GraphPosition }[];
  segments: { geometry: GraphPosition[]; road_class: string }[];
}

/** Emoji only — the readable name comes from i18n (`map.vehicle.*`). */
const VEHICLE_ICONS: Record<string, string> = {
  car: "🚗",
  van: "🚐",
  truck: "🚛",
  bus: "🚌",
  emergency: "🚑",
};

/** Engine status as data, so it re-renders translated when the language changes. */
type EngineStatus =
  | { kind: "loading" }
  | { kind: "ready"; version: string; nodes: number; roads: number }
  | { kind: "engineUnavailable" }
  | { kind: "graphUnavailable" };

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
  const { t, dir } = useLanguage();
  const [status, setStatus] = useState<EngineStatus>({ kind: "loading" });
  const [regionName, setRegionName] = useState<string | null>(null);
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
      } catch {
        // The engine's only failure mode here is "no legal route"; surface it
        // translated rather than leaking the raw Rust string to the user.
        setRoute(null);
        setRouteError("noRoute");
        drawRoute(null);
      }
    },
    [drawRoute]
  );

  /** Place the next route point. Shared by pointer and keyboard input so
   *  both paths are exactly equivalent — a keyboard user is never second
   *  class on a map. */
  const placePoint = useCallback(
    (lat: number, lon: number) => {
      onMapClick?.(lat, lon);
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
      const next: [number, number][] = [...base, [lat, lon]];

      const marker = new maplibregl.Marker({
        color: next.length === 1 ? "#22c55e" : "#ef4444",
      })
        .setLngLat([lon, lat])
        .addTo(map);
      markersRef.current.push(marker);

      setClicks(next);
    },
    [clicks, drawRoute, onMapClick]
  );

  const handleClick = useCallback(
    (e: maplibregl.MapMouseEvent) => placePoint(e.lngLat.lat, e.lngLat.lng),
    [placePoint]
  );

  /** Full keyboard control of the map, owned by this container rather than
   *  MapLibre's canvas-level handler — focus lands on the container (it is
   *  what carries role/aria), so relying on the canvas would leave arrow keys
   *  dead and a keyboard user unable to move between two points at all.
   *  Enter/Space places a point at the center; arrows pan; +/- zoom. */
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLDivElement>) => {
      const map = mapRef.current;
      if (!map) return;

      // A screenful is too coarse and a pixel too fine; a quarter-screen step
      // matches how far a mouse drag typically moves the view.
      const step = () => Math.max(80, map.getContainer().clientHeight / 4);

      switch (e.key) {
        case "Enter":
        case " ": {
          e.preventDefault();
          const c = map.getCenter();
          placePoint(c.lat, c.lng);
          return;
        }
        case "ArrowLeft":
          e.preventDefault();
          map.panBy([-step(), 0]);
          return;
        case "ArrowRight":
          e.preventDefault();
          map.panBy([step(), 0]);
          return;
        case "ArrowUp":
          e.preventDefault();
          map.panBy([0, -step()]);
          return;
        case "ArrowDown":
          e.preventDefault();
          map.panBy([0, step()]);
          return;
        case "+":
        case "=":
          e.preventDefault();
          map.zoomIn();
          return;
        case "-":
        case "_":
          e.preventDefault();
          map.zoomOut();
          return;
        default:
          return;
      }
    },
    [placePoint]
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
        const region: Region = await fetch(engineAssetUrl("regions.json"))
          .then(r => (r.ok ? r.json() : null))
          .then(m => m?.regions?.[0] ?? DEFAULT_REGION)
          .catch(() => DEFAULT_REGION);
        setRegionName(region.name);
        map.jumpTo({ center: region.center, zoom: region.zoom });
        const [graphResp, engine] = await Promise.all([
          fetch(engineAssetUrl(region.graph)),
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
          setStatus({
            kind: "ready",
            version: engine.version(),
            nodes: graph.nodes.length,
            roads: graph.segments.length,
          });
        } else {
          setStatus({ kind: "engineUnavailable" });
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
        setStatus({ kind: "graphUnavailable" });
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

  const statusText =
    status.kind === "ready"
      ? `v${status.version} · ${status.nodes} ${t("map.status.nodes")} · ${status.roads} ${t("map.status.roads")}`
      : t(`map.status.${status.kind}` as Parameters<typeof t>[0]);

  const vehicleName = (v: string) =>
    t(`map.vehicle.${v}` as Parameters<typeof t>[0]);

  /** Single source for what the bottom panel says — also what screen
   *  readers announce, so sighted and non-sighted users get one truth. */
  const panelText = routeError
    ? t("map.error.noRoute")
    : route
      ? `${(route.distanceM / 1000).toFixed(2)} ${t("map.route.km")} · ~${minutes} ${t("map.route.min")} · ${vehicleName(vehicle)}`
      : clicks.length === 1
        ? t("map.hint.destination")
        : t("map.hint.origin");

  return (
    <div
      className="absolute inset-0"
      data-testid="open-navigation-map"
      dir={dir}
    >
      {/* Inline position: maplibre-gl.css sets `.maplibregl-map{position:relative}`,
          which can out-cascade the utility class and collapse the height to 0.
          role=application + tabIndex: the map is a focusable widget that owns
          its own keys (arrows pan, Enter/Space places a point). */}
      <div
        ref={containerRef}
        style={{ position: "absolute", inset: 0 }}
        role="application"
        aria-label={t("map.aria.label")}
        aria-describedby="gane-map-instructions"
        tabIndex={0}
        onKeyDown={handleKeyDown}
      />

      {/* Keyboard/screen-reader instructions — visually hidden, always read. */}
      <p id="gane-map-instructions" className="sr-only">
        {t("map.aria.instructions")}
      </p>

      {/* Status chip */}
      <div className="pointer-events-none absolute left-1/2 top-16 z-20 -translate-x-1/2 rounded-lg bg-slate-900/80 px-3 py-1.5 font-mono text-xs text-emerald-300 backdrop-blur">
        G.A.N.E · {regionName ? `${regionName} · ` : ""}
        {statusText}
      </div>

      {/* Vehicle picker */}
      <div
        className="absolute left-1/2 top-24 z-20 flex -translate-x-1/2 gap-1 rounded-lg bg-slate-900/80 p-1 backdrop-blur"
        role="group"
        aria-label={t("map.aria.vehiclePicker")}
      >
        {Object.keys(VEHICLE_PRESETS).map(v => (
          <button
            key={v}
            type="button"
            onClick={() => setVehicle(v)}
            aria-pressed={vehicle === v}
            aria-label={vehicleName(v)}
            className={`rounded-md px-2 py-1 text-xs transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-400 ${
              vehicle === v
                ? "bg-blue-600 text-white"
                : "text-slate-300 hover:bg-slate-700"
            }`}
          >
            <span aria-hidden="true">{VEHICLE_ICONS[v] ?? ""}</span>{" "}
            {vehicleName(v)}
          </button>
        ))}
      </div>

      {/* Route result / hint — announced politely on every change. */}
      <div
        className="absolute bottom-32 left-1/2 z-20 -translate-x-1/2 rounded-lg bg-slate-900/85 px-4 py-2 text-center text-sm text-slate-100 backdrop-blur"
        role="status"
        aria-live="polite"
        aria-label={t("map.aria.routeStatus")}
      >
        {routeError ? (
          <span className="text-amber-400">{panelText}</span>
        ) : route ? (
          <span>
            <span className="font-semibold text-blue-400">
              {(route.distanceM / 1000).toFixed(2)} {t("map.route.km")}
            </span>
            {" · "}
            <span className="font-semibold text-emerald-400">
              ~{minutes} {t("map.route.min")}
            </span>
            {" · "}
            <span className="text-slate-400">
              <span aria-hidden="true">{VEHICLE_ICONS[vehicle]}</span>{" "}
              {vehicleName(vehicle)}
            </span>
          </span>
        ) : (
          <span className="text-slate-400">{panelText}</span>
        )}
      </div>
    </div>
  );
}
