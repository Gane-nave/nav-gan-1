/**
 * G.A.N.E — V2X Communication Engine
 * =====================================
 * Vehicle-to-Everything communication simulation.
 *
 * PROTOCOLS:
 *   - V2V: Vehicle-to-Vehicle (collision avoidance, platooning)
 *   - V2I: Vehicle-to-Infrastructure (traffic lights, signs)
 *   - V2N: Vehicle-to-Network (cloud services)
 *   - V2P: Vehicle-to-Pedestrian (crosswalk alerts)
 *
 * MESSAGE TYPES:
 *   - BSM: Basic Safety Message (position, speed, heading)
 *   - SPaT: Signal Phase and Timing
 *   - MAP: Map/intersection geometry
 *   - TIM: Traveler Information Message
 *   - PSM: Personal Safety Message (pedestrians)
 */

// ─── Types ───────────────────────────────────────────────

export type V2XProtocol = 'v2v' | 'v2i' | 'v2n' | 'v2p';
export type V2XMessageType = 'bsm' | 'spat' | 'map' | 'tim' | 'psm' | 'emergency';

export interface V2XMessage {
  id: string;
  protocol: V2XProtocol;
  type: V2XMessageType;
  senderId: string;
  senderType: 'vehicle' | 'infrastructure' | 'pedestrian' | 'network';
  lat: number;
  lon: number;
  timestamp: number;
  ttlMs: number;
  priority: number;                  // 0-7 (7 = highest)
  payload: Record<string, unknown>;
  signalStrength: number;            // -120 to 0 dBm
  encrypted: boolean;
}

export interface BSMPayload {
  speed: number;                     // m/s
  heading: number;                   // degrees
  acceleration: number;              // m/s²
  brakeStatus: boolean;
  steeringAngle: number;            // degrees
  vehicleType: 'car' | 'truck' | 'bus' | 'motorcycle' | 'emergency';
  size: { length: number; width: number };
}

export interface SPaTPayload {
  intersectionId: string;
  phases: {
    id: number;
    state: 'red' | 'yellow' | 'green' | 'flashing';
    remainingMs: number;
    nextState: 'red' | 'yellow' | 'green';
  }[];
}

export interface TIMPayload {
  category: 'warning' | 'info' | 'restriction';
  message: string;
  messageHe: string;
  affectedLanes: number[];
  validFrom: number;
  validTo: number;
}

export interface V2XConfig {
  maxRange: number;                  // max communication range in meters
  maxMessages: number;               // max messages in buffer
  bsmIntervalMs: number;             // BSM broadcast interval
  enabled: boolean;
}

export interface V2XState {
  isActive: boolean;
  connectedVehicles: number;
  connectedInfrastructure: number;
  messagesReceived: number;
  messagesSent: number;
  avgLatencyMs: number;
  signalQuality: number;            // 0-1
  lastBSMAt: number;
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: V2XConfig = {
  maxRange: 300,
  maxMessages: 200,
  bsmIntervalMs: 100,
  enabled: true,
};

// ─── V2X Engine ─────────────────────────────────────────

export class V2XEngine {
  private config: V2XConfig;
  private state: V2XState;
  private messageBuffer: V2XMessage[] = [];
  private nearbyVehicles: Map<string, V2XMessage> = new Map();
  private nearbyInfrastructure: Map<string, V2XMessage> = new Map();
  private bsmTimer: ReturnType<typeof setInterval> | null = null;

  // Callbacks
  private onMessage: ((msg: V2XMessage) => void) | null = null;
  private onCollisionWarning: ((msg: V2XMessage, ttcSeconds: number) => void) | null = null;
  private onSPaT: ((msg: V2XMessage, payload: SPaTPayload) => void) | null = null;

  // Own vehicle state
  private ownPosition: { lat: number; lon: number; speed: number; heading: number } = {
    lat: 0, lon: 0, speed: 0, heading: 0,
  };

  constructor(config: Partial<V2XConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.state = {
      isActive: false,
      connectedVehicles: 0,
      connectedInfrastructure: 0,
      messagesReceived: 0,
      messagesSent: 0,
      avgLatencyMs: 0,
      signalQuality: 0,
      lastBSMAt: 0,
    };
  }

  // ─── Lifecycle ─────────────────────────────────────────

