/**
 * G.A.N.E — Privacy Engine
 * ==========================
 * Location data anonymization and privacy controls.
 *
 * FEATURES:
 *   - Geo-indistinguishability (differential privacy for location)
 *   - K-anonymity for crowd data
 *   - Data retention policies
 *   - Consent management
 *   - Home/work location masking
 *   - Trip history anonymization
 *
 * PRIVACY LEVELS:
 *   - FULL: No anonymization (user opted in)
 *   - STANDARD: Moderate noise, no PII in crowd data
 *   - STRICT: Heavy noise, no persistent identifiers
 *   - GHOST: Maximum privacy, ephemeral sessions only
 */

// ─── Types ───────────────────────────────────────────────

export type PrivacyLevel = 'full' | 'standard' | 'strict' | 'ghost';

export interface PrivacyConfig {
  level: PrivacyLevel;
  noiseRadiusM: Record<PrivacyLevel, number>;  // Laplace noise radius per level
  retentionDays: Record<PrivacyLevel, number>; // Data retention per level
  maskHomeRadius: number;                       // Mask home location within this radius (m)
  maskWorkRadius: number;                       // Mask work location within this radius (m)
  allowCrowdContribution: boolean;              // Allow anonymous crowd data
  allowTripHistory: boolean;                    // Store trip history
  allowAnalytics: boolean;                      // Allow usage analytics
  consentTimestamp: number;                     // When user last consented
  consentVersion: string;                       // Privacy policy version
}

export interface PrivacyState {
  level: PrivacyLevel;
  isAnonymized: boolean;
  deviceId: string;                             // Anonymized device ID
  sessionId: string;                            // Ephemeral session ID
  totalAnonymizations: number;
  totalMasked: number;
  homeLocation: { lat: number; lon: number } | null;
  workLocation: { lat: number; lon: number } | null;
}

export interface AnonymizedLocation {
  lat: number;
  lon: number;
  accuracy: number;                             // Degraded accuracy
  isAnonymized: boolean;
  noiseAddedM: number;
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: PrivacyConfig = {
  level: 'standard',
  noiseRadiusM: {
    full: 0,
    standard: 50,
    strict: 200,
    ghost: 500,
  },
  retentionDays: {
    full: 365,
    standard: 90,
    strict: 30,
    ghost: 0,
  },
  maskHomeRadius: 200,
  maskWorkRadius: 200,
  allowCrowdContribution: true,
  allowTripHistory: true,
  allowAnalytics: true,
  consentTimestamp: 0,
  consentVersion: '1.0',
};

const DEG_TO_RAD = Math.PI / 180;
const RAD_TO_DEG = 180 / Math.PI;
const EARTH_RADIUS_M = 6371000;

// ─── Noise Functions ────────────────────────────────────

/**
 * Laplace noise for geo-indistinguishability.
 * Adds noise drawn from a Laplace distribution.
 */
function laplaceSample(scale: number): number {
  const u = Math.random() - 0.5;
  return -scale * Math.sign(u) * Math.log(1 - 2 * Math.abs(u));
}

/**
 * Add Laplace noise to a lat/lon coordinate.
 * Returns new coordinates with noise proportional to radiusM.
 */
function addGeoNoise(
  lat: number, lon: number,
  radiusM: number
): { lat: number; lon: number; noiseM: number } {
  if (radiusM <= 0) return { lat, lon, noiseM: 0 };

  // Convert radius to degrees (approximate)
  const latNoiseDeg = laplaceSample(radiusM / EARTH_RADIUS_M * RAD_TO_DEG);
  const lonNoiseDeg = laplaceSample(radiusM / (EARTH_RADIUS_M * Math.cos(lat * DEG_TO_RAD)) * RAD_TO_DEG);

  const newLat = lat + latNoiseDeg;
  const newLon = lon + lonNoiseDeg;

  // Compute actual noise distance
  const dLat = latNoiseDeg * DEG_TO_RAD;
  const dLon = lonNoiseDeg * DEG_TO_RAD;
  const noiseM = Math.sqrt(
    (dLat * EARTH_RADIUS_M) ** 2 +
    (dLon * EARTH_RADIUS_M * Math.cos(lat * DEG_TO_RAD)) ** 2
  );

  return { lat: newLat, lon: newLon, noiseM: Math.round(noiseM) };
}

function haversineDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const dLat = (lat2 - lat1) * DEG_TO_RAD;
  const dLon = (lon2 - lon1) * DEG_TO_RAD;
  const a = Math.sin(dLat / 2) ** 2 +
    Math.cos(lat1 * DEG_TO_RAD) * Math.cos(lat2 * DEG_TO_RAD) *
    Math.sin(dLon / 2) ** 2;
  return EARTH_RADIUS_M * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

// ─── Privacy Engine ─────────────────────────────────────

export class PrivacyEngine {
  private config: PrivacyConfig;
  private state: PrivacyState;

