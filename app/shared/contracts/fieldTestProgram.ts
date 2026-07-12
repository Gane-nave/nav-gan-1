/**
 * G.A.N.E — Field Test Program + Acceptance Criteria + Build Verification Gates
 * 
 * Covers:
 * - Field test scenarios across 12 environment categories
 * - Acceptance criteria per subsystem (positioning, fusion, routing, alerts, payments, AI)
 * - Build verification gates (unit → integration → contract → simulation → replay → load → chaos → security)
 * - Test device matrix (low-end to high-end, car head units)
 */

// ═══════════════════════════════════════════════════════════
// FIELD TEST PROGRAM
// ═══════════════════════════════════════════════════════════

export type TestEnvironment =
  | 'urban_canyon'
  | 'tunnel'
  | 'highway'
  | 'rural_dead_zone'
  | 'dense_rain'
  | 'spoof_lab'
  | 'degraded_sensors'
  | 'low_end_phone'
  | 'car_head_unit'
  | 'multi_floor_parking'
  | 'border_crossing'
  | 'mountain_pass';

export type TestPriority = 'P0_critical' | 'P1_high' | 'P2_medium' | 'P3_low';

export interface FieldTestScenario {
  id: string;
  environment: TestEnvironment;
  name: string;
  description: string;
  priority: TestPriority;
  preconditions: string[];
  steps: string[];
  expectedOutcomes: string[];
  passCriteria: PassCriterion[];
  duration_minutes: number;
  requiredEquipment: string[];
  safetyNotes: string[];
}

export interface PassCriterion {
  metric: string;
  operator: '<' | '<=' | '>' | '>=' | '==' | 'within';
  value: number;
  unit: string;
  tolerance?: number;
}