  start() {
    this.state.isActive = true;

    // Start BSM broadcast simulation
    this.bsmTimer = setInterval(() => {
      this.broadcastBSM();
    }, this.config.bsmIntervalMs);
  }

  stop() {
    this.state.isActive = false;
    if (this.bsmTimer) {
      clearInterval(this.bsmTimer);
      this.bsmTimer = null;
    }
  }

  // ─── Message Handling ─────────────────────────────────

  /**
   * Receive a V2X message.
   */
  receiveMessage(msg: V2XMessage) {
    if (!this.state.isActive) return;

    // Check range
    const dist = this.haversine(
      this.ownPosition.lat, this.ownPosition.lon,
      msg.lat, msg.lon
    );
    if (dist > this.config.maxRange) return;

    // Add signal strength based on distance
    msg.signalStrength = Math.round(-40 - (dist / this.config.maxRange) * 80);

    // Buffer message
    this.messageBuffer.push(msg);
    if (this.messageBuffer.length > this.config.maxMessages) {
      this.messageBuffer = this.messageBuffer.slice(-this.config.maxMessages);
    }

    this.state.messagesReceived++;

    // Process by type
    switch (msg.type) {
      case 'bsm':
        this.processBSM(msg);
        break;
      case 'spat':
        this.processSPaT(msg);
        break;
      case 'tim':
        this.processTIM(msg);
        break;
      case 'psm':
        this.processPSM(msg);
        break;
      case 'emergency':
        this.processEmergency(msg);
        break;
    }

    // Notify
    if (this.onMessage) {
      this.onMessage(msg);
    }
  }

  // ─── BSM Processing ───────────────────────────────────

  private processBSM(msg: V2XMessage) {
    const payload = msg.payload as unknown as BSMPayload;

    // Track nearby vehicle
    this.nearbyVehicles.set(msg.senderId, msg);

    // Collision detection
    if (payload.speed > 0) {
      const ttc = this.computeTTC(msg, payload);
      if (ttc !== null && ttc < 5) { // Less than 5 seconds to collision
        if (this.onCollisionWarning) {
          this.onCollisionWarning(msg, ttc);
        }
      }
    }

    // Update connected count
    this.state.connectedVehicles = this.nearbyVehicles.size;
    this.state.lastBSMAt = Date.now();
  }

  private processSPaT(msg: V2XMessage) {
    this.nearbyInfrastructure.set(msg.senderId, msg);
    this.state.connectedInfrastructure = this.nearbyInfrastructure.size;

    if (this.onSPaT) {
      this.onSPaT(msg, msg.payload as unknown as SPaTPayload);
    }
  }

  private processTIM(msg: V2XMessage) {
    // Traveler information — forward to alert system
    if (this.onMessage) {
      this.onMessage(msg);
    }
  }

  private processPSM(msg: V2XMessage) {
    // Pedestrian safety — check proximity
    const dist = this.haversine(
      this.ownPosition.lat, this.ownPosition.lon,
      msg.lat, msg.lon
    );
    if (dist < 50) { // Within 50m
      if (this.onCollisionWarning) {
        const ttc = dist / Math.max(this.ownPosition.speed, 1);
        this.onCollisionWarning(msg, ttc);
      }
    }
  }

  private processEmergency(msg: V2XMessage) {
    // Emergency vehicle — always forward
    if (this.onMessage) {
      this.onMessage(msg);
    }
  }

  // ─── BSM Broadcasting ─────────────────────────────────

  private broadcastBSM() {
    const bsm: V2XMessage = {
      id: `bsm_${Date.now()}`,
      protocol: 'v2v',
      type: 'bsm',
      senderId: 'self',
      senderType: 'vehicle',
      lat: this.ownPosition.lat,
      lon: this.ownPosition.lon,
      timestamp: Date.now(),
      ttlMs: 1000,
      priority: 5,
      payload: {
        speed: this.ownPosition.speed,
        heading: this.ownPosition.heading,
        acceleration: 0,
        brakeStatus: false,
        steeringAngle: 0,
        vehicleType: 'car',
        size: { length: 4.5, width: 1.8 },
      } as unknown as Record<string, unknown>,
      signalStrength: 0,
      encrypted: true,
    };

    this.state.messagesSent++;
    this.state.lastBSMAt = Date.now();
  }

  // ─── Collision Detection ──────────────────────────────

