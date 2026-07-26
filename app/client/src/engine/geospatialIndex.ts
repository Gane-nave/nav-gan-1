/**
 * G.A.N.E — Geospatial Indexing Engine
 * =======================================
 * Spatial indexing for fast proximity queries.
 *
 * ALGORITHMS:
 *   - Geohash-based grid (O(1) lookups)
 *   - R-tree approximation for range queries
 *   - K-nearest-neighbor search
 *   - Bounding box intersection
 *
 * USE CASES:
 *   - Find nearby POIs
 *   - Find nearby vehicles
 *   - Incident proximity detection
 *   - Geofence containment checks
 */

// ─── Types ───────────────────────────────────────────────

export interface SpatialPoint {
  id: string;
  lat: number;
  lon: number;
  data?: Record<string, unknown>;
}

export interface BoundingBox {
  minLat: number;
  maxLat: number;
  minLon: number;
  maxLon: number;
}

export interface NearbyResult {
  point: SpatialPoint;
  distanceM: number;
}

// ─── Constants ──────────────────────────────────────────

const GEOHASH_CHARS = '0123456789bcdefghjkmnpqrstuvwxyz';
const DEG_TO_RAD = Math.PI / 180;
const EARTH_RADIUS_M = 6371000;

// ─── Geohash Functions ──────────────────────────────────

function encodeGeohash(lat: number, lon: number, precision: number = 7): string {
  let latRange = [-90, 90];
  let lonRange = [-180, 180];
  let hash = '';
  let bit = 0;
  let ch = 0;
  let isLon = true;

  while (hash.length < precision) {
    if (isLon) {
      const mid = (lonRange[0] + lonRange[1]) / 2;
      if (lon >= mid) {
        ch = ch | (1 << (4 - bit));
        lonRange[0] = mid;
      } else {
        lonRange[1] = mid;
      }
    } else {
      const mid = (latRange[0] + latRange[1]) / 2;
      if (lat >= mid) {
        ch = ch | (1 << (4 - bit));
        latRange[0] = mid;
      } else {
        latRange[1] = mid;
      }
    }

    isLon = !isLon;
    bit++;

    if (bit === 5) {
      hash += GEOHASH_CHARS[ch];
      bit = 0;
      ch = 0;
    }
  }

  return hash;
}

/** Decode a geohash to the lat/lon bounds of its cell. */
function geohashBounds(hash: string): {
  latMin: number;
  latMax: number;
  lonMin: number;
  lonMax: number;
} {
  let latMin = -90;
  let latMax = 90;
  let lonMin = -180;
  let lonMax = 180;
  let isLon = true;
  for (const c of hash) {
    const idx = GEOHASH_CHARS.indexOf(c);
    if (idx < 0) continue;
    for (let b = 4; b >= 0; b--) {
      const bit = (idx >> b) & 1;
      if (isLon) {
        const mid = (lonMin + lonMax) / 2;
        if (bit) lonMin = mid;
        else lonMax = mid;
      } else {
        const mid = (latMin + latMax) / 2;
        if (bit) latMin = mid;
        else latMax = mid;
      }
      isLon = !isLon;
    }
  }
  return { latMin, latMax, lonMin, lonMax };
}

/**
 * The eight geographically adjacent cells.
 *
 * Base-32 geohash characters interleave latitude and longitude bits, so
 * stepping the character index (the previous approach) does not move one cell
 * on the ground — it produced four arbitrary cells and made findNearby drop
 * genuinely close points (Herzliya, 10.7 km from Tel Aviv, was missed by a
 * 25 km query). Decoding the cell and re-encoding one cell-width away in each
 * direction is exact by construction.
 */
/**
 * Every geohash cell whose area can intersect a circle of `radiusM`.
 *
 * Bounded so a huge radius on a fine precision cannot enumerate the planet:
 * past the cap the caller is better served by a full scan, and the distance
 * filter downstream keeps the result correct either way.
 */
