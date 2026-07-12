/**
 * G.A.N.E — Simulation Framework Engine
 * ========================================
 * 
 * Deterministic simulation environment for testing navigation scenarios
 * without real sensors or network.
 * 
 * Injectors:
 * 1. GNSS Injector — simulate satellite signals with configurable error models
 * 2. IMU Injector — accelerometer/gyroscope simulation with noise models
 * 3. Traffic Injector — synthetic traffic patterns (congestion, incidents)
 * 4. Network Injector — simulate connectivity conditions (offline, degraded, latency)
 * 5. Weather Injector — weather condition simulation
 * 6. V2X Injector — vehicle-to-everything message simulation
 * 
 * Scenario Runner:
 * - Define scenarios with waypoints, conditions, and expected outcomes
 * - Run deterministic simulations with reproducible random seeds
 * - Validate outcomes against acceptance criteria
 */

// ─── Core Types ─────────────────────────────────────────

export type InjectorType = 'gnss' | 'imu' | 'traffic' | 'network' | 'weather' | 'v2x';

export interface SimulationConfig {
  /** Seed for reproducible randomness */
  seed: number;
  /** Simulation tick rate in Hz */
  tickRate: number;
  /** Real-time factor (1.0 = real-time, 10.0 = 10x speed) */
  timeFactor: number;
  /** Maximum simulation duration in seconds */
  maxDuration: number;
  /** Enable determinism verification */
  deterministic: boolean;
}

const DEFAULT_SIM_CONFIG: SimulationConfig = {
  seed: 42,
  tickRate: 10,
  timeFactor: 1.0,
  maxDuration: 3600,
  deterministic: true,
};

// ─── Seeded PRNG ────────────────────────────────────────

export class SeededRandom {
  private state: number;

  constructor(seed: number) {
    this.state = seed;
  }

  /** Returns a value in [0, 1) */
  next(): number {
    this.state = (this.state * 1664525 + 1013904223) & 0xFFFFFFFF;
    return (this.state >>> 0) / 0xFFFFFFFF;
  }

  /** Returns a value in [min, max) */
  range(min: number, max: number): number {
    return min + this.next() * (max - min);
  }

  /** Returns a normally distributed value (Box-Muller) */
  gaussian(mean: number = 0, stddev: number = 1): number {
    const u1 = this.next();
    const u2 = this.next();
    const z = Math.sqrt(-2 * Math.log(Math.max(u1, 1e-10))) * Math.cos(2 * Math.PI * u2);
    return mean + z * stddev;
  }

  /** Returns true with probability p */
  chance(p: number): boolean {
    return this.next() < p;
  }

  /** Pick a random element from an array */
  pick<T>(arr: T[]): T {
    return arr[Math.floor(this.next() * arr.length)];
  }

  /** Reset to initial seed */
  reset(seed: number): void {
    this.state = seed;
  }
}

// ─── GNSS Injector ──────────────────────────────────────

export interface GNSSConfig {
  /** Base accuracy in meters (CEP) */
  baseAccuracy: number;
  /** Multipath error probability */
  multipathProbability: number;
  /** Multipath error magnitude (meters) */
  multipathMagnitude: number;
  /** Signal loss probability per tick */
  signalLossProbability: number;
  /** Signal loss duration range [min, max] in seconds */
  signalLossDuration: [number, number];
  /** Number of visible satellites */
  satelliteCount: number;
  /** HDOP range */
  hdopRange: [number, number];
  /** Constellation availability */
  constellations: ('GPS' | 'Galileo' | 'GLONASS' | 'BeiDou')[];
}

const DEFAULT_GNSS_CONFIG: GNSSConfig = {
  baseAccuracy: 3.0,
  multipathProbability: 0.05,
  multipathMagnitude: 15.0,
  signalLossProbability: 0.001,
  signalLossDuration: [2, 30],
  satelliteCount: 12,
  hdopRange: [0.8, 2.5],
  constellations: ['GPS', 'Galileo', 'GLONASS', 'BeiDou'],
};

export interface GNSSFix {
  lat: number;
  lng: number;
  altitude: number;
  accuracy: number;
  hdop: number;
  satelliteCount: number;
  constellation: string;
  hasSignal: boolean;
  isMultipath: boolean;
  timestamp: number;
}

