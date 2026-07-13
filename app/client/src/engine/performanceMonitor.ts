/**
 * G.A.N.E — Performance Monitor Engine
 * ========================================
 * Real-time performance monitoring and profiling.
 *
 * METRICS:
 *   - Frame rate (FPS)
 *   - Memory usage
 *   - Engine cycle times
 *   - Network latency
 *   - Battery drain rate
 *   - CPU load estimation
 *   - Render pipeline timing
 *
 * FEATURES:
 *   - Automatic performance budgets
 *   - Degradation detection
 *   - Adaptive quality scaling
 *   - Performance snapshots
 */

// ─── Types ───────────────────────────────────────────────

export interface PerfMetrics {
  fps: number;
  avgFrameTimeMs: number;
  memoryUsedMB: number;
  memoryLimitMB: number;
  engineCycleMs: number;
  networkLatencyMs: number;
  batteryDrainPerHour: number;
  cpuLoadEstimate: number;          // 0-1
  renderTimeMs: number;
  jankFrames: number;               // frames > 16.67ms
  timestamp: number;
}

export interface PerfBudget {
  maxFrameTimeMs: number;           // Target: 16.67ms (60fps)
  maxEngineCycleMs: number;         // Target: 10ms
  maxMemoryMB: number;              // Target: 200MB
  maxNetworkLatencyMs: number;      // Target: 100ms
  maxRenderTimeMs: number;          // Target: 8ms
}

export interface PerfSnapshot {
  id: string;
  timestamp: number;
  metrics: PerfMetrics;
  violations: string[];
  qualityLevel: 'ultra' | 'high' | 'medium' | 'low' | 'minimal';
}

export type QualityLevel = 'ultra' | 'high' | 'medium' | 'low' | 'minimal';

export interface QualitySettings {
  level: QualityLevel;
  particleCount: number;
  animationComplexity: number;      // 0-1
  mapDetailLevel: number;           // 0-1
  shadowQuality: number;            // 0-1
  updateFrequencyHz: number;
  enableBlur: boolean;
  enableGlow: boolean;
  enableParallax: boolean;
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_BUDGET: PerfBudget = {
  maxFrameTimeMs: 16.67,
  maxEngineCycleMs: 10,
  maxMemoryMB: 200,
  maxNetworkLatencyMs: 100,
  maxRenderTimeMs: 8,
};

const QUALITY_PRESETS: Record<QualityLevel, QualitySettings> = {
  ultra: {
    level: 'ultra',
    particleCount: 200,
    animationComplexity: 1.0,
    mapDetailLevel: 1.0,
    shadowQuality: 1.0,
    updateFrequencyHz: 60,
    enableBlur: true,
    enableGlow: true,
    enableParallax: true,
  },
  high: {
    level: 'high',
    particleCount: 100,
    animationComplexity: 0.8,
    mapDetailLevel: 0.9,
    shadowQuality: 0.7,
    updateFrequencyHz: 60,
    enableBlur: true,
    enableGlow: true,
    enableParallax: true,
  },
  medium: {
    level: 'medium',
    particleCount: 50,
    animationComplexity: 0.5,
    mapDetailLevel: 0.7,
    shadowQuality: 0.3,
    updateFrequencyHz: 30,
    enableBlur: false,
    enableGlow: true,
    enableParallax: false,
  },
  low: {
    level: 'low',
    particleCount: 20,
    animationComplexity: 0.2,
    mapDetailLevel: 0.5,
    shadowQuality: 0,
    updateFrequencyHz: 20,
    enableBlur: false,
    enableGlow: false,
    enableParallax: false,
  },
  minimal: {
    level: 'minimal',
    particleCount: 0,
    animationComplexity: 0,
    mapDetailLevel: 0.3,
    shadowQuality: 0,
    updateFrequencyHz: 10,
    enableBlur: false,
    enableGlow: false,
    enableParallax: false,
  },
};

// ─── Performance Monitor ────────────────────────────────

export class PerformanceMonitor {
  private budget: PerfBudget;
  private currentMetrics: PerfMetrics;
  private snapshots: PerfSnapshot[] = [];
  private maxSnapshots = 100;
  private currentQuality: QualityLevel = 'high';
  private autoScale = true;

  // FPS tracking
  private frameTimestamps: number[] = [];
  private frameTimes: number[] = [];
  private lastFrameTime = 0;
  private rafId: number | null = null;