export const FIELD_TEST_SCENARIOS: FieldTestScenario[] = [
  {
    id: 'FT-001',
    environment: 'urban_canyon',
    name: 'Urban Canyon GNSS Degradation',
    description: 'Navigate through dense urban canyon (buildings >30m) with intermittent GNSS signal loss. Verify fusion engine maintains position within acceptable drift.',
    priority: 'P0_critical',
    preconditions: [
      'Device has calibrated IMU',
      'Offline map package loaded for test area',
      'Route pre-computed with known ground truth',
    ],
    steps: [
      'Start navigation on known route through downtown corridor',
      'Enter canyon zone where GNSS C/N0 drops below 25 dB-Hz',
      'Continue for 2km through canyon',
      'Exit canyon and verify GNSS reacquisition',
      'Compare recorded trajectory against ground truth',
    ],
    expectedOutcomes: [
      'Position maintained via fusion (IMU + map matching)',
      'Drift bounded to <15m over 2km canyon segment',
      'Smooth transition back to GNSS on canyon exit',
      'No false reroute triggered during canyon transit',
    ],
    passCriteria: [
      { metric: 'max_drift_m', operator: '<', value: 15, unit: 'meters' },
      { metric: 'gnss_reacquisition_time_s', operator: '<', value: 10, unit: 'seconds' },
      { metric: 'false_reroute_count', operator: '==', value: 0, unit: 'count' },
      { metric: 'position_confidence_min', operator: '>', value: 0.6, unit: 'ratio' },
    ],
    duration_minutes: 45,
    requiredEquipment: ['Test vehicle', 'Reference GNSS receiver (RTK)', 'Dashcam'],
    safetyNotes: ['Driver must not interact with device', 'Co-driver operates test equipment'],
  },
  {
    id: 'FT-002',
    environment: 'tunnel',
    name: 'Extended Tunnel Navigation',
    description: 'Navigate through tunnel >1km with complete GNSS blackout. Verify dead reckoning maintains usable position.',
    priority: 'P0_critical',
    preconditions: [
      'Tunnel map data available with lane geometry',
      'IMU calibrated within last 5 minutes',
      'Speed sensor active',
    ],
    steps: [
      'Approach tunnel at highway speed',
      'Enter tunnel — GNSS signal lost completely',
      'Continue through tunnel for full length',
      'Exit tunnel — GNSS reacquires',
      'Verify position accuracy at tunnel exit',
    ],
    expectedOutcomes: [
      'Seamless transition to dead reckoning mode',
      'Position maintained via IMU + odometry + tunnel map',
      'Exit position error <20m',
      'No navigation interruption visible to user',
    ],
    passCriteria: [
      { metric: 'exit_position_error_m', operator: '<', value: 20, unit: 'meters' },
      { metric: 'mode_transition_latency_ms', operator: '<', value: 500, unit: 'ms' },
      { metric: 'user_visible_interruption', operator: '==', value: 0, unit: 'count' },
    ],
    duration_minutes: 30,
    requiredEquipment: ['Test vehicle', 'Reference IMU', 'Video recorder'],
    safetyNotes: ['Maintain safe following distance', 'Do not stop in tunnel'],
  },
  {
    id: 'FT-003',
    environment: 'highway',
    name: 'Highway High-Speed Navigation',
    description: 'Navigate at 120km/h+ on highway. Verify real-time guidance, lane-level accuracy, and timely reroute on incidents.',
    priority: 'P0_critical',
    preconditions: [
      'Multi-constellation GNSS active',
      'Traffic data feed active',
      'Route with known interchange complexity',
    ],
    steps: [
      'Start highway navigation at speed',
      'Pass through 3+ interchanges',
      'Simulate incident ahead (inject via test harness)',
      'Verify reroute computation and guidance update',
      'Complete alternate route',
    ],
    expectedOutcomes: [
      'Lane-level guidance accurate at interchanges',
      'Reroute computed within 500ms of incident detection',
      'Voice guidance timely (>300m before maneuver at 120km/h)',
      'No missed exits',
    ],
    passCriteria: [
      { metric: 'reroute_latency_ms', operator: '<', value: 500, unit: 'ms' },
      { metric: 'guidance_advance_distance_m', operator: '>', value: 300, unit: 'meters' },
      { metric: 'missed_exit_count', operator: '==', value: 0, unit: 'count' },
      { metric: 'position_accuracy_m', operator: '<', value: 3, unit: 'meters' },
    ],
    duration_minutes: 60,
    requiredEquipment: ['Test vehicle', 'RTK reference', 'Test harness for incident injection'],
    safetyNotes: ['Obey speed limits', 'Professional test driver required'],
  },
  {
    id: 'FT-004',
    environment: 'rural_dead_zone',
    name: 'Rural Network Dead Zone',
    description: 'Navigate in area with no cellular coverage. Verify offline routing, cached maps, and sync on reconnection.',
    priority: 'P1_high',
    preconditions: [
      'Offline map package downloaded for test area',
      'Cached route available',
      'Device in airplane mode (simulated dead zone)',
    ],
    steps: [
      'Start navigation with network available',
      'Enter dead zone (disable network)',
      'Continue navigation for 30+ minutes offline',
      'Verify all guidance continues normally',
      'Re-enable network and verify sync',
    ],
    expectedOutcomes: [
      'Navigation continues seamlessly offline',
      'Cached traffic data used for ETA',
      'Telemetry queued locally',
      'Delta sync completes within 30s of reconnection',
    ],
    passCriteria: [
      { metric: 'offline_navigation_continuity', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'sync_completion_time_s', operator: '<', value: 30, unit: 'seconds' },
      { metric: 'queued_events_lost', operator: '==', value: 0, unit: 'count' },
    ],
    duration_minutes: 60,
    requiredEquipment: ['Test vehicle', 'Faraday bag (optional)'],
    safetyNotes: ['Ensure emergency communication available via separate device'],
  },
  {
    id: 'FT-005',
    environment: 'dense_rain',
    name: 'Heavy Rain Navigation',
    description: 'Navigate during heavy rainfall. Verify weather-aware routing, reduced speed recommendations, and sensor resilience.',
    priority: 'P1_high',
    preconditions: [
      'Weather data feed active',
      'Camera-based features active',
      'Route through flood-prone areas',
    ],
    steps: [
      'Start navigation during heavy rain',
      'Verify weather overlay on map',
      'Check if route avoids known flood zones',
      'Verify camera-based features degrade gracefully',
      'Complete route with weather-adjusted ETA',
    ],
    expectedOutcomes: [
      'Weather overlay visible and accurate',
      'ETA adjusted for weather conditions',
      'Camera features disabled or degraded gracefully',
      'No false hazard alerts from rain noise',
    ],
    passCriteria: [
      { metric: 'eta_weather_adjustment_applied', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'false_hazard_alerts', operator: '==', value: 0, unit: 'count' },
      { metric: 'camera_graceful_degradation', operator: '==', value: 1, unit: 'boolean' },
    ],
    duration_minutes: 45,
    requiredEquipment: ['Test vehicle', 'Weather monitoring equipment'],
    safetyNotes: ['Do not drive through standing water', 'Reduce speed as conditions require'],
  },
  {
    id: 'FT-006',
    environment: 'spoof_lab',
    name: 'GNSS Spoofing Detection',
    description: 'Controlled lab test with GNSS signal spoofing. Verify detection, alerting, and fallback to alternate positioning.',
    priority: 'P0_critical',
    preconditions: [
      'Spoofing simulator configured',
      'Multi-constellation receiver active',
      'Integrity engine enabled',
    ],
    steps: [
      'Establish normal GNSS fix',
      'Inject spoofed signals (gradual drift)',
      'Verify spoofing detection triggers',
      'Verify fallback to non-spoofed sources',
      'Remove spoofing and verify recovery',
    ],
    expectedOutcomes: [
      'Spoofing detected within 5 seconds',
      'Alert generated to user',
      'Position falls back to IMU + map matching',
      'Spoofed position never used for routing',
    ],
    passCriteria: [
      { metric: 'spoof_detection_time_s', operator: '<', value: 5, unit: 'seconds' },
      { metric: 'spoofed_position_used', operator: '==', value: 0, unit: 'boolean' },
      { metric: 'fallback_activated', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'recovery_time_s', operator: '<', value: 10, unit: 'seconds' },
    ],
    duration_minutes: 90,
    requiredEquipment: ['GNSS simulator', 'RF shielded room', 'Reference receiver'],
    safetyNotes: ['Lab environment only — never spoof in open air'],
  },
  {
    id: 'FT-007',
    environment: 'degraded_sensors',
    name: 'Degraded Sensor Operation',
    description: 'Navigate with intentionally degraded sensors (covered camera, noisy IMU, single GNSS constellation).',
    priority: 'P1_high',
    preconditions: [
      'Sensor quality monitor active',
      'Fusion engine configured for degraded mode',
    ],
    steps: [
      'Cover camera sensor',
      'Add noise to IMU (vibration mount)',
      'Disable all constellations except GPS',
      'Navigate known route for 20 minutes',
      'Verify graceful degradation at each step',
    ],
    expectedOutcomes: [
      'Camera features disabled with user notification',
      'IMU noise detected, weight reduced in fusion',
      'Single constellation still provides usable position',
      'Overall navigation continues with reduced confidence',
    ],
    passCriteria: [
      { metric: 'navigation_continuity', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'sensor_degradation_detected', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'confidence_reduction_reported', operator: '==', value: 1, unit: 'boolean' },
    ],
    duration_minutes: 45,
    requiredEquipment: ['Sensor covers', 'Vibration mount', 'GNSS constellation filter'],
    safetyNotes: ['Maintain visual contact with road at all times'],
  },
  {
    id: 'FT-008',
    environment: 'low_end_phone',
    name: 'Low-End Device Performance',
    description: 'Run full navigation on budget device (2GB RAM, old SoC). Verify performance budgets met.',
    priority: 'P1_high',
    preconditions: [
      'Budget device (Android Go or equivalent)',
      'App installed with offline package',
      'Performance monitor active',
    ],
    steps: [
      'Launch app and measure cold start time',
      'Start navigation and measure frame rate',
      'Run for 30 minutes and measure memory usage',
      'Verify battery consumption rate',
      'Check thermal throttling behavior',
    ],
    expectedOutcomes: [
      'Cold start <5 seconds',
      'Frame rate >30fps during navigation',
      'Memory usage <300MB',
      'No thermal throttling during normal use',
      'Battery drain <10% per hour',
    ],
    passCriteria: [
      { metric: 'cold_start_time_s', operator: '<', value: 5, unit: 'seconds' },
      { metric: 'min_fps', operator: '>', value: 30, unit: 'fps' },
      { metric: 'max_memory_mb', operator: '<', value: 300, unit: 'MB' },
      { metric: 'battery_drain_pct_per_hour', operator: '<', value: 10, unit: 'percent' },
    ],
    duration_minutes: 60,
    requiredEquipment: ['Budget Android device', 'Battery monitor', 'Thermal camera'],
    safetyNotes: [],
  },
  {
    id: 'FT-009',
    environment: 'car_head_unit',
    name: 'Car Head Unit Integration',
    description: 'Test on automotive head unit (Android Auto / CarPlay). Verify display adaptation, voice routing, and touch latency.',
    priority: 'P2_medium',
    preconditions: [
      'Head unit with Android Auto or CarPlay',
      'Vehicle integration active',
      'CAN bus data available (if supported)',
    ],
    steps: [
      'Connect device to head unit',
      'Verify display renders correctly on head unit screen',
      'Test voice commands through car microphone',
      'Test touch interaction latency',
      'Verify audio routing (navigation voice through car speakers)',
    ],
    expectedOutcomes: [
      'Display adapts to head unit resolution',
      'Voice commands work through car mic',
      'Touch latency <100ms',
      'Audio routes correctly to car speakers',
    ],
    passCriteria: [
      { metric: 'display_adaptation', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'voice_recognition_accuracy', operator: '>', value: 0.9, unit: 'ratio' },
      { metric: 'touch_latency_ms', operator: '<', value: 100, unit: 'ms' },
      { metric: 'audio_routing_correct', operator: '==', value: 1, unit: 'boolean' },
    ],
    duration_minutes: 60,
    requiredEquipment: ['Vehicle with Android Auto/CarPlay', 'Test phone'],
    safetyNotes: ['Vehicle must be stationary for initial setup'],
  },
  {
    id: 'FT-010',
    environment: 'multi_floor_parking',
    name: 'Multi-Floor Parking Structure',
    description: 'Navigate within multi-floor parking structure. Verify floor detection, indoor positioning, and exit guidance.',
    priority: 'P2_medium',
    preconditions: [
      'Barometer active for floor detection',
      'Indoor map data available',
      'WiFi/BLE beacons mapped (if available)',
    ],
    steps: [
      'Enter parking structure from street level',
      'Drive to level -3',
      'Verify floor level detection',
      'Navigate to exit',
      'Verify transition back to outdoor navigation',
    ],
    expectedOutcomes: [
      'Floor level detected within 1 floor accuracy',
      'Position maintained via barometer + IMU',
      'Exit guidance provided',
      'Smooth transition to outdoor GNSS on exit',
    ],
    passCriteria: [
      { metric: 'floor_detection_accuracy', operator: '<=', value: 1, unit: 'floors' },
      { metric: 'exit_guidance_provided', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'outdoor_transition_time_s', operator: '<', value: 15, unit: 'seconds' },
    ],
    duration_minutes: 30,
    requiredEquipment: ['Test vehicle', 'Barometer reference'],
    safetyNotes: ['Drive slowly in parking structure', 'Watch for pedestrians'],
  },
  {
    id: 'FT-011',
    environment: 'border_crossing',
    name: 'Cross-Border Navigation',
    description: 'Navigate across country border. Verify map data transition, regulation changes, and unit system adaptation.',
    priority: 'P2_medium',
    preconditions: [
      'Map data for both countries loaded',
      'Regulation rules configured for both jurisdictions',
    ],
    steps: [
      'Start navigation in country A',
      'Cross border into country B',
      'Verify map data transitions seamlessly',
      'Verify speed limit units change if applicable',
      'Verify routing rules adapt to local regulations',
    ],
    expectedOutcomes: [
      'Map data transitions without gap',
      'Units adapt to local convention',
      'Speed limits reflect local regulations',
      'No navigation interruption at border',
    ],
    passCriteria: [
      { metric: 'map_transition_gap_s', operator: '<', value: 2, unit: 'seconds' },
      { metric: 'regulation_adaptation', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'navigation_interruption', operator: '==', value: 0, unit: 'count' },
    ],
    duration_minutes: 90,
    requiredEquipment: ['Test vehicle', 'Valid border crossing documents'],
    safetyNotes: ['Comply with all border regulations'],
  },
  {
    id: 'FT-012',
    environment: 'mountain_pass',
    name: 'Mountain Pass with Elevation Changes',
    description: 'Navigate mountain pass with >1000m elevation change. Verify elevation-aware routing, barometer fusion, and gradient-adjusted ETA.',
    priority: 'P2_medium',
    preconditions: [
      'Elevation data in map package',
      'Barometer calibrated',
      'Vehicle profile with gradient capability',
    ],
    steps: [
      'Start navigation at base of mountain',
      'Ascend pass with switchbacks',
      'Verify elevation display accuracy',
      'Verify ETA accounts for gradient',
      'Descend and verify brake warnings if applicable',
    ],
    expectedOutcomes: [
      'Elevation display within 10m of actual',
      'ETA adjusted for uphill gradient',
      'Barometer contributes to altitude fusion',
      'Route prefers safer gradient if alternatives exist',
    ],
    passCriteria: [
      { metric: 'elevation_accuracy_m', operator: '<', value: 10, unit: 'meters' },
      { metric: 'eta_gradient_adjustment', operator: '==', value: 1, unit: 'boolean' },
      { metric: 'barometer_fusion_active', operator: '==', value: 1, unit: 'boolean' },
    ],
    duration_minutes: 120,
    requiredEquipment: ['Test vehicle', 'Altimeter reference', 'Dashcam'],
    safetyNotes: ['Check weather before mountain driving', 'Carry emergency supplies'],
  },
];