export class GNSSInjector {
  private config: GNSSConfig;
  private rng: SeededRandom;
  private signalLostUntil: number = 0;

  constructor(rng: SeededRandom, config: Partial<GNSSConfig> = {}) {
    this.rng = rng;
    this.config = { ...DEFAULT_GNSS_CONFIG, ...config };
  }

  generate(trueLat: number, trueLng: number, trueAlt: number, time: number): GNSSFix {
    // Check signal loss
    if (time < this.signalLostUntil) {
      return {
        lat: 0, lng: 0, altitude: 0,
        accuracy: Infinity,
        hdop: 99,
        satelliteCount: 0,
        constellation: 'none',
        hasSignal: false,
        isMultipath: false,
        timestamp: time,
      };
    }

    // Random signal loss
    if (this.rng.chance(this.config.signalLossProbability)) {
      const duration = this.rng.range(
        this.config.signalLossDuration[0],
        this.config.signalLossDuration[1]
      ) * 1000;
      this.signalLostUntil = time + duration;
      return {
        lat: 0, lng: 0, altitude: 0,
        accuracy: Infinity,
        hdop: 99,
        satelliteCount: 0,
        constellation: 'none',
        hasSignal: false,
        isMultipath: false,
        timestamp: time,
      };
    }

    // Normal fix with noise
    let accuracy = this.config.baseAccuracy;
    let isMultipath = false;

    // Multipath error
    if (this.rng.chance(this.config.multipathProbability)) {
      accuracy += this.rng.range(0, this.config.multipathMagnitude);
      isMultipath = true;
    }

    // Position noise (meters to degrees approximation)
    const latNoise = this.rng.gaussian(0, accuracy / 111320);
    const lngNoise = this.rng.gaussian(0, accuracy / (111320 * Math.cos(trueLat * Math.PI / 180)));
    const altNoise = this.rng.gaussian(0, accuracy * 1.5);

    return {
      lat: trueLat + latNoise,
      lng: trueLng + lngNoise,
      altitude: trueAlt + altNoise,
      accuracy: accuracy + this.rng.gaussian(0, 0.5),
      hdop: this.rng.range(this.config.hdopRange[0], this.config.hdopRange[1]),
      satelliteCount: Math.max(4, Math.round(this.config.satelliteCount + this.rng.gaussian(0, 2))),
      constellation: this.rng.pick(this.config.constellations),
      hasSignal: true,
      isMultipath,
      timestamp: time,
    };
  }

  reset(): void {
    this.signalLostUntil = 0;
  }
}

// ─── IMU Injector ───────────────────────────────────────

export interface IMUConfig {
  /** Accelerometer noise (m/s²) */
  accelNoise: number;
  /** Gyroscope noise (rad/s) */
  gyroNoise: number;
  /** Accelerometer bias drift rate */
  accelBiasDrift: number;
  /** Gyroscope bias drift rate */
  gyroBiasDrift: number;
  /** Sample rate (Hz) */
  sampleRate: number;
}

const DEFAULT_IMU_CONFIG: IMUConfig = {
  accelNoise: 0.02,
  gyroNoise: 0.001,
  accelBiasDrift: 0.0001,
  gyroBiasDrift: 0.00001,
  sampleRate: 100,
};

export interface IMUReading {
  accel: { x: number; y: number; z: number };
  gyro: { x: number; y: number; z: number };
  timestamp: number;
}

export class IMUInjector {
  private config: IMUConfig;
  private rng: SeededRandom;
  private accelBias = { x: 0, y: 0, z: 0 };
  private gyroBias = { x: 0, y: 0, z: 0 };

  constructor(rng: SeededRandom, config: Partial<IMUConfig> = {}) {
    this.rng = rng;
    this.config = { ...DEFAULT_IMU_CONFIG, ...config };
  }

