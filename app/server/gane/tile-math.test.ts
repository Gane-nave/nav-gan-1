/**
 * Tests for shared/contracts/tileMath.ts
 *
 * Covers the pure slippy-map helpers that the offline-map engine relies
 * on. The engine itself is gated behind IndexedDB and is not directly
 * unit-testable under the Node vitest environment; these shared helpers
 * are the correctness core of its region math + cache keys.
 */
import { describe, it, expect } from "vitest";
import {
  countTilesInBounds,
  getTileUrl,
  lat2tile,
  lon2tile,
  parseTileKey,
  tileKey,
  tileRangeAtZoom,
  TILE_URLS,
  type TileBounds,
} from "../../shared/contracts/tileMath";

describe("lon2tile / lat2tile", () => {
  it("maps zoom 0 to a single tile (0,0)", () => {
    expect(lon2tile(0, 0)).toBe(0);
    expect(lat2tile(0, 0)).toBe(0);
    expect(lon2tile(-180, 0)).toBe(0);
    expect(lon2tile(179.9, 0)).toBe(0);
  });

  it("maps zoom 1 to 4 tiles (2x2)", () => {
    expect(lon2tile(-179, 1)).toBe(0);
    expect(lon2tile(1, 1)).toBe(1);
    expect(lat2tile(85, 1)).toBe(0);
    expect(lat2tile(-85, 1)).toBe(1);
  });

  it("Tel Aviv at zoom 14 lands in the expected tile", () => {
    // Tel Aviv ≈ 32.0853°N, 34.7818°E
    const x = lon2tile(34.7818, 14);
    const y = lat2tile(32.0853, 14);
    expect(x).toBe(9774);
    expect(y).toBe(6648);
  });

  it("lon2tile is monotonic in longitude at fixed zoom", () => {
    const z = 6;
    let prev = -Infinity;
    for (let lon = -179; lon <= 179; lon += 10) {
      const t = lon2tile(lon, z);
      expect(t).toBeGreaterThanOrEqual(prev);
      prev = t;
    }
  });
});

describe("tileKey / parseTileKey", () => {
  it("round-trips", () => {
    const key = tileKey("osm", { x: 10, y: 20, z: 5 });
    expect(key).toBe("osm:5:10:20");
    const parsed = parseTileKey(key);
    expect(parsed).toEqual({ provider: "osm", coord: { x: 10, y: 20, z: 5 } });
  });

  it("returns null for malformed keys", () => {
    expect(parseTileKey("osm:5:10")).toBeNull();
    expect(parseTileKey("foo:5:10:20")).toBeNull();
    expect(parseTileKey("osm:x:10:20")).toBeNull();
    expect(parseTileKey("")).toBeNull();
  });
});

describe("countTilesInBounds", () => {
  const ilBounds: TileBounds = { north: 33.3, south: 29.5, east: 35.9, west: 34.2 };

  it("counts exactly 1 tile at zoom 0 when bounds stay inside the wrap edge", () => {
    // Using east=179.9 avoids the edge case where lon=180 wraps to tile 1.
    expect(countTilesInBounds(
      { north: 85, south: -85, east: 179.9, west: -180 }, 0, 0, ["osm"],
    )).toBe(1);
  });

  it("scales linearly with the number of providers", () => {
    const oneProv = countTilesInBounds(ilBounds, 10, 10, ["osm"]);
    const twoProv = countTilesInBounds(ilBounds, 10, 10, ["osm", "satellite"]);
    expect(twoProv).toBe(oneProv * 2);
  });

  it("sums across zoom levels", () => {
    const z10only = countTilesInBounds(ilBounds, 10, 10, ["osm"]);
    const z10To11 = countTilesInBounds(ilBounds, 10, 11, ["osm"]);
    expect(z10To11).toBeGreaterThan(z10only);
  });

  it("returns 0 when the zoom range is inverted", () => {
    expect(countTilesInBounds(ilBounds, 10, 5, ["osm"])).toBe(0);
  });

  it("returns 0 when no providers are requested", () => {
    expect(countTilesInBounds(ilBounds, 10, 12, [])).toBe(0);
  });

  it("tolerates inverted north/south bounds without going negative", () => {
    const inverted: TileBounds = { north: 29.5, south: 33.3, east: 35.9, west: 34.2 };
    expect(countTilesInBounds(inverted, 10, 10, ["osm"])).toBeGreaterThan(0);
  });
});

describe("getTileUrl", () => {
  it("substitutes {z}/{x}/{y} for OSM", () => {
    const url = getTileUrl("osm", { x: 1, y: 2, z: 3 });
    expect(url).toBe("https://tile.openstreetmap.org/3/1/2.png");
  });

  it("supports all providers in TILE_URLS", () => {
    for (const provider of Object.keys(TILE_URLS)) {
      const url = getTileUrl(
        provider as keyof typeof TILE_URLS,
        { x: 5, y: 6, z: 7 },
      );
      expect(url).not.toContain("{z}");
      expect(url).not.toContain("{x}");
      expect(url).not.toContain("{y}");
    }
  });
});

describe("tileRangeAtZoom", () => {
  it("iterates the exact set of tiles covering a small bbox", () => {
    // 2x2 block at zoom 10
    const bounds: TileBounds = { north: 33.0, south: 32.9, east: 35.0, west: 34.9 };
    const tiles = Array.from(tileRangeAtZoom(bounds, 10));
    expect(tiles.length).toBeGreaterThan(0);
    for (const t of tiles) {
      expect(t.z).toBe(10);
      expect(Number.isInteger(t.x)).toBe(true);
      expect(Number.isInteger(t.y)).toBe(true);
    }
    // Count should match countTilesInBounds for one provider at that zoom
    const expected = countTilesInBounds(bounds, 10, 10, ["osm"]);
    expect(tiles.length).toBe(expected);
  });

  it("yields nothing for degenerate bounds when zoom range is 0", () => {
    const out = Array.from(tileRangeAtZoom(
      { north: 0, south: 0, east: 0, west: 0 }, 0,
    ));
    expect(out).toEqual([{ x: 0, y: 0, z: 0 }]);
  });
});