// ═══════════════════════════════════════════════════════════
// ACCEPTANCE CRITERIA PER SUBSYSTEM
// ═══════════════════════════════════════════════════════════

export type Subsystem =
  | 'positioning'
  | 'fusion'
  | 'routing'
  | 'guidance'
  | 'alerting'
  | 'payments'
  | 'ai_intelligence'
  | 'map_ingest'
  | 'telemetry'
  | 'hud'
  | 'ai_copilot'
  | 'offline';

export interface AcceptanceCriteria {
  subsystem: Subsystem;
  criteria: CriterionItem[];
  definitionOfDone: string[];
}

export interface CriterionItem {
  id: string;
  description: string;
  testType: 'unit' | 'integration' | 'contract' | 'simulation' | 'replay' | 'load' | 'chaos' | 'security' | 'field';
  metric?: string;
  threshold?: string;
  mandatory: boolean;
}

export const ACCEPTANCE_CRITERIA: AcceptanceCriteria[] = [
  {
    subsystem: 'positioning',
    criteria: [
      { id: 'POS-AC-01', description: 'Resolves stable position under nominal GNSS within 3 seconds', testType: 'integration', metric: 'ttff_s', threshold: '<3', mandatory: true },
      { id: 'POS-AC-02', description: 'Falls back to fusion under partial source loss within 500ms', testType: 'simulation', metric: 'fallback_latency_ms', threshold: '<500', mandatory: true },
      { id: 'POS-AC-03', description: 'Emits confidence + reason codes on every position update', testType: 'contract', mandatory: true },
      { id: 'POS-AC-04', description: 'Replay deterministic within 1m tolerance', testType: 'replay', metric: 'replay_deviation_m', threshold: '<1', mandatory: true },
      { id: 'POS-AC-05', description: 'Multi-constellation support (GPS + Galileo + GLONASS + BeiDou)', testType: 'integration', mandatory: true },
      { id: 'POS-AC-06', description: 'Spoofing detection within 5 seconds', testType: 'simulation', metric: 'detection_time_s', threshold: '<5', mandatory: true },
    ],
    definitionOfDone: [
      'Formal contract defined',
      'Schema versioned',
      'Health endpoint exposed',
      'Structured logs emitted',
      'Metrics exported',
      'Traces connected',
      'Replay compatible',
      'Failure mode specified',
      'Rollback path documented',
      'Security policy applied',
      'Acceptance tests passing',
    ],
  },
  {
    subsystem: 'fusion',
    criteria: [
      { id: 'FUS-AC-01', description: 'Maintains bounded drift <15m/km under GNSS dropout', testType: 'simulation', metric: 'drift_m_per_km', threshold: '<15', mandatory: true },
      { id: 'FUS-AC-02', description: 'Rejects conflicting sensor outliers automatically', testType: 'unit', mandatory: true },
      { id: 'FUS-AC-03', description: 'Exposes covariance matrix and integrity state', testType: 'contract', mandatory: true },
      { id: 'FUS-AC-04', description: 'Sensor weight adjustment within 100ms of quality change', testType: 'integration', metric: 'weight_adjustment_ms', threshold: '<100', mandatory: true },
    ],
    definitionOfDone: [
      'Formal contract defined',
      'Covariance model validated',
      'Health endpoint exposed',
      'Replay compatible',
      'Failure mode specified',
    ],
  },
  {
    subsystem: 'routing',
    criteria: [
      { id: 'RTE-AC-01', description: 'Returns primary + at least 2 alternatives', testType: 'integration', mandatory: true },
      { id: 'RTE-AC-02', description: 'Honors all constraints (tolls, risk, vehicle profile)', testType: 'contract', mandatory: true },
      { id: 'RTE-AC-03', description: 'Route computation <100ms for <100km routes', testType: 'load', metric: 'compute_time_ms', threshold: '<100', mandatory: true },
      { id: 'RTE-AC-04', description: 'Reroute <500ms on incident detection', testType: 'simulation', metric: 'reroute_ms', threshold: '<500', mandatory: true },
      { id: 'RTE-AC-05', description: 'Explains reroute reason via reason_codes', testType: 'contract', mandatory: true },
      { id: 'RTE-AC-06', description: 'Multi-objective cost breakdown in response', testType: 'contract', mandatory: true },
    ],
    definitionOfDone: [
      'Formal contract defined',
      'Graph engine tested with corpus',
      'Health endpoint exposed',
      'Load tested at 1000 req/s',
      'Failure mode specified',
    ],
  },
  {
    subsystem: 'guidance',
    criteria: [
      { id: 'GUI-AC-01', description: 'Maneuver instruction generated <20ms after position update', testType: 'integration', metric: 'guidance_latency_ms', threshold: '<20', mandatory: true },
      { id: 'GUI-AC-02', description: 'Lane guidance accurate at interchanges', testType: 'field', mandatory: true },
      { id: 'GUI-AC-03', description: 'Voice instruction advance distance appropriate for speed', testType: 'simulation', mandatory: true },
    ],
    definitionOfDone: [
      'Formal contract defined',
      'Voice text templates localized',
      'HUD overlay specification complete',
      'Replay compatible',
    ],
  },
  {
    subsystem: 'alerting',
    criteria: [
      { id: 'ALR-AC-01', description: 'Deduplicates repeated events within 5-minute window', testType: 'unit', mandatory: true },
      { id: 'ALR-AC-02', description: 'Retries per policy (exponential backoff, max 5)', testType: 'integration', mandatory: true },
      { id: 'ALR-AC-03', description: 'Emits audit records for every dispatch', testType: 'contract', mandatory: true },
      { id: 'ALR-AC-04', description: 'Survives provider outage with fallback channel', testType: 'chaos', mandatory: true },
      { id: 'ALR-AC-05', description: 'Alert dispatch <200ms for critical priority', testType: 'load', metric: 'dispatch_latency_ms', threshold: '<200', mandatory: true },
    ],
    definitionOfDone: [
      'Formal contract defined',
      'Multi-channel fallback tested',
      'Audit trail verified',
      'Rate limiting applied',
    ],
  },
  {
    subsystem: 'payments',
    criteria: [
      { id: 'PAY-AC-01', description: 'Idempotent charge handling (duplicate requests safe)', testType: 'integration', mandatory: true },
      { id: 'PAY-AC-02', description: 'Secure webhook validation (signature + timestamp)', testType: 'security', mandatory: true },
      { id: 'PAY-AC-03', description: 'Full audit trail for every transaction', testType: 'contract', mandatory: true },
      { id: 'PAY-AC-04', description: 'Reconciliation consistency check daily', testType: 'integration', mandatory: true },
    ],
    definitionOfDone: [
      'PCI compliance verified',
      'Webhook forgery protection tested',
      'Audit trail immutable',
      'Refund flow tested',
    ],
  },
  {
    subsystem: 'ai_intelligence',
    criteria: [
      { id: 'AI-AC-01', description: 'Deterministic fallback when model unavailable', testType: 'chaos', mandatory: true },
      { id: 'AI-AC-02', description: 'Offline or stale-model safety behavior defined', testType: 'simulation', mandatory: true },
      { id: 'AI-AC-03', description: 'Drift monitoring active with alerting', testType: 'integration', mandatory: true },
      { id: 'AI-AC-04', description: 'Version rollback supported within 5 minutes', testType: 'integration', metric: 'rollback_time_min', threshold: '<5', mandatory: true },
      { id: 'AI-AC-05', description: 'Explainability logging for every prediction', testType: 'contract', mandatory: false },
    ],
    definitionOfDone: [
      'Model registered in registry',
      'Shadow mode tested',
      'Canary deployment validated',
      'Fallback path verified',
      'Drift threshold configured',
    ],
  },
  {
    subsystem: 'map_ingest',
    criteria: [
      { id: 'MAP-AC-01', description: 'Multi-source ingestion (OSM + government + proprietary)', testType: 'integration', mandatory: true },
      { id: 'MAP-AC-02', description: 'Map diff computation <5 minutes for city-scale update', testType: 'load', metric: 'diff_time_min', threshold: '<5', mandatory: true },
      { id: 'MAP-AC-03', description: 'Corrupted tile detection and isolation', testType: 'chaos', mandatory: true },
      { id: 'MAP-AC-04', description: 'Versioned storage with rollback', testType: 'integration', mandatory: true },
    ],
    definitionOfDone: [
      'Validation pipeline active',
      'Checksum verification on every tile',
      'Rollback tested',
      'Offline package generation automated',
    ],
  },
  {
    subsystem: 'telemetry',
    criteria: [
      { id: 'TEL-AC-01', description: 'Ingest rate >10,000 events/second per node', testType: 'load', metric: 'events_per_second', threshold: '>10000', mandatory: true },
      { id: 'TEL-AC-02', description: 'Event ordering preserved per device', testType: 'contract', mandatory: true },
      { id: 'TEL-AC-03', description: 'Replay-compatible event format', testType: 'replay', mandatory: true },
    ],
    definitionOfDone: [
      'Schema versioned',
      'Retention policy applied',
      'Backpressure handling tested',
    ],
  },
  {
    subsystem: 'hud',
    criteria: [
      { id: 'HUD-AC-01', description: 'Render frame <16ms (60fps)', testType: 'load', metric: 'frame_time_ms', threshold: '<16', mandatory: true },
      { id: 'HUD-AC-02', description: 'Glance time <2 seconds for any information', testType: 'field', metric: 'glance_time_s', threshold: '<2', mandatory: true },
      { id: 'HUD-AC-03', description: 'Night mode transition <500ms', testType: 'integration', metric: 'transition_ms', threshold: '<500', mandatory: true },
    ],
    definitionOfDone: [
      'Performance budget met',
      'Accessibility audit passed',
      'Driving safety review completed',
    ],
  },
  {
    subsystem: 'ai_copilot',
    criteria: [
      { id: 'COP-AC-01', description: 'Response latency <2 seconds for simple queries', testType: 'load', metric: 'response_latency_s', threshold: '<2', mandatory: true },
      { id: 'COP-AC-02', description: 'Graceful degradation when LLM unavailable', testType: 'chaos', mandatory: true },
      { id: 'COP-AC-03', description: 'Context-aware suggestions based on current navigation state', testType: 'integration', mandatory: true },
    ],
    definitionOfDone: [
      'Fallback responses defined',
      'Rate limiting applied',
      'Safety filter active',
    ],
  },
  {
    subsystem: 'offline',
    criteria: [
      { id: 'OFF-AC-01', description: 'Map package download with resume support', testType: 'integration', mandatory: true },
      { id: 'OFF-AC-02', description: 'Delta updates <10% of full package size', testType: 'integration', metric: 'delta_ratio', threshold: '<0.1', mandatory: true },
      { id: 'OFF-AC-03', description: 'Checksum validation on every package', testType: 'unit', mandatory: true },
      { id: 'OFF-AC-04', description: 'Rollback to previous version on corruption', testType: 'chaos', mandatory: true },
    ],
    definitionOfDone: [
      'Package manifest versioned',
      'Delta generation automated',
      'Corruption recovery tested',
      'Storage budget enforced',
    ],
  },
];