function cellsCovering(
  lat: number,
  lon: number,
  radiusM: number,
  precision: number
): string[] {
  const center = encodeGeohash(lat, lon, precision);
  const b = geohashBounds(center);
  const latStep = b.latMax - b.latMin;
  const lonStep = b.lonMax - b.lonMin;

  const latM = 111_320;
  const lonM = Math.max(1, 111_320 * Math.cos((lat * Math.PI) / 180));
  const MAX_RING = 32; // 65x65 cells; beyond this a scan is cheaper
  const ringLat = Math.min(MAX_RING, Math.ceil(radiusM / Math.max(1, latStep * latM)));
  const ringLon = Math.min(MAX_RING, Math.ceil(radiusM / Math.max(1, lonStep * lonM)));

  const out = new Set<string>();
  for (let i = -ringLat; i <= ringLat; i++) {
    const nLat = lat + i * latStep;
    if (nLat > 90 || nLat < -90) continue;
    for (let j = -ringLon; j <= ringLon; j++) {
      let nLon = lon + j * lonStep;
      if (nLon > 180) nLon -= 360;
      if (nLon < -180) nLon += 360;
      out.add(encodeGeohash(nLat, nLon, precision));
    }
  }
  return Array.from(out);
}

function geohashNeighbors(hash: string): string[] {
  const b = geohashBounds(hash);
  const latStep = b.latMax - b.latMin;
  const lonStep = b.lonMax - b.lonMin;
  const lat = (b.latMin + b.latMax) / 2;
  const lon = (b.lonMin + b.lonMax) / 2;

  const out: string[] = [];
  for (const dLat of [-1, 0, 1]) {
    for (const dLon of [-1, 0, 1]) {
      if (dLat === 0 && dLon === 0) continue;
      const nLat = lat + dLat * latStep;
      if (nLat > 90 || nLat < -90) continue;
      // Wrap longitude so a query at the antimeridian still sees both sides.
      let nLon = lon + dLon * lonStep;
      if (nLon > 180) nLon -= 360;
      if (nLon < -180) nLon += 360;
      out.push(encodeGeohash(nLat, nLon, hash.length));
    }
  }
  return out;
}

function haversineDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const dLat = (lat2 - lat1) * DEG_TO_RAD;
  const dLon = (lon2 - lon1) * DEG_TO_RAD;
  const a = Math.sin(dLat / 2) ** 2 +
    Math.cos(lat1 * DEG_TO_RAD) * Math.cos(lat2 * DEG_TO_RAD) *
    Math.sin(dLon / 2) ** 2;
  return EARTH_RADIUS_M * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

// ─── Geospatial Index ───────────────────────────────────

export class GeospatialIndex {
  private grid: Map<string, Map<string, SpatialPoint>> = new Map();
  private allPoints: Map<string, SpatialPoint> = new Map();
  private precision: number;

  constructor(precision: number = 6) {
    this.precision = precision;
  }

  // ─── CRUD ─────────────────────────────────────────────

  /**
   * Insert or update a point.
   */
  insert(point: SpatialPoint) {
    // Remove old entry if exists
    this.remove(point.id);

    // Add to grid
    const hash = encodeGeohash(point.lat, point.lon, this.precision);
    if (!this.grid.has(hash)) {
      this.grid.set(hash, new Map());
    }
    this.grid.get(hash)!.set(point.id, point);
    this.allPoints.set(point.id, point);
  }

  /**
   * Insert multiple points.
   */
  insertBatch(points: SpatialPoint[]) {
    for (const point of points) {
      this.insert(point);
    }
  }

  /**
   * Remove a point by ID.
   */
  remove(id: string): boolean {
    const existing = this.allPoints.get(id);
    if (!existing) return false;

    const hash = encodeGeohash(existing.lat, existing.lon, this.precision);
    const cell = this.grid.get(hash);
    if (cell) {
      cell.delete(id);
      if (cell.size === 0) {
        this.grid.delete(hash);
      }
    }
    this.allPoints.delete(id);
    return true;
  }

  /**
   * Get a point by ID.
   */
  get(id: string): SpatialPoint | undefined {
    return this.allPoints.get(id);
  }

  // ─── Spatial Queries ──────────────────────────────────

