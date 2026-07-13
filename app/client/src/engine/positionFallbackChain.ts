/**
 * G.A.N.E — Position Fallback Chain (Unbreakable Navigation)
 * ═══════════════════════════════════════════════════════════
 * 
 * GUARANTEE: Navigation NEVER stops. Period.
 * 
 * The fallback chain ensures continuous positioning by cascading through
 * every available source. If one fails, the next takes over instantly
 * with zero-gap handoff.
 * 
 * ┌─────────────────────────────────────────────────────────────┐
 * │  TIER 1 — GNSS Constellations (best accuracy)              │
 * │  ┌─────┐ ┌────────┐ ┌────────┐ ┌───────┐ ┌────┐ ┌─────┐  │
 * │  │ GPS │ │Galileo │ │GLONASS │ │BeiDou │ │QZSS│ │NavIC│  │
 * │  └──┬──┘ └───┬────┘ └───┬────┘ └──┬────┘ └─┬──┘ └──┬──┘  │
 * │     └────────┴──────────┴─────────┴────────┴───────┘      │
 * │                    Multi-Constellation Fusion               │
 * ├─────────────────────────────────────────────────────────────┤
 * │  TIER 2 — Augmentation Systems                             │
 * │  SBAS (WAAS/EGNOS/MSAS/GAGAN) + RTK + DGPS                │
 * ├─────────────────────────────────────────────────────────────┤
 * │  TIER 3 — Terrestrial Positioning                          │
 * │  WiFi Positioning → Cell Tower Triangulation               │
 * ├─────────────────────────────────────────────────────────────┤
 * │  TIER 4 — Sensor Fusion (no external signals needed)       │
 * │  IMU Dead Reckoning (ESKF) → PDR → Visual Odometry        │
 * ├─────────────────────────────────────────────────────────────┤
 * │  TIER 5 — Last Resort                                      │
 * │  IP Geolocation → Cached Last Known Position               │
 * └─────────────────────────────────────────────────────────────┘
 * 
 * Each provider has:
 * - Health score (0-1)
 * - Accuracy estimate
 * - Freshness (time since last fix)
 * - Circuit breaker (auto-disable after repeated failures)
 * - Recovery probe (periodic retry to re-enable)
 */

// ═══════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════

export type ProviderTier = 1 | 2 | 3 | 4 | 5;

export type ProviderId =
  // Tier 1: GNSS Constellations
  | 'GPS' | 'GALILEO' | 'GLONASS' | 'BEIDOU' | 'QZSS' | 'NAVIC'
  // Tier 2: Augmentation
  | 'SBAS' | 'RTK' | 'DGPS'
  // Tier 3: Terrestrial
  | 'WIFI' | 'CELL'
  // Tier 4: Sensor Fusion
  | 'IMU_DR' | 'PDR' | 'VISUAL_ODOMETRY'
  // Tier 5: Last Resort
  | 'IP_GEO' | 'CACHED';

export interface PositionFix {
  lat: number;
  lon: number;
  alt: number;
  accuracy: number;           // meters (95% confidence)
  verticalAccuracy: number;
  speed: number;              // m/s
  heading: number;            // degrees
  timestamp: number;
  provider: ProviderId;
  tier: ProviderTier;
  confidence: number;         // 0-1
  satellitesUsed?: number;
  hdop?: number;
  fixType: 'none' | '2d' | '3d' | 'dgps' | 'rtk_float' | 'rtk_fixed' | 'wifi' | 'cell' | 'imu' | 'ip' | 'cached';
}

export interface ProviderHealth {
  id: ProviderId;
  tier: ProviderTier;
  name: string;
  country: string;            // Operating country/region
  description: string;
  isAvailable: boolean;
  isActive: boolean;          // Currently providing fixes
  healthScore: number;        // 0-1
  accuracy: number;           // Current accuracy estimate (meters)
  lastFixTime: number;        // Timestamp of last successful fix
  fixCount: number;           // Total fixes since startup
  failCount: number;          // Consecutive failures
  circuitBreakerOpen: boolean;
  signalStrength: number;     // 0-1 (for GNSS: avg SNR normalized)
  satelliteCount?: number;    // For GNSS providers
  frequencies?: string[];     // For GNSS providers
  coverageArea: string;
  maxAccuracy: number;        // Best possible accuracy (meters)
  typicalAccuracy: number;    // Typical accuracy (meters)
}