// ═══════════════════════════════════════════════════════════
// BUILD VERIFICATION GATES
// ═══════════════════════════════════════════════════════════

export type GateStage = 'unit' | 'integration' | 'contract' | 'simulation' | 'replay' | 'load' | 'chaos' | 'security' | 'regression' | 'release';

export interface VerificationGate {
  stage: GateStage;
  order: number;
  name: string;
  description: string;
  passThreshold: string;
  blocking: boolean;
  tools: string[];
  artifacts: string[];
  timeout_minutes: number;
}

export const BUILD_VERIFICATION_GATES: VerificationGate[] = [
  {
    stage: 'unit',
    order: 1,
    name: 'Unit Tests',
    description: 'All unit tests must pass. Coverage >80% for core modules.',
    passThreshold: '100% pass, >80% coverage',
    blocking: true,
    tools: ['vitest', 'c8/istanbul'],
    artifacts: ['coverage-report.html', 'test-results.xml'],
    timeout_minutes: 10,
  },
  {
    stage: 'integration',
    order: 2,
    name: 'Integration Tests',
    description: 'Cross-module integration tests. Database, API, and service interactions verified.',
    passThreshold: '100% pass',
    blocking: true,
    tools: ['vitest', 'supertest', 'testcontainers'],
    artifacts: ['integration-results.xml'],
    timeout_minutes: 15,
  },
  {
    stage: 'contract',
    order: 3,
    name: 'Contract Tests',
    description: 'All API contracts, event schemas, and interface contracts validated against specifications.',
    passThreshold: '100% pass, 0 schema violations',
    blocking: true,
    tools: ['zod', 'ajv', 'custom validators'],
    artifacts: ['contract-report.json'],
    timeout_minutes: 5,
  },
  {
    stage: 'simulation',
    order: 4,
    name: 'Simulation Tests',
    description: 'Run simulation scenarios (GNSS loss, network failure, sensor degradation). All critical scenarios must pass.',
    passThreshold: '100% critical scenarios pass',
    blocking: true,
    tools: ['simulationEngine', 'GNSS simulator'],
    artifacts: ['simulation-results.json', 'trajectory-plots.png'],
    timeout_minutes: 30,
  },
  {
    stage: 'replay',
    order: 5,
    name: 'Replay Tests',
    description: 'Replay recorded field test data through system. Output must match within tolerance.',
    passThreshold: 'Determinism within 1m position, 5s ETA',
    blocking: true,
    tools: ['replayEngine', 'field-test-corpus'],
    artifacts: ['replay-comparison.json'],
    timeout_minutes: 20,
  },
  {
    stage: 'load',
    order: 6,
    name: 'Load Tests',
    description: 'Sustained load at 2x expected peak. Latency budgets must hold.',
    passThreshold: 'p95 latency within budget, 0 errors',
    blocking: true,
    tools: ['k6', 'artillery', 'custom harness'],
    artifacts: ['load-report.html', 'latency-histograms.png'],
    timeout_minutes: 30,
  },
  {
    stage: 'chaos',
    order: 7,
    name: 'Chaos Tests',
    description: 'Inject failures (service crash, network partition, disk full). System must degrade gracefully.',
    passThreshold: 'All critical services recover within SLO',
    blocking: true,
    tools: ['chaosTesting engine', 'litmus'],
    artifacts: ['chaos-report.json'],
    timeout_minutes: 45,
  },
  {
    stage: 'security',
    order: 8,
    name: 'Security Tests',
    description: 'OWASP top 10 scan, dependency audit, secret scanning, penetration test basics.',
    passThreshold: '0 critical/high vulnerabilities',
    blocking: true,
    tools: ['snyk', 'trivy', 'zap', 'custom scanners'],
    artifacts: ['security-report.html', 'dependency-audit.json'],
    timeout_minutes: 20,
  },
  {
    stage: 'regression',
    order: 9,
    name: 'Regression Tests',
    description: 'Full regression suite against known-good baselines. No regressions in core metrics.',
    passThreshold: '0 regressions vs baseline',
    blocking: true,
    tools: ['vitest', 'benchmark-harness'],
    artifacts: ['regression-report.json'],
    timeout_minutes: 15,
  },
  {
    stage: 'release',
    order: 10,
    name: 'Release Gate',
    description: 'Final human review. All previous gates passed. Release notes generated. Rollback plan documented.',
    passThreshold: 'Human approval + all gates green',
    blocking: true,
    tools: ['release-manager', 'changelog-generator'],
    artifacts: ['release-notes.md', 'rollback-plan.md'],
    timeout_minutes: 30,
  },
];

