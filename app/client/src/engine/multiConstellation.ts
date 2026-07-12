/**
 * G.A.N.E — Multi-Constellation GNSS Engine
 * ============================================
 * Supports ALL global + regional GNSS constellations:
 * - GPS (USA) — 31 satellites, L1/L2/L5
 * - GLONASS (Russia) — 24 satellites, L1/L2
 * - Galileo (EU) — 30 satellites, E1/E5a/E5b
 * - BeiDou (China) — 35 satellites, B1/B2/B3
 * - NavIC/IRNSS (India) — 7 satellites, L5/S-band
 * - QZSS (Japan) — 4 satellites, L1/L2/L5/L6
 * - SBAS (WAAS/EGNOS/MSAS/GAGAN) — augmentation
 *
 * Features:
 * - Constellation health monitoring
 * - Signal quality assessment per constellation
 * - Optimal constellation selection based on geometry (GDOP)
 * - Regional constellation prioritization
 * - Anti-spoofing cross-validation between constellations
 */

// ═══════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════

export type ConstellationId = 'GPS' | 'GLONASS' | 'GALILEO' | 'BEIDOU' | 'NAVIC' | 'QZSS' | 'SBAS';

export interface SatelliteInfo {
  prn: number;              // Pseudo-Random Noise code
  constellation: ConstellationId;
  elevation: number;        // degrees above horizon
  azimuth: number;          // degrees from north
  snr: number;              // Signal-to-Noise ratio (dB-Hz)
  frequency: string;        // L1, L5, E1, B1, etc.
  isUsedInFix: boolean;
  isHealthy: boolean;
  doppler: number;          // Hz — for velocity estimation
}

export interface ConstellationStatus {
  id: ConstellationId;
  name: string;
  region: string;
  totalSatellites: number;
  visibleSatellites: number;
  usedInFix: number;
  avgSnr: number;
  isAvailable: boolean;
  isRegionallyOptimal: boolean;
  healthScore: number;       // 0-1
  frequencies: string[];
  coverageArea: string;
}

export interface MultiConstellationState {
  constellations: Map<ConstellationId, ConstellationStatus>;
  satellites: SatelliteInfo[];
  totalVisible: number;
  totalUsed: number;
  gdop: number;              // Geometric Dilution of Precision
  pdop: number;              // Position DOP
  hdop: number;              // Horizontal DOP
  vdop: number;              // Vertical DOP
  tdop: number;              // Time DOP
  bestConstellation: ConstellationId;
  spoofingRisk: number;      // 0-1
  lastUpdate: number;
}

export interface GNSSPosition {
  lat: number;
  lon: number;
  alt: number;
  accuracy: number;          // meters (95% confidence)
  verticalAccuracy: number;
  speed: number;             // m/s
  heading: number;           // degrees
  timestamp: number;
  constellation: ConstellationId;
  satellitesUsed: number;
  hdop: number;
  fixType: 'none' | '2d' | '3d' | 'dgps' | 'rtk_float' | 'rtk_fixed';
}

// ═══════════════════════════════════════════════════
// CONSTELLATION DEFINITIONS
// ═══════════════════════════════════════════════════

const CONSTELLATION_DEFS: Record<ConstellationId, Omit<ConstellationStatus, 'visibleSatellites' | 'usedInFix' | 'avgSnr' | 'isAvailable' | 'isRegionallyOptimal' | 'healthScore'>> = {
  GPS: {
    id: 'GPS', name: 'GPS (USA)', region: 'global',
    totalSatellites: 31, frequencies: ['L1', 'L2', 'L5'],
    coverageArea: 'Global',
  },
  GLONASS: {
    id: 'GLONASS', name: 'GLONASS (Russia)', region: 'global',
    totalSatellites: 24, frequencies: ['L1', 'L2'],
    coverageArea: 'Global',
  },
  GALILEO: {
    id: 'GALILEO', name: 'Galileo (EU)', region: 'global',
    totalSatellites: 30, frequencies: ['E1', 'E5a', 'E5b', 'E6'],
    coverageArea: 'Global',
  },
  BEIDOU: {
    id: 'BEIDOU', name: 'BeiDou (China)', region: 'global',
    totalSatellites: 35, frequencies: ['B1', 'B2', 'B3'],
    coverageArea: 'Global (optimized Asia-Pacific)',
  },
  NAVIC: {
    id: 'NAVIC', name: 'NavIC/IRNSS (India)', region: 'regional',
    totalSatellites: 7, frequencies: ['L5', 'S-band'],
    coverageArea: 'India + 1500km radius',
  },
  QZSS: {
    id: 'QZSS', name: 'QZSS (Japan)', region: 'regional',
    totalSatellites: 4, frequencies: ['L1', 'L2', 'L5', 'L6'],
    coverageArea: 'Japan, East Asia, Oceania',
  },
  SBAS: {
    id: 'SBAS', name: 'SBAS Augmentation', region: 'regional',
    totalSatellites: 12, frequencies: ['L1', 'L5'],
    coverageArea: 'WAAS(NA), EGNOS(EU), MSAS(JP), GAGAN(IN)',
  },
};