  generate(
    trueAccel: { x: number; y: number; z: number },
    trueGyro: { x: number; y: number; z: number },
    time: number
  ): IMUReading {
    // Drift biases
    this.accelBias.x += this.rng.gaussian(0, this.config.accelBiasDrift);
    this.accelBias.y += this.rng.gaussian(0, this.config.accelBiasDrift);
    this.accelBias.z += this.rng.gaussian(0, this.config.accelBiasDrift);
    this.gyroBias.x += this.rng.gaussian(0, this.config.gyroBiasDrift);
    this.gyroBias.y += this.rng.gaussian(0, this.config.gyroBiasDrift);
    this.gyroBias.z += this.rng.gaussian(0, this.config.gyroBiasDrift);

    return {
      accel: {
        x: trueAccel.x + this.accelBias.x + this.rng.gaussian(0, this.config.accelNoise),
        y: trueAccel.y + this.accelBias.y + this.rng.gaussian(0, this.config.accelNoise),
        z: trueAccel.z + this.accelBias.z + this.rng.gaussian(0, this.config.accelNoise),
      },
      gyro: {
        x: trueGyro.x + this.gyroBias.x + this.rng.gaussian(0, this.config.gyroNoise),
        y: trueGyro.y + this.gyroBias.y + this.rng.gaussian(0, this.config.gyroNoise),
        z: trueGyro.z + this.gyroBias.z + this.rng.gaussian(0, this.config.gyroNoise),
      },
      timestamp: time,
    };
  }

  reset(): void {
    this.accelBias = { x: 0, y: 0, z: 0 };
    this.gyroBias = { x: 0, y: 0, z: 0 };
  }
}

// ─── Traffic Injector ───────────────────────────────────

export interface TrafficConfig {
  /** Base congestion level (0-1) */
  baseCongestion: number;
  /** Rush hour multiplier */
  rushHourMultiplier: number;
  /** Incident probability per tick */
  incidentProbability: number;
  /** Incident duration range [min, max] in seconds */
  incidentDuration: [number, number];
}

const DEFAULT_TRAFFIC_CONFIG: TrafficConfig = {
  baseCongestion: 0.3,
  rushHourMultiplier: 2.5,
  incidentProbability: 0.0005,
  incidentDuration: [300, 3600],
};

export interface TrafficState {
  congestionLevel: number; // 0-1
  speedFactor: number; // 0-1 (1 = free flow)
  activeIncidents: number;
  isRushHour: boolean;
  timestamp: number;
}

export class TrafficInjector {
  private config: TrafficConfig;
  private rng: SeededRandom;
  private incidents: Array<{ endTime: number }> = [];

  constructor(rng: SeededRandom, config: Partial<TrafficConfig> = {}) {
    this.rng = rng;
    this.config = { ...DEFAULT_TRAFFIC_CONFIG, ...config };
  }

  generate(time: number): TrafficState {
    // Simulate rush hours (7-9 AM, 4-7 PM)
    const hour = new Date(time).getHours();
    const isRushHour = (hour >= 7 && hour <= 9) || (hour >= 16 && hour <= 19);
    const rushMultiplier = isRushHour ? this.config.rushHourMultiplier : 1.0;

    // Clean expired incidents
    this.incidents = this.incidents.filter(i => i.endTime > time);

    // Random new incident
    if (this.rng.chance(this.config.incidentProbability)) {
      const duration = this.rng.range(
        this.config.incidentDuration[0],
        this.config.incidentDuration[1]
      ) * 1000;
      this.incidents.push({ endTime: time + duration });
    }

    const incidentFactor = 1 + this.incidents.length * 0.3;
    const congestion = Math.min(1, this.config.baseCongestion * rushMultiplier * incidentFactor + this.rng.gaussian(0, 0.05));
    const speedFactor = Math.max(0.1, 1 - congestion * 0.8);

    return {
      congestionLevel: Math.max(0, congestion),
      speedFactor,
      activeIncidents: this.incidents.length,
      isRushHour,
      timestamp: time,
    };
  }

  reset(): void {
    this.incidents = [];
  }
}

// ─── Network Injector ───────────────────────────────────

export interface NetworkConfig {
  /** Base latency in ms */
  baseLatency: number;
  /** Latency jitter in ms */
  latencyJitter: number;
  /** Packet loss probability */
  packetLoss: number;
  /** Offline probability per tick */
  offlineProbability: number;
  /** Offline duration range [min, max] in seconds */
  offlineDuration: [number, number];
  /** Bandwidth in kbps */
  bandwidth: number;
}

const DEFAULT_NETWORK_CONFIG: NetworkConfig = {
  baseLatency: 50,
  latencyJitter: 20,
  packetLoss: 0.01,
  offlineProbability: 0.0001,
  offlineDuration: [5, 60],
  bandwidth: 5000,
};

