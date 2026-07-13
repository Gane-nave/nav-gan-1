/**
 * G.A.N.E Engine — Barrel Export
 * ================================
 * All navigation engine modules in one import.
 */

// Core Navigation
export { ESKFEngine, geodeticToENU, enuToGeodetic } from './eskf';
export type { Vec3, Quaternion, GNSSMeasurement, IMUMeasurement, VisionMeasurement, ESKFState } from './eskf';

// Dead Reckoning
export { PDREngine } from './pdr';
export type { PDRConfig, PDRState, RoadSegment } from './pdr';

// Visual Odometry
export { VisualOdometryEngine } from './visualOdometry';
export type { VOConfig, VOResult, VOState } from './visualOdometry';

// Spatial Audio
export { SpatialAudioEngine } from './spatialAudio';
export type { SpatialAudioConfig, AudioCue } from './spatialAudio';

// Navigation Manager (orchestrator)
export { NavigationManager, getNavigationManager, resetNavigationManager } from './navigationManager';
export type { NavigationMode, NavigationEvent, TelemetryPacket, NavigationState, RouteInfo, RouteStep } from './navigationManager';

// Offline Store
export { OfflineStore, getOfflineStore, lwwMerge, gCounterIncrement, gCounterMerge, gCounterValue } from './offlineStore';
export type { LWWRegister, GCounter, CRDTDocument } from './offlineStore';

// FSM State Machine
export { NavigationFSM, getUIProfile, getFSMStateInfo, UI_PROFILES } from './fsm';
export type { FSMState, SensorHealth, FSMTransition, FSMConfig, FSMEventCallback } from './fsm';
export type { UIProfile } from './fsm';

// WebSocket Client
export { wsClient } from './wsClient';

// AR HUD
export { ARHudRenderer } from './arHud';
export type { ARHudConfig, ARWaypoint, ARHazard } from './arHud';

// Cognitive UI (Polymorphic Interface)
export { CognitiveUIEngine, getCognitiveUI } from './cognitiveUI';
export type { ColorPalette, HUDLayout, AudioProfile, PanelVisibility, CognitiveUIState, DrivingContext } from './cognitiveUI';

// Multi-Source Maps
export { MultiSourceMapsEngine, getMultiSourceMaps } from './multiSourceMaps';
export type { MapSource, TileCoord, CachedTile, MapDelta } from './multiSourceMaps';

// WebGPU / NeRF 3D Rendering
export { WebGPUNerfRenderer } from './webgpuNerf';
export type { GaussianSplat, NeRFScene, RenderConfig, CameraState, RenderStats } from './webgpuNerf';

// Predictive Intent Recognition
export { PredictiveIntentEngine } from './predictiveIntent';
export type { PredictedDestination, PredictionReason, DestinationCategory, CalendarEvent, VisitRecord, UserContext } from './predictiveIntent';

// Super Admin AI Chat Terminal
export { AdminTerminal } from './adminTerminal';
export type { TerminalCommand, ParsedCommand, CommandResult, CommandDomain, TerminalSession, SystemHealthReport } from './adminTerminal';

// Chaos Testing Engine
export { ChaosTestingEngine } from './chaosTesting';
export type { ChaosTest, ChaosCategory, TestStatus, TestResult, ChaosConfig, TestSuite } from './chaosTesting';

// Multi-Constellation GNSS (NavIC + QZSS)
export { MultiConstellationEngine } from './multiConstellation';
export type { ConstellationId, SatelliteInfo, ConstellationStatus, MultiConstellationState, GNSSPosition } from './multiConstellation';

// Tunnel Recovery (Dead Reckoning during GNSS blackout)
export { TunnelRecoveryEngine } from './tunnelRecovery';
export type { TunnelState } from './tunnelRecovery';

// Urban Canyon + Map Matching + Route Graph
export { UrbanCanyonDetector, MapMatchingEngine, RouteGraphEngine } from './urbanCanyon';
export type { UrbanCanyonState, MapMatchResult, RoadNode, RoadEdge, EnvironmentClass, RouteObjective } from './urbanCanyon';

// Trip Replay System
export { TripReplayEngine, tripReplayEngine } from './tripReplay';
export type { ReplayPoint, ReplayEvent, ReplayData, ReplayState, ReplayFrame } from './tripReplay';

