/**
 * G.A.N.E — Finite State Machine (FSM)
 * =======================================
 * Navigation system states with automatic transitions based on sensor health.
 * 
 * States:
 * - BOOTING:           System initializing, acquiring satellites
 * - OPTIMAL_FUSION:    All sensors healthy, full ESKF fusion
 * - DEGRADED_MODE:     Some sensors failed, using PDR/VO fallback
 * - EMERGENCY_BOUNDED: Critical failure, dead reckoning only
 * 
 * Transitions are triggered by sensor health events and confidence scores.
 */

export type FSMState = 'BOOTING' | 'OPTIMAL_FUSION' | 'DEGRADED_MODE' | 'EMERGENCY_BOUNDED';

export interface SensorHealth {
  gnss: boolean;
  imu: boolean;
  vision: boolean;
  network: boolean;
  gnssAccuracy: number;    // meters
  confidenceScore: number; // 0-1
  satellites: number;
  spoofingDetected: boolean;
  lastGNSSFix: number;     // timestamp
}

export interface FSMTransition {
  from: FSMState;
  to: FSMState;
  reason: string;
  timestamp: number;
}

export interface FSMConfig {
  /** Minimum satellites for OPTIMAL */
  minSatellites: number;
  /** Maximum HDOP for OPTIMAL */
  maxHdop: number;
  /** Minimum confidence for OPTIMAL */
  minConfidence: number;
  /** Max time without GNSS fix before DEGRADED (ms) */
  gnssTimeout: number;
  /** Max time without GNSS fix before EMERGENCY (ms) */
  emergencyTimeout: number;
  /** Boot duration (ms) */
  bootDuration: number;
}

const DEFAULT_CONFIG: FSMConfig = {
  minSatellites: 4,
  maxHdop: 5.0,
  minConfidence: 0.6,
  gnssTimeout: 30000,      // 30 seconds
  emergencyTimeout: 120000, // 2 minutes
  bootDuration: 3000,       // 3 seconds
};

export type FSMEventCallback = (transition: FSMTransition, state: FSMState) => void;

export class NavigationFSM {
  private state: FSMState = 'BOOTING';
  private config: FSMConfig;
  private bootStartTime: number;
  private transitionHistory: FSMTransition[] = [];
  private listeners: FSMEventCallback[] = [];
  private lastSensorHealth: SensorHealth | null = null;

  constructor(config: Partial<FSMConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.bootStartTime = Date.now();
  }

  /** Get current state */
  getState(): FSMState {
    return this.state;
  }

  /** Get transition history */
  getHistory(): FSMTransition[] {
    return [...this.transitionHistory];
  }

  /** Get last sensor health */
  getSensorHealth(): SensorHealth | null {
    return this.lastSensorHealth;
  }

  /** Subscribe to state transitions */
  onTransition(callback: FSMEventCallback): () => void {
    this.listeners.push(callback);
    return () => {
      this.listeners = this.listeners.filter(l => l !== callback);
    };
  }

  /** Update sensor health and compute state transition */
  update(health: SensorHealth): FSMState {
    this.lastSensorHealth = health;
    const prevState = this.state;
    const now = Date.now();

    // ─── State Evaluation ───
    switch (this.state) {
      case 'BOOTING': {
        if (now - this.bootStartTime >= this.config.bootDuration) {
          // Boot complete — evaluate sensor health
          if (this.isOptimal(health)) {
            this.transition('OPTIMAL_FUSION', 'Boot complete, all sensors healthy');
          } else if (this.isDegraded(health)) {
            this.transition('DEGRADED_MODE', 'Boot complete, some sensors degraded');
          } else {
            this.transition('EMERGENCY_BOUNDED', 'Boot complete, critical sensor failure');
          }
        }
        break;
      }

      case 'OPTIMAL_FUSION': {
        if (health.spoofingDetected) {
          this.transition('EMERGENCY_BOUNDED', 'GNSS spoofing detected — switching to dead reckoning');
        } else if (!this.isOptimal(health)) {
          if (this.isDegraded(health)) {
            this.transition('DEGRADED_MODE', this.getDegradedReason(health));
          } else {
            this.transition('EMERGENCY_BOUNDED', this.getEmergencyReason(health));
          }
        }
        break;
      }

      case 'DEGRADED_MODE': {
        if (health.spoofingDetected) {
          this.transition('EMERGENCY_BOUNDED', 'GNSS spoofing detected');
        } else if (this.isOptimal(health)) {
          this.transition('OPTIMAL_FUSION', 'All sensors recovered');
        } else if (!this.isDegraded(health)) {
          this.transition('EMERGENCY_BOUNDED', this.getEmergencyReason(health));
        }
        break;
      }

      case 'EMERGENCY_BOUNDED': {
        if (!health.spoofingDetected && this.isOptimal(health)) {
          this.transition('OPTIMAL_FUSION', 'Full sensor recovery');
        } else if (!health.spoofingDetected && this.isDegraded(health)) {
          this.transition('DEGRADED_MODE', 'Partial sensor recovery');
        }
        break;
      }
    }

    return this.state;
  }

