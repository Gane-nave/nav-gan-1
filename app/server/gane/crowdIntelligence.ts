/**
 * G.A.N.E — Crowd Intelligence Engine
 * =====================================
 * Aggregates crowd-sourced speed data from multiple devices
 * to build real-time traffic picture.
 *
 * INPUT:
 *   - Speed samples from connected devices
 *   - GPS traces with timestamps
 *
 * PROCESSING:
 *   - Spatial bucketing (H3-like grid cells)
 *   - Temporal windowing (5-min rolling average)
 *   - Outlier rejection (Median Absolute Deviation)
 *   - Speed percentile computation (P50, P85)
 *
 * OUTPUT:
 *   - Per-cell average speed
 *   - Congestion classification
 *   - Confidence based on sample count
 */

import { z } from 'zod';
import { publicProcedure, router } from '../_core/trpc';

// ─── Types ───────────────────────────────────────────────

export interface SpeedSample {
  lat: number;
  lon: number;
  speedKmh: number;
  heading: number;
  deviceId: string;
  timestamp: number;
}

export interface GridCell {
  cellId: string;
  centerLat: number;
  centerLon: number;
  samples: SpeedSample[];
  avgSpeedKmh: number;
  medianSpeedKmh: number;
  p85SpeedKmh: number;
  sampleCount: number;
  uniqueDevices: number;
  congestionLevel: 'free_flow' | 'light' | 'moderate' | 'heavy' | 'standstill';
  confidence: number;          // 0-1
  lastUpdated: number;
}

// ─── Constants ──────────────────────────────────────────

const CELL_SIZE_DEG = 0.001;    // ~111m at equator
const WINDOW_MS = 5 * 60 * 1000; // 5-minute rolling window
const MAX_SAMPLES_PER_CELL = 200;
const MIN_SAMPLES_FOR_CONFIDENCE = 3;
const OUTLIER_MAD_THRESHOLD = 3.0;

// Free-flow speed thresholds for congestion classification
const CONGESTION_THRESHOLDS = {
  free_flow: 0.8,    // > 80% of free-flow
  light: 0.6,        // 60-80%
  moderate: 0.4,     // 40-60%
  heavy: 0.2,        // 20-40%
  standstill: 0,     // < 20%
};

// ─── Grid Helpers ───────────────────────────────────────

function latLonToCellId(lat: number, lon: number): string {
  const cellLat = Math.floor(lat / CELL_SIZE_DEG);
  const cellLon = Math.floor(lon / CELL_SIZE_DEG);
  return `${cellLat}_${cellLon}`;
}

function cellIdToCenter(cellId: string): { lat: number; lon: number } {
  const [latStr, lonStr] = cellId.split('_');
  return {
    lat: parseInt(latStr) * CELL_SIZE_DEG + CELL_SIZE_DEG / 2,
    lon: parseInt(lonStr) * CELL_SIZE_DEG + CELL_SIZE_DEG / 2,
  };
}

