/**
 * G.A.N.E — Geospatial Utilities
 * ================================
 * Shared geospatial functions used across all GANE routers.
 * Single source of truth — no duplication.
 */

const EARTH_RADIUS_M = 6_371_000;
const DEG_TO_RAD = Math.PI / 180;
const METERS_PER_DEGREE_LAT = 111_320;

/**
 * Haversine distance between two WGS-84 coordinates.
 * Returns distance in meters.
 */
export function haversineDistance(
  lat1: number,
  lon1: number,
  lat2: number,
  lon2: number,
): number {
  const dLat = (lat2 - lat1) * DEG_TO_RAD;
  const dLon = (lon2 - lon1) * DEG_TO_RAD;
  const a =
    Math.sin(dLat / 2) ** 2 +
    Math.cos(lat1 * DEG_TO_RAD) *
      Math.cos(lat2 * DEG_TO_RAD) *
      Math.sin(dLon / 2) ** 2;
  return EARTH_RADIUS_M * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

/**
 * Compute a bounding box in degrees for a given center + radius in meters.
 * Useful for SQL WHERE clause pre-filtering before Haversine refinement.
 */
export function boundingBox(
  lat: number,
  lon: number,
  radiusM: number,
): { minLat: number; maxLat: number; minLon: number; maxLon: number } {
  const latDelta = radiusM / METERS_PER_DEGREE_LAT;
  const lonDelta =
    radiusM / (METERS_PER_DEGREE_LAT * Math.cos(lat * DEG_TO_RAD));
  return {
    minLat: lat - latDelta,
    maxLat: lat + latDelta,
    minLon: lon - lonDelta,
    maxLon: lon + lonDelta,
  };
}

/**
 * Calculate total route distance through an ordered list of stops,
 * starting and ending at a depot.
 */
export function calculateRouteDistance(
  stops: { lat: number; lon: number }[],
  depotLat: number,
  depotLon: number,
): number {
  let dist = 0;
  let prevLat = depotLat;
  let prevLon = depotLon;
  for (const s of stops) {
    dist += haversineDistance(prevLat, prevLon, s.lat, s.lon);
    prevLat = s.lat;
    prevLon = s.lon;
  }
  dist += haversineDistance(prevLat, prevLon, depotLat, depotLon);
  return Math.round(dist);
}

/**
 * 2-Opt local search improvement for TSP/VRP routes.
 * Iteratively reverses segments to reduce total distance.
 */
export function twoOptImprove<T extends { lat: number; lon: number }>(
  route: T[],
  depotLat: number,
  depotLon: number,
  maxIterations: number = 1000,
): T[] {
  const result = [...route];
  let improved = true;
  let iterations = 0;

  while (improved && iterations < maxIterations) {
    improved = false;
    iterations++;

    for (let i = 0; i < result.length - 1; i++) {
      for (let j = i + 1; j < result.length; j++) {
        const currentDist = calculateRouteDistance(result, depotLat, depotLon);
        const newRoute = [...result];
        const segment = newRoute.slice(i, j + 1).reverse();
        newRoute.splice(i, j - i + 1, ...segment);
        const newDist = calculateRouteDistance(newRoute, depotLat, depotLon);

        if (newDist < currentDist) {
          result.splice(0, result.length, ...newRoute);
          improved = true;
        }
      }
    }
  }

  return result;
}