  /** Force a specific state (for testing or manual override) */
  forceState(state: FSMState, reason: string = 'Manual override') {
    this.transition(state, reason);
  }

  // ─── Private Methods ───

  private isOptimal(health: SensorHealth): boolean {
    return (
      health.gnss &&
      health.imu &&
      health.satellites >= this.config.minSatellites &&
      health.confidenceScore >= this.config.minConfidence &&
      health.gnssAccuracy <= this.config.maxHdop * 2 &&
      !health.spoofingDetected &&
      (Date.now() - health.lastGNSSFix) < this.config.gnssTimeout
    );
  }

  private isDegraded(health: SensorHealth): boolean {
    const timeSinceGNSS = Date.now() - health.lastGNSSFix;
    return (
      !health.spoofingDetected &&
      (health.imu || health.vision) &&
      timeSinceGNSS < this.config.emergencyTimeout &&
      health.confidenceScore >= 0.3
    );
  }

  private getDegradedReason(health: SensorHealth): string {
    const reasons: string[] = [];
    if (!health.gnss) reasons.push('GNSS offline');
    if (!health.imu) reasons.push('IMU offline');
    if (health.satellites < this.config.minSatellites) reasons.push(`Low satellites (${health.satellites})`);
    if (health.confidenceScore < this.config.minConfidence) reasons.push(`Low confidence (${health.confidenceScore.toFixed(2)})`);
    if (health.gnssAccuracy > this.config.maxHdop * 2) reasons.push(`Poor accuracy (${health.gnssAccuracy.toFixed(1)}m)`);
    return reasons.join(', ') || 'Sensor degradation';
  }

  private getEmergencyReason(health: SensorHealth): string {
    if (health.spoofingDetected) return 'GNSS spoofing detected';
    if (!health.imu && !health.vision) return 'All secondary sensors offline';
    if (health.confidenceScore < 0.1) return 'Critical confidence drop';
    const timeSinceGNSS = Date.now() - health.lastGNSSFix;
    if (timeSinceGNSS > this.config.emergencyTimeout) return `GNSS timeout (${Math.round(timeSinceGNSS / 1000)}s)`;
    return 'Critical sensor failure';
  }

  private transition(newState: FSMState, reason: string) {
    if (newState === this.state) return;

    const transition: FSMTransition = {
      from: this.state,
      to: newState,
      reason,
      timestamp: Date.now(),
    };

    this.transitionHistory.push(transition);
    // Keep only last 100 transitions
    if (this.transitionHistory.length > 100) {
      this.transitionHistory = this.transitionHistory.slice(-100);
    }

    console.log(`[FSM] ${this.state} → ${newState}: ${reason}`);
    this.state = newState;

    // Notify listeners
    this.listeners.forEach(cb => {
      try { cb(transition, newState); } catch (e) { /* ignore listener errors */ }
    });
  }
}

// ─── UI Profile Mapping ───

export interface UIProfile {
  id: 'private' | 'sports' | 'ems' | 'logistics';
  name: string;
  nameHe: string;
  description: string;
  colors: {
    primary: string;
    accent: string;
    danger: string;
    background: string;
  };
  features: {
    showSpeedometer: boolean;
    showAltimeter: boolean;
    showHeartRate: boolean;
    showFleetPanel: boolean;
    showEmergencyButton: boolean;
    showDeliveryQueue: boolean;
    showSirenControl: boolean;
    showPerformanceMetrics: boolean;
    compactMode: boolean;
    voiceGuidance: boolean;
    hapticFeedback: boolean;
  };
  mapStyle: 'default' | 'satellite' | 'terrain' | 'dark' | 'high_contrast';
  hudLayout: 'minimal' | 'standard' | 'full' | 'tactical';
}