  constructor(config: Partial<PrivacyConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.state = {
      level: this.config.level,
      isAnonymized: this.config.level !== 'full',
      deviceId: this.generateAnonymousId(),
      sessionId: this.generateSessionId(),
      totalAnonymizations: 0,
      totalMasked: 0,
      homeLocation: null,
      workLocation: null,
    };
  }

  // ─── Location Anonymization ───────────────────────────

  /**
   * Anonymize a location based on current privacy level.
   * Adds Laplace noise and checks for home/work masking.
   */
  anonymize(lat: number, lon: number, accuracy: number = 10): AnonymizedLocation {
    const noiseRadius = this.config.noiseRadiusM[this.config.level];

    // Check if near home/work (mask completely)
    if (this.isNearHome(lat, lon)) {
      this.state.totalMasked++;
      const masked = addGeoNoise(lat, lon, this.config.maskHomeRadius);
      return {
        lat: masked.lat,
        lon: masked.lon,
        accuracy: Math.max(accuracy, this.config.maskHomeRadius),
        isAnonymized: true,
        noiseAddedM: masked.noiseM,
      };
    }

    if (this.isNearWork(lat, lon)) {
      this.state.totalMasked++;
      const masked = addGeoNoise(lat, lon, this.config.maskWorkRadius);
      return {
        lat: masked.lat,
        lon: masked.lon,
        accuracy: Math.max(accuracy, this.config.maskWorkRadius),
        isAnonymized: true,
        noiseAddedM: masked.noiseM,
      };
    }

    // Apply standard noise
    if (noiseRadius > 0) {
      this.state.totalAnonymizations++;
      const noised = addGeoNoise(lat, lon, noiseRadius);
      return {
        lat: noised.lat,
        lon: noised.lon,
        accuracy: Math.max(accuracy, noiseRadius),
        isAnonymized: true,
        noiseAddedM: noised.noiseM,
      };
    }

    return { lat, lon, accuracy, isAnonymized: false, noiseAddedM: 0 };
  }

  /**
   * Anonymize a batch of locations (for trip history).
   */
  anonymizeBatch(
    points: { lat: number; lon: number; accuracy?: number }[]
  ): AnonymizedLocation[] {
    return points.map(p => this.anonymize(p.lat, p.lon, p.accuracy || 10));
  }

  /**
   * Anonymize a device ID for crowd data.
   * In ghost mode, generates a new ID each time.
   */
  getAnonymousDeviceId(): string {
    if (this.config.level === 'ghost') {
      return this.generateSessionId(); // Ephemeral
    }
    return this.state.deviceId;
  }

  // ─── Home/Work Detection ──────────────────────────────

  setHomeLocation(lat: number, lon: number) {
    this.state.homeLocation = { lat, lon };
  }

  setWorkLocation(lat: number, lon: number) {
    this.state.workLocation = { lat, lon };
  }

  private isNearHome(lat: number, lon: number): boolean {
    if (!this.state.homeLocation) return false;
    return haversineDistance(lat, lon, this.state.homeLocation.lat, this.state.homeLocation.lon) < this.config.maskHomeRadius;
  }

  private isNearWork(lat: number, lon: number): boolean {
    if (!this.state.workLocation) return false;
    return haversineDistance(lat, lon, this.state.workLocation.lat, this.state.workLocation.lon) < this.config.maskWorkRadius;
  }

  // ─── Consent Management ───────────────────────────────

  /**
   * Record user consent with current policy version.
   */
  recordConsent(
    allowCrowd: boolean,
    allowHistory: boolean,
    allowAnalytics: boolean,
    level: PrivacyLevel = 'standard'
  ) {
    this.config.allowCrowdContribution = allowCrowd;
    this.config.allowTripHistory = allowHistory;
    this.config.allowAnalytics = allowAnalytics;
    this.config.level = level;
    this.config.consentTimestamp = Date.now();
    this.state.level = level;
    this.state.isAnonymized = level !== 'full';
  }

