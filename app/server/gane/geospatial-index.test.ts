/**
 * Tests for client/src/engine/geospatialIndex.ts
 *
 * GeospatialIndex is a pure in-memory geohash grid (no IndexedDB, no
 * fetch, no DOM), so it is directly unit-testable under the Node vitest
 * env. Continues the engine-coverage drive from AUDIT_CODE_LEVEL_PASS2.md.
 */
import { describe, it, expect, beforeEach } from "vitest";
import {
  GeospatialIndex,
  type SpatialPoint,
} from "../../client/src/engine/geospatialIndex";

// Tel Aviv region reference points (approx)
const TEL_AVIV: SpatialPoint   = { id: "ta", lat: 32.0853, lon: 34.7818 };
const HERZLIYA: SpatialPoint   = { id: "hz", lat: 32.1663, lon: 34.8433 };
const JERUSALEM: SpatialPoint  = { id: "jm", lat: 31.7683, lon: 35.2137 };
const EILAT: SpatialPoint      = { id: "el", lat: 29.5581, lon: 34.9482 };
const HAIFA: SpatialPoint      = { id: "ha", lat: 32.7940, lon: 34.9896 };

describe("GeospatialIndex · CRUD", () => {
  let idx: GeospatialIndex;
  beforeEach(() => { idx = new GeospatialIndex(); });

  it("insert adds a point and size reflects it", () => {
    idx.insert(TEL_AVIV);
    expect(idx.size).toBe(1);
    expect(idx.get("ta")).toEqual(TEL_AVIV);
  });

  it("insertBatch inserts all points", () => {
    idx.insertBatch([TEL_AVIV, HERZLIYA, JERUSALEM]);
    expect(idx.size).toBe(3);
  });

  it("insert with the same id replaces previous location", () => {
    idx.insert(TEL_AVIV);
    idx.insert({ ...TEL_AVIV, lat: 1, lon: 1 });
    expect(idx.size).toBe(1);
    expect(idx.get("ta")?.lat).toBe(1);
  });

  it("remove returns true on hit and false on miss", () => {
    idx.insert(TEL_AVIV);
    expect(idx.remove("ta")).toBe(true);
    expect(idx.remove("ta")).toBe(false);
    expect(idx.size).toBe(0);
  });

  it("clear empties the index", () => {
    idx.insertBatch([TEL_AVIV, HERZLIYA]);
    idx.clear();
    expect(idx.size).toBe(0);
    expect(idx.get("ta")).toBeUndefined();
  });
});

describe("GeospatialIndex · findNearby", () => {
  // findNearby only searches the center geohash cell + 8 neighbors, so
  // precision=2 (~1200 km cells) is required to cover Israel-scale
  // distances in a single query. Document this explicitly in the fixture.
  let idx: GeospatialIndex;
  beforeEach(() => {
    idx = new GeospatialIndex(2);
    idx.insertBatch([TEL_AVIV, HERZLIYA, JERUSALEM, EILAT, HAIFA]);
  });

  it("returns points within radius sorted by distance", () => {
    const results = idx.findNearby(TEL_AVIV.lat, TEL_AVIV.lon, 25_000);
    const ids = results.map((r) => r.point.id);
    expect(ids).toContain("ta");
    expect(ids).toContain("hz");
    expect(ids).not.toContain("jm");
    for (let i = 1; i < results.length; i++) {
      expect(results[i].distanceM).toBeGreaterThanOrEqual(results[i - 1].distanceM);
    }
  });

  it("returns empty array when no points are in range", () => {
    // Query a region with no indexed points at all
    const sparse = new GeospatialIndex(2);
    sparse.insert(TEL_AVIV);
    expect(sparse.findNearby(0, -60, 100_000)).toEqual([]);
  });

  it("respects the limit parameter", () => {
    const results = idx.findNearby(TEL_AVIV.lat, TEL_AVIV.lon, 1_000_000, 2);
    expect(results.length).toBe(2);
  });

  it("distance to self is effectively zero (small rounding)", () => {
    const results = idx.findNearby(TEL_AVIV.lat, TEL_AVIV.lon, 500);
    const self = results.find((r) => r.point.id === "ta");
    expect(self).toBeDefined();
    expect(self!.distanceM).toBeLessThanOrEqual(1);
  });
});

