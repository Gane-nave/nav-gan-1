/**
 * G.A.N.E Engine Context — Full Integration
 * ============================================
 * Wires ALL G.A.N.E engine modules into React:
 *
 * SENSOR LAYER:
 *   - SensorBridge (Geolocation API, DeviceMotion, DeviceOrientation → real sensor data)
 *
 * CORE NAVIGATION:
 *   - NavigationManager (orchestrates ESKF + PDR + VO + SpatialAudio)
 *   - NavigationFSM (state machine)
 *   - MultiConstellation GNSS (satellite constellation management)
 *   - TunnelRecovery (GPS-denied navigation in tunnels)
 *   - UrbanCanyon (map matching + A* routing)
 *   - RerouteEngine (deviation detection + rerouting)
 *   - ETAEngine (arrival time estimation)
 *
 * AI & PREDICTION:
 *   - MLPrediction (traffic + travel time prediction)
 *   - PredictiveIntent (destination prediction)
 *   - CognitiveUI (adaptive interface)
 *
 * REAL-TIME:
 *   - WebSocket Client (real-time data)
 *   - EventBus (central message broker)
 *   - AlertSystem (alert routing → notifications)
 *   - V2XEngine (vehicle-to-everything communication)
 *
 * DATA & STORAGE:
 *   - OfflineStore (IndexedDB + CRDT)
 *   - OfflineSync (auto-sync when online)
 *   - MultiSourceMaps (tile management)
 *   - GeospatialIndex (spatial queries)
 *
 * RENDERING & AUDIO:
 *   - WebGPU/NeRF Renderer (3D map rendering)
 *   - SpatialAudio (3D directional cues)
 *   - AR HUD (augmented reality overlay)
 *
 * MONITORING & QUALITY:
 *   - SensorQuality (sensor health monitoring)
 *   - PerformanceMonitor (FPS, memory, latency)
 *   - Observability (distributed tracing)
 *   - BatteryOptimizer (power management)
 *
 * SIMULATION & TESTING:
 *   - SimulationEngine (scenario simulation)
 *   - ChaosTestingEngine (fault injection)
 *   - BenchmarkHarness (performance benchmarks)
 *   - ReplayEngine (session replay)
 *   - TripReplay (trip history playback)
 *
 * ADMIN & POLICY:
 *   - AdminTerminal (Tier-0 Omni-Control)
 *   - PolicyEngine (navigation policies)
 *   - PrivacyEngine (data anonymization)
 *   - AccessibilityEngine (a11y support)
 *   - DigitalTwin (digital twin state)
 *
 * ROUTING:
 *   - MultiProviderRouting (Mapbox, HERE, TomTom, internal)
 */

import { createContext, useContext, useEffect, useRef, useState, useCallback, type ReactNode } from "react";

// Core
import { NavigationFSM, type FSMState, type SensorHealth, type FSMTransition } from "@/engine/fsm";
import { getUIProfile, type UIProfile } from "@/engine/fsm";
import { getNavigationManager, type NavigationState } from "@/engine/navigationManager";
import { getSensorBridge, type SensorStatus } from "@/engine/sensorBridge";
import { wsClient } from "@/engine/wsClient";
import { getOfflineStore, type OfflineStore } from "@/engine/offlineStore";

// Rendering & Audio
import { WebGPUNerfRenderer, type RenderStats } from "@/engine/webgpuNerf";
import { SpatialAudioEngine } from "@/engine/spatialAudio";

// AI & Prediction
import { PredictiveIntentEngine, type PredictedDestination } from "@/engine/predictiveIntent";
import { MLPredictionEngine } from "@/engine/mlPrediction";

// Admin & Policy
import { AdminTerminal, type TerminalCommand } from "@/engine/adminTerminal";
import { PolicyEngine } from "@/engine/policyEngine";
import { PrivacyEngine } from "@/engine/privacyEngine";

// Testing & Simulation
import { ChaosTestingEngine, type TestSuite } from "@/engine/chaosTesting";
import { BenchmarkHarness } from "@/engine/benchmarkHarness";