// Traffic Pipeline is server-side only (server/gane/trafficPipeline.ts)

// Real-Time Reroute Engine
export { RerouteEngine } from './rerouteEngine';
export type { RerouteConfig, RouteAlternative, RerouteResult, RerouteState, RerouteReason } from './rerouteEngine';

// ETA Correction Engine
export { ETAEngine } from './etaEngine';
export type { ETAConfig, ETAResult, ETAState, SegmentETA } from './etaEngine';

// Offline Sync Engine (CRDT-based)
export { OfflineSyncEngine } from './offlineSync';
export type { OfflineConfig, SyncStatus, SyncQueueItem, StorageStats } from './offlineSync';

// Battery Optimizer Engine
export { BatteryOptimizer } from './batteryOptimizer';
export type { BatteryConfig, BatteryState, PowerMode } from './batteryOptimizer';

// Privacy Engine (GDPR/CCPA Compliance)
export { PrivacyEngine } from './privacyEngine';
export type { PrivacyConfig, PrivacyState, PrivacyLevel, AnonymizedLocation } from './privacyEngine';

// ML Prediction Engine
export { MLPredictionEngine } from './mlPrediction';
export type { PredictionConfig, PredictionState, SpeedPrediction, CongestionPrediction, AnomalyScore } from './mlPrediction';

// Alert System Engine
export { AlertSystem, ALERT_TEMPLATES } from './alertSystem';
export type { AlertConfig, Alert, AlertState, AlertPriority, AlertChannel } from './alertSystem';

// Geospatial Indexing Engine
export { GeospatialIndex } from './geospatialIndex';
export type { SpatialPoint, BoundingBox, NearbyResult } from './geospatialIndex';

// Sensor Quality Monitor
export { SensorQualityMonitor } from './sensorQuality';
export type { SensorType, SensorStatus, SensorReading, SensorQualityMetrics, FusionQuality } from './sensorQuality';

// V2X Communication Engine
export { V2XEngine } from './v2xEngine';
export type { V2XProtocol, V2XMessageType, V2XMessage, BSMPayload, SPaTPayload, TIMPayload, V2XConfig, V2XState } from './v2xEngine';

// Digital Twin Engine
export { DigitalTwinEngine } from './digitalTwin';
export type { TwinNode, TwinEdge, TwinVehicle, TwinEnvironment, SimulationScenario, SimulationResult, TwinState } from './digitalTwin';

// Accessibility Engine
export { AccessibilityEngine } from './accessibilityEngine';
export type { ColorBlindMode, TextSize, VoiceSpeed, AccessibilityConfig, AccessibilityState, Announcement } from './accessibilityEngine';

// Performance Monitor Engine
export { PerformanceMonitor } from './performanceMonitor';
export type { PerfMetrics, PerfBudget, PerfSnapshot, QualityLevel, QualitySettings } from './performanceMonitor';
// Observability Stack (OpenTelemetry-compatible)
export { Tracer, getTracer, startSpan, trace, incrementCounter, recordHistogram, extractTraceContext, createTraceparent } from './observability';
export type { SpanKind, SpanStatus, SpanContext, SpanAttributes, SpanEvent, Span, ActiveSpan, MetricType, MetricPoint, LogEntry, ObservabilityConfig } from './observability';

// Event Bus Abstraction (typed, idempotent)
export { EventBus, getEventBus, emit, on, createLoggingMiddleware, createPriorityFilter, createRateLimiter } from './eventBus';
export type { EventPriority, BaseEvent, GANEEvent, EventType, EventPayload, EventHandler, EventMiddleware, Subscription, DeadLetterEntry, EventBusConfig } from './eventBus';

// Policy Engine (DSL, emergency overrides, compliance)
export { PolicyEngine, getPolicyEngine, resetPolicyEngine, parseDSL } from './policyEngine';
export type { PolicyRule, PolicyContext, PolicyAction, PolicyEvalResult, RuleEvaluation, PolicyAuditEntry, PolicyEngineConfig, ComparisonOp, LogicalOp, RuleAction, RulePriority, VehicleClass, ComplianceRegion, Condition, ConditionGroup } from './policyEngine';