  // Callbacks
  private onQualityChange: ((quality: QualitySettings) => void) | null = null;
  private onBudgetViolation: ((violations: string[]) => void) | null = null;

  constructor(budget: Partial<PerfBudget> = {}) {
    this.budget = { ...DEFAULT_BUDGET, ...budget };
    this.currentMetrics = {
      fps: 60,
      avgFrameTimeMs: 16.67,
      memoryUsedMB: 0,
      memoryLimitMB: 0,
      engineCycleMs: 0,
      networkLatencyMs: 0,
      batteryDrainPerHour: 0,
      cpuLoadEstimate: 0,
      renderTimeMs: 0,
      jankFrames: 0,
      timestamp: Date.now(),
    };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  start() {
    this.measureFrame();
  }

  stop() {
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }
  }

  // ─── Frame Measurement ────────────────────────────────

  private measureFrame() {
    if (typeof requestAnimationFrame === 'undefined') return;

    const tick = (timestamp: number) => {
      if (this.lastFrameTime > 0) {
        const frameTime = timestamp - this.lastFrameTime;
        this.frameTimes.push(frameTime);

        // Keep last 120 frames
        if (this.frameTimes.length > 120) {
          this.frameTimes.shift();
        }

        // Track jank
        if (frameTime > 16.67) {
          this.currentMetrics.jankFrames++;
        }
      }

      this.lastFrameTime = timestamp;
      this.frameTimestamps.push(timestamp);

      // Keep last 120 timestamps
      if (this.frameTimestamps.length > 120) {
        this.frameTimestamps.shift();
      }

      // Calculate FPS every 30 frames
      if (this.frameTimestamps.length >= 30) {
        const elapsed = this.frameTimestamps[this.frameTimestamps.length - 1] - this.frameTimestamps[0];
        this.currentMetrics.fps = Math.round((this.frameTimestamps.length - 1) / (elapsed / 1000));
        this.currentMetrics.avgFrameTimeMs = Math.round(
          this.frameTimes.reduce((s, t) => s + t, 0) / this.frameTimes.length * 100
        ) / 100;
      }

      // Update memory
      this.updateMemory();

      // Check budget
      this.checkBudget();

      // Auto-scale quality
      if (this.autoScale) {
        this.autoScaleQuality();
      }

      this.currentMetrics.timestamp = Date.now();
      this.rafId = requestAnimationFrame(tick);
    };

    this.rafId = requestAnimationFrame(tick);
  }

  // ─── Memory ───────────────────────────────────────────

  private updateMemory() {
    if (typeof performance !== 'undefined' && (performance as any).memory) {
      const mem = (performance as any).memory;
      this.currentMetrics.memoryUsedMB = Math.round(mem.usedJSHeapSize / 1048576);
      this.currentMetrics.memoryLimitMB = Math.round(mem.jsHeapSizeLimit / 1048576);
    }
  }

  // ─── Timing ───────────────────────────────────────────

  /**
   * Record an engine cycle time.
   */
  recordEngineCycle(durationMs: number) {
    this.currentMetrics.engineCycleMs = Math.round(
      (this.currentMetrics.engineCycleMs * 0.9 + durationMs * 0.1) * 100
    ) / 100;
  }

  /**
   * Record a render time.
   */
  recordRenderTime(durationMs: number) {
    this.currentMetrics.renderTimeMs = Math.round(
      (this.currentMetrics.renderTimeMs * 0.9 + durationMs * 0.1) * 100
    ) / 100;
  }

  /**
   * Record network latency.
   */
  recordNetworkLatency(latencyMs: number) {
    this.currentMetrics.networkLatencyMs = Math.round(
      (this.currentMetrics.networkLatencyMs * 0.8 + latencyMs * 0.2) * 100
    ) / 100;
  }

  /**
   * Record CPU load estimate.
   */
  recordCPULoad(load: number) {
    this.currentMetrics.cpuLoadEstimate = Math.round(
      (this.currentMetrics.cpuLoadEstimate * 0.9 + load * 0.1) * 1000
    ) / 1000;
  }

  // ─── Budget Checking ──────────────────────────────────

