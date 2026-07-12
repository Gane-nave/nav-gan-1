/**
 * G.A.N.E Cognitive UI — Polymorphic Interface Engine
 * =====================================================
 * 
 * The UI adapts to the user's context, role, and driving conditions.
 * Each profile defines: color scheme, visible panels, HUD layout,
 * audio cues, and interaction patterns.
 * 
 * Profiles:
 * - PRIVATE: Clean, minimal, consumer-friendly
 * - SPORTS: Performance metrics, aggressive colors, lap timing
 * - EMS: Emergency priority, red/blue scheme, siren integration
 * - LOGISTICS: Fleet-aware, delivery optimization, manifest view
 * 
 * The engine monitors driving context (speed, time of day, weather)
 * and suggests profile switches automatically.
 */

// ─── Color Palettes ───

export interface ColorPalette {
  primary: string;
  secondary: string;
  accent: string;
  danger: string;
  success: string;
  warning: string;
  background: string;
  surface: string;
  text: string;
  textMuted: string;
  border: string;
  glow: string;
}

const PALETTES: Record<string, ColorPalette> = {
  private: {
    primary: '#00e5ff',
    secondary: '#aa66ff',
    accent: '#00ff88',
    danger: '#ff3355',
    success: '#00ff88',
    warning: '#ff9900',
    background: '#010206',
    surface: 'rgba(6,10,20,0.92)',
    text: 'rgba(255,255,255,0.85)',
    textMuted: 'rgba(255,255,255,0.45)',
    border: 'rgba(255,255,255,0.06)',
    glow: 'rgba(0,229,255,0.15)',
  },
  sports: {
    primary: '#ff3355',
    secondary: '#ff9900',
    accent: '#ffd700',
    danger: '#ff0033',
    success: '#00ff88',
    warning: '#ff6600',
    background: '#0a0008',
    surface: 'rgba(20,4,12,0.92)',
    text: 'rgba(255,255,255,0.9)',
    textMuted: 'rgba(255,200,200,0.5)',
    border: 'rgba(255,51,85,0.12)',
    glow: 'rgba(255,51,85,0.2)',
  },
  ems: {
    primary: '#ff0033',
    secondary: '#0066ff',
    accent: '#ffffff',
    danger: '#ff0000',
    success: '#00ff44',
    warning: '#ffaa00',
    background: '#020008',
    surface: 'rgba(10,2,16,0.95)',
    text: 'rgba(255,255,255,0.95)',
    textMuted: 'rgba(255,255,255,0.6)',
    border: 'rgba(255,0,51,0.15)',
    glow: 'rgba(255,0,51,0.25)',
  },
  logistics: {
    primary: '#4488ff',
    secondary: '#00cc88',
    accent: '#ffd700',
    danger: '#ff3355',
    success: '#00ff88',
    warning: '#ff9900',
    background: '#000810',
    surface: 'rgba(2,8,20,0.92)',
    text: 'rgba(255,255,255,0.85)',
    textMuted: 'rgba(180,200,255,0.5)',
    border: 'rgba(68,136,255,0.1)',
    glow: 'rgba(68,136,255,0.15)',
  },
};

// ─── HUD Layouts ───

export interface HUDLayout {
  showSpeedometer: boolean;
  showCompass: boolean;
  showAltimeter: boolean;
  showGForce: boolean;
  showLapTimer: boolean;
  showDeliveryManifest: boolean;
  showSirenControl: boolean;
  showFleetStatus: boolean;
  showETA: boolean;
  showFuelGauge: boolean;
  showBatteryLevel: boolean;
  showSatelliteCount: boolean;
  showConfidenceScore: boolean;
  speedUnit: 'kmh' | 'mph' | 'knots';
  mapZoomDefault: number;
  mapTilt: number;
  minimapEnabled: boolean;
  arOverlayEnabled: boolean;
}