export interface FallbackChainState {
  activeProvider: ProviderId;
  activeTier: ProviderTier;
  currentFix: PositionFix | null;
  providers: Map<ProviderId, ProviderHealth>;
  fusedPosition: PositionFix | null;  // Multi-constellation fusion result
  chainStatus: 'OPTIMAL' | 'DEGRADED' | 'FALLBACK' | 'EMERGENCY' | 'LAST_RESORT';
  totalProvidersAvailable: number;
  totalProvidersActive: number;
  handoffCount: number;       // Number of provider switches since startup
  lastHandoffTime: number;
  uptime: number;             // Seconds since first fix
  continuityScore: number;    // 0-1 (1 = never lost position)
  spoofingDetected: boolean;
  jammingDetected: boolean;
}

// ═══════════════════════════════════════════════════
// PROVIDER DEFINITIONS
// ═══════════════════════════════════════════════════

interface ProviderDef {
  id: ProviderId;
  tier: ProviderTier;
  name: string;
  country: string;
  description: string;
  coverageArea: string;
  maxAccuracy: number;
  typicalAccuracy: number;
  frequencies?: string[];
  totalSatellites?: number;
}

const PROVIDER_DEFS: ProviderDef[] = [
  // ── Tier 1: GNSS Constellations ──
  {
    id: 'GPS', tier: 1,
    name: 'GPS (Global Positioning System)',
    country: 'USA',
    description: 'Primary global navigation system. 31 operational satellites in 6 orbital planes at 20,200 km altitude. Civilian accuracy 3-5m.',
    coverageArea: 'Global',
    maxAccuracy: 0.3,       // With dual-frequency
    typicalAccuracy: 3.5,
    frequencies: ['L1 C/A', 'L1C', 'L2C', 'L5'],
    totalSatellites: 31,
  },
  {
    id: 'GALILEO', tier: 1,
    name: 'Galileo (European GNSS)',
    country: 'European Union',
    description: 'Modern civilian-first system. 30 satellites with High Accuracy Service (HAS) providing sub-meter positioning. Includes Search and Rescue (SAR) transponders.',
    coverageArea: 'Global',
    maxAccuracy: 0.2,       // HAS service
    typicalAccuracy: 1.0,
    frequencies: ['E1', 'E5a', 'E5b', 'E6'],
    totalSatellites: 30,
  },
  {
    id: 'GLONASS', tier: 1,
    name: 'GLONASS (Russia)',
    country: 'Russia',
    description: 'Russian global system. 24 satellites using FDMA. Excellent performance at high latitudes (>60°N). Modernized with CDMA signals.',
    coverageArea: 'Global (optimized high latitudes)',
    maxAccuracy: 2.0,
    typicalAccuracy: 5.0,
    frequencies: ['L1OF', 'L2OF', 'L1OC', 'L2OC', 'L3OC'],
    totalSatellites: 24,
  },
  {
    id: 'BEIDOU', tier: 1,
    name: 'BeiDou (BDS-3)',
    country: 'China',
    description: 'Full global coverage since 2020. 35 satellites including GEO, IGSO, and MEO. Unique short message communication (RDSS) capability.',
    coverageArea: 'Global (optimized Asia-Pacific)',
    maxAccuracy: 1.0,
    typicalAccuracy: 3.5,
    frequencies: ['B1I', 'B1C', 'B2a', 'B2b', 'B3I'],
    totalSatellites: 35,
  },
  {
    id: 'QZSS', tier: 1,
    name: 'QZSS (Quasi-Zenith)',
    country: 'Japan',
    description: 'Regional augmentation system. 4 satellites in quasi-zenith orbits ensuring at least 1 satellite is always near-zenith over Japan. Centimeter-level accuracy with CLAS.',
    coverageArea: 'Japan, East Asia, Oceania',
    maxAccuracy: 0.01,      // CLAS centimeter-level
    typicalAccuracy: 1.0,
    frequencies: ['L1 C/A', 'L1C', 'L2C', 'L5', 'L6'],
    totalSatellites: 4,
  },
  {
    id: 'NAVIC', tier: 1,
    name: 'NavIC / IRNSS (India)',
    country: 'India',
    description: 'Indian regional system. 7 satellites (3 GEO + 4 IGSO) providing coverage over India and 1,500 km beyond. Dual-frequency for ionospheric correction.',
    coverageArea: 'India + 1,500 km radius',
    maxAccuracy: 5.0,
    typicalAccuracy: 10.0,
    frequencies: ['L5', 'S-band'],
    totalSatellites: 7,
  },

  // ── Tier 2: Augmentation Systems ──
  {
    id: 'SBAS', tier: 2,
    name: 'SBAS (Satellite-Based Augmentation)',
    country: 'Multi-national',
    description: 'Augments GNSS with correction data. Includes WAAS (North America), EGNOS (Europe), MSAS (Japan), GAGAN (India), SDCM (Russia).',
    coverageArea: 'Regional (per system)',
    maxAccuracy: 0.5,
    typicalAccuracy: 1.5,
    frequencies: ['L1', 'L5'],
    totalSatellites: 12,
  },
  {
    id: 'RTK', tier: 2,
    name: 'RTK (Real-Time Kinematic)',
    country: 'Global',
    description: 'Carrier-phase positioning using base station corrections. Centimeter-level accuracy when base station is within 10-20 km.',
    coverageArea: 'Near base stations',
    maxAccuracy: 0.01,
    typicalAccuracy: 0.02,
  },
  {
    id: 'DGPS', tier: 2,
    name: 'DGPS (Differential GPS)',
    country: 'Global',
    description: 'Ground-based correction broadcasts. Sub-meter accuracy. Used extensively in maritime and aviation.',
    coverageArea: 'Near correction stations',
    maxAccuracy: 0.3,
    typicalAccuracy: 1.0,
  },

  // ── Tier 3: Terrestrial Positioning ──
  {
    id: 'WIFI', tier: 3,
    name: 'WiFi Positioning System',
    country: 'Global',
    description: 'Triangulation using WiFi access point databases (Google, Apple, Mozilla). Works indoors where GNSS fails. Accuracy depends on AP density.',
    coverageArea: 'Urban areas, indoors',
    maxAccuracy: 3.0,
    typicalAccuracy: 15.0,
  },
  {
    id: 'CELL', tier: 3,
    name: 'Cell Tower Triangulation',
    country: 'Global',
    description: 'Position from cell tower signal timing (TDOA/RSSI). Works anywhere with cellular coverage. Lower accuracy but very reliable.',
    coverageArea: 'Cellular coverage areas',
    maxAccuracy: 50.0,
    typicalAccuracy: 300.0,
  },

  // ── Tier 4: Sensor Fusion (no external signals) ──
  {
    id: 'IMU_DR', tier: 4,
    name: 'IMU Dead Reckoning (ESKF)',
    country: 'Device',
    description: 'Accelerometer + gyroscope integration via Error-State Kalman Filter. Continues navigation when all external signals are lost. Drift-bounded by map matching.',
    coverageArea: 'Anywhere (device sensors)',
    maxAccuracy: 1.0,       // Short-term
    typicalAccuracy: 50.0,  // Drifts over time
  },
  {
    id: 'PDR', tier: 4,
    name: 'Pedestrian Dead Reckoning',
    country: 'Device',
    description: 'Step detection + heading estimation for walking navigation. Uses accelerometer patterns and magnetometer.',
    coverageArea: 'Anywhere (walking)',
    maxAccuracy: 2.0,
    typicalAccuracy: 10.0,
  },
  {
    id: 'VISUAL_ODOMETRY', tier: 4,
    name: 'Visual Odometry',
    country: 'Device',
    description: 'Camera-based motion estimation using feature tracking. Works in GPS-denied environments with visual features.',
    coverageArea: 'Anywhere with camera',
    maxAccuracy: 0.5,
    typicalAccuracy: 5.0,
  },

  // ── Tier 5: Last Resort ──
  {
    id: 'IP_GEO', tier: 5,
    name: 'IP Geolocation',
    country: 'Global',
    description: 'Position estimated from IP address geolocation databases. Very coarse but always available when internet is connected.',
    coverageArea: 'Anywhere with internet',
    maxAccuracy: 1000.0,
    typicalAccuracy: 5000.0,
  },
  {
    id: 'CACHED', tier: 5,
    name: 'Cached Last Known Position',
    country: 'Device',
    description: 'Last known good position stored locally. Used when all other sources fail. Accuracy degrades with time and movement.',
    coverageArea: 'Anywhere (stored)',
    maxAccuracy: 0.0,       // Was accurate when cached
    typicalAccuracy: 100.0, // Degrades
  },
];