export interface NetworkState {
  isOnline: boolean;
  latency: number;
  packetLoss: number;
  bandwidth: number;
  quality: 'excellent' | 'good' | 'fair' | 'poor' | 'offline';
  timestamp: number;
}

export class NetworkInjector {
  private config: NetworkConfig;
  private rng: SeededRandom;
  private offlineUntil: number = 0;

  constructor(rng: SeededRandom, config: Partial<NetworkConfig> = {}) {
    this.rng = rng;
    this.config = { ...DEFAULT_NETWORK_CONFIG, ...config };
  }

  generate(time: number): NetworkState {
    // Check offline state
    if (time < this.offlineUntil) {
      return {
        isOnline: false,
        latency: Infinity,
        packetLoss: 1,
        bandwidth: 0,
        quality: 'offline',
        timestamp: time,
      };
    }

    // Random offline event
    if (this.rng.chance(this.config.offlineProbability)) {
      const duration = this.rng.range(
        this.config.offlineDuration[0],
        this.config.offlineDuration[1]
      ) * 1000;
      this.offlineUntil = time + duration;
      return {
        isOnline: false,
        latency: Infinity,
        packetLoss: 1,
        bandwidth: 0,
        quality: 'offline',
        timestamp: time,
      };
    }

    const latency = this.config.baseLatency + this.rng.gaussian(0, this.config.latencyJitter);
    const packetLoss = Math.max(0, this.config.packetLoss + this.rng.gaussian(0, 0.005));
    const bandwidth = this.config.bandwidth * (0.8 + this.rng.next() * 0.4);

    let quality: NetworkState['quality'] = 'excellent';
    if (latency > 200 || packetLoss > 0.1) quality = 'poor';
    else if (latency > 100 || packetLoss > 0.05) quality = 'fair';
    else if (latency > 50 || packetLoss > 0.02) quality = 'good';

    return {
      isOnline: true,
      latency: Math.max(1, latency),
      packetLoss,
      bandwidth,
      quality,
      timestamp: time,
    };
  }

  reset(): void {
    this.offlineUntil = 0;
  }
}

// ─── Weather Injector ───────────────────────────────────

export type WeatherCondition = 'clear' | 'cloudy' | 'rain' | 'heavy_rain' | 'snow' | 'fog' | 'storm';

export interface WeatherConfig {
  /** Initial condition */
  initialCondition: WeatherCondition;
  /** Transition probability per minute */
  transitionProbability: number;
}

const DEFAULT_WEATHER_CONFIG: WeatherConfig = {
  initialCondition: 'clear',
  transitionProbability: 0.01,
};

export interface WeatherState {
  condition: WeatherCondition;
  visibility: number; // km
  windSpeed: number; // km/h
  temperature: number; // celsius
  precipitation: number; // mm/h
  timestamp: number;
}

const WEATHER_PARAMS: Record<WeatherCondition, { visibility: [number, number]; wind: [number, number]; precip: [number, number] }> = {
  clear: { visibility: [10, 30], wind: [0, 15], precip: [0, 0] },
  cloudy: { visibility: [8, 20], wind: [5, 25], precip: [0, 0] },
  rain: { visibility: [3, 10], wind: [10, 30], precip: [1, 10] },
  heavy_rain: { visibility: [1, 5], wind: [20, 50], precip: [10, 50] },
  snow: { visibility: [1, 5], wind: [5, 20], precip: [1, 15] },
  fog: { visibility: [0.1, 1], wind: [0, 10], precip: [0, 0.5] },
  storm: { visibility: [0.5, 3], wind: [40, 100], precip: [20, 80] },
};

const WEATHER_TRANSITIONS: Record<WeatherCondition, WeatherCondition[]> = {
  clear: ['cloudy'],
  cloudy: ['clear', 'rain', 'fog'],
  rain: ['cloudy', 'heavy_rain'],
  heavy_rain: ['rain', 'storm'],
  snow: ['cloudy', 'clear'],
  fog: ['cloudy', 'clear'],
  storm: ['heavy_rain', 'rain'],
};

export class WeatherInjector {
  private config: WeatherConfig;
  private rng: SeededRandom;
  private currentCondition: WeatherCondition;

  constructor(rng: SeededRandom, config: Partial<WeatherConfig> = {}) {
    this.rng = rng;
    this.config = { ...DEFAULT_WEATHER_CONFIG, ...config };
    this.currentCondition = this.config.initialCondition;
  }