// Multi-Provider Routing (Mapbox/HERE/TomTom fallback)
export { MultiProviderRouter, getMultiProviderRouter, resetMultiProviderRouter } from './multiProviderRouting';
export type { ProviderId, ProviderStatus, RouteProfile, AvoidFeature, RouteRequest, RouteStep as MultiProviderRouteStep, RouteLeg, RouteResult, MultiRouteResult, ProviderConfig, ProviderHealth, MultiProviderConfig } from './multiProviderRouting';

// Benchmark Harness (automated regression testing)
export { BenchmarkHarness, getBenchmarkHarness, resetBenchmarkHarness, createGANEBenchmarks } from './benchmarkHarness';
export type { BenchmarkDef, BenchmarkResult, BenchmarkStats, BenchmarkStatus, Baseline, RegressionResult, SuiteResult, TrendPoint, HarnessConfig, RegressionSeverity } from './benchmarkHarness';

// Replay System Engine (event loader, timeline reconstructor, determinism checker)
export { ReplayEngine, getReplayEngine, resetReplayEngine, EventLoader, TimelineReconstructor, FrameInterpolator, DeterminismChecker, PlaybackController, AnnotationManager } from './replayEngine';
export type { ReplayEventType, ReplayEventRecord, ReplayTimeline, PlaybackState, InterpolatedFrame, ReplayAnnotation, DeterminismResult, DeterminismMismatch, ReplayConfig } from './replayEngine';

// Simulation Framework Engine (GNSS/IMU/traffic/network/weather injectors)
export { SimulationRunner, getSimulationRunner, resetSimulationRunner, SeededRandom, GNSSInjector, IMUInjector, TrafficInjector, NetworkInjector, WeatherInjector, BUILTIN_SCENARIOS } from './simulationEngine';
export type { SimulationConfig, GNSSConfig, GNSSFix, IMUConfig, IMUReading, TrafficConfig, TrafficState as SimTrafficState, NetworkConfig, NetworkState, WeatherCondition, WeatherConfig, WeatherState, ScenarioWaypoint, ScenarioCondition, AcceptanceCriterion, SimulationScenario as SimScenario, SimulationResults, InjectorType } from './simulationEngine';

// Position Fallback Chain (Unbreakable Navigation)
export { PositionFallbackChain, getPositionFallbackChain, destroyPositionFallbackChain } from './positionFallbackChain';
export type { ProviderTier, ProviderId as FallbackProviderId, PositionFix, ProviderHealth as FallbackProviderHealth, FallbackChainState } from './positionFallbackChain';

// VHF/UHF/HF Radio Communications Engine
export { RadioCommsEngine, getRadioEngine, EMERGENCY_FREQUENCIES, STANDARD_FREQUENCIES } from './radioCommsEngine';
export type { RadioBand, RadioState, RadioFrequency, RadioChannel, APRSPacket, MeshNode, RadioTransmission, RadioStatus, ModulationType } from './radioCommsEngine';

// Real-Time Collaboration Engine (WebRTC P2P + SSE)
export { getRealtimeCollabEngine } from './realtimeCollabEngine';
export type { RealtimeCollabEngine, RealtimeCollabState, TeamMember, CollabSession, Geofence, CollabAlert, ConnectionTier } from './realtimeCollabEngine';

// Camera Bridge (AR/StreetView camera access)
export { CameraBridge } from './cameraBridge';
export type { CameraBridgeConfig, CameraState as CameraBridgeState } from './cameraBridge';

// Offline Map Engine (tile caching + offline routing)
export { OfflineMapEngine } from './offlineMapEngine';
export type { TileProvider, CachedTile as OfflineCachedTile } from './offlineMapEngine';

// OSM Routing Graph (offline OpenStreetMap routing)
export { OSMRoutingGraph } from './osmRouter';
export type { OSMNode, OSMWay, GraphNode } from './osmRouter';

// POI Engine (Points of Interest search)
export { POIEngine } from './poiEngine';
export type { POIResult, POICategory, POISearchParams } from './poiEngine';

// Sensor Bridge (hardware sensor access)
export { SensorBridge } from './sensorBridge';
export type { SensorBridgeConfig, SensorStatus as SensorBridgeStatus } from './sensorBridge';

// Street View Engine (panoramic imagery)
export { StreetViewEngine } from './streetViewEngine';
export type { StreetImage, StreetSequence, StreetViewState } from './streetViewEngine';