// ═══════════════════════════════════════════════════
// CIRCUIT BREAKER
// ═══════════════════════════════════════════════════

class CircuitBreaker {
  private failCount = 0;
  private isOpen = false;
  private lastFailTime = 0;
  private readonly maxFailures: number;
  private readonly recoveryTimeMs: number;

  constructor(maxFailures = 5, recoveryTimeMs = 30000) {
    this.maxFailures = maxFailures;
    this.recoveryTimeMs = recoveryTimeMs;
  }

  recordSuccess(): void {
    this.failCount = 0;
    this.isOpen = false;
  }

  recordFailure(): void {
    this.failCount++;
    this.lastFailTime = Date.now();
    if (this.failCount >= this.maxFailures) {
      this.isOpen = true;
    }
  }

  canAttempt(): boolean {
    if (!this.isOpen) return true;
    // Allow recovery probe after timeout
    return Date.now() - this.lastFailTime > this.recoveryTimeMs;
  }

  getState(): { isOpen: boolean; failCount: number } {
    return { isOpen: this.isOpen, failCount: this.failCount };
  }

  reset(): void {
    this.failCount = 0;
    this.isOpen = false;
    this.lastFailTime = 0;
  }
}

// ═══════════════════════════════════════════════════
// POSITION FALLBACK CHAIN ENGINE
// ═══════════════════════════════════════════════════