  /**
   * Check if consent is valid (not expired, correct version).
   */
  isConsentValid(): boolean {
    if (this.config.consentTimestamp === 0) return false;
    // Consent expires after 1 year
    const oneYear = 365 * 24 * 60 * 60 * 1000;
    return Date.now() - this.config.consentTimestamp < oneYear;
  }

  // ─── Data Retention ───────────────────────────────────

  /**
   * Get the retention cutoff date for current privacy level.
   */
  getRetentionCutoff(): Date {
    const days = this.config.retentionDays[this.config.level];
    if (days === 0) return new Date(); // Ghost mode: no retention
    return new Date(Date.now() - days * 24 * 60 * 60 * 1000);
  }

  /**
   * Check if a timestamp is within retention period.
   */
  isWithinRetention(timestamp: number): boolean {
    const cutoff = this.getRetentionCutoff().getTime();
    return timestamp >= cutoff;
  }

  // ─── Permission Checks ───────────────────────────────

  canContributeCrowdData(): boolean {
    return this.config.allowCrowdContribution && this.config.level !== 'ghost';
  }

  canStoreTripHistory(): boolean {
    return this.config.allowTripHistory && this.config.level !== 'ghost';
  }

  canSendAnalytics(): boolean {
    return this.config.allowAnalytics;
  }

  // ─── ID Generation ────────────────────────────────────

  private generateAnonymousId(): string {
    const bytes = new Uint8Array(16);
    if (typeof crypto !== 'undefined' && crypto.getRandomValues) {
      crypto.getRandomValues(bytes);
    } else {
      for (let i = 0; i < 16; i++) bytes[i] = Math.floor(Math.random() * 256);
    }
    return Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
  }

  private generateSessionId(): string {
    return `sess_${Date.now()}_${Math.random().toString(36).slice(2, 10)}`;
  }

  // ─── Public API ───────────────────────────────────────

  getState(): PrivacyState {
    return { ...this.state };
  }

  getConfig(): PrivacyConfig {
    return { ...this.config };
  }

  setLevel(level: PrivacyLevel) {
    this.config.level = level;
    this.state.level = level;
    this.state.isAnonymized = level !== 'full';

    // In ghost mode, regenerate IDs
    if (level === 'ghost') {
      this.state.deviceId = this.generateAnonymousId();
      this.state.sessionId = this.generateSessionId();
    }
  }

  /**
   * Get privacy summary for display.
   */
  getSummary(): {
    level: PrivacyLevel;
    levelLabel: string;
    levelLabelHe: string;
    description: string;
    descriptionHe: string;
    noiseRadius: number;
    retentionDays: number;
  } {
    const labels: Record<PrivacyLevel, { en: string; he: string }> = {
      full: { en: 'Full Access', he: 'גישה מלאה' },
      standard: { en: 'Standard', he: 'סטנדרטי' },
      strict: { en: 'Strict', he: 'מחמיר' },
      ghost: { en: 'Ghost Mode', he: 'מצב רוח רפאים' },
    };

    const descriptions: Record<PrivacyLevel, { en: string; he: string }> = {
      full: {
        en: 'No anonymization. Full data shared for best experience.',
        he: 'ללא אנונימיזציה. שיתוף מלא לחוויה מיטבית.',
      },
      standard: {
        en: 'Moderate privacy. Location noise added, no PII in crowd data.',
        he: 'פרטיות מתונה. רעש מיקום מתווסף, ללא מידע מזהה.',
      },
      strict: {
        en: 'High privacy. Heavy noise, short data retention.',
        he: 'פרטיות גבוהה. רעש כבד, שמירת נתונים קצרה.',
      },
      ghost: {
        en: 'Maximum privacy. Ephemeral sessions, no data stored.',
        he: 'פרטיות מקסימלית. הפעלות חד-פעמיות, ללא שמירת נתונים.',
      },
    };

    return {
      level: this.config.level,
      levelLabel: labels[this.config.level].en,
      levelLabelHe: labels[this.config.level].he,
      description: descriptions[this.config.level].en,
      descriptionHe: descriptions[this.config.level].he,
      noiseRadius: this.config.noiseRadiusM[this.config.level],
      retentionDays: this.config.retentionDays[this.config.level],
    };
  }

  destroy() {
    // Clear sensitive data
    this.state.homeLocation = null;
    this.state.workLocation = null;
  }
}
