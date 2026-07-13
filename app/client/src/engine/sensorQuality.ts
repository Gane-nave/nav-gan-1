/**
 * G.A.N.E — Sensor Fusion Quality Monitor
 * ==========================================
 * Monitors quality and reliability of sensor inputs.
 *
 * SENSORS MONITORED:
 *   - GNSS (GPS/GLONASS/Galileo/BeiDou)
 *   - IMU (Accelerometer + Gyroscope)
 *   - Magnetometer (Compass)
 *   - Barometer (Altitude)
 *   - Camera (Visual odometry)
 *   - Network (Cell/WiFi positioning)
 *
 * QUALITY METRICS:
 *   - Availability (% time active)
 *   - Accuracy (measurement noise)
 *   - Freshness (time since last reading)
 *   - Consistency (cross-sensor agreement)
 *   - Drift (accumulated error)
 */

// ─── Types ───────────────────────────────────────────────

export type SensorType = 'gnss' | 'imu' | 'magnetometer' | 'barometer' | 'camera' | 'network';

export type SensorStatus = 'active' | 'degraded' | 'unavailable' | 'error';

export interface SensorReading {
  type: SensorType;
  timestamp: number;
  values: number[];
  accuracy: number;          // 0-1 (1 = perfect)
  status: SensorStatus;
}

export interface SensorQualityMetrics {
  type: SensorType;
  status: SensorStatus;
  availability: number;      // 0-1
  accuracy: number;          // 0-1
  freshness: number;         // 0-1 (1 = just received)
  consistency: number;       // 0-1
  drift: number;             // accumulated error in meters
  lastReadingAt: number;
  readingCount: number;
  errorCount: number;
}

export interface FusionQuality {
  overallScore: number;      // 0-1
  grade: 'A' | 'B' | 'C' | 'D' | 'F';
  activeSensors: number;
  totalSensors: number;
  weakestSensor: SensorType | null;
  recommendation: string;
  recommendationHe: string;
}

// ─── Constants ──────────────────────────────────────────

const SENSOR_WEIGHTS: Record<SensorType, number> = {
  gnss: 0.35,
  imu: 0.25,
  magnetometer: 0.10,
  barometer: 0.05,
  camera: 0.15,
  network: 0.10,
};

const MAX_STALE_MS: Record<SensorType, number> = {
  gnss: 5000,
  imu: 200,
  magnetometer: 1000,
  barometer: 5000,
  camera: 500,
  network: 10000,
};

// ─── Sensor Quality Monitor ────────────────────────────

export class SensorQualityMonitor {
  private metrics: Map<SensorType, SensorQualityMetrics> = new Map();
  private readingHistory: Map<SensorType, number[]> = new Map(); // timestamps
  private windowMs = 60000; // 1-minute window for availability

  constructor() {
    // Initialize all sensors
    const sensors: SensorType[] = ['gnss', 'imu', 'magnetometer', 'barometer', 'camera', 'network'];
    for (const type of sensors) {
      this.metrics.set(type, {
        type,
        status: 'unavailable',
        availability: 0,
        accuracy: 0,
        freshness: 0,
        consistency: 1,
        drift: 0,
        lastReadingAt: 0,
        readingCount: 0,
        errorCount: 0,
      });
      this.readingHistory.set(type, []);
    }
  }

  // ─── Reading Ingestion ────────────────────────────────

  /**
   * Record a sensor reading and update quality metrics.
   */
  recordReading(reading: SensorReading) {
    const m = this.metrics.get(reading.type);
    if (!m) return;

    m.lastReadingAt = reading.timestamp;
    m.readingCount++;
    m.status = reading.status;
    m.accuracy = this.updateEMA(m.accuracy, reading.accuracy, 0.1);

    if (reading.status === 'error') {
      m.errorCount++;
    }

    // Record timestamp for availability calculation
    const history = this.readingHistory.get(reading.type)!;
    history.push(reading.timestamp);

    // Trim old entries
    const cutoff = Date.now() - this.windowMs;
    while (history.length > 0 && history[0] < cutoff) {
      history.shift();
    }

    // Update availability
    const expectedReadings = this.getExpectedReadings(reading.type);
    m.availability = Math.min(1, history.length / expectedReadings);

    // Update freshness
    m.freshness = this.computeFreshness(reading.type, reading.timestamp);
  }

  /**
   * Record a sensor error.
   */
  recordError(type: SensorType, errorMessage: string) {
    const m = this.metrics.get(type);
    if (!m) return;

    m.status = 'error';
    m.errorCount++;
    m.accuracy = Math.max(0, m.accuracy - 0.1);
  }

  /**
   * Update cross-sensor consistency.
   */
  updateConsistency(type: SensorType, consistencyScore: number) {
    const m = this.metrics.get(type);
    if (!m) return;

    m.consistency = this.updateEMA(m.consistency, consistencyScore, 0.2);
  }

  /**
   * Update accumulated drift.
   */
  updateDrift(type: SensorType, driftM: number) {
    const m = this.metrics.get(type);
    if (!m) return;

    m.drift = driftM;
  }

  // ─── Quality Assessment ───────────────────────────────