const HUD_LAYOUTS: Record<string, HUDLayout> = {
  private: {
    showSpeedometer: true,
    showCompass: true,
    showAltimeter: false,
    showGForce: false,
    showLapTimer: false,
    showDeliveryManifest: false,
    showSirenControl: false,
    showFleetStatus: false,
    showETA: true,
    showFuelGauge: true,
    showBatteryLevel: true,
    showSatelliteCount: false,
    showConfidenceScore: false,
    speedUnit: 'kmh',
    mapZoomDefault: 16,
    mapTilt: 45,
    minimapEnabled: false,
    arOverlayEnabled: false,
  },
  sports: {
    showSpeedometer: true,
    showCompass: true,
    showAltimeter: true,
    showGForce: true,
    showLapTimer: true,
    showDeliveryManifest: false,
    showSirenControl: false,
    showFleetStatus: false,
    showETA: true,
    showFuelGauge: true,
    showBatteryLevel: true,
    showSatelliteCount: true,
    showConfidenceScore: true,
    speedUnit: 'kmh',
    mapZoomDefault: 17,
    mapTilt: 60,
    minimapEnabled: true,
    arOverlayEnabled: true,
  },
  ems: {
    showSpeedometer: true,
    showCompass: true,
    showAltimeter: false,
    showGForce: false,
    showLapTimer: false,
    showDeliveryManifest: false,
    showSirenControl: true,
    showFleetStatus: true,
    showETA: true,
    showFuelGauge: true,
    showBatteryLevel: true,
    showSatelliteCount: true,
    showConfidenceScore: true,
    speedUnit: 'kmh',
    mapZoomDefault: 15,
    mapTilt: 30,
    minimapEnabled: false,
    arOverlayEnabled: true,
  },
  logistics: {
    showSpeedometer: true,
    showCompass: true,
    showAltimeter: false,
    showGForce: false,
    showLapTimer: false,
    showDeliveryManifest: true,
    showSirenControl: false,
    showFleetStatus: true,
    showETA: true,
    showFuelGauge: true,
    showBatteryLevel: true,
    showSatelliteCount: false,
    showConfidenceScore: false,
    speedUnit: 'kmh',
    mapZoomDefault: 14,
    mapTilt: 30,
    minimapEnabled: true,
    arOverlayEnabled: false,
  },
};

// ─── Audio Profiles ───

export interface AudioProfile {
  navigationVoice: 'standard' | 'military' | 'calm' | 'urgent';
  spatialAudioEnabled: boolean;
  turnAlertDistance: number;     // meters before turn
  hazardAlertEnabled: boolean;
  speedAlertEnabled: boolean;
  sirenEnabled: boolean;
  engineSoundEnabled: boolean;
  volume: number;               // 0-1
}

const AUDIO_PROFILES: Record<string, AudioProfile> = {
  private: {
    navigationVoice: 'calm',
    spatialAudioEnabled: true,
    turnAlertDistance: 200,
    hazardAlertEnabled: true,
    speedAlertEnabled: true,
    sirenEnabled: false,
    engineSoundEnabled: false,
    volume: 0.7,
  },
  sports: {
    navigationVoice: 'standard',
    spatialAudioEnabled: true,
    turnAlertDistance: 150,
    hazardAlertEnabled: true,
    speedAlertEnabled: false,
    sirenEnabled: false,
    engineSoundEnabled: true,
    volume: 0.8,
  },
  ems: {
    navigationVoice: 'urgent',
    spatialAudioEnabled: true,
    turnAlertDistance: 300,
    hazardAlertEnabled: true,
    speedAlertEnabled: false,
    sirenEnabled: true,
    engineSoundEnabled: false,
    volume: 1.0,
  },
  logistics: {
    navigationVoice: 'standard',
    spatialAudioEnabled: true,
    turnAlertDistance: 250,
    hazardAlertEnabled: true,
    speedAlertEnabled: true,
    sirenEnabled: false,
    engineSoundEnabled: false,
    volume: 0.7,
  },
};

// ─── Panel Visibility ───

export interface PanelVisibility {
  visiblePanels: string[];
  defaultOpenPanel: string | null;
  sidebarGroups: string[];
}

const PANEL_VISIBILITY: Record<string, PanelVisibility> = {
  private: {
    visiblePanels: [
      'map-layers', 'parking', 'charging', 'weather', 'route-history',
      'multimodal', 'ar-nav', 'analytics', 'payments', 'offline-mode',
      'accessibility', 'gane-status',
    ],
    defaultOpenPanel: null,
    sidebarGroups: ['NAV', 'INTEL', 'AI', 'SYS'],
  },
  sports: {
    visiblePanels: [
      'map-layers', 'weather', 'driver-score', 'analytics', 'ar-nav',
      'route-history', 'gane-status', 'gnss-manager',
    ],
    defaultOpenPanel: 'driver-score',
    sidebarGroups: ['NAV', 'INTEL', 'AI'],
  },
  ems: {
    visiblePanels: [
      'map-layers', 'weather', 'command-center', 'eoc', 'fleet',
      'evidence', 'smart-alerts', 'v2x', 'risk-engine', 'c4isr',
      'gane-status', 'traffic-cam',
    ],
    defaultOpenPanel: 'command-center',
    sidebarGroups: ['NAV', 'INTEL', 'OPS', 'SYS'],
  },
  logistics: {
    visiblePanels: [
      'map-layers', 'parking', 'weather', 'fleet', 'command-center',
      'analytics', 'c4isr', 'multimodal', 'route-history', 'payments',
      'gane-status', 'offline-mode',
    ],
    defaultOpenPanel: 'fleet',
    sidebarGroups: ['NAV', 'INTEL', 'OPS', 'AI', 'SYS'],
  },
};