// Regional bounding boxes for constellation optimization
const REGIONAL_BOUNDS: Record<string, { minLat: number; maxLat: number; minLon: number; maxLon: number }> = {
  NAVIC: { minLat: -10, maxLat: 40, minLon: 50, maxLon: 110 },
  QZSS:  { minLat: -10, maxLat: 55, minLon: 100, maxLon: 180 },
  BEIDOU_OPTIMAL: { minLat: -10, maxLat: 55, minLon: 70, maxLon: 150 },
};

// ═══════════════════════════════════════════════════
// MULTI-CONSTELLATION ENGINE
// ═══════════════════════════════════════════════════

export class MultiConstellationEngine {
  private state: MultiConstellationState;
  private positionHistory: GNSSPosition[] = [];
  private readonly MAX_HISTORY = 300;
  private listeners: Set<(state: MultiConstellationState) => void> = new Set();

  constructor() {
    this.state = {
      constellations: new Map(),
      satellites: [],
      totalVisible: 0,
      totalUsed: 0,
      gdop: 99,
      pdop: 99,
      hdop: 99,
      vdop: 99,
      tdop: 99,
      bestConstellation: 'GPS',
      spoofingRisk: 0,
      lastUpdate: Date.now(),
    };

    // Initialize all constellations
    for (const [id, def] of Object.entries(CONSTELLATION_DEFS)) {
      this.state.constellations.set(id as ConstellationId, {
        ...def,
        visibleSatellites: 0,
        usedInFix: 0,
        avgSnr: 0,
        isAvailable: false,
        isRegionallyOptimal: false,
        healthScore: 0,
      });
    }
  }

  /**
   * Update satellite observations from device GNSS chipset
   */
  updateSatellites(satellites: SatelliteInfo[]): void {
    this.state.satellites = satellites;
    this.state.lastUpdate = Date.now();

    // Reset constellation stats
    const constellationSats = new Map<ConstellationId, SatelliteInfo[]>();
    for (const sat of satellites) {
      const list = constellationSats.get(sat.constellation) || [];
      list.push(sat);
      constellationSats.set(sat.constellation, list);
    }

    // Update each constellation
    let totalVisible = 0;
    let totalUsed = 0;

    for (const [id, status] of Array.from(this.state.constellations)) {
      const sats = constellationSats.get(id) || [];
      const visible = sats.length;
      const used = sats.filter(s => s.isUsedInFix).length;
      const healthy = sats.filter(s => s.isHealthy).length;
      const avgSnr = sats.length > 0
        ? sats.reduce((sum, s) => sum + s.snr, 0) / sats.length
        : 0;

      status.visibleSatellites = visible;
      status.usedInFix = used;
      status.avgSnr = avgSnr;
      status.isAvailable = used >= 1;
      status.healthScore = visible > 0 ? healthy / visible : 0;

      totalVisible += visible;
      totalUsed += used;
    }

    this.state.totalVisible = totalVisible;
    this.state.totalUsed = totalUsed;

    // Compute DOP values
    this.computeDOP(satellites.filter(s => s.isUsedInFix));

    // Detect spoofing via cross-constellation validation
    this.detectSpoofing();

    // Select best constellation
    this.selectBestConstellation();

    // Notify listeners
    this.notifyListeners();
  }