  /**
   * Get quality metrics for a specific sensor.
   */
  getSensorQuality(type: SensorType): SensorQualityMetrics {
    const m = this.metrics.get(type)!;

    // Update freshness in real-time
    m.freshness = this.computeFreshness(type, m.lastReadingAt);

    // Update status based on freshness
    if (m.freshness < 0.1 && m.readingCount > 0) {
      m.status = 'unavailable';
    } else if (m.freshness < 0.5 || m.accuracy < 0.5) {
      m.status = 'degraded';
    }

    return { ...m };
  }

  /**
   * Get overall fusion quality.
   */
  getFusionQuality(): FusionQuality {
    let weightedScore = 0;
    let totalWeight = 0;
    let activeSensors = 0;
    let weakestScore = Infinity;
    let weakestSensor: SensorType | null = null;

    for (const [type, weight] of Object.entries(SENSOR_WEIGHTS)) {
      const m = this.getSensorQuality(type as SensorType);
      const sensorScore = (m.availability * 0.3 + m.accuracy * 0.3 + m.freshness * 0.2 + m.consistency * 0.2);

      if (m.status === 'active' || m.status === 'degraded') {
        activeSensors++;
      }

      weightedScore += sensorScore * weight;
      totalWeight += weight;

      if (sensorScore < weakestScore && m.readingCount > 0) {
        weakestScore = sensorScore;
        weakestSensor = type as SensorType;
      }
    }

    const overallScore = totalWeight > 0 ? Math.round(weightedScore / totalWeight * 1000) / 1000 : 0;

    let grade: FusionQuality['grade'];
    if (overallScore >= 0.9) grade = 'A';
    else if (overallScore >= 0.75) grade = 'B';
    else if (overallScore >= 0.6) grade = 'C';
    else if (overallScore >= 0.4) grade = 'D';
    else grade = 'F';

    const { recommendation, recommendationHe } = this.getRecommendation(grade, weakestSensor);

    return {
      overallScore,
      grade,
      activeSensors,
      totalSensors: 6,
      weakestSensor,
      recommendation,
      recommendationHe,
    };
  }

  /**
   * Get all sensor metrics.
   */
  getAllMetrics(): SensorQualityMetrics[] {
    return Array.from(this.metrics.keys()).map(type => this.getSensorQuality(type));
  }

  // ─── Helpers ──────────────────────────────────────────

  private computeFreshness(type: SensorType, lastReadingAt: number): number {
    if (lastReadingAt === 0) return 0;
    const age = Date.now() - lastReadingAt;
    const maxStale = MAX_STALE_MS[type];
    return Math.max(0, 1 - age / maxStale);
  }

  private getExpectedReadings(type: SensorType): number {
    // Expected readings per window based on sensor type
    const ratesHz: Record<SensorType, number> = {
      gnss: 1,
      imu: 100,
      magnetometer: 10,
      barometer: 1,
      camera: 30,
      network: 0.1,
    };
    return ratesHz[type] * (this.windowMs / 1000);
  }

  private updateEMA(current: number, newValue: number, alpha: number): number {
    return Math.round((alpha * newValue + (1 - alpha) * current) * 1000) / 1000;
  }

  private getRecommendation(
    grade: FusionQuality['grade'],
    weakest: SensorType | null
  ): { recommendation: string; recommendationHe: string } {
    if (grade === 'A') {
      return {
        recommendation: 'All sensors operating optimally. Full precision navigation active.',
        recommendationHe: 'כל החיישנים פועלים באופן מיטבי. ניווט מדויק מלא פעיל.',
      };
    }

    if (grade === 'F') {
      return {
        recommendation: 'Critical sensor failure. Navigation accuracy severely degraded.',
        recommendationHe: 'כשל חיישנים קריטי. דיוק הניווט ירד משמעותית.',
      };
    }

    const sensorLabels: Record<SensorType, { en: string; he: string }> = {
      gnss: { en: 'GNSS/GPS', he: 'GNSS/GPS' },
      imu: { en: 'IMU', he: 'IMU' },
      magnetometer: { en: 'Compass', he: 'מצפן' },
      barometer: { en: 'Barometer', he: 'ברומטר' },
      camera: { en: 'Camera', he: 'מצלמה' },
      network: { en: 'Network', he: 'רשת' },
    };

    if (weakest) {
      const label = sensorLabels[weakest];
      return {
        recommendation: `${label.en} sensor quality is low. Consider recalibration or moving to open area.`,
        recommendationHe: `איכות חיישן ${label.he} נמוכה. שקול כיול מחדש או מעבר לאזור פתוח.`,
      };
    }

    return {
      recommendation: 'Sensor quality is moderate. Navigation continues with reduced precision.',
      recommendationHe: 'איכות החיישנים בינונית. הניווט ממשיך בדיוק מופחת.',
    };
  }

  // ─── Lifecycle ────────────────────────────────────────

  reset() {
    for (const [type, m] of Array.from(this.metrics.entries())) {
      m.status = 'unavailable';
      m.availability = 0;
      m.accuracy = 0;
      m.freshness = 0;
      m.consistency = 1;
      m.drift = 0;
      m.lastReadingAt = 0;
      m.readingCount = 0;
      m.errorCount = 0;
    }
    for (const history of Array.from(this.readingHistory.values())) {
      history.length = 0;
    }
  }

  destroy() {
    this.metrics.clear();
    this.readingHistory.clear();
  }
}