  private computeTTC(msg: V2XMessage, payload: BSMPayload): number | null {
    const dist = this.haversine(
      this.ownPosition.lat, this.ownPosition.lon,
      msg.lat, msg.lon
    );

    // Relative speed (simplified)
    const ownSpeed = this.ownPosition.speed;
    const otherSpeed = payload.speed;

    // Check if vehicles are approaching
    const headingDiff = Math.abs(this.ownPosition.heading - payload.heading);
    const isApproaching = headingDiff > 90 && headingDiff < 270;

    if (!isApproaching) return null;

    const closingSpeed = ownSpeed + otherSpeed;
    if (closingSpeed <= 0) return null;

    return dist / closingSpeed;
  }

  // ─── Position Updates ─────────────────────────────────

  updatePosition(lat: number, lon: number, speed: number, heading: number) {
    this.ownPosition = { lat, lon, speed, heading };

    // Clean up stale vehicles (older than 5 seconds)
    const now = Date.now();
    for (const [id, msg] of Array.from(this.nearbyVehicles.entries())) {
      if (now - msg.timestamp > 5000) {
        this.nearbyVehicles.delete(id);
      }
    }
    this.state.connectedVehicles = this.nearbyVehicles.size;

    // Clean up stale infrastructure (older than 30 seconds)
    for (const [id, msg] of Array.from(this.nearbyInfrastructure.entries())) {
      if (now - msg.timestamp > 30000) {
        this.nearbyInfrastructure.delete(id);
      }
    }
    this.state.connectedInfrastructure = this.nearbyInfrastructure.size;

    // Update signal quality
    this.state.signalQuality = Math.min(1, this.nearbyVehicles.size / 10);
  }

  // ─── Queries ──────────────────────────────────────────

  /**
   * Get nearby traffic light state.
   */
  getNearestTrafficLight(): SPaTPayload | null {
    let nearest: V2XMessage | null = null;
    let nearestDist = Infinity;

    for (const msg of Array.from(this.nearbyInfrastructure.values())) {
      if (msg.type === 'spat') {
        const dist = this.haversine(
          this.ownPosition.lat, this.ownPosition.lon,
          msg.lat, msg.lon
        );
        if (dist < nearestDist) {
          nearestDist = dist;
          nearest = msg;
        }
      }
    }

    return nearest ? (nearest.payload as unknown as SPaTPayload) : null;
  }

  /**
   * Get all nearby vehicles.
   */
  getNearbyVehicles(): { id: string; lat: number; lon: number; speed: number; distance: number }[] {
    return Array.from(this.nearbyVehicles.entries()).map(([id, msg]) => ({
      id,
      lat: msg.lat,
      lon: msg.lon,
      speed: (msg.payload as any).speed || 0,
      distance: Math.round(this.haversine(
        this.ownPosition.lat, this.ownPosition.lon,
        msg.lat, msg.lon
      )),
    }));
  }

  // ─── Callbacks ────────────────────────────────────────

  setOnMessage(callback: (msg: V2XMessage) => void) {
    this.onMessage = callback;
  }

  setOnCollisionWarning(callback: (msg: V2XMessage, ttcSeconds: number) => void) {
    this.onCollisionWarning = callback;
  }

  setOnSPaT(callback: (msg: V2XMessage, payload: SPaTPayload) => void) {
    this.onSPaT = callback;
  }

  // ─── Helpers ──────────────────────────────────────────

  private haversine(lat1: number, lon1: number, lat2: number, lon2: number): number {
    const R = 6371000;
    const dLat = (lat2 - lat1) * Math.PI / 180;
    const dLon = (lon2 - lon1) * Math.PI / 180;
    const a = Math.sin(dLat / 2) ** 2 +
      Math.cos(lat1 * Math.PI / 180) * Math.cos(lat2 * Math.PI / 180) *
      Math.sin(dLon / 2) ** 2;
    return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  }

  // ─── Public API ───────────────────────────────────────

  getState(): V2XState {
    return { ...this.state };
  }

  getConfig(): V2XConfig {
    return { ...this.config };
  }

  getRecentMessages(count: number = 20): V2XMessage[] {
    return this.messageBuffer.slice(-count);
  }

  destroy() {
    this.stop();
    this.messageBuffer = [];
    this.nearbyVehicles.clear();
    this.nearbyInfrastructure.clear();
    this.onMessage = null;
    this.onCollisionWarning = null;
    this.onSPaT = null;
  }
}