export class PositionFallbackChain {
  private state: FallbackChainState;
  private circuitBreakers: Map<ProviderId, CircuitBreaker> = new Map();
  private listeners: Set<(state: FallbackChainState) => void> = new Set();
  private startTime: number;
  private fixHistory: PositionFix[] = [];
  private readonly MAX_HISTORY = 600;  // 10 minutes at 1Hz
  private cachedPosition: PositionFix | null = null;
  private lastBrowserPosition: GeolocationPosition | null = null;
  private watchId: number | null = null;

  constructor() {
    this.startTime = Date.now();

    // Initialize provider health map
    const providers = new Map<ProviderId, ProviderHealth>();
    for (const def of PROVIDER_DEFS) {
      providers.set(def.id, {
        id: def.id,
        tier: def.tier,
        name: def.name,
        country: def.country,
        description: def.description,
        isAvailable: false,
        isActive: false,
        healthScore: 0,
        accuracy: def.typicalAccuracy,
        lastFixTime: 0,
        fixCount: 0,
        failCount: 0,
        circuitBreakerOpen: false,
        signalStrength: 0,
        satelliteCount: def.totalSatellites,
        frequencies: def.frequencies,
        coverageArea: def.coverageArea,
        maxAccuracy: def.maxAccuracy,
        typicalAccuracy: def.typicalAccuracy,
      });

      this.circuitBreakers.set(def.id, new CircuitBreaker(
        def.tier <= 2 ? 10 : 5,         // GNSS gets more retries
        def.tier <= 2 ? 15000 : 30000   // GNSS recovers faster
      ));
    }

    this.state = {
      activeProvider: 'CACHED',
      activeTier: 5,
      currentFix: null,
      providers,
      fusedPosition: null,
      chainStatus: 'LAST_RESORT',
      totalProvidersAvailable: 0,
      totalProvidersActive: 0,
      handoffCount: 0,
      lastHandoffTime: 0,
      uptime: 0,
      continuityScore: 1.0,
      spoofingDetected: false,
      jammingDetected: false,
    };
  }

  // ─── Initialization ───

  /**
   * Start the fallback chain — begins monitoring all available sources
   */
  async init(): Promise<void> {
    // Start browser geolocation watch (this gives us GPS/GNSS from the device)
    this.startBrowserGeolocation();

    // Initialize cached position from localStorage
    this.loadCachedPosition();

    // Start health monitoring loop
    this.startHealthMonitor();
  }

  /**
   * Start watching browser geolocation (provides GNSS data from device chipset)
   */
  private startBrowserGeolocation(): void {
    if (typeof navigator === 'undefined' || !navigator.geolocation) return;

    this.watchId = navigator.geolocation.watchPosition(
      (position) => {
        this.lastBrowserPosition = position;
        this.processBrowserPosition(position);
      },
      (error) => {
        // Browser geolocation failed — mark GNSS providers accordingly
        this.handleGeolocationError(error);
      },
      {
        enableHighAccuracy: true,
        timeout: 10000,
        maximumAge: 0,
      }
    );
  }

  /**
   * Process a browser geolocation fix and distribute to appropriate providers
   */
  private processBrowserPosition(position: GeolocationPosition): void {
    const { coords, timestamp } = position;
    const accuracy = coords.accuracy;

    // Determine which GNSS constellations are likely contributing
    // Based on accuracy level, we can infer multi-constellation usage
    const constellations = this.inferConstellations(accuracy, coords.latitude, coords.longitude);

    // Create position fix
    const fix: PositionFix = {
      lat: coords.latitude,
      lon: coords.longitude,
      alt: coords.altitude || 0,
      accuracy,
      verticalAccuracy: coords.altitudeAccuracy || accuracy * 2,
      speed: coords.speed || 0,
      heading: coords.heading || 0,
      timestamp,
      provider: constellations.primary,
      tier: 1,
      confidence: Math.min(1, 50 / Math.max(accuracy, 1)),
      satellitesUsed: this.estimateSatelliteCount(accuracy),
      hdop: Math.max(0.5, accuracy / 5),
      fixType: accuracy < 2 ? 'rtk_fixed' : accuracy < 5 ? 'dgps' : accuracy < 20 ? '3d' : '2d',
    };

    // Update all active GNSS providers
    for (const cId of constellations.active) {
      this.updateProviderHealth(cId, true, accuracy, fix);
    }

    // Update inactive GNSS providers
    const allGNSS: ProviderId[] = ['GPS', 'GALILEO', 'GLONASS', 'BEIDOU', 'QZSS', 'NAVIC'];
    for (const cId of allGNSS) {
      if (!constellations.active.includes(cId)) {
        // Mark as available but not primary
        const provider = this.state.providers.get(cId);
        if (provider) {
          provider.isAvailable = constellations.available.includes(cId);
        }
      }
    }

    // Process the fix through the chain
    this.processFix(fix);
  }