// Monitoring
import { SensorQualityMonitor } from "@/engine/sensorQuality";
import { PerformanceMonitor } from "@/engine/performanceMonitor";
import { BatteryOptimizer } from "@/engine/batteryOptimizer";

// Navigation extensions
import { TunnelRecoveryEngine } from "@/engine/tunnelRecovery";
import { UrbanCanyonDetector } from "@/engine/urbanCanyon";
import { RerouteEngine } from "@/engine/rerouteEngine";
import { ETAEngine } from "@/engine/etaEngine";
import { MultiConstellationEngine } from "@/engine/multiConstellation";
import { V2XEngine } from "@/engine/v2xEngine";
import { DigitalTwinEngine } from "@/engine/digitalTwin";

// Data
import { MultiSourceMapsEngine } from "@/engine/multiSourceMaps";
import { GeospatialIndex } from "@/engine/geospatialIndex";
import { AlertSystem } from "@/engine/alertSystem";
import { AccessibilityEngine } from "@/engine/accessibilityEngine";
import { CognitiveUIEngine } from "@/engine/cognitiveUI";
import { Tracer, getTracer } from "@/engine/observability";
import { TripReplayEngine } from "@/engine/tripReplay";
import { ReplayEngine } from "@/engine/replayEngine";
import { ARHudRenderer } from "@/engine/arHud";
import { getMultiProviderRouter } from "@/engine/multiProviderRouting";
import { CameraBridge } from "@/engine/cameraBridge";
import type { MultiProviderConfig } from "@/engine/multiProviderRouting";
import { getOSMRouter, type OSMRoutingGraph } from "@/engine/osmRouter";
import { getStreetViewEngine, type StreetViewEngine } from "@/engine/streetViewEngine";
import { POIEngine } from "@/engine/poiEngine";

// ─── Types ───

interface GANEEngineState {
  // FSM
  fsmState: FSMState;
  sensorHealth: SensorHealth | null;
  lastTransition: FSMTransition | null;
  transitionHistory: FSMTransition[];

  // UI Profile
  activeProfile: UIProfile;
  profileId: string;

  // Sensor Bridge
  sensorStatus: SensorStatus;
  sensorActive: boolean;

  // Navigation Manager
  navState: NavigationState | null;

  // WebSocket
  wsConnected: boolean;
  wsClientId: string | null;

  // Telemetry
  position: { lat: number; lon: number; alt: number } | null;
  heading: number;
  speed: number;
  confidence: number;
  satellites: number;

  // Anomalies received via WS
  nearbyAnomalies: Array<{
    type: string;
    lat: number;
    lon: number;
    severity: number;
    description?: string;
    timestamp: number;
  }>;

  // Delta updates received
  deltaUpdates: Array<{
    type: string;
    payload: Record<string, unknown>;
    timestamp: number;
  }>;

  // NeRF Renderer
  nerfStats: RenderStats | null;
  nerfBackend: string;

  // Predictive Intent
  predictions: PredictedDestination[];
  predictionAccuracy: number;

  // Admin Terminal
  terminalActive: boolean;
  lastTerminalCommand: TerminalCommand | null;

  // Chaos Testing
  chaosActive: boolean;
  chaosLastSuite: TestSuite | null;

  // Engine statuses
  engineStatuses: Record<string, boolean>;
}

interface GANEContextValue {
  state: GANEEngineState;

  // Core engines
  fsm: NavigationFSM;
  offlineStore: OfflineStore;
  nerfRenderer: WebGPUNerfRenderer;
  predictiveIntent: PredictiveIntentEngine;
  adminTerminal: AdminTerminal;
  chaosEngine: ChaosTestingEngine;