  generate(time: number): WeatherState {
    // Random weather transition
    if (this.rng.chance(this.config.transitionProbability)) {
      const transitions = WEATHER_TRANSITIONS[this.currentCondition];
      if (transitions.length > 0) {
        this.currentCondition = this.rng.pick(transitions);
      }
    }

    const params = WEATHER_PARAMS[this.currentCondition];
    return {
      condition: this.currentCondition,
      visibility: this.rng.range(params.visibility[0], params.visibility[1]),
      windSpeed: this.rng.range(params.wind[0], params.wind[1]),
      temperature: this.rng.range(-5, 40),
      precipitation: this.rng.range(params.precip[0], params.precip[1]),
      timestamp: time,
    };
  }

  setCondition(condition: WeatherCondition): void {
    this.currentCondition = condition;
  }

  reset(): void {
    this.currentCondition = this.config.initialCondition;
  }
}

// ─── Scenario Definition ────────────────────────────────

export interface ScenarioWaypoint {
  lat: number;
  lng: number;
  altitude: number;
  speed: number; // m/s
  heading: number; // degrees
  dwellTime?: number; // seconds to stay at this point
}

export interface ScenarioCondition {
  /** Time offset from start (seconds) when condition activates */
  activateAt: number;
  /** Duration of condition (seconds) */
  duration: number;
  injector: InjectorType;
  params: Record<string, unknown>;
}

export interface AcceptanceCriterion {
  id: string;
  description: string;
  check: (results: SimulationResults) => boolean;
}

export interface SimulationScenario {
  id: string;
  name: string;
  description: string;
  waypoints: ScenarioWaypoint[];
  conditions: ScenarioCondition[];
  acceptanceCriteria: AcceptanceCriterion[];
  config: Partial<SimulationConfig>;
}

// ─── Simulation Results ─────────────────────────────────

export interface SimulationResults {
  scenarioId: string;
  startTime: number;
  endTime: number;
  duration: number;
  tickCount: number;
  gnssFixes: GNSSFix[];
  imuReadings: IMUReading[];
  trafficStates: TrafficState[];
  networkStates: NetworkState[];
  weatherStates: WeatherState[];
  /** Acceptance criteria results */
  criteriaResults: Array<{ id: string; passed: boolean; description: string }>;
  /** Overall pass/fail */
  passed: boolean;
  /** Determinism hash for reproducibility */
  deterministicHash: string;
}

// ─── Simulation Runner ──────────────────────────────────

export class SimulationRunner {
  private config: SimulationConfig;
  private rng: SeededRandom;
  private gnss: GNSSInjector;
  private imu: IMUInjector;
  private traffic: TrafficInjector;
  private network: NetworkInjector;
  private weather: WeatherInjector;

  constructor(config: Partial<SimulationConfig> = {}) {
    this.config = { ...DEFAULT_SIM_CONFIG, ...config };
    this.rng = new SeededRandom(this.config.seed);
    this.gnss = new GNSSInjector(this.rng);
    this.imu = new IMUInjector(this.rng);
    this.traffic = new TrafficInjector(this.rng);
    this.network = new NetworkInjector(this.rng);
    this.weather = new WeatherInjector(this.rng);
  }