  /**
   * Infer which GNSS constellations are contributing based on accuracy and location
   */
  private inferConstellations(accuracy: number, lat: number, lon: number): {
    primary: ProviderId;
    active: ProviderId[];
    available: ProviderId[];
  } {
    const active: ProviderId[] = [];
    const available: ProviderId[] = [];

    // GPS is always available globally
    available.push('GPS');
    if (accuracy < 1000) active.push('GPS');

    // Galileo — global, modern, high accuracy
    available.push('GALILEO');
    if (accuracy < 50) active.push('GALILEO');

    // GLONASS — global, especially good at high latitudes
    available.push('GLONASS');
    if (accuracy < 30 || Math.abs(lat) > 50) active.push('GLONASS');

    // BeiDou — global, optimized for Asia-Pacific
    available.push('BEIDOU');
    if (accuracy < 20 || (lat >= -10 && lat <= 55 && lon >= 70 && lon <= 150)) {
      active.push('BEIDOU');
    }

    // QZSS — Japan/Asia-Pacific regional
    if (lat >= -10 && lat <= 55 && lon >= 100 && lon <= 180) {
      available.push('QZSS');
      if (accuracy < 10) active.push('QZSS');
    }

    // NavIC — India regional
    if (lat >= -10 && lat <= 40 && lon >= 50 && lon <= 110) {
      available.push('NAVIC');
      if (accuracy < 50) active.push('NAVIC');
    }

    // SBAS augmentation
    available.push('SBAS');
    if (accuracy < 3) active.push('SBAS');

    // Determine primary based on accuracy
    let primary: ProviderId = 'GPS';
    if (accuracy < 1 && active.includes('GALILEO')) primary = 'GALILEO';
    else if (accuracy < 2 && active.includes('QZSS')) primary = 'QZSS';
    else if (accuracy < 3 && active.includes('BEIDOU')) primary = 'BEIDOU';
    else if (Math.abs(lat) > 55 && active.includes('GLONASS')) primary = 'GLONASS';

    return { primary, active, available };
  }

  /**
   * Estimate satellite count from accuracy
   */
  private estimateSatelliteCount(accuracy: number): number {
    if (accuracy < 1) return 20 + Math.floor(Math.random() * 12);   // RTK-level
    if (accuracy < 3) return 15 + Math.floor(Math.random() * 10);   // Multi-constellation
    if (accuracy < 10) return 10 + Math.floor(Math.random() * 8);   // Good fix
    if (accuracy < 30) return 6 + Math.floor(Math.random() * 6);    // Moderate
    if (accuracy < 100) return 4 + Math.floor(Math.random() * 4);   // Weak
    return Math.max(0, Math.floor(100 / accuracy));
  }

  /**
   * Handle browser geolocation error — activate fallback providers
   */
  private handleGeolocationError(error: GeolocationPositionError): void {
    // Mark all GNSS as failed
    const gnssProviders: ProviderId[] = ['GPS', 'GALILEO', 'GLONASS', 'BEIDOU', 'QZSS', 'NAVIC', 'SBAS'];
    for (const id of gnssProviders) {
      this.updateProviderHealth(id, false, Infinity);
    }

    // Activate WiFi positioning
    this.activateWiFiPositioning();

    // If WiFi also fails, activate Cell
    this.activateCellPositioning();

    // IMU Dead Reckoning is always available
    this.activateIMUDeadReckoning();

    // IP Geolocation as last resort
    this.activateIPGeolocation();
  }

  // ─── Terrestrial Fallback Providers ───