// ─── Cognitive UI Engine ───

export interface CognitiveUIState {
  activeProfile: string;
  palette: ColorPalette;
  hudLayout: HUDLayout;
  audioProfile: AudioProfile;
  panelVisibility: PanelVisibility;
  contextualSuggestion: string | null;
  isAutoSwitching: boolean;
}

export interface DrivingContext {
  speed: number;
  timeOfDay: number;        // 0-23
  isRaining: boolean;
  isNight: boolean;
  isHighway: boolean;
  isUrban: boolean;
  hasActiveEmergency: boolean;
  hasActiveDelivery: boolean;
}

export class CognitiveUIEngine {
  private state: CognitiveUIState;
  private listeners: Set<(state: CognitiveUIState) => void> = new Set();
  private autoSwitchEnabled: boolean = true;

  constructor(initialProfile: string = 'private') {
    this.state = this.buildState(initialProfile);
  }

  private buildState(profile: string): CognitiveUIState {
    const p = profile in PALETTES ? profile : 'private';
    return {
      activeProfile: p,
      palette: PALETTES[p],
      hudLayout: HUD_LAYOUTS[p],
      audioProfile: AUDIO_PROFILES[p],
      panelVisibility: PANEL_VISIBILITY[p],
      contextualSuggestion: null,
      isAutoSwitching: this.autoSwitchEnabled,
    };
  }

  /** Switch to a specific profile */
  switchProfile(profile: string): void {
    this.state = this.buildState(profile);
    this.notify();
  }

  /** Get current state */
  getState(): CognitiveUIState {
    return { ...this.state };
  }

  /** Get available profiles */
  getProfiles(): string[] {
    return Object.keys(PALETTES);
  }

  /** Analyze driving context and suggest profile switch */
  analyzeContext(ctx: DrivingContext): string | null {
    if (!this.autoSwitchEnabled) return null;

    let suggestion: string | null = null;

    // Emergency takes priority
    if (ctx.hasActiveEmergency && this.state.activeProfile !== 'ems') {
      suggestion = 'ems';
    }
    // Active delivery suggests logistics
    else if (ctx.hasActiveDelivery && this.state.activeProfile !== 'logistics') {
      suggestion = 'logistics';
    }
    // High speed on highway suggests sports
    else if (ctx.isHighway && ctx.speed > 120 && this.state.activeProfile !== 'sports') {
      suggestion = 'sports';
    }
    // Normal driving defaults to private
    else if (!ctx.hasActiveEmergency && !ctx.hasActiveDelivery && ctx.speed < 80 && ctx.isUrban) {
      if (this.state.activeProfile !== 'private') {
        suggestion = 'private';
      }
    }

    if (suggestion) {
      this.state.contextualSuggestion = suggestion;
      this.notify();
    }

    return suggestion;
  }

  /** Accept the contextual suggestion */
  acceptSuggestion(): void {
    if (this.state.contextualSuggestion) {
      this.switchProfile(this.state.contextualSuggestion);
    }
  }

  /** Dismiss the contextual suggestion */
  dismissSuggestion(): void {
    this.state.contextualSuggestion = null;
    this.notify();
  }

  /** Toggle auto-switching */
  setAutoSwitch(enabled: boolean): void {
    this.autoSwitchEnabled = enabled;
    this.state.isAutoSwitching = enabled;
    this.notify();
  }

  /** Subscribe to state changes */
  subscribe(listener: (state: CognitiveUIState) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notify(): void {
    const snapshot = this.getState();
    this.listeners.forEach(fn => fn(snapshot));
  }

  /** Get CSS variables for the current palette */
  getCSSVariables(): Record<string, string> {
    const p = this.state.palette;
    return {
      '--gane-primary': p.primary,
      '--gane-secondary': p.secondary,
      '--gane-accent': p.accent,
      '--gane-danger': p.danger,
      '--gane-success': p.success,
      '--gane-warning': p.warning,
      '--gane-bg': p.background,
      '--gane-surface': p.surface,
      '--gane-text': p.text,
      '--gane-text-muted': p.textMuted,
      '--gane-border': p.border,
      '--gane-glow': p.glow,
    };
  }

  /** Get night mode adjustments */
  getNightModeAdjustments(): Partial<ColorPalette> {
    return {
      background: '#000000',
      surface: 'rgba(2,4,8,0.96)',
      text: 'rgba(255,200,180,0.7)',
      textMuted: 'rgba(255,200,180,0.35)',
      glow: 'rgba(255,100,50,0.08)',
    };
  }
}

// Singleton
let cognitiveUIInstance: CognitiveUIEngine | null = null;

export function getCognitiveUI(): CognitiveUIEngine {
  if (!cognitiveUIInstance) {
    cognitiveUIInstance = new CognitiveUIEngine();
  }
  return cognitiveUIInstance;
}