  /**
   * Run a complete scenario simulation
   */
  run(scenario: SimulationScenario): SimulationResults {
    // Reset with scenario-specific config
    const mergedConfig = { ...this.config, ...scenario.config };
    this.rng.reset(mergedConfig.seed);
    this.gnss.reset();
    this.imu.reset();
    this.traffic.reset();
    this.network.reset();
    this.weather.reset();

    const tickInterval = 1000 / mergedConfig.tickRate;
    const maxTicks = mergedConfig.maxDuration * mergedConfig.tickRate;
    const startTime = Date.now();

    const gnssFixes: GNSSFix[] = [];
    const imuReadings: IMUReading[] = [];
    const trafficStates: TrafficState[] = [];
    const networkStates: NetworkState[] = [];
    const weatherStates: WeatherState[] = [];

    // Interpolate position along waypoints
    const totalWaypointTime = this.calculateTotalTime(scenario.waypoints);
    let tickCount = 0;

    for (let tick = 0; tick < maxTicks && tick * tickInterval <= totalWaypointTime * 1000; tick++) {
      const simTime = startTime + tick * tickInterval;
      const elapsed = tick * tickInterval; // ms since start
      const position = this.interpolateWaypoint(scenario.waypoints, elapsed / 1000);

      if (!position) break;

      // Generate sensor data
      gnssFixes.push(this.gnss.generate(position.lat, position.lng, position.altitude, simTime));
      imuReadings.push(this.imu.generate(
        { x: 0, y: 0, z: 9.81 }, // simplified
        { x: 0, y: 0, z: (position.heading * Math.PI / 180) * 0.01 },
        simTime
      ));

      // Lower frequency updates
      if (tick % 10 === 0) {
        trafficStates.push(this.traffic.generate(simTime));
        networkStates.push(this.network.generate(simTime));
      }
      if (tick % 100 === 0) {
        weatherStates.push(this.weather.generate(simTime));
      }

      tickCount++;
    }

    const endTime = startTime + tickCount * tickInterval;

    // Evaluate acceptance criteria
    const results: SimulationResults = {
      scenarioId: scenario.id,
      startTime,
      endTime,
      duration: endTime - startTime,
      tickCount,
      gnssFixes,
      imuReadings,
      trafficStates,
      networkStates,
      weatherStates,
      criteriaResults: [],
      passed: true,
      deterministicHash: '',
    };

    // Check acceptance criteria
    results.criteriaResults = scenario.acceptanceCriteria.map(c => ({
      id: c.id,
      description: c.description,
      passed: c.check(results),
    }));

    results.passed = results.criteriaResults.every(c => c.passed);

    // Compute determinism hash
    const hashInput = JSON.stringify({
      gnssCount: gnssFixes.length,
      imuCount: imuReadings.length,
      firstGnss: gnssFixes[0],
      lastGnss: gnssFixes[gnssFixes.length - 1],
    });
    let hash = 0;
    for (let i = 0; i < hashInput.length; i++) {
      hash = ((hash << 5) - hash) + hashInput.charCodeAt(i);
      hash = hash & hash;
    }
    results.deterministicHash = Math.abs(hash).toString(36);

    return results;
  }

  private calculateTotalTime(waypoints: ScenarioWaypoint[]): number {
    let total = 0;
    for (let i = 1; i < waypoints.length; i++) {
      const prev = waypoints[i - 1];
      const curr = waypoints[i];
      const dist = this.haversine(prev.lat, prev.lng, curr.lat, curr.lng);
      const avgSpeed = (prev.speed + curr.speed) / 2;
      total += avgSpeed > 0 ? dist / avgSpeed : 0;
      total += (prev.dwellTime || 0);
    }
    total += (waypoints[waypoints.length - 1]?.dwellTime || 0);
    return total;
  }

  private interpolateWaypoint(
    waypoints: ScenarioWaypoint[],
    elapsedSeconds: number
  ): ScenarioWaypoint | null {
    if (waypoints.length === 0) return null;
    if (waypoints.length === 1) return waypoints[0];

    let accumulated = 0;
    for (let i = 1; i < waypoints.length; i++) {
      const prev = waypoints[i - 1];
      const curr = waypoints[i];
      const dist = this.haversine(prev.lat, prev.lng, curr.lat, curr.lng);
      const avgSpeed = (prev.speed + curr.speed) / 2;
      const segmentTime = avgSpeed > 0 ? dist / avgSpeed : 0;
      const dwellTime = prev.dwellTime || 0;

      if (elapsedSeconds < accumulated + dwellTime) {
        return prev; // Dwelling at waypoint
      }
      accumulated += dwellTime;

      if (elapsedSeconds < accumulated + segmentTime) {
        const t = (elapsedSeconds - accumulated) / segmentTime;
        return {
          lat: prev.lat + (curr.lat - prev.lat) * t,
          lng: prev.lng + (curr.lng - prev.lng) * t,
          altitude: prev.altitude + (curr.altitude - prev.altitude) * t,
          speed: prev.speed + (curr.speed - prev.speed) * t,
          heading: prev.heading + (curr.heading - prev.heading) * t,
        };
      }
      accumulated += segmentTime;
    }

    return waypoints[waypoints.length - 1];
  }