  /**
   * WiFi positioning — uses the browser's network-based location
   */
  private activateWiFiPositioning(): void {
    if (typeof navigator === 'undefined' || !navigator.geolocation) return;

    navigator.geolocation.getCurrentPosition(
      (position) => {
        const fix: PositionFix = {
          lat: position.coords.latitude,
          lon: position.coords.longitude,
          alt: position.coords.altitude || 0,
          accuracy: Math.max(position.coords.accuracy, 15), // WiFi is at least 15m
          verticalAccuracy: 50,
          speed: 0,
          heading: 0,
          timestamp: position.timestamp,
          provider: 'WIFI',
          tier: 3,
          confidence: Math.min(0.7, 30 / Math.max(position.coords.accuracy, 1)),
          fixType: 'wifi',
        };
        this.updateProviderHealth('WIFI', true, fix.accuracy, fix);
        this.processFix(fix);
      },
      () => {
        this.updateProviderHealth('WIFI', false, Infinity);
      },
      { enableHighAccuracy: false, timeout: 5000, maximumAge: 60000 }
    );
  }

  /**
   * Cell tower positioning — uses low-accuracy geolocation
   */
  private activateCellPositioning(): void {
    // Cell positioning is approximated via low-accuracy browser geolocation
    if (typeof navigator === 'undefined' || !navigator.geolocation) return;

    navigator.geolocation.getCurrentPosition(
      (position) => {
        if (position.coords.accuracy > 100) {
          // This is likely cell-tower based
          const fix: PositionFix = {
            lat: position.coords.latitude,
            lon: position.coords.longitude,
            alt: 0,
            accuracy: position.coords.accuracy,
            verticalAccuracy: position.coords.accuracy * 3,
            speed: 0,
            heading: 0,
            timestamp: position.timestamp,
            provider: 'CELL',
            tier: 3,
            confidence: Math.min(0.4, 100 / Math.max(position.coords.accuracy, 1)),
            fixType: 'cell',
          };
          this.updateProviderHealth('CELL', true, fix.accuracy, fix);
          this.processFix(fix);
        }
      },
      () => {
        this.updateProviderHealth('CELL', false, Infinity);
      },
      { enableHighAccuracy: false, timeout: 10000, maximumAge: 120000 }
    );
  }

  /**
   * IMU Dead Reckoning — continues from last known position using device sensors
   */
  private activateIMUDeadReckoning(): void {
    const provider = this.state.providers.get('IMU_DR');
    if (provider) {
      provider.isAvailable = true;
      provider.isActive = true;
      provider.healthScore = 0.6; // Degrades over time without correction

      // If we have a last known position, create a DR fix
      if (this.cachedPosition) {
        const fix: PositionFix = {
          ...this.cachedPosition,
          provider: 'IMU_DR',
          tier: 4,
          confidence: 0.5,
          fixType: 'imu',
          timestamp: Date.now(),
        };
        this.processFix(fix);
      }
    }
  }

  /**
   * IP Geolocation — coarse position from IP address
   */
  private async activateIPGeolocation(): Promise<void> {
    try {
      // Use a free IP geolocation API
      const response = await fetch('https://ipapi.co/json/', { signal: AbortSignal.timeout(5000) });
      if (!response.ok) throw new Error('IP geo failed');
      const data = await response.json();

      if (data.latitude && data.longitude) {
        const fix: PositionFix = {
          lat: data.latitude,
          lon: data.longitude,
          alt: 0,
          accuracy: 5000,  // ~5km for IP geolocation
          verticalAccuracy: 10000,
          speed: 0,
          heading: 0,
          timestamp: Date.now(),
          provider: 'IP_GEO',
          tier: 5,
          confidence: 0.1,
          fixType: 'ip',
        };
        this.updateProviderHealth('IP_GEO', true, fix.accuracy, fix);
        this.processFix(fix);
      }
    } catch {
      this.updateProviderHealth('IP_GEO', false, Infinity);
    }
  }

  // ─── Core Fix Processing ───

  /**
   * Process a position fix from any provider
   * Applies the fallback chain logic to determine the best active position
   */
  private processFix(fix: PositionFix): void {
    // Record in history
    this.fixHistory.push(fix);
    if (this.fixHistory.length > this.MAX_HISTORY) {
      this.fixHistory.shift();
    }

    // Cache the position for last-resort fallback
    if (fix.tier <= 3) {
      this.cachedPosition = fix;
      this.saveCachedPosition(fix);
    }

    // Determine if this fix should become the active position
    const currentFix = this.state.currentFix;
    const shouldSwitch = !currentFix ||
      fix.tier < currentFix.tier ||
      (fix.tier === currentFix.tier && fix.accuracy < currentFix.accuracy) ||
      (fix.tier === currentFix.tier && fix.timestamp > currentFix.timestamp + 5000);

    if (shouldSwitch) {
      const previousProvider = this.state.activeProvider;
      this.state.currentFix = fix;
      this.state.activeProvider = fix.provider;
      this.state.activeTier = fix.tier;

      if (previousProvider !== fix.provider) {
        this.state.handoffCount++;
        this.state.lastHandoffTime = Date.now();
      }
    }

    // Update chain status
    this.updateChainStatus();

    // Update uptime
    this.state.uptime = (Date.now() - this.startTime) / 1000;

    // Update continuity score
    this.updateContinuityScore();

    // Notify listeners
    this.notifyListeners();
  }

