/**
 * G.A.N.E — Tile Math + URL Helpers (shared, pure)
 * ==================================================
 * Slippy-map tile math (XYZ) and cache-key / URL helpers shared between
 * the offline-map engine and any future map-related surface. Pure — no
 * IndexedDB / fetch / DOM references — so the server-side vitest runner
 * can cover it without a browser shim.
 *
 * Addresses `AUDIT_CODE_LEVEL_PASS2.md` §2.5 — continues the engine
 * coverage drive started in `policy-engine.test.ts`.
 *
 * Conventions:
 *   - XYZ scheme (Google/OSM), origin NW corner
 *   - Web Mercator projection (EPSG:3857)
 *   - Zoom 0 = one 256x256 tile covering the world
 */

export type TileProvider =
  | "osm"
  | "satellite"
  | "terrain"
  | "topo"
  | "dark"
  | "hybrid";

export interface TileCoord {
  x: number;
  y: number;
  z: number;
}

export interface TileBounds {
  north: number;
  south: number;
  east: number;
  west: number;
}

export const TILE_URLS: Record<TileProvider, string> = {
  osm: "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
  satellite:
    "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}",
  terrain: "https://tile.opentopomap.org/{z}/{x}/{y}.png",
  topo:
    "https://basemap.nationalmap.gov/arcgis/rest/services/USGSTopo/MapServer/tile/{z}/{y}/{x}",
  dark:
    "https://tiles.stadiamaps.com/tiles/alidade_smooth_dark/{z}/{x}/{y}.png",
  hybrid: "https://mt1.google.com/vt/lyrs=y&x={x}&y={y}&z={z}",
};

/** Longitude (deg) → tile X at a given zoom. */
export function lon2tile(lon: number, zoom: number): number {
  return Math.floor(((lon + 180) / 360) * Math.pow(2, zoom));
}

/** Latitude (deg) → tile Y at a given zoom. Web Mercator, NW-origin. */
export function lat2tile(lat: number, zoom: number): number {
  const rad = (lat * Math.PI) / 180;
  return Math.floor(
    ((1 - Math.log(Math.tan(rad) + 1 / Math.cos(rad)) / Math.PI) / 2) *
      Math.pow(2, zoom),
  );
}

/** Stable cache key: "provider:z:x:y". */
export function tileKey(provider: TileProvider, coord: TileCoord): string {
  return `${provider}:${coord.z}:${coord.x}:${coord.y}`;
}

/** Parse a cache key back into its components. Returns null on malformed input. */
export function parseTileKey(
  key: string,
): { provider: TileProvider; coord: TileCoord } | null {
  const parts = key.split(":");
  if (parts.length !== 4) return null;
  const [provider, zStr, xStr, yStr] = parts;
  if (!(provider in TILE_URLS)) return null;
  const z = Number(zStr);
  const x = Number(xStr);
  const y = Number(yStr);
  if (!Number.isFinite(z) || !Number.isFinite(x) || !Number.isFinite(y)) {
    return null;
  }
  return { provider: provider as TileProvider, coord: { x, y, z } };
}

/**
 * Count the number of tiles inside `bounds` across the [minZoom..maxZoom]
 * range for every requested provider. Guards against inverted bounds.
 */
export function countTilesInBounds(
  bounds: TileBounds,
  minZoom: number,
  maxZoom: number,
  providers: TileProvider[],
): number {
  if (minZoom > maxZoom) return 0;
  if (providers.length === 0) return 0;
  let total = 0;
  for (let z = minZoom; z <= maxZoom; z++) {
    const xMin = Math.min(lon2tile(bounds.west, z), lon2tile(bounds.east, z));
    const xMax = Math.max(lon2tile(bounds.west, z), lon2tile(bounds.east, z));
    const yMin = Math.min(lat2tile(bounds.north, z), lat2tile(bounds.south, z));
    const yMax = Math.max(lat2tile(bounds.north, z), lat2tile(bounds.south, z));
    const tilesAtZoom = (xMax - xMin + 1) * (yMax - yMin + 1);
    total += tilesAtZoom * providers.length;
  }
  return total;
}

/** Substitute {z}/{x}/{y} in the provider's URL template. */
export function getTileUrl(provider: TileProvider, coord: TileCoord): string {
  return TILE_URLS[provider]
    .replace("{z}", String(coord.z))
    .replace("{x}", String(coord.x))
    .replace("{y}", String(coord.y));
}

/**
 * Iterate every (x,y) tile covered by `bounds` at a single zoom level.
 * Order: row-major, north-to-south, west-to-east. Useful for priming a
 * download queue in a predictable order.
 */
export function* tileRangeAtZoom(
  bounds: TileBounds,
  zoom: number,
): Generator<TileCoord> {
  const xMin = Math.min(lon2tile(bounds.west, zoom), lon2tile(bounds.east, zoom));
  const xMax = Math.max(lon2tile(bounds.west, zoom), lon2tile(bounds.east, zoom));
  const yMin = Math.min(lat2tile(bounds.north, zoom), lat2tile(bounds.south, zoom));
  const yMax = Math.max(lat2tile(bounds.north, zoom), lat2tile(bounds.south, zoom));
  for (let y = yMin; y <= yMax; y++) {
    for (let x = xMin; x <= xMax; x++) {
      yield { x, y, z: zoom };
    }
  }
}