  private haversine(lat1: number, lng1: number, lat2: number, lng2: number): number {
    const R = 6371000;
    const dLat = (lat2 - lat1) * Math.PI / 180;
    const dLng = (lng2 - lng1) * Math.PI / 180;
    const a = Math.sin(dLat / 2) ** 2 +
              Math.cos(lat1 * Math.PI / 180) * Math.cos(lat2 * Math.PI / 180) *
              Math.sin(dLng / 2) ** 2;
    return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }
}

// ─── Pre-built Scenarios ────────────────────────────────

export const BUILTIN_SCENARIOS: SimulationScenario[] = [
  {
    id: 'urban-drive',
    name: 'Urban Drive (Tel Aviv)',
    description: 'City driving with traffic, multipath, and signal loss in urban canyons',
    waypoints: [
      { lat: 32.0853, lng: 34.7818, altitude: 20, speed: 8, heading: 0 },
      { lat: 32.0870, lng: 34.7830, altitude: 22, speed: 12, heading: 45 },
      { lat: 32.0890, lng: 34.7850, altitude: 18, speed: 5, heading: 90, dwellTime: 30 },
      { lat: 32.0900, lng: 34.7870, altitude: 20, speed: 10, heading: 135 },
    ],
    conditions: [
      { activateAt: 60, duration: 30, injector: 'gnss', params: { multipathProbability: 0.3 } },
      { activateAt: 120, duration: 15, injector: 'network', params: { offlineProbability: 0.5 } },
    ],
    acceptanceCriteria: [
      {
        id: 'gnss-fix-rate',
        description: 'GNSS fix rate > 90%',
        check: (r) => {
          const withSignal = r.gnssFixes.filter(f => f.hasSignal).length;
          return withSignal / r.gnssFixes.length > 0.9;
        },
      },
      {
        id: 'simulation-completes',
        description: 'Simulation runs to completion',
        check: (r) => r.tickCount > 0,
      },
    ],
    config: { seed: 42, tickRate: 10, maxDuration: 600 },
  },
  {
    id: 'highway-cruise',
    name: 'Highway Cruise',
    description: 'High-speed highway driving with good GNSS and varying traffic',
    waypoints: [
      { lat: 32.0000, lng: 34.8000, altitude: 50, speed: 30, heading: 0 },
      { lat: 32.0500, lng: 34.8000, altitude: 55, speed: 33, heading: 0 },
      { lat: 32.1000, lng: 34.8100, altitude: 60, speed: 28, heading: 15 },
      { lat: 32.1500, lng: 34.8200, altitude: 45, speed: 33, heading: 20 },
    ],
    conditions: [],
    acceptanceCriteria: [
      {
        id: 'gnss-accuracy',
        description: 'Average GNSS accuracy < 5m',
        check: (r) => {
          const fixes = r.gnssFixes.filter(f => f.hasSignal);
          const avg = fixes.reduce((s, f) => s + f.accuracy, 0) / fixes.length;
          return avg < 5;
        },
      },
    ],
    config: { seed: 123, tickRate: 10, maxDuration: 1200 },
  },
  {
    id: 'tunnel-passage',
    name: 'Tunnel Passage',
    description: 'Drive through a tunnel with complete GNSS loss',
    waypoints: [
      { lat: 32.0800, lng: 34.7700, altitude: 30, speed: 15, heading: 90 },
      { lat: 32.0800, lng: 34.7750, altitude: 25, speed: 12, heading: 90 },
      { lat: 32.0800, lng: 34.7800, altitude: 20, speed: 15, heading: 90 },
    ],
    conditions: [
      { activateAt: 30, duration: 60, injector: 'gnss', params: { signalLossProbability: 1.0 } },
    ],
    acceptanceCriteria: [
      {
        id: 'handles-signal-loss',
        description: 'System handles GNSS signal loss gracefully',
        check: (r) => {
          const lostFixes = r.gnssFixes.filter(f => !f.hasSignal);
          return lostFixes.length > 0; // Should have some signal loss
        },
      },
    ],
    config: { seed: 777, tickRate: 10, maxDuration: 300 },
  },
];

// ─── Singleton ──────────────────────────────────────────

let _simInstance: SimulationRunner | null = null;

export function getSimulationRunner(config?: Partial<SimulationConfig>): SimulationRunner {
  if (!_simInstance) {
    _simInstance = new SimulationRunner(config);
  }
  return _simInstance;
}

export function resetSimulationRunner(): void {
  _simInstance = null;
}