function median(arr: number[]): number {
  if (arr.length === 0) return 0;
  const sorted = [...arr].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 !== 0 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

function percentile(arr: number[], p: number): number {
  if (arr.length === 0) return 0;
  const sorted = [...arr].sort((a, b) => a - b);
  const index = (p / 100) * (sorted.length - 1);
  const lower = Math.floor(index);
  const upper = Math.ceil(index);
  if (lower === upper) return sorted[lower];
  return sorted[lower] + (sorted[upper] - sorted[lower]) * (index - lower);
}

/**
 * Median Absolute Deviation outlier rejection.
 * Returns samples within MAD_THRESHOLD * MAD of the median.
 */
function rejectOutliers(speeds: number[]): number[] {
  if (speeds.length < 4) return speeds;
  const med = median(speeds);
  const deviations = speeds.map(s => Math.abs(s - med));
  const mad = median(deviations);
  if (mad === 0) return speeds;
  return speeds.filter(s => Math.abs(s - med) / mad <= OUTLIER_MAD_THRESHOLD);
}

// ─── Crowd Intelligence Store ───────────────────────────

class CrowdIntelligenceStore {
  private cells: Map<string, GridCell> = new Map();
  private cleanupTimer: ReturnType<typeof setInterval> | null = null;
  private freeFlowSpeeds: Map<string, number> = new Map(); // learned free-flow per cell

  // Metrics
  private metrics = {
    totalSamples: 0,
    totalCells: 0,
    activeCells: 0,
    avgSamplesPerCell: 0,
    outlierRate: 0,
  };

  start() {
    this.cleanupTimer = setInterval(() => this.cleanup(), 30000);
  }

  stop() {
    if (this.cleanupTimer) {
      clearInterval(this.cleanupTimer);
      this.cleanupTimer = null;
    }
  }

  /**
   * Ingest a speed sample from a device.
   */
  ingest(sample: SpeedSample) {
    const cellId = latLonToCellId(sample.lat, sample.lon);
    this.metrics.totalSamples++;

    let cell = this.cells.get(cellId);
    if (!cell) {
      const center = cellIdToCenter(cellId);
      cell = {
        cellId,
        centerLat: center.lat,
        centerLon: center.lon,
        samples: [],
        avgSpeedKmh: 0,
        medianSpeedKmh: 0,
        p85SpeedKmh: 0,
        sampleCount: 0,
        uniqueDevices: 0,
        congestionLevel: 'free_flow',
        confidence: 0,
        lastUpdated: Date.now(),
      };
      this.cells.set(cellId, cell);
      this.metrics.totalCells++;
    }

    // Add sample
    cell.samples.push(sample);

    // Cap samples
    if (cell.samples.length > MAX_SAMPLES_PER_CELL) {
      cell.samples = cell.samples.slice(-MAX_SAMPLES_PER_CELL);
    }

    // Recompute cell statistics
    this.recomputeCell(cell);
  }

  /**
   * Batch ingest multiple samples.
   */
  ingestBatch(samples: SpeedSample[]) {
    for (const sample of samples) {
      this.ingest(sample);
    }
  }

  /**
   * Get cell data for a specific location.
   */
  getCell(lat: number, lon: number): GridCell | null {
    const cellId = latLonToCellId(lat, lon);
    return this.cells.get(cellId) || null;
  }

  /**
   * Get all active cells within a bounding box.
   */
  getCellsInBounds(
    minLat: number, maxLat: number,
    minLon: number, maxLon: number
  ): GridCell[] {
    const result: GridCell[] = [];
    const allCells = Array.from(this.cells.values());
    for (const cell of allCells) {
      if (cell.centerLat >= minLat && cell.centerLat <= maxLat &&
          cell.centerLon >= minLon && cell.centerLon <= maxLon &&
          cell.sampleCount > 0) {
        result.push(cell);
      }
    }
    return result;
  }

  /**
   * Get congestion heatmap data for visualization.
   */
  getHeatmap(
    minLat: number, maxLat: number,
    minLon: number, maxLon: number
  ): { lat: number; lon: number; intensity: number; speed: number }[] {
    const cells = this.getCellsInBounds(minLat, maxLat, minLon, maxLon);
    return cells.map(c => ({
      lat: c.centerLat,
      lon: c.centerLon,
      intensity: this.congestionToIntensity(c.congestionLevel),
      speed: c.avgSpeedKmh,
    }));
  }

  getMetrics() {
    const allCells = Array.from(this.cells.values());
    this.metrics.activeCells = allCells.filter(c => c.sampleCount > 0).length;
    this.metrics.avgSamplesPerCell = this.metrics.activeCells > 0
      ? Math.round(allCells.reduce((s, c) => s + c.sampleCount, 0) / this.metrics.activeCells)
      : 0;
    return { ...this.metrics };
  }

  // ─── Internal ─────────────────────────────────────────

  private recomputeCell(cell: GridCell) {
    const now = Date.now();

    // Filter to window
    cell.samples = cell.samples.filter(s => now - s.timestamp < WINDOW_MS);

    if (cell.samples.length === 0) {
      cell.sampleCount = 0;
      cell.confidence = 0;
      return;
    }

    // Extract speeds and reject outliers
    const rawSpeeds = cell.samples.map(s => s.speedKmh);
    const cleanSpeeds = rejectOutliers(rawSpeeds);

    const outlierCount = rawSpeeds.length - cleanSpeeds.length;
    if (rawSpeeds.length > 0) {
      this.metrics.outlierRate = outlierCount / rawSpeeds.length;
    }

    // Compute statistics
    cell.avgSpeedKmh = Math.round(
      (cleanSpeeds.reduce((s, v) => s + v, 0) / cleanSpeeds.length) * 10
    ) / 10;
    cell.medianSpeedKmh = Math.round(median(cleanSpeeds) * 10) / 10;
    cell.p85SpeedKmh = Math.round(percentile(cleanSpeeds, 85) * 10) / 10;
    cell.sampleCount = cleanSpeeds.length;

    // Unique devices
    const devices = new Set(cell.samples.map(s => s.deviceId));
    cell.uniqueDevices = devices.size;

    // Learn free-flow speed (use P85 as proxy)
    const existingFreeFlow = this.freeFlowSpeeds.get(cell.cellId);
    if (!existingFreeFlow || cell.p85SpeedKmh > existingFreeFlow) {
      this.freeFlowSpeeds.set(cell.cellId, cell.p85SpeedKmh);
    }

    // Classify congestion
    const freeFlow = this.freeFlowSpeeds.get(cell.cellId) || 60;
    const ratio = cell.avgSpeedKmh / freeFlow;

    if (ratio > CONGESTION_THRESHOLDS.free_flow) {
      cell.congestionLevel = 'free_flow';
    } else if (ratio > CONGESTION_THRESHOLDS.light) {
      cell.congestionLevel = 'light';
    } else if (ratio > CONGESTION_THRESHOLDS.moderate) {
      cell.congestionLevel = 'moderate';
    } else if (ratio > CONGESTION_THRESHOLDS.heavy) {
      cell.congestionLevel = 'heavy';
    } else {
      cell.congestionLevel = 'standstill';
    }

    // Confidence
    cell.confidence = Math.min(1, Math.round(
      (Math.min(cell.uniqueDevices, 10) / 10 * 0.5 +
       Math.min(cell.sampleCount, 20) / 20 * 0.3 +
       (now - cell.samples[cell.samples.length - 1].timestamp < 60000 ? 0.2 : 0.05))
      * 100
    ) / 100);

    cell.lastUpdated = now;
  }

  private congestionToIntensity(level: string): number {
    switch (level) {
      case 'standstill': return 1.0;
      case 'heavy': return 0.8;
      case 'moderate': return 0.5;
      case 'light': return 0.3;
      default: return 0.1;
    }
  }

  private cleanup() {
    const now = Date.now();
    const entries = Array.from(this.cells.entries());
    for (const [id, cell] of entries) {
      // Remove cells with no recent data (>15 min)
      if (now - cell.lastUpdated > 15 * 60 * 1000 && cell.sampleCount === 0) {
        this.cells.delete(id);
      }
    }
  }
}

export const crowdIntelligenceStore = new CrowdIntelligenceStore();

// ─── tRPC Router ────────────────────────────────────────

export const crowdRouter = router({
  /**
   * Ingest speed samples from a device.
   */
  ingest: publicProcedure
    .input(z.object({
      samples: z.array(z.object({
        lat: z.number().min(-90).max(90),
        lon: z.number().min(-180).max(180),
        speedKmh: z.number().min(0).max(300),
        heading: z.number().min(0).max(360),
        deviceId: z.string().min(1).max(64),
        timestamp: z.number(),
      })).min(1).max(100),
    }))
    .mutation(({ input }) => {
      crowdIntelligenceStore.ingestBatch(input.samples);
      return { accepted: input.samples.length };
    }),

  /**
   * Get congestion heatmap for a bounding box.
   */
  heatmap: publicProcedure
    .input(z.object({
      minLat: z.number().min(-90).max(90),
      maxLat: z.number().min(-90).max(90),
      minLon: z.number().min(-180).max(180),
      maxLon: z.number().min(-180).max(180),
    }))
    .query(({ input }) => {
      return crowdIntelligenceStore.getHeatmap(
        input.minLat, input.maxLat,
        input.minLon, input.maxLon
      );
    }),

  /**
   * Get detailed cell data for a location.
   */
  cell: publicProcedure
    .input(z.object({
      lat: z.number().min(-90).max(90),
      lon: z.number().min(-180).max(180),
    }))
    .query(({ input }) => {
      return crowdIntelligenceStore.getCell(input.lat, input.lon);
    }),

  /**
   * Get crowd intelligence metrics.
   */
  metrics: publicProcedure
    .query(() => {
      return crowdIntelligenceStore.getMetrics();
    }),
});