  // All engines (exposed for panels)
  engines: {
    sensorBridge: ReturnType<typeof getSensorBridge>;
    navigationManager: ReturnType<typeof getNavigationManager>;
    spatialAudio: SpatialAudioEngine;
    mlPrediction: MLPredictionEngine;
    policyEngine: PolicyEngine;
    privacyEngine: PrivacyEngine;
    benchmarkHarness: BenchmarkHarness;
    sensorQuality: SensorQualityMonitor;
    performanceMonitor: PerformanceMonitor;
    batteryOptimizer: BatteryOptimizer;
    tunnelRecovery: TunnelRecoveryEngine;
    urbanCanyon: UrbanCanyonDetector;
    rerouteEngine: RerouteEngine;
    etaEngine: ETAEngine;
    multiConstellation: MultiConstellationEngine;
    v2xEngine: V2XEngine;
    digitalTwin: DigitalTwinEngine;
    multiSourceMaps: MultiSourceMapsEngine;
    geospatialIndex: GeospatialIndex;
    alertSystem: AlertSystem;
    accessibilityEngine: AccessibilityEngine;
    cognitiveUI: CognitiveUIEngine;
    observability: Tracer;
    tripReplay: TripReplayEngine;
    replayEngine: ReplayEngine;
    arHud: ARHudRenderer;
    multiProviderRouter: ReturnType<typeof getMultiProviderRouter>;
    cameraBridge: CameraBridge;
    osmRouter: OSMRoutingGraph;
    streetView: StreetViewEngine;
    poiEngine: POIEngine;
  };

  // Actions
  setProfile: (profileId: string) => void;
  updateSensorHealth: (health: SensorHealth) => void;
  updatePosition: (lat: number, lon: number, alt?: number) => void;
  updateHeading: (heading: number) => void;
  updateSpeed: (speed: number) => void;
  reportAnomaly: (type: string, lat: number, lon: number, severity?: number, description?: string) => void;
  connectWS: (deviceId?: string) => void;
  disconnectWS: () => void;
  startSensors: () => Promise<void>;
  stopSensors: () => void;
  startCamera: () => Promise<boolean>;
  stopCamera: () => void;

  // Module actions
  executeTerminalCommand: (input: string) => Promise<TerminalCommand>;
  getPredictions: () => PredictedDestination[];
  runChaosTest: () => Promise<TestSuite>;
}

const GANEContext = createContext<GANEContextValue | null>(null);

// ─── Provider ───