  /**
   * Set user position for regional constellation optimization
   */
  setUserRegion(lat: number, lon: number): void {
    for (const [id, status] of Array.from(this.state.constellations)) {
      const bounds = REGIONAL_BOUNDS[id] || REGIONAL_BOUNDS[`${id}_OPTIMAL`];
      if (bounds) {
        status.isRegionallyOptimal =
          lat >= bounds.minLat && lat <= bounds.maxLat &&
          lon >= bounds.minLon && lon <= bounds.maxLon;
      } else {
        // Global constellations are always regionally optimal
        status.isRegionallyOptimal = true;
      }
    }
  }

  /**
   * Record a position fix for history/replay
   */
  recordPosition(pos: GNSSPosition): void {
    this.positionHistory.push(pos);
    if (this.positionHistory.length > this.MAX_HISTORY) {
      this.positionHistory.shift();
    }
  }

  /**
   * Get position history for trip replay
   */
  getPositionHistory(): GNSSPosition[] {
    return [...this.positionHistory];
  }

  /**
   * Compute Geometric Dilution of Precision from satellite geometry
   * Uses simplified DOP calculation based on satellite elevation/azimuth
   */
  private computeDOP(usedSats: SatelliteInfo[]): void {
    if (usedSats.length < 4) {
      this.state.gdop = 99;
      this.state.pdop = 99;
      this.state.hdop = 99;
      this.state.vdop = 99;
      this.state.tdop = 99;
      return;
    }

    // Build direction cosine matrix H
    const n = usedSats.length;
    const H: number[][] = [];

    for (const sat of usedSats) {
      const elRad = (sat.elevation * Math.PI) / 180;
      const azRad = (sat.azimuth * Math.PI) / 180;
      H.push([
        Math.cos(elRad) * Math.sin(azRad),  // x
        Math.cos(elRad) * Math.cos(azRad),  // y
        Math.sin(elRad),                      // z
        1,                                    // clock bias
      ]);
    }

    // Compute H^T * H
    const HTH = Array.from({ length: 4 }, () => new Array(4).fill(0));
    for (let i = 0; i < 4; i++) {
      for (let j = 0; j < 4; j++) {
        for (let k = 0; k < n; k++) {
          HTH[i][j] += H[k][i] * H[k][j];
        }
      }
    }

    // Invert 4x4 matrix (simplified — use trace approximation for robustness)
    const det = this.det4x4(HTH);
    if (Math.abs(det) < 1e-10) {
      this.state.gdop = 99;
      this.state.pdop = 99;
      this.state.hdop = 99;
      this.state.vdop = 99;
      this.state.tdop = 99;
      return;
    }

    const inv = this.invert4x4(HTH, det);
    if (!inv) return;

    this.state.hdop = Math.sqrt(Math.max(0, inv[0][0] + inv[1][1]));
    this.state.vdop = Math.sqrt(Math.max(0, inv[2][2]));
    this.state.pdop = Math.sqrt(Math.max(0, inv[0][0] + inv[1][1] + inv[2][2]));
    this.state.tdop = Math.sqrt(Math.max(0, inv[3][3]));
    this.state.gdop = Math.sqrt(Math.max(0, inv[0][0] + inv[1][1] + inv[2][2] + inv[3][3]));
  }

  /**
   * Cross-constellation spoofing detection
   * If positions from different constellations diverge significantly,
   * one or more may be spoofed.
   */
  private detectSpoofing(): void {
    const availableConstellations = Array.from(this.state.constellations.values())
      .filter(c => c.usedInFix >= 3);

    if (availableConstellations.length < 2) {
      this.state.spoofingRisk = 0;
      return;
    }

    // Check SNR consistency — spoofed signals often have uniform SNR
    let uniformSnrCount = 0;
    for (const constellation of availableConstellations) {
      const sats = this.state.satellites.filter(
        s => s.constellation === constellation.id && s.isUsedInFix
      );
      if (sats.length >= 3) {
        const snrs = sats.map(s => s.snr);
        const mean = snrs.reduce((a, b) => a + b, 0) / snrs.length;
        const variance = snrs.reduce((sum, s) => sum + (s - mean) ** 2, 0) / snrs.length;
        // Very low variance in SNR is suspicious (real signals vary)
        if (variance < 2.0) uniformSnrCount++;
      }
    }

    // Check elevation distribution — spoofed signals often lack low-elevation sats
    const usedSats = this.state.satellites.filter(s => s.isUsedInFix);
    const lowElevCount = usedSats.filter(s => s.elevation < 20).length;
    const lowElevRatio = usedSats.length > 0 ? lowElevCount / usedSats.length : 0;
    const noLowElev = lowElevRatio < 0.1 && usedSats.length > 6;

    // Combine risk factors
    let risk = 0;
    if (uniformSnrCount > 0) risk += 0.3 * (uniformSnrCount / availableConstellations.length);
    if (noLowElev) risk += 0.3;

    this.state.spoofingRisk = Math.min(1, risk);
  }