  private checkBudget() {
    const violations: string[] = [];

    if (this.currentMetrics.avgFrameTimeMs > this.budget.maxFrameTimeMs) {
      violations.push(`Frame time ${this.currentMetrics.avgFrameTimeMs}ms > ${this.budget.maxFrameTimeMs}ms`);
    }
    if (this.currentMetrics.engineCycleMs > this.budget.maxEngineCycleMs) {
      violations.push(`Engine cycle ${this.currentMetrics.engineCycleMs}ms > ${this.budget.maxEngineCycleMs}ms`);
    }
    if (this.currentMetrics.memoryUsedMB > this.budget.maxMemoryMB) {
      violations.push(`Memory ${this.currentMetrics.memoryUsedMB}MB > ${this.budget.maxMemoryMB}MB`);
    }
    if (this.currentMetrics.networkLatencyMs > this.budget.maxNetworkLatencyMs) {
      violations.push(`Latency ${this.currentMetrics.networkLatencyMs}ms > ${this.budget.maxNetworkLatencyMs}ms`);
    }
    if (this.currentMetrics.renderTimeMs > this.budget.maxRenderTimeMs) {
      violations.push(`Render ${this.currentMetrics.renderTimeMs}ms > ${this.budget.maxRenderTimeMs}ms`);
    }

    if (violations.length > 0 && this.onBudgetViolation) {
      this.onBudgetViolation(violations);
    }
  }

  // ─── Auto Quality Scaling ─────────────────────────────

  private autoScaleQuality() {
    const fps = this.currentMetrics.fps;
    const mem = this.currentMetrics.memoryUsedMB;
    const memLimit = this.currentMetrics.memoryLimitMB || 2048;

    let targetQuality: QualityLevel = this.currentQuality;

    // Downgrade if struggling
    if (fps < 20 || mem > memLimit * 0.9) {
      targetQuality = 'minimal';
    } else if (fps < 30 || mem > memLimit * 0.8) {
      targetQuality = 'low';
    } else if (fps < 45 || mem > memLimit * 0.7) {
      targetQuality = 'medium';
    } else if (fps >= 55 && mem < memLimit * 0.5) {
      // Upgrade if headroom
      const levels: QualityLevel[] = ['minimal', 'low', 'medium', 'high', 'ultra'];
      const currentIdx = levels.indexOf(this.currentQuality);
      if (currentIdx < levels.length - 1) {
        targetQuality = levels[currentIdx + 1];
      }
    }

    if (targetQuality !== this.currentQuality) {
      this.setQuality(targetQuality);
    }
  }

  // ─── Quality Control ──────────────────────────────────

  setQuality(level: QualityLevel) {
    this.currentQuality = level;
    if (this.onQualityChange) {
      this.onQualityChange(QUALITY_PRESETS[level]);
    }
  }

  getQualitySettings(): QualitySettings {
    return { ...QUALITY_PRESETS[this.currentQuality] };
  }

  // ─── Snapshots ────────────────────────────────────────

  takeSnapshot(): PerfSnapshot {
    const violations: string[] = [];
    if (this.currentMetrics.avgFrameTimeMs > this.budget.maxFrameTimeMs) violations.push('frame_time');
    if (this.currentMetrics.engineCycleMs > this.budget.maxEngineCycleMs) violations.push('engine_cycle');
    if (this.currentMetrics.memoryUsedMB > this.budget.maxMemoryMB) violations.push('memory');

    const snapshot: PerfSnapshot = {
      id: `snap_${Date.now()}`,
      timestamp: Date.now(),
      metrics: { ...this.currentMetrics },
      violations,
      qualityLevel: this.currentQuality,
    };

    this.snapshots.push(snapshot);
    if (this.snapshots.length > this.maxSnapshots) {
      this.snapshots.shift();
    }

    return snapshot;
  }

  getSnapshots(): PerfSnapshot[] {
    return [...this.snapshots];
  }

  // ─── Callbacks ────────────────────────────────────────

  setOnQualityChange(callback: (quality: QualitySettings) => void) {
    this.onQualityChange = callback;
  }

  setOnBudgetViolation(callback: (violations: string[]) => void) {
    this.onBudgetViolation = callback;
  }

  // ─── Public API ───────────────────────────────────────

  getMetrics(): PerfMetrics {
    return { ...this.currentMetrics };
  }

  getBudget(): PerfBudget {
    return { ...this.budget };
  }

  setAutoScale(enabled: boolean) {
    this.autoScale = enabled;
  }

  destroy() {
    this.stop();
    this.snapshots = [];
    this.frameTimestamps = [];
    this.frameTimes = [];
    this.onQualityChange = null;
    this.onBudgetViolation = null;
  }
}
