/**
 * G.A.N.E — Constellation Handoff Classification (shared, pure)
 * ===============================================================
 * Given the previous and next set of available GNSS constellations plus
 * the active "best" constellation id, classifies what happened between
 * two state ticks. Used by `useConstellationHandoffAlerts` to decide
 * which user-visible toast to fire.
 *
 * Constellation IDs mirror engine/multiConstellation.ts.
 */

export type ConstellationId =
  | "GPS"
  | "GLONASS"
  | "GALILEO"
  | "BEIDOU"
  | "NAVIC"
  | "QZSS"
  | "SBAS";

export interface ConstellationSnapshot {
  available: ReadonlySet<ConstellationId>;
  best: ConstellationId;
  spoofingRisk: number; // 0..1
  totalUsed: number;
  pdop: number;
}

export interface HandoffDelta {
  lost: ConstellationId[];
  gained: ConstellationId[];
  bestChanged: boolean;
  fromBest: ConstellationId;
  toBest: ConstellationId;
  spoofingRoseAboveThreshold: boolean;
  spoofingFellBelowThreshold: boolean;
  crossedMinUsedFloor: boolean;      // dropped below 4 sats (can't compute 3D fix)
  restoredMinUsedFloor: boolean;     // came back up to >= 4
}

export const SPOOFING_ALERT_THRESHOLD = 0.6;
export const MIN_SATELLITES_FOR_3D_FIX = 4;

export function diffSnapshots(
  prev: ConstellationSnapshot,
  next: ConstellationSnapshot,
): HandoffDelta {
  const lost: ConstellationId[] = [];
  const gained: ConstellationId[] = [];
  Array.from(prev.available).forEach((id) => {
    if (!next.available.has(id)) lost.push(id);
  });
  Array.from(next.available).forEach((id) => {
    if (!prev.available.has(id)) gained.push(id);
  });

  const spoofingRoseAboveThreshold =
    prev.spoofingRisk < SPOOFING_ALERT_THRESHOLD &&
    next.spoofingRisk >= SPOOFING_ALERT_THRESHOLD;
  const spoofingFellBelowThreshold =
    prev.spoofingRisk >= SPOOFING_ALERT_THRESHOLD &&
    next.spoofingRisk < SPOOFING_ALERT_THRESHOLD;

  const crossedMinUsedFloor =
    prev.totalUsed >= MIN_SATELLITES_FOR_3D_FIX &&
    next.totalUsed < MIN_SATELLITES_FOR_3D_FIX;
  const restoredMinUsedFloor =
    prev.totalUsed < MIN_SATELLITES_FOR_3D_FIX &&
    next.totalUsed >= MIN_SATELLITES_FOR_3D_FIX;

  return {
    lost: lost.sort(),
    gained: gained.sort(),
    bestChanged: prev.best !== next.best,
    fromBest: prev.best,
    toBest: next.best,
    spoofingRoseAboveThreshold,
    spoofingFellBelowThreshold,
    crossedMinUsedFloor,
    restoredMinUsedFloor,
  };
}

export type HandoffAlert =
  | { kind: "constellation-lost"; ids: ConstellationId[] }
  | { kind: "constellation-regained"; ids: ConstellationId[] }
  | { kind: "best-changed"; from: ConstellationId; to: ConstellationId }
  | { kind: "spoofing-risk-rose" }
  | { kind: "spoofing-risk-cleared" }
  | { kind: "below-3d-fix-floor" }
  | { kind: "3d-fix-floor-restored" };

/**
 * Map a delta to an ordered list of alerts. Most severe first so the
 * consumer can choose to show only the top N without losing the critical
 * events during rapid changes.
 */
export function deltaToAlerts(d: HandoffDelta): HandoffAlert[] {
  const out: HandoffAlert[] = [];
  if (d.crossedMinUsedFloor) out.push({ kind: "below-3d-fix-floor" });
  if (d.spoofingRoseAboveThreshold) out.push({ kind: "spoofing-risk-rose" });
  if (d.lost.length > 0) out.push({ kind: "constellation-lost", ids: d.lost });
  if (d.bestChanged) {
    out.push({ kind: "best-changed", from: d.fromBest, to: d.toBest });
  }
  if (d.gained.length > 0) {
    out.push({ kind: "constellation-regained", ids: d.gained });
  }
  if (d.restoredMinUsedFloor) out.push({ kind: "3d-fix-floor-restored" });
  if (d.spoofingFellBelowThreshold) out.push({ kind: "spoofing-risk-cleared" });
  return out;
}