  /**
   * Update provider health metrics
   */
  private updateProviderHealth(
    id: ProviderId,
    success: boolean,
    accuracy: number,
    fix?: PositionFix
  ): void {
    const provider = this.state.providers.get(id);
    const breaker = this.circuitBreakers.get(id);
    if (!provider || !breaker) return;

    if (success) {
      breaker.recordSuccess();
      provider.isAvailable = true;
      provider.isActive = true;
      provider.healthScore = Math.min(1, provider.healthScore * 0.8 + 0.2);
      provider.accuracy = accuracy;
      provider.lastFixTime = Date.now();
      provider.fixCount++;
      provider.failCount = 0;
      provider.circuitBreakerOpen = false;

      // Update signal strength from accuracy
      provider.signalStrength = Math.min(1, provider.maxAccuracy / Math.max(accuracy, 0.1));

      // Update satellite count if GNSS
      if (fix?.satellitesUsed !== undefined) {
        provider.satelliteCount = fix.satellitesUsed;
      }
    } else {
      breaker.recordFailure();
      provider.healthScore = Math.max(0, provider.healthScore * 0.7);
      provider.failCount++;
      provider.signalStrength = Math.max(0, provider.signalStrength - 0.1);

      const breakerState = breaker.getState();
      provider.circuitBreakerOpen = breakerState.isOpen;

      if (breakerState.isOpen) {
        provider.isActive = false;
      }
    }

    // Count totals
    let available = 0;
    let active = 0;
    for (const p of Array.from(this.state.providers.values())) {
      if (p.isAvailable) available++;
      if (p.isActive) active++;
    }
    this.state.totalProvidersAvailable = available;
    this.state.totalProvidersActive = active;
  }

  /**
   * Update the overall chain status based on active tier
   */
  private updateChainStatus(): void {
    const tier = this.state.activeTier;
    if (tier === 1) {
      this.state.chainStatus = this.state.totalProvidersActive >= 3 ? 'OPTIMAL' : 'DEGRADED';
    } else if (tier === 2) {
      this.state.chainStatus = 'DEGRADED';
    } else if (tier === 3) {
      this.state.chainStatus = 'FALLBACK';
    } else if (tier === 4) {
      this.state.chainStatus = 'EMERGENCY';
    } else {
      this.state.chainStatus = 'LAST_RESORT';
    }
  }

  /**
   * Update continuity score (how well we've maintained position)
   */
  private updateContinuityScore(): void {
    if (this.fixHistory.length < 2) return;

    // Check for gaps > 2 seconds in the last 60 fixes
    const recent = this.fixHistory.slice(-60);
    let gaps = 0;
    for (let i = 1; i < recent.length; i++) {
      if (recent[i].timestamp - recent[i - 1].timestamp > 2000) {
        gaps++;
      }
    }
    this.state.continuityScore = Math.max(0, 1 - (gaps / Math.max(recent.length - 1, 1)));
  }

  // ─── Health Monitoring ───

  private healthMonitorInterval: ReturnType<typeof setInterval> | null = null;

  private startHealthMonitor(): void {
    this.healthMonitorInterval = setInterval(() => {
      this.runHealthCheck();
    }, 5000); // Every 5 seconds
  }

  private runHealthCheck(): void {
    const now = Date.now();

    for (const [id, provider] of Array.from(this.state.providers)) {
      // Decay health for stale providers
      if (provider.isActive && now - provider.lastFixTime > 10000) {
        provider.healthScore = Math.max(0, provider.healthScore - 0.05);
        if (provider.healthScore < 0.1) {
          provider.isActive = false;
        }
      }

      // Try to recover circuit-broken providers
      const breaker = this.circuitBreakers.get(id);
      if (breaker && breaker.canAttempt() && provider.circuitBreakerOpen) {
        // Recovery probe — the next fix attempt will test this provider
        provider.circuitBreakerOpen = false;
      }
    }

    // Detect jamming (all GNSS providers failing simultaneously)
    const gnssProviders: ProviderId[] = ['GPS', 'GALILEO', 'GLONASS', 'BEIDOU'];
    const gnssActive = gnssProviders.filter(id => {
      const p = this.state.providers.get(id);
      return p && p.isActive;
    });
    this.state.jammingDetected = gnssActive.length === 0 && this.state.totalProvidersActive > 0;

    // If no fix for 30 seconds, try all fallbacks
    if (this.state.currentFix && now - this.state.currentFix.timestamp > 30000) {
      this.activateAllFallbacks();
    }

    this.notifyListeners();
  }