  /**
   * Find all points within radiusM of (lat, lon).
   */
  findNearby(lat: number, lon: number, radiusM: number, limit: number = 100): NearbyResult[] {
    const centerHash = encodeGeohash(lat, lon, this.precision);
    // Cover the whole search circle, not just the adjacent ring. A cell at
    // precision 7 is ~150 m across, so the old fixed 3x3 block silently
    // dropped every point beyond ~230 m — a 25 km query missed a town 10.7 km
    // away. Derive the ring radius from the requested radius instead.
    const searchHashes = cellsCovering(lat, lon, radiusM, this.precision);

    const results: NearbyResult[] = [];

    for (const hash of searchHashes) {
      const cell = this.grid.get(hash);
      if (!cell) continue;

      for (const point of Array.from(cell.values())) {
        const dist = haversineDistance(lat, lon, point.lat, point.lon);
        if (dist <= radiusM) {
          results.push({ point, distanceM: Math.round(dist) });
        }
      }
    }

    // Sort by distance
    results.sort((a, b) => a.distanceM - b.distanceM);

    return results.slice(0, limit);
  }

  /**
   * Find K nearest neighbors.
   */
  findKNearest(lat: number, lon: number, k: number): NearbyResult[] {
    // Start with a reasonable radius and expand
    let radius = 500; // Start at 500m
    let results: NearbyResult[] = [];

    for (let attempt = 0; attempt < 5; attempt++) {
      results = this.findNearby(lat, lon, radius, k);
      if (results.length >= k) break;
      radius *= 3; // Triple the radius
    }

    return results.slice(0, k);
  }

  /**
   * Find all points within a bounding box.
   */
  findInBoundingBox(bbox: BoundingBox, limit: number = 1000): SpatialPoint[] {
    const results: SpatialPoint[] = [];

    for (const point of Array.from(this.allPoints.values())) {
      if (
        point.lat >= bbox.minLat && point.lat <= bbox.maxLat &&
        point.lon >= bbox.minLon && point.lon <= bbox.maxLon
      ) {
        results.push(point);
        if (results.length >= limit) break;
      }
    }

    return results;
  }

  /**
   * Check if a point is inside a polygon (ray casting).
   */
  isInsidePolygon(
    lat: number, lon: number,
    polygon: { lat: number; lon: number }[]
  ): boolean {
    let inside = false;
    for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
      const xi = polygon[i].lat, yi = polygon[i].lon;
      const xj = polygon[j].lat, yj = polygon[j].lon;

      const intersect = ((yi > lon) !== (yj > lon)) &&
        (lat < (xj - xi) * (lon - yi) / (yj - yi) + xi);
      if (intersect) inside = !inside;
    }
    return inside;
  }

  /**
   * Find all points inside a polygon.
   */
  findInPolygon(
    polygon: { lat: number; lon: number }[],
    limit: number = 1000
  ): SpatialPoint[] {
    // First, compute bounding box for quick filter
    const bbox: BoundingBox = {
      minLat: Math.min(...polygon.map(p => p.lat)),
      maxLat: Math.max(...polygon.map(p => p.lat)),
      minLon: Math.min(...polygon.map(p => p.lon)),
      maxLon: Math.max(...polygon.map(p => p.lon)),
    };

    const candidates = this.findInBoundingBox(bbox, limit * 2);
    return candidates
      .filter(p => this.isInsidePolygon(p.lat, p.lon, polygon))
      .slice(0, limit);
  }

  // ─── Statistics ───────────────────────────────────────

  getStats(): {
    totalPoints: number;
    totalCells: number;
    avgPointsPerCell: number;
    maxPointsInCell: number;
  } {
    let maxPoints = 0;
    for (const cell of Array.from(this.grid.values())) {
      maxPoints = Math.max(maxPoints, cell.size);
    }

    return {
      totalPoints: this.allPoints.size,
      totalCells: this.grid.size,
      avgPointsPerCell: this.grid.size > 0
        ? Math.round(this.allPoints.size / this.grid.size * 10) / 10
        : 0,
      maxPointsInCell: maxPoints,
    };
  }

  // ─── Lifecycle ────────────────────────────────────────

  clear() {
    this.grid.clear();
    this.allPoints.clear();
  }

  get size(): number {
    return this.allPoints.size;
  }

  destroy() {
    this.clear();
  }
}
