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

function geohashNeighbors(hash: string): string[] {
  // Simplified: return the 8 neighboring geohashes
  // For production, use proper neighbor calculation
  const neighbors: string[] = [];
  const prefix = hash.slice(0, -1);
  const lastChar = hash[hash.length - 1];
  const idx = GEOHASH_CHARS.indexOf(lastChar);

  for (let d = -1; d <= 1; d++) {
    const ni = idx + d;
    if (ni >= 0 && ni < GEOHASH_CHARS.length) {
      neighbors.push(prefix + GEOHASH_CHARS[ni]);
    }
  }

  // Also add prefix-level neighbors
  if (prefix.length > 0) {
    const prefixLast = prefix[prefix.length - 1];
    const prefixIdx = GEOHASH_CHARS.indexOf(prefixLast);
    for (let d = -1; d <= 1; d++) {
      const ni = prefixIdx + d;
      if (ni >= 0 && ni < GEOHASH_CHARS.length) {
        const newPrefix = prefix.slice(0, -1) + GEOHASH_CHARS[ni];
        neighbors.push(newPrefix + lastChar);
      }
    }
  }

  return Array.from(new Set(neighbors)).filter(n => n !== hash);
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
    const searchHashes = [centerHash, ...geohashNeighbors(centerHash)];

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