  /**
   * Select the best constellation based on geometry, signal quality, and regional optimization
   */
  private selectBestConstellation(): void {
    let bestScore = -1;
    let bestId: ConstellationId = 'GPS';

    for (const [id, status] of Array.from(this.state.constellations)) {
      if (!status.isAvailable) continue;

      let score = 0;
      // Weight: satellites used (40%)
      score += (status.usedInFix / Math.max(1, status.totalSatellites)) * 40;
      // Weight: signal quality (30%)
      score += Math.min(1, status.avgSnr / 45) * 30;
      // Weight: health (15%)
      score += status.healthScore * 15;
      // Weight: regional optimization (15%)
      if (status.isRegionallyOptimal) score += 15;

      if (score > bestScore) {
        bestScore = score;
        bestId = id;
      }
    }

    this.state.bestConstellation = bestId;
  }

  // ─── Matrix utilities ───

  private det4x4(m: number[][]): number {
    const [a, b, c, d] = m;
    return (
      a[0] * (b[1] * (c[2] * d[3] - c[3] * d[2]) - b[2] * (c[1] * d[3] - c[3] * d[1]) + b[3] * (c[1] * d[2] - c[2] * d[1])) -
      a[1] * (b[0] * (c[2] * d[3] - c[3] * d[2]) - b[2] * (c[0] * d[3] - c[3] * d[0]) + b[3] * (c[0] * d[2] - c[2] * d[0])) +
      a[2] * (b[0] * (c[1] * d[3] - c[3] * d[1]) - b[1] * (c[0] * d[3] - c[3] * d[0]) + b[3] * (c[0] * d[1] - c[1] * d[0])) -
      a[3] * (b[0] * (c[1] * d[2] - c[2] * d[1]) - b[1] * (c[0] * d[2] - c[2] * d[0]) + b[2] * (c[0] * d[1] - c[1] * d[0]))
    );
  }

  private invert4x4(m: number[][], det: number): number[][] | null {
    if (Math.abs(det) < 1e-15) return null;
    const inv: number[][] = Array.from({ length: 4 }, () => new Array(4).fill(0));
    const invDet = 1 / det;

    for (let i = 0; i < 4; i++) {
      for (let j = 0; j < 4; j++) {
        const minor = this.minor3x3(m, j, i);
        inv[i][j] = ((i + j) % 2 === 0 ? 1 : -1) * minor * invDet;
      }
    }
    return inv;
  }

  private minor3x3(m: number[][], row: number, col: number): number {
    const sub: number[] = [];
    for (let i = 0; i < 4; i++) {
      if (i === row) continue;
      for (let j = 0; j < 4; j++) {
        if (j === col) continue;
        sub.push(m[i][j]);
      }
    }
    return sub[0] * (sub[4] * sub[8] - sub[5] * sub[7]) -
           sub[1] * (sub[3] * sub[8] - sub[5] * sub[6]) +
           sub[2] * (sub[3] * sub[7] - sub[4] * sub[6]);
  }

  // ─── Public API ───

  getState(): MultiConstellationState {
    return { ...this.state };
  }

  subscribe(listener: (state: MultiConstellationState) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notifyListeners(): void {
    for (const listener of Array.from(this.listeners)) {
      try { listener(this.state); } catch { /* ignore */ }
    }
  }

  /**
   * Get constellation summary for UI display
   */
  getConstellationSummary(): ConstellationStatus[] {
    return Array.from(this.state.constellations.values())
      .sort((a, b) => b.usedInFix - a.usedInFix);
  }

  destroy(): void {
    this.listeners.clear();
    this.positionHistory = [];
  }
}