export function GANEProvider({ children }: { children: ReactNode }) {
  // Core engine refs
  const fsmRef = useRef(new NavigationFSM());
  const offlineStoreRef = useRef<OfflineStore | null>(null);
  const nerfRef = useRef(new WebGPUNerfRenderer());
  const predictiveRef = useRef(new PredictiveIntentEngine());
  const terminalRef = useRef(new AdminTerminal());
  const chaosRef = useRef(new ChaosTestingEngine());

  // All engine refs
  const spatialAudioRef = useRef(new SpatialAudioEngine());
  const mlPredictionRef = useRef(new MLPredictionEngine());
  const policyRef = useRef(new PolicyEngine());
  const privacyRef = useRef(new PrivacyEngine());
  const benchmarkRef = useRef(new BenchmarkHarness());
  const sensorQualityRef = useRef(new SensorQualityMonitor());
  const perfMonitorRef = useRef(new PerformanceMonitor());
  const batteryRef = useRef(new BatteryOptimizer());
  const tunnelRef = useRef(new TunnelRecoveryEngine());
  const urbanCanyonRef = useRef(new UrbanCanyonDetector());
  const rerouteRef = useRef(new RerouteEngine());
  const etaRef = useRef(new ETAEngine());
  const multiConstRef = useRef(new MultiConstellationEngine());
  const v2xRef = useRef(new V2XEngine());
  const digitalTwinRef = useRef(new DigitalTwinEngine());
  const multiSourceMapsRef = useRef(new MultiSourceMapsEngine());
  const geospatialRef = useRef(new GeospatialIndex());
  const alertRef = useRef(new AlertSystem());
  const a11yRef = useRef(new AccessibilityEngine());
  const cognitiveRef = useRef(new CognitiveUIEngine());
  const observabilityRef = useRef(getTracer());
  const tripReplayRef = useRef(new TripReplayEngine());
  const replayRef = useRef(new ReplayEngine());
  const arHudRef = useRef(new ARHudRenderer({ fov: 60 }));
  const multiProviderRouterRef = useRef(getMultiProviderRouter());
  const cameraBridgeRef = useRef(new CameraBridge());
  const osmRouterRef = useRef(getOSMRouter());
  const streetViewRef = useRef(getStreetViewEngine());
  const poiEngineRef = useRef(new POIEngine());

  const [state, setState] = useState<GANEEngineState>({
    fsmState: 'BOOTING',
    sensorHealth: null,
    lastTransition: null,
    transitionHistory: [],
    activeProfile: getUIProfile('private'),
    profileId: 'private',
    sensorStatus: {
      geolocation: 'unavailable',
      deviceMotion: 'unavailable',
      deviceOrientation: 'unavailable',
      magnetometer: 'unavailable',
    },
    sensorActive: false,
    navState: null,
    wsConnected: false,
    wsClientId: null,
    position: null,
    heading: 0,
    speed: 0,
    confidence: 0,
    satellites: 0,
    nearbyAnomalies: [],
    deltaUpdates: [],
    nerfStats: null,
    nerfBackend: 'initializing',
    predictions: [],
    predictionAccuracy: 0,
    terminalActive: false,
    lastTerminalCommand: null,
    chaosActive: false,
    chaosLastSuite: null,
    engineStatuses: {},
  });

  // ─── Initialize NavigationManager + SensorBridge ───
  useEffect(() => {
    const navManager = getNavigationManager();
    const sensorBridge = getSensorBridge();

    // Wire NavigationManager state changes into React state
    navManager.setOnStateChange((navState) => {
      setState(prev => ({
        ...prev,
        navState,
        position: navState.position,
        heading: navState.heading,
        speed: navState.velocity,
        confidence: navState.confidence,
        satellites: navState.satellites,
      }));
    });

    // Wire NavigationManager telemetry to WebSocket
    navManager.setOnTelemetry((packet) => {
      if (wsClient.connected) {
        wsClient.sendBinaryTelemetry({
          lat: packet.lat,
          lon: packet.lon,
          alt: packet.alt,
          velocity: packet.velocity,
          heading: packet.heading,
          confidence: packet.confidence,
          satellites: packet.satellites,
          flags: 0xF,
        });
      }
    });

    // Wire SensorBridge status changes
    sensorBridge.setOnStatusChange((sensorStatus) => {
      setState(prev => ({ ...prev, sensorStatus }));
    });

    // Wire SensorBridge heading to NavigationManager state
    sensorBridge.setOnHeadingUpdate((heading) => {
      setState(prev => ({ ...prev, heading }));
    });

    // Initialize NavigationManager
    navManager.init().then(() => {
      setState(prev => ({
        ...prev,
        engineStatuses: {
          ...prev.engineStatuses,
          navigationManager: true,
          eskf: true,
          pdr: true,
          visualOdometry: true,
          spatialAudio: true,
        },
      }));
    }).catch(err => {
      console.warn('[GANE] NavigationManager init failed:', err);
    });

    // Start SensorBridge (DeviceMotion + DeviceOrientation only; geolocation comes from RealDataContext)
    sensorBridge.start().then((status) => {
      setState(prev => ({
        ...prev,
        sensorActive: true,
        sensorStatus: status,
        engineStatuses: {
          ...prev.engineStatuses,
          sensorBridge: true,
          deviceMotion: status.deviceMotion === 'active',
          deviceOrientation: status.deviceOrientation === 'active',
        },
      }));
    }).catch(err => {
      console.warn('[GANE] SensorBridge start failed:', err);
    });

    // Bridge: Listen for RealDataContext location updates via custom event
    // RealDataContext dispatches 'gane:location' events when GPS updates arrive
    const handleLocationEvent = (e: Event) => {
      const detail = (e as CustomEvent).detail;
      if (detail?.latitude != null && detail?.longitude != null) {
        sensorBridge.injectGNSS(
          detail.latitude,
          detail.longitude,
          detail.altitude ?? 0,
          detail.accuracy ?? 15,
          detail.speed ?? 0,
          detail.heading ?? 0,
          Date.now(),
        );
        setState(prev => ({
          ...prev,
          position: { lat: detail.latitude, lon: detail.longitude, alt: detail.altitude ?? 0 },
          engineStatuses: { ...prev.engineStatuses, geolocation: true },
        }));
      }
    };
    window.addEventListener('gane:location', handleLocationEvent);

    return () => {
      sensorBridge.stop();
      window.removeEventListener('gane:location', handleLocationEvent);
    };
  }, []);

  // ─── Initialize offline store ───
  useEffect(() => {
    const initStore = async () => {
      try {
        offlineStoreRef.current = getOfflineStore();
        await offlineStoreRef.current.init();
        setState(prev => ({
          ...prev,
          engineStatuses: { ...prev.engineStatuses, offlineStore: true },
        }));
      } catch (err) {
        console.warn('[GANE] Offline store init failed:', err);
      }
    };
    initStore();
  }, []);

  // ─── Initialize predictive intent engine ───
  useEffect(() => {
    predictiveRef.current.initialize().then(() => {
      setState(prev => ({
        ...prev,
        engineStatuses: { ...prev.engineStatuses, predictiveIntent: true },
      }));
    }).catch(err => {
      console.warn('[GANE] Predictive intent init failed:', err);
    });
  }, []);

  // ─── Initialize remaining engines ───
  useEffect(() => {
    // These engines initialize synchronously or have no-op init
    const engineNames = [
      'mlPrediction', 'policyEngine', 'privacyEngine', 'benchmarkHarness',
      'sensorQuality', 'performanceMonitor', 'batteryOptimizer',
      'tunnelRecovery', 'urbanCanyon', 'rerouteEngine', 'etaEngine',
      'multiConstellation', 'v2xEngine', 'digitalTwin',
      'multiSourceMaps', 'geospatialIndex', 'alertSystem',
      'accessibilityEngine', 'cognitiveUI', 'observability',
      'tripReplay', 'replayEngine',
    ];

    setState(prev => ({
      ...prev,
      engineStatuses: {
        ...prev.engineStatuses,
        ...Object.fromEntries(engineNames.map(name => [name, true])),
      },
    }));
  }, []);

  // ─── FSM transition listener ───
  useEffect(() => {
    const unsub = fsmRef.current.onTransition((transition, newState) => {
      setState(prev => ({
        ...prev,
        fsmState: newState,
        lastTransition: transition,
        transitionHistory: [...prev.transitionHistory.slice(-49), transition],
      }));
    });
    return unsub;
  }, []);

  // ─── WebSocket event listeners ───
  useEffect(() => {
    const unsubConnected = wsClient.on('connected', () => {
      setState(prev => ({ ...prev, wsConnected: true }));
    });

    const unsubDisconnected = wsClient.on('disconnected', () => {
      setState(prev => ({ ...prev, wsConnected: false, wsClientId: null }));
    });

    const unsubWelcome = wsClient.on('welcome', (msg) => {
      setState(prev => ({
        ...prev,
        wsClientId: msg.payload.clientId as string,
      }));
    });

    const unsubAnomaly = wsClient.on('anomaly_alert', (msg) => {
      const anomaly = {
        type: msg.payload.anomalyType as string,
        lat: msg.payload.lat as number,
        lon: msg.payload.lon as number,
        severity: msg.payload.severity as number,
        description: msg.payload.description as string | undefined,
        timestamp: msg.payload.timestamp as number,
      };
      setState(prev => ({
        ...prev,
        nearbyAnomalies: [...prev.nearbyAnomalies.slice(-49), anomaly],
      }));

      // Feed anomaly to alert system
      alertRef.current.push({
        type: anomaly.type,
        priority: anomaly.severity >= 3 ? 'critical' : anomaly.severity >= 2 ? 'warning' : 'info',
        title: anomaly.type,
        message: anomaly.description || anomaly.type,
      });
    });

    const unsubDelta = wsClient.on('delta_update', (msg) => {
      setState(prev => ({
        ...prev,
        deltaUpdates: [...prev.deltaUpdates.slice(-49), {
          type: 'delta',
          payload: msg.payload,
          timestamp: Date.now(),
        }],
      }));
    });

    return () => {
      unsubConnected();
      unsubDisconnected();
      unsubWelcome();
      unsubAnomaly();
      unsubDelta();
    };
  }, []);

  // ─── Admin terminal listener ───
  useEffect(() => {
    const unsub = terminalRef.current.onCommand((cmd) => {
      setState(prev => ({ ...prev, lastTerminalCommand: cmd }));
    });
    return unsub;
  }, []);

  // ─── Auto-boot FSM after 3 seconds ───
  useEffect(() => {
    const timer = setTimeout(() => {
      const initialHealth: SensorHealth = {
        gnss: true,
        imu: true,
        vision: false,
        network: navigator.onLine,
        gnssAccuracy: 15,
        confidenceScore: 0.75,
        satellites: 8,
        spoofingDetected: false,
        lastGNSSFix: Date.now(),
      };
      fsmRef.current.update(initialHealth);
    }, 3500);
    return () => clearTimeout(timer);
  }, []);

  // ─── Cleanup on unmount ───
  useEffect(() => {
    return () => {
      nerfRef.current.destroy();
      predictiveRef.current.destroy();
      terminalRef.current.destroy();
      chaosRef.current.destroy();
      batteryRef.current.destroy();
      perfMonitorRef.current.destroy();
    };
  }, []);

  // ─── Actions ───

  const setProfile = useCallback((profileId: string) => {
    setState(prev => ({
      ...prev,
      profileId,
      activeProfile: getUIProfile(profileId),
    }));
    getNavigationManager().setUIProfile(profileId as 'private' | 'sports' | 'ems' | 'logistics');
  }, []);

  const updateSensorHealth = useCallback((health: SensorHealth) => {
    fsmRef.current.update(health);
    setState(prev => ({ ...prev, sensorHealth: health }));
  }, []);

  const updatePosition = useCallback((lat: number, lon: number, alt: number = 0) => {
    setState(prev => ({ ...prev, position: { lat, lon, alt } }));

    // Send via WebSocket if connected
    if (wsClient.connected) {
      wsClient.sendBinaryTelemetry({
        lat, lon, alt,
        velocity: state.speed,
        heading: state.heading,
        confidence: state.confidence,
        satellites: state.satellites,
        flags: 0xF,
      });
    }
  }, [state.speed, state.heading, state.confidence, state.satellites]);

  const updateHeading = useCallback((heading: number) => {
    setState(prev => ({ ...prev, heading }));
  }, []);

  const updateSpeed = useCallback((speed: number) => {
    setState(prev => ({ ...prev, speed }));
  }, []);

  const reportAnomaly = useCallback((type: string, lat: number, lon: number, severity: number = 1, description?: string) => {
    wsClient.reportAnomaly(type, lat, lon, severity, description);
  }, []);

  const connectWS = useCallback((deviceId?: string) => {
    wsClient.connect(deviceId);
  }, []);

  const disconnectWS = useCallback(() => {
    wsClient.disconnect();
  }, []);

  const startSensors = useCallback(async () => {
    const bridge = getSensorBridge();
    const status = await bridge.start();
    setState(prev => ({ ...prev, sensorActive: true, sensorStatus: status }));
  }, []);

  const stopSensors = useCallback(() => {
    const bridge = getSensorBridge();
    bridge.stop();
    setState(prev => ({ ...prev, sensorActive: false }));
  }, []);

  // Camera actions
  const startCamera = useCallback(async (): Promise<boolean> => {
    const cam = cameraBridgeRef.current;
    // Connect VO engine from NavigationManager
    const navManager = getNavigationManager();
    const voEngine = navManager.getVOEngine();
    if (voEngine) {
      cam.connectVO(voEngine);
    }
    const ok = await cam.start();
    if (ok) {
      setState(prev => ({
        ...prev,
        engineStatuses: { ...prev.engineStatuses, camera: true, visualOdometry: true },
      }));
    }
    return ok;
  }, []);

  const stopCamera = useCallback(() => {
    cameraBridgeRef.current.stop();
    setState(prev => ({
      ...prev,
      engineStatuses: { ...prev.engineStatuses, camera: false },
    }));
  }, []);

  // Module actions
  const executeTerminalCommand = useCallback(async (input: string): Promise<TerminalCommand> => {
    setState(prev => ({ ...prev, terminalActive: true }));
    const result = await terminalRef.current.execute(input);
    setState(prev => ({ ...prev, lastTerminalCommand: result }));
    return result;
  }, []);

  const getPredictions = useCallback((): PredictedDestination[] => {
    if (!state.position) return [];
    const predictions = predictiveRef.current.predict({
      currentLat: state.position.lat,
      currentLng: state.position.lon,
      currentHeading: state.heading,
      currentSpeed: state.speed,
      timestamp: Date.now(),
      recentSearches: [],
    });
    setState(prev => ({
      ...prev,
      predictions,
      predictionAccuracy: predictiveRef.current.getStats().predictionAccuracy,
    }));
    return predictions;
  }, [state.position, state.heading, state.speed]);

  const runChaosTest = useCallback(async (): Promise<TestSuite> => {
    setState(prev => ({ ...prev, chaosActive: true }));
    const suite = await chaosRef.current.runSuite();
    setState(prev => ({
      ...prev,
      chaosActive: false,
      chaosLastSuite: suite,
    }));
    return suite;
  }, []);

  const value: GANEContextValue = {
    state,
    fsm: fsmRef.current,
    offlineStore: offlineStoreRef.current ?? getOfflineStore(),
    nerfRenderer: nerfRef.current,
    predictiveIntent: predictiveRef.current,
    adminTerminal: terminalRef.current,
    chaosEngine: chaosRef.current,

    engines: {
      sensorBridge: getSensorBridge(),
      navigationManager: getNavigationManager(),
      spatialAudio: spatialAudioRef.current,
      mlPrediction: mlPredictionRef.current,
      policyEngine: policyRef.current,
      privacyEngine: privacyRef.current,
      benchmarkHarness: benchmarkRef.current,
      sensorQuality: sensorQualityRef.current,
      performanceMonitor: perfMonitorRef.current,
      batteryOptimizer: batteryRef.current,
      tunnelRecovery: tunnelRef.current,
      urbanCanyon: urbanCanyonRef.current,
      rerouteEngine: rerouteRef.current,
      etaEngine: etaRef.current,
      multiConstellation: multiConstRef.current,
      v2xEngine: v2xRef.current,
      digitalTwin: digitalTwinRef.current,
      multiSourceMaps: multiSourceMapsRef.current,
      geospatialIndex: geospatialRef.current,
      alertSystem: alertRef.current,
      accessibilityEngine: a11yRef.current,
      cognitiveUI: cognitiveRef.current,
      observability: observabilityRef.current,
      tripReplay: tripReplayRef.current,
      replayEngine: replayRef.current,
      arHud: arHudRef.current,
      multiProviderRouter: multiProviderRouterRef.current,
      cameraBridge: cameraBridgeRef.current,
      osmRouter: osmRouterRef.current,
      streetView: streetViewRef.current,
      poiEngine: poiEngineRef.current,
    },

    setProfile,
    updateSensorHealth,
    updatePosition,
    updateHeading,
    updateSpeed,
    reportAnomaly,
    connectWS,
    disconnectWS,
    startSensors,
    stopSensors,
    startCamera,
    stopCamera,
    executeTerminalCommand,
    getPredictions,
    runChaosTest,
  };

  return (
    <GANEContext.Provider value={value}>
      {children}
    </GANEContext.Provider>
  );
}

// ─── Hooks ───

export function useGANE() {
  const ctx = useContext(GANEContext);
  if (!ctx) throw new Error("useGANE must be used within GANEProvider");
  return ctx;
}

export function useGANEState() {
  const { state } = useGANE();
  return state;
}

export function useGANEActions() {
  const {
    setProfile, updateSensorHealth, updatePosition, updateHeading,
    updateSpeed, reportAnomaly, connectWS, disconnectWS,
    startSensors, stopSensors,
    executeTerminalCommand, getPredictions, runChaosTest,
  } = useGANE();
  return {
    setProfile, updateSensorHealth, updatePosition, updateHeading,
    updateSpeed, reportAnomaly, connectWS, disconnectWS,
    startSensors, stopSensors,
    executeTerminalCommand, getPredictions, runChaosTest,
  };
}

export function useGANEEngines() {
  const { engines } = useGANE();
  return engines;
}
