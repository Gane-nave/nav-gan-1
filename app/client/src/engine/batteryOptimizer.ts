/**
 * G.A.N.E — Battery Optimizer Engine
 * ====================================
 * Adaptive GPS sampling and power management.
 *
 * STRATEGY:
 *   - Dynamic GPS interval based on speed and context
 *   - Reduce sampling when stationary
 *   - Increase sampling in turns/intersections
 *   - Batch telemetry uploads
 *   - Dim non-essential UI during low battery
 *
 * MODES:
 *   - HIGH_ACCURACY: 1s GPS, full sensors (navigation active)
 *   - BALANCED: 3-5s GPS, reduced sensors (background tracking)
 *   - LOW_POWER: 10-30s GPS, minimal sensors (battery < 20%)
 *   - ULTRA_LOW: 60s GPS, no sensors (battery < 10%)
 */

// ─── Types ───────────────────────────────────────────────

export type PowerMode = 'high_accuracy' | 'balanced' | 'low_power' | 'ultra_low';

export interface BatteryConfig {
  highAccuracyIntervalMs: number;    // GPS interval in high accuracy (default: 1000)
  balancedIntervalMs: number;        // GPS interval in balanced (default: 3000)
  lowPowerIntervalMs: number;        // GPS interval in low power (default: 10000)
  ultraLowIntervalMs: number;        // GPS interval in ultra low (default: 60000)
  lowBatteryThreshold: number;       // switch to low_power below this (default: 0.2)
  criticalBatteryThreshold: number;  // switch to ultra_low below this (default: 0.1)
  telemetryBatchSize: number;        // batch N samples before upload (default: 10)
  telemetryBatchIntervalMs: number;  // max time before forced upload (default: 30000)
  stationaryThresholdKmh: number;    // below this speed = stationary (default: 2)
  stationaryMultiplier: number;      // multiply interval when stationary (default: 5)
  enabled: boolean;
}

export interface BatteryState {
  level: number;                     // 0-1
  isCharging: boolean;
  powerMode: PowerMode;
  currentIntervalMs: number;
  isStationary: boolean;
  lastSpeedKmh: number;
  samplesInBatch: number;
  totalSamples: number;
  totalBatches: number;
  estimatedHoursRemaining: number;
  powerSavingsPercent: number;       // estimated power saved vs always-on
}

export interface GPSSample {
  lat: number;
  lon: number;
  accuracy: number;
  speed: number;
  heading: number;
  timestamp: number;
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: BatteryConfig = {
  highAccuracyIntervalMs: 1000,
  balancedIntervalMs: 3000,
  lowPowerIntervalMs: 10000,
  ultraLowIntervalMs: 60000,
  lowBatteryThreshold: 0.2,
  criticalBatteryThreshold: 0.1,
  telemetryBatchSize: 10,
  telemetryBatchIntervalMs: 30000,
  stationaryThresholdKmh: 2,
  stationaryMultiplier: 5,
  enabled: true,
};

// ─── Battery Optimizer ──────────────────────────────────

export class BatteryOptimizer {
  private config: BatteryConfig;
  private state: BatteryState;
  private gpsWatchId: number | null = null;
  private batchTimer: ReturnType<typeof setInterval> | null = null;
  private sampleBatch: GPSSample[] = [];

  // Callbacks
  private onSample: ((sample: GPSSample) => void) | null = null;
  private onBatchReady: ((batch: GPSSample[]) => void) | null = null;
  private onModeChange: ((mode: PowerMode) => void) | null = null;

  // Battery API
  private batteryManager: any = null;
  private boundUpdateBattery = () => this.updateBatteryState();