export const UI_PROFILES: Record<string, UIProfile> = {
  private: {
    id: 'private',
    name: 'Private',
    nameHe: 'פרטי',
    description: 'Standard navigation for daily driving',
    colors: {
      primary: '#00e5ff',
      accent: '#aa66ff',
      danger: '#ff3355',
      background: '#010206',
    },
    features: {
      showSpeedometer: true,
      showAltimeter: false,
      showHeartRate: false,
      showFleetPanel: false,
      showEmergencyButton: false,
      showDeliveryQueue: false,
      showSirenControl: false,
      showPerformanceMetrics: false,
      compactMode: false,
      voiceGuidance: true,
      hapticFeedback: true,
    },
    mapStyle: 'default',
    hudLayout: 'standard',
  },
  sports: {
    id: 'sports',
    name: 'Sports',
    nameHe: 'ספורט',
    description: 'Performance metrics for cycling, running, hiking',
    colors: {
      primary: '#00ff88',
      accent: '#ffd700',
      danger: '#ff3355',
      background: '#010206',
    },
    features: {
      showSpeedometer: true,
      showAltimeter: true,
      showHeartRate: true,
      showFleetPanel: false,
      showEmergencyButton: true,
      showDeliveryQueue: false,
      showSirenControl: false,
      showPerformanceMetrics: true,
      compactMode: false,
      voiceGuidance: true,
      hapticFeedback: true,
    },
    mapStyle: 'terrain',
    hudLayout: 'full',
  },
  ems: {
    id: 'ems',
    name: 'EMS',
    nameHe: 'חירום',
    description: 'Emergency Medical Services — high contrast, siren control',
    colors: {
      primary: '#ff3355',
      accent: '#ff9900',
      danger: '#ff0000',
      background: '#0a0000',
    },
    features: {
      showSpeedometer: true,
      showAltimeter: false,
      showHeartRate: false,
      showFleetPanel: true,
      showEmergencyButton: true,
      showDeliveryQueue: false,
      showSirenControl: true,
      showPerformanceMetrics: false,
      compactMode: false,
      voiceGuidance: true,
      hapticFeedback: true,
    },
    mapStyle: 'high_contrast',
    hudLayout: 'tactical',
  },
  logistics: {
    id: 'logistics',
    name: 'Logistics',
    nameHe: 'לוגיסטיקה',
    description: 'Delivery optimization with queue management',
    colors: {
      primary: '#4488ff',
      accent: '#00ff88',
      danger: '#ff3355',
      background: '#010206',
    },
    features: {
      showSpeedometer: true,
      showAltimeter: false,
      showHeartRate: false,
      showFleetPanel: true,
      showEmergencyButton: false,
      showDeliveryQueue: true,
      showSirenControl: false,
      showPerformanceMetrics: false,
      compactMode: true,
      voiceGuidance: true,
      hapticFeedback: false,
    },
    mapStyle: 'default',
    hudLayout: 'minimal',
  },
};

/** Get UI profile by ID */
export function getUIProfile(id: string): UIProfile {
  return UI_PROFILES[id] ?? UI_PROFILES.private;
}

/** Get FSM state display info */
export function getFSMStateInfo(state: FSMState): {
  label: string;
  labelHe: string;
  color: string;
  icon: string;
  description: string;
} {
  switch (state) {
    case 'BOOTING':
      return {
        label: 'BOOTING',
        labelHe: 'אתחול',
        color: '#aa66ff',
        icon: '⟳',
        description: 'System initializing, acquiring satellites...',
      };
    case 'OPTIMAL_FUSION':
      return {
        label: 'OPTIMAL',
        labelHe: 'מיטבי',
        color: '#00ff88',
        icon: '◉',
        description: 'All sensors healthy, full ESKF fusion active',
      };
    case 'DEGRADED_MODE':
      return {
        label: 'DEGRADED',
        labelHe: 'מופחת',
        color: '#ff9900',
        icon: '◎',
        description: 'Some sensors degraded, using PDR/VO fallback',
      };
    case 'EMERGENCY_BOUNDED':
      return {
        label: 'EMERGENCY',
        labelHe: 'חירום',
        color: '#ff3355',
        icon: '⚠',
        description: 'Critical failure, dead reckoning only',
      };
  }
}