describe("GeospatialIndex · findKNearest", () => {
  let idx: GeospatialIndex;
  beforeEach(() => {
    // findKNearest starts at 500 m and expands 3× per attempt up to 5
    // attempts → max ~40 km effective radius. Populate the fixture with
    // points inside that radius of Tel Aviv so the assertion is about the
    // algorithm, not geography.
    idx = new GeospatialIndex(2);
    idx.insertBatch([
      TEL_AVIV,
      HERZLIYA,                                      // ~10 km N
      { id: "bnei", lat: 32.0809, lon: 34.8338 },    // ~5 km E (Bnei Brak)
      { id: "rg",   lat: 32.0719, lon: 34.7922 },    // ~2 km SE (Ramat Gan)
      { id: "hold", lat: 32.0167, lon: 34.7722 },    // ~8 km S (Holon)
      { id: "rish", lat: 31.9730, lon: 34.8066 },    // ~12 km S (Rishon LeZion)
    ]);
  });

  it("returns exactly K nearest points when K <= total", () => {
    const results = idx.findKNearest(TEL_AVIV.lat, TEL_AVIV.lon, 3);
    expect(results.length).toBe(3);
    expect(results[0].point.id).toBe("ta");
    // Next two must be closer than the others
    for (let i = 1; i < results.length; i++) {
      expect(results[i].distanceM).toBeGreaterThanOrEqual(results[i - 1].distanceM);
    }
  });

  it("expands its search radius until K are found", () => {
    // K=6 forces expansion past the 500 m starting radius; fixture has
    // points out to ~12 km, well within the 40 km cap.
    const results = idx.findKNearest(TEL_AVIV.lat, TEL_AVIV.lon, 6);
    expect(results.length).toBe(6);
  });
});

describe("GeospatialIndex · findInBoundingBox", () => {
  it("returns only points inside the bbox", () => {
    const idx = new GeospatialIndex();
    idx.insertBatch([TEL_AVIV, HERZLIYA, JERUSALEM, EILAT]);
    const bbox = { minLat: 32, maxLat: 33, minLon: 34.5, maxLon: 35 };
    const ids = idx.findInBoundingBox(bbox).map((p) => p.id).sort();
    expect(ids).toEqual(["hz", "ta"]);
  });

  it("limit caps the result length", () => {
    const idx = new GeospatialIndex();
    for (let i = 0; i < 50; i++) {
      idx.insert({ id: `p${i}`, lat: 32 + i * 0.001, lon: 34.78 });
    }
    const bbox = { minLat: 0, maxLat: 90, minLon: -180, maxLon: 180 };
    expect(idx.findInBoundingBox(bbox, 10).length).toBe(10);
  });
});

describe("GeospatialIndex · polygon queries", () => {
  it("isInsidePolygon correctly classifies interior and exterior points", () => {
    const idx = new GeospatialIndex();
    // Square polygon around Tel Aviv, ~0.1° each side
    const poly = [
      { lat: 32.0, lon: 34.7 },
      { lat: 32.2, lon: 34.7 },
      { lat: 32.2, lon: 34.9 },
      { lat: 32.0, lon: 34.9 },
    ];
    expect(idx.isInsidePolygon(TEL_AVIV.lat, TEL_AVIV.lon, poly)).toBe(true);
    expect(idx.isInsidePolygon(EILAT.lat, EILAT.lon, poly)).toBe(false);
  });

  it("findInPolygon returns only points inside", () => {
    const idx = new GeospatialIndex();
    idx.insertBatch([TEL_AVIV, HERZLIYA, JERUSALEM, EILAT]);
    const poly = [
      { lat: 31.9, lon: 34.6 },
      { lat: 32.3, lon: 34.6 },
      { lat: 32.3, lon: 35.0 },
      { lat: 31.9, lon: 35.0 },
    ];
    const ids = idx.findInPolygon(poly).map((p) => p.id).sort();
    expect(ids).toEqual(["hz", "ta"]);
  });
});

describe("GeospatialIndex · stats", () => {
  it("getStats aggregates totals and per-cell averages", () => {
    const idx = new GeospatialIndex(5);
    idx.insertBatch([TEL_AVIV, HERZLIYA, JERUSALEM, EILAT]);
    const s = idx.getStats();
    expect(s.totalPoints).toBe(4);
    expect(s.totalCells).toBeGreaterThan(0);
    expect(s.avgPointsPerCell).toBeGreaterThan(0);
    expect(s.maxPointsInCell).toBeGreaterThanOrEqual(1);
  });

  it("returns zeros for an empty index", () => {
    const s = new GeospatialIndex().getStats();
    expect(s.totalPoints).toBe(0);
    expect(s.totalCells).toBe(0);
    expect(s.avgPointsPerCell).toBe(0);
    expect(s.maxPointsInCell).toBe(0);
  });
});
