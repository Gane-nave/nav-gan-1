/**
 * G.A.N.E — Fallback Severity Mapping (shared, pure)
 * ===================================================
 * Pure helpers shared between the positioning-fallback chain engine
 * (client/src/engine/positionFallbackChain.ts) and the UI severity
 * indicator. Kept in `shared/` so both server tests and client code
 * can import without React/DOM dependencies.
 *
 * ProviderTier values (1 = best, 5 = worst):
 *   1 — GNSS (GPS, Galileo, GLONASS, BeiDou, QZSS, NavIC)
 *   2 — SBAS (WAAS, EGNOS, GAGAN, MSAS)
 *   3 — Network positioning (WiFi, Cell tower triangulation)
 *   4 — Dead-reckoning (IMU ESKF, PDR, Visual Odometry)
 *   5 — Cached fix / IP geolocation
 */

export type ProviderTier = 1 | 2 | 3 | 4 | 5;
export type FallbackSeverity = "green" | "yellow" | "orange" | "red";

export const SEVERITY_LABEL: Record<FallbackSeverity, string> = {
  green: "OPTIMAL",
  yellow: "DEGRADED",
  orange: "FALLBACK",
  red: "LAST_RESORT",
};

export function tierToSeverity(tier: ProviderTier): FallbackSeverity {
  if (tier <= 2) return "green";
  if (tier === 3) return "yellow";
  if (tier === 4) return "orange";
  return "red";
}

/**
 * Classify a transition between two tiers into one of four action categories.
 * Consumers (e.g. toast emitters) can branch on this instead of duplicating
 * the conditional logic.
 */
export type TransitionKind =
  | "recovered-to-gnss"
  | "gnss-to-network"
  | "network-to-dead-reckoning"
  | "dead-reckoning-to-cached"
  | "improved"
  | "degraded"
  | "unchanged";

export function classifyTransition(
  fromTier: ProviderTier,
  toTier: ProviderTier,
): TransitionKind {
  if (fromTier === toTier) return "unchanged";
  const improving = toTier < fromTier;
  const degrading = toTier > fromTier;

  if (improving && toTier <= 2) return "recovered-to-gnss";
  if (degrading && fromTier <= 2 && toTier === 3) return "gnss-to-network";
  if (degrading && fromTier === 3 && toTier === 4) return "network-to-dead-reckoning";
  if (degrading && fromTier === 4 && toTier === 5) return "dead-reckoning-to-cached";
  if (improving) return "improved";
  return "degraded";
}