// ═══════════════════════════════════════════════════════════
// TEST DEVICE MATRIX
// ═══════════════════════════════════════════════════════════

export interface TestDevice {
  id: string;
  category: 'low_end' | 'mid_range' | 'high_end' | 'car_head_unit' | 'tablet';
  name: string;
  os: string;
  ram_gb: number;
  gnss_capabilities: string[];
  sensors: string[];
  constraints: string[];
}

export const TEST_DEVICE_MATRIX: TestDevice[] = [
  {
    id: 'DEV-001',
    category: 'low_end',
    name: 'Budget Android (2GB RAM)',
    os: 'Android 12 Go',
    ram_gb: 2,
    gnss_capabilities: ['GPS', 'GLONASS'],
    sensors: ['accelerometer', 'gyroscope'],
    constraints: ['No dual-frequency GNSS', 'Limited GPU', 'Small storage'],
  },
  {
    id: 'DEV-002',
    category: 'mid_range',
    name: 'Mid-Range Android (6GB RAM)',
    os: 'Android 14',
    ram_gb: 6,
    gnss_capabilities: ['GPS', 'GLONASS', 'Galileo', 'BeiDou'],
    sensors: ['accelerometer', 'gyroscope', 'magnetometer', 'barometer'],
    constraints: ['Single-frequency GNSS'],
  },
  {
    id: 'DEV-003',
    category: 'high_end',
    name: 'Flagship Android (12GB RAM)',
    os: 'Android 15',
    ram_gb: 12,
    gnss_capabilities: ['GPS L1/L5', 'Galileo E1/E5a', 'GLONASS', 'BeiDou B1/B2a'],
    sensors: ['accelerometer', 'gyroscope', 'magnetometer', 'barometer', 'camera'],
    constraints: [],
  },
  {
    id: 'DEV-004',
    category: 'high_end',
    name: 'iPhone Pro (8GB RAM)',
    os: 'iOS 18',
    ram_gb: 8,
    gnss_capabilities: ['GPS L1/L5', 'Galileo E1/E5a', 'GLONASS', 'BeiDou', 'QZSS'],
    sensors: ['accelerometer', 'gyroscope', 'magnetometer', 'barometer', 'LiDAR', 'camera'],
    constraints: ['No raw GNSS measurements before iOS 16'],
  },
  {
    id: 'DEV-005',
    category: 'car_head_unit',
    name: 'Android Automotive Head Unit',
    os: 'Android Automotive OS 14',
    ram_gb: 4,
    gnss_capabilities: ['GPS', 'GLONASS', 'Galileo'],
    sensors: ['CAN bus', 'wheel speed', 'steering angle', 'external GNSS antenna'],
    constraints: ['Fixed display size', 'No battery concern', 'CAN bus integration required'],
  },
  {
    id: 'DEV-006',
    category: 'tablet',
    name: 'iPad (WiFi only)',
    os: 'iPadOS 18',
    ram_gb: 8,
    gnss_capabilities: [],
    sensors: ['accelerometer', 'gyroscope', 'magnetometer'],
    constraints: ['No GNSS — WiFi positioning only', 'Large display'],
  },
];

// ═══════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════

export function getScenariosByEnvironment(env: TestEnvironment): FieldTestScenario[] {
  return FIELD_TEST_SCENARIOS.filter(s => s.environment === env);
}

export function getScenariosByPriority(priority: TestPriority): FieldTestScenario[] {
  return FIELD_TEST_SCENARIOS.filter(s => s.priority === priority);
}

export function getCriteriaBySubsystem(subsystem: Subsystem): AcceptanceCriteria | undefined {
  return ACCEPTANCE_CRITERIA.find(ac => ac.subsystem === subsystem);
}

export function getGateByStage(stage: GateStage): VerificationGate | undefined {
  return BUILD_VERIFICATION_GATES.find(g => g.stage === stage);
}

export function getBlockingGates(): VerificationGate[] {
  return BUILD_VERIFICATION_GATES.filter(g => g.blocking);
}

export function getTotalTestDuration(): number {
  return FIELD_TEST_SCENARIOS.reduce((sum, s) => sum + s.duration_minutes, 0);
}

export function getDevicesByCategory(category: TestDevice['category']): TestDevice[] {
  return TEST_DEVICE_MATRIX.filter(d => d.category === category);
}