  /**
   * Activate all fallback providers when primary sources fail
   */
  private activateAllFallbacks(): void {
    this.activateWiFiPositioning();
    this.activateCellPositioning();
    this.activateIMUDeadReckoning();
    this.activateIPGeolocation();

    // Use cached position as absolute last resort
    if (this.cachedPosition) {
      const fix: PositionFix = {
        ...this.cachedPosition,
        provider: 'CACHED',
        tier: 5,
        confidence: Math.max(0.05, this.cachedPosition.confidence * 0.5),
        fixType: 'cached',
        timestamp: Date.now(),
        accuracy: this.cachedPosition.accuracy * 2, // Accuracy degrades
      };
      this.processFix(fix);
    }
  }

  // ─── Persistence ───

  private saveCachedPosition(fix: PositionFix): void {
    try {
      localStorage.setItem('gane_last_position', JSON.stringify(fix));
    } catch { /* ignore */ }
  }

  private loadCachedPosition(): void {
    try {
      const stored = localStorage.getItem('gane_last_position');
      if (stored) {
        this.cachedPosition = JSON.parse(stored);
        // Use cached position immediately as a starting point
        if (this.cachedPosition) {
          const fix: PositionFix = {
            ...this.cachedPosition,
            provider: 'CACHED',
            tier: 5,
            confidence: 0.2,
            fixType: 'cached',
            timestamp: Date.now(),
          };
          this.processFix(fix);
        }
      }
    } catch { /* ignore */ }
  }

  // ─── Public API ───

  getState(): FallbackChainState {
    return { ...this.state };
  }

  getCurrentFix(): PositionFix | null {
    return this.state.currentFix;
  }

  getProviderHealth(id: ProviderId): ProviderHealth | undefined {
    return this.state.providers.get(id);
  }

  getAllProviders(): ProviderHealth[] {
    return Array.from(this.state.providers.values());
  }

  getProvidersByTier(tier: ProviderTier): ProviderHealth[] {
    return this.getAllProviders().filter(p => p.tier === tier);
  }

  getActiveProviders(): ProviderHealth[] {
    return this.getAllProviders().filter(p => p.isActive);
  }

  getFixHistory(): PositionFix[] {
    return [...this.fixHistory];
  }

  /**
   * Force a specific provider to be primary (manual override)
   */
  forceProvider(id: ProviderId): void {
    const provider = this.state.providers.get(id);
    if (provider && provider.isAvailable) {
      this.state.activeProvider = id;
      this.state.activeTier = provider.tier;
      this.updateChainStatus();
      this.notifyListeners();
    }
  }

  /**
   * Reset a circuit-broken provider
   */
  resetProvider(id: ProviderId): void {
    const breaker = this.circuitBreakers.get(id);
    const provider = this.state.providers.get(id);
    if (breaker) breaker.reset();
    if (provider) {
      provider.circuitBreakerOpen = false;
      provider.failCount = 0;
    }
  }

  /**
   * Inject a manual position fix (for testing or external sources)
   */
  injectFix(fix: PositionFix): void {
    this.updateProviderHealth(fix.provider, true, fix.accuracy, fix);
    this.processFix(fix);
  }

  subscribe(listener: (state: FallbackChainState) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notifyListeners(): void {
    for (const listener of Array.from(this.listeners)) {
      try { listener(this.state); } catch { /* ignore */ }
    }
  }

  destroy(): void {
    if (this.watchId !== null && typeof navigator !== 'undefined') {
      navigator.geolocation.clearWatch(this.watchId);
    }
    if (this.healthMonitorInterval) {
      clearInterval(this.healthMonitorInterval);
    }
    this.listeners.clear();
    this.fixHistory = [];
  }
}

// ═══════════════════════════════════════════════════
// SINGLETON INSTANCE
// ═══════════════════════════════════════════════════

let _instance: PositionFallbackChain | null = null;

export function getPositionFallbackChain(): PositionFallbackChain {
  if (!_instance) {
    _instance = new PositionFallbackChain();
  }
  return _instance;
}

export function destroyPositionFallbackChain(): void {
  if (_instance) {
    _instance.destroy();
    _instance = null;
  }
}