  constructor(config: Partial<BatteryConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.state = {
      level: 1,
      isCharging: false,
      powerMode: 'balanced',
      currentIntervalMs: this.config.balancedIntervalMs,
      isStationary: false,
      lastSpeedKmh: 0,
      samplesInBatch: 0,
      totalSamples: 0,
      totalBatches: 0,
      estimatedHoursRemaining: 24,
      powerSavingsPercent: 0,
    };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  async start() {
    // Initialize Battery API
    await this.initBatteryAPI();

    // Start GPS sampling
    this.startGPS();

    // Start batch timer
    this.batchTimer = setInterval(() => {
      if (this.sampleBatch.length > 0) {
        this.flushBatch();
      }
    }, this.config.telemetryBatchIntervalMs);
  }

  stop() {
    this.stopGPS();
    if (this.batchTimer) {
      clearInterval(this.batchTimer);
      this.batchTimer = null;
    }
    // Flush remaining samples
    if (this.sampleBatch.length > 0) {
      this.flushBatch();
    }
  }

  // ─── Battery API ──────────────────────────────────────

  private async initBatteryAPI() {
    if (typeof navigator === 'undefined') return;

    try {
      if ('getBattery' in navigator) {
        this.batteryManager = await (navigator as any).getBattery();
        this.updateBatteryState();

        this.batteryManager.addEventListener('levelchange', this.boundUpdateBattery);
        this.batteryManager.addEventListener('chargingchange', this.boundUpdateBattery);
      }
    } catch (e) {
      console.warn('[BatteryOptimizer] Battery API not available');
    }
  }

  private updateBatteryState() {
    if (!this.batteryManager) return;

    this.state.level = this.batteryManager.level;
    this.state.isCharging = this.batteryManager.charging;

    // Auto-adjust power mode based on battery
    if (this.state.isCharging) {
      this.setPowerMode('high_accuracy');
    } else if (this.state.level < this.config.criticalBatteryThreshold) {
      this.setPowerMode('ultra_low');
    } else if (this.state.level < this.config.lowBatteryThreshold) {
      this.setPowerMode('low_power');
    }

    // Estimate hours remaining
    if (this.batteryManager.dischargingTime && this.batteryManager.dischargingTime !== Infinity) {
      this.state.estimatedHoursRemaining = Math.round(this.batteryManager.dischargingTime / 3600 * 10) / 10;
    }
  }

  // ─── GPS Management ───────────────────────────────────

  private startGPS() {
    if (typeof navigator === 'undefined' || !navigator.geolocation) return;

    const options = this.getGPSOptions();

    this.gpsWatchId = navigator.geolocation.watchPosition(
      (position) => this.handlePosition(position),
      (error) => console.warn('[BatteryOptimizer] GPS error:', error.message),
      options
    );
  }

  private stopGPS() {
    if (this.gpsWatchId !== null && typeof navigator !== 'undefined') {
      navigator.geolocation.clearWatch(this.gpsWatchId);
      this.gpsWatchId = null;
    }
  }

  private getGPSOptions(): PositionOptions {
    switch (this.state.powerMode) {
      case 'high_accuracy':
        return { enableHighAccuracy: true, maximumAge: 0, timeout: 5000 };
      case 'balanced':
        return { enableHighAccuracy: true, maximumAge: 2000, timeout: 10000 };
      case 'low_power':
        return { enableHighAccuracy: false, maximumAge: 5000, timeout: 15000 };
      case 'ultra_low':
        return { enableHighAccuracy: false, maximumAge: 30000, timeout: 30000 };
    }
  }

  private handlePosition(position: GeolocationPosition) {
    const speedKmh = (position.coords.speed || 0) * 3.6;

    // Detect stationary
    const wasStationary = this.state.isStationary;
    this.state.isStationary = speedKmh < this.config.stationaryThresholdKmh;
    this.state.lastSpeedKmh = speedKmh;

    // If stationary status changed, adjust interval
    if (wasStationary !== this.state.isStationary) {
      this.updateInterval();
    }

    const sample: GPSSample = {
      lat: position.coords.latitude,
      lon: position.coords.longitude,
      accuracy: position.coords.accuracy,
      speed: position.coords.speed || 0,
      heading: position.coords.heading || 0,
      timestamp: position.timestamp,
    };

    this.state.totalSamples++;

    // Notify per-sample callback
    if (this.onSample) {
      this.onSample(sample);
    }

    // Add to batch
    this.sampleBatch.push(sample);
    this.state.samplesInBatch = this.sampleBatch.length;

    // Flush batch if full
    if (this.sampleBatch.length >= this.config.telemetryBatchSize) {
      this.flushBatch();
    }
  }

  private flushBatch() {
    if (this.sampleBatch.length === 0) return;

    const batch = [...this.sampleBatch];
    this.sampleBatch = [];
    this.state.samplesInBatch = 0;
    this.state.totalBatches++;

    if (this.onBatchReady) {
      this.onBatchReady(batch);
    }
  }

  // ─── Power Mode Management ────────────────────────────

  setPowerMode(mode: PowerMode) {
    if (this.state.powerMode === mode) return;

    const oldMode = this.state.powerMode;
    this.state.powerMode = mode;
    this.updateInterval();

    // Restart GPS with new options
    this.stopGPS();
    this.startGPS();

    // Calculate power savings
    this.state.powerSavingsPercent = this.calculateSavings(mode);

    if (this.onModeChange) {
      this.onModeChange(mode);
    }
  }

  private updateInterval() {
    let baseInterval: number;

    switch (this.state.powerMode) {
      case 'high_accuracy':
        baseInterval = this.config.highAccuracyIntervalMs;
        break;
      case 'balanced':
        baseInterval = this.config.balancedIntervalMs;
        break;
      case 'low_power':
        baseInterval = this.config.lowPowerIntervalMs;
        break;
      case 'ultra_low':
        baseInterval = this.config.ultraLowIntervalMs;
        break;
    }

    // Apply stationary multiplier
    if (this.state.isStationary) {
      baseInterval *= this.config.stationaryMultiplier;
    }

    this.state.currentIntervalMs = baseInterval;
  }

  private calculateSavings(mode: PowerMode): number {
    // Rough estimates of power savings vs high_accuracy
    switch (mode) {
      case 'high_accuracy': return 0;
      case 'balanced': return 35;
      case 'low_power': return 65;
      case 'ultra_low': return 85;
    }
  }

  // ─── Callbacks ────────────────────────────────────────

  setOnSample(callback: (sample: GPSSample) => void) {
    this.onSample = callback;
  }

  setOnBatchReady(callback: (batch: GPSSample[]) => void) {
    this.onBatchReady = callback;
  }

  setOnModeChange(callback: (mode: PowerMode) => void) {
    this.onModeChange = callback;
  }

  // ─── Public API ───────────────────────────────────────

  getState(): BatteryState {
    return { ...this.state };
  }

  getConfig(): BatteryConfig {
    return { ...this.config };
  }

  updateConfig(partial: Partial<BatteryConfig>) {
    this.config = { ...this.config, ...partial };
    this.updateInterval();
  }

  /**
   * Get recommended UI adjustments for current power mode.
   */
  getUIRecommendations(): {
    reduceAnimations: boolean;
    dimBrightness: boolean;
    disableParticles: boolean;
    reduceTileQuality: boolean;
    disableAutoRefresh: boolean;
  } {
    return {
      reduceAnimations: this.state.powerMode === 'low_power' || this.state.powerMode === 'ultra_low',
      dimBrightness: this.state.powerMode === 'ultra_low',
      disableParticles: this.state.powerMode !== 'high_accuracy',
      reduceTileQuality: this.state.powerMode === 'low_power' || this.state.powerMode === 'ultra_low',
      disableAutoRefresh: this.state.powerMode === 'ultra_low',
    };
  }

  destroy() {
    this.stop();
    // Remove battery event listeners to prevent memory leaks
    if (this.batteryManager) {
      this.batteryManager.removeEventListener('levelchange', this.boundUpdateBattery);
      this.batteryManager.removeEventListener('chargingchange', this.boundUpdateBattery);
      this.batteryManager = null;
    }
    this.onSample = null;
    this.onBatchReady = null;
    this.onModeChange = null;
  }
}
