/**
 * G.A.N.E — Core System Audit Contract
 * 
 * Complete end-to-end audit framework for all 12 core subsystems.
 * Each subsystem has: audit criteria, validation rules, test scenarios,
 * pass/fail thresholds, and remediation actions.
 * 
 * Covers: Connectivity, GNSS, Communication, Synchronization, Updates,
 * Automation, Autonomy, AI/ML, Responsiveness, Emergency/SOS, Navigation/Maps, Integration
 */

// ═══════════════════════════════════════════════════════════
// AUDIT SEVERITY & STATUS TYPES
// ═══════════════════════════════════════════════════════════

export type AuditSeverity = 'critical' | 'high' | 'medium' | 'low' | 'info';
export type AuditStatus = 'pass' | 'fail' | 'warning' | 'not_tested' | 'remediated';
export type AuditMode = 'nominal' | 'degraded' | 'failure' | 'hostile' | 'overload' | 'cascading_failure' | 'recovery';

export interface AuditCriterion {
  readonly id: string;
  readonly subsystem: string;
  readonly category: string;
  readonly description: string;
  readonly severity: AuditSeverity;
  readonly validationRule: string;
  readonly testScenarios: readonly string[];
  readonly passThreshold: string;
  readonly failAction: string;
  readonly modes: readonly AuditMode[];
}

export interface AuditResult {
  readonly criterionId: string;
  readonly status: AuditStatus;
  readonly mode: AuditMode;
  readonly evidence: string;
  readonly timestamp: number;
  readonly remediationApplied?: string;
  readonly retestResult?: AuditStatus;
}

// ═══════════════════════════════════════════════════════════
// 1. CONNECTIVITY AUDIT
// ═══════════════════════════════════════════════════════════

export const CONNECTIVITY_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'CONN-001',
    subsystem: 'connectivity',
    category: 'protocols',
    description: 'All communication protocols use TLS 1.3+ with certificate pinning',
    severity: 'critical',
    validationRule: 'Every outbound connection must negotiate TLS 1.3 or higher; certificate chain must be pinned to known CA roots',
    testScenarios: ['Attempt TLS 1.2 downgrade', 'Present invalid certificate', 'MITM with valid but wrong cert'],
    passThreshold: '100% connections use TLS 1.3+, 0 downgrade successes',
    failAction: 'Block connection, alert security team, log attempt with source IP',
    modes: ['nominal', 'degraded', 'hostile'],
  },
  {
    id: 'CONN-002',
    subsystem: 'connectivity',
    category: 'retries',
    description: 'Exponential backoff with jitter on all retriable operations',
    severity: 'high',
    validationRule: 'Retry delay = min(baseDelay * 2^attempt + random(0, jitter), maxDelay); max 5 retries',
    testScenarios: ['Server returns 503 repeatedly', 'Network timeout mid-request', 'Partial response then disconnect'],
    passThreshold: 'No thundering herd; retry intervals increase monotonically with jitter',
    failAction: 'Circuit breaker opens after 5 consecutive failures; fallback to cached data',
    modes: ['nominal', 'degraded', 'failure', 'overload'],
  },
  {
    id: 'CONN-003',
    subsystem: 'connectivity',
    category: 'failover',
    description: 'Automatic failover to secondary endpoints within 3 seconds',
    severity: 'critical',
    validationRule: 'Primary failure detected within 1s; secondary endpoint active within 2s additional',
    testScenarios: ['Kill primary server', 'DNS resolution failure', 'Primary returns errors but stays up'],
    passThreshold: 'Failover completes in <3s; no data loss during transition',
    failAction: 'Activate offline mode; queue operations for replay',
    modes: ['failure', 'cascading_failure', 'recovery'],
  },
  {
    id: 'CONN-004',
    subsystem: 'connectivity',
    category: 'qos',
    description: 'Quality of Service prioritization: SOS > navigation > telemetry > analytics',
    severity: 'high',
    validationRule: 'Under bandwidth constraint, higher priority traffic gets 100% allocation before lower priority',
    testScenarios: ['Throttle bandwidth to 10kbps', 'Saturate with analytics while SOS active', 'Burst telemetry during navigation'],
    passThreshold: 'SOS messages delivered within 500ms even at 10kbps; navigation within 2s',
    failAction: 'Drop lowest priority traffic; never drop SOS',
    modes: ['nominal', 'degraded', 'overload'],
  },
  {
    id: 'CONN-005',
    subsystem: 'connectivity',
    category: 'encryption',
    description: 'All data encrypted in transit with AES-256-GCM or ChaCha20-Poly1305',
    severity: 'critical',
    validationRule: 'No plaintext data on any network interface; cipher suite restricted to AEAD only',
    testScenarios: ['Packet capture on all interfaces', 'Attempt cipher downgrade', 'Check for plaintext headers'],
    passThreshold: '0 plaintext bytes detected across all interfaces',
    failAction: 'Terminate connection; alert; block endpoint',
    modes: ['nominal', 'hostile'],
  },
  {
    id: 'CONN-006',
    subsystem: 'connectivity',
    category: 'latency',
    description: 'End-to-end latency monitoring with percentile tracking',
    severity: 'medium',
    validationRule: 'p50 < 50ms, p95 < 200ms, p99 < 500ms for all API calls',
    testScenarios: ['Measure under normal load', 'Measure under 10x load', 'Measure with 100ms added network delay'],
    passThreshold: 'p99 < 500ms under normal load; p99 < 2s under 10x load',
    failAction: 'Activate request shedding; alert ops team',
    modes: ['nominal', 'overload'],
  },
  {
    id: 'CONN-007',
    subsystem: 'connectivity',
    category: 'jitter',
    description: 'Network jitter compensation for real-time data streams',
    severity: 'medium',
    validationRule: 'Jitter buffer absorbs up to 100ms variance; adaptive buffer sizing based on network conditions',
    testScenarios: ['Inject 50ms jitter', 'Inject 200ms jitter', 'Alternating 0ms and 300ms delays'],
    passThreshold: 'Smooth data delivery with <10ms apparent jitter after buffering',
    failAction: 'Increase buffer size; if >500ms jitter, switch to batch mode',
    modes: ['nominal', 'degraded'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 2. GNSS AUDIT
// ═══════════════════════════════════════════════════════════

export const GNSS_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'GNSS-001',
    subsystem: 'gnss',
    category: 'multi_constellation',
    description: 'Simultaneous tracking of GPS, GLONASS, Galileo, BeiDou, QZSS, NavIC',
    severity: 'critical',
    validationRule: 'Position fix uses minimum 3 constellations; single-constellation mode only as fallback',
    testScenarios: ['Block GPS signals', 'Block all except Galileo', 'Urban canyon with limited sky view'],
    passThreshold: 'Position fix maintained with any 2+ constellations; accuracy <5m CEP',
    failAction: 'Activate PDR/IMU dead reckoning; alert user of degraded accuracy',
    modes: ['nominal', 'degraded', 'failure'],
  },
  {
    id: 'GNSS-002',
    subsystem: 'gnss',
    category: 'rtk_ppp',
    description: 'RTK/PPP correction stream validation and integrity monitoring',
    severity: 'high',
    validationRule: 'Correction data authenticated via RTCM 3.3 with OSNMA; age of correction <30s',
    testScenarios: ['Inject stale corrections', 'Inject forged corrections', 'Correction stream interruption'],
    passThreshold: 'Forged corrections rejected 100%; stale corrections flagged; graceful degradation on interruption',
    failAction: 'Fall back to standalone GNSS; flag position as uncorrected',
    modes: ['nominal', 'hostile', 'failure'],
  },
  {
    id: 'GNSS-003',
    subsystem: 'gnss',
    category: 'spoof_resistance',
    description: 'GNSS spoofing detection using multi-signal cross-validation',
    severity: 'critical',
    validationRule: 'Cross-validate position across constellations; detect >10m divergence as potential spoof',
    testScenarios: ['Meaconing attack', 'Sophisticated replay attack', 'Gradual position drift injection'],
    passThreshold: 'Detect spoofing within 5s; never navigate to spoofed position',
    failAction: 'Freeze last known good position; activate IMU-only navigation; alert user',
    modes: ['hostile'],
  },
  {
    id: 'GNSS-004',
    subsystem: 'gnss',
    category: 'jam_resistance',
    description: 'GNSS jamming detection and mitigation',
    severity: 'critical',
    validationRule: 'Detect C/N0 drop >10dB across multiple satellites as jamming indicator',
    testScenarios: ['Broadband jamming', 'Narrowband jamming on L1', 'Pulsed interference'],
    passThreshold: 'Jamming detected within 2s; alternative navigation activated within 5s',
    failAction: 'Switch to IMU/PDR; log jamming event with spectrum snapshot; alert',
    modes: ['hostile', 'failure'],
  },
  {
    id: 'GNSS-005',
    subsystem: 'gnss',
    category: 'trust_scoring',
    description: 'Per-satellite trust scoring based on signal quality, geometry, and history',
    severity: 'high',
    validationRule: 'Each satellite gets trust score 0-100; exclude satellites with score <30 from solution',
    testScenarios: ['Satellite with multipath', 'Satellite with anomalous clock', 'Satellite flagged by NANU'],
    passThreshold: 'Low-trust satellites excluded; position accuracy improves vs unfiltered solution',
    failAction: 'Weight solution by trust scores; alert if average trust <50',
    modes: ['nominal', 'degraded'],
  },
  {
    id: 'GNSS-006',
    subsystem: 'gnss',
    category: 'timing',
    description: 'GNSS-derived timing accuracy within 100ns of UTC',
    severity: 'high',
    validationRule: 'Time solution validated against multiple constellations; reject if divergence >1μs',
    testScenarios: ['Single constellation time', 'Time spoofing attempt', 'Leap second handling'],
    passThreshold: 'Time accuracy <100ns; leap seconds handled without glitch',
    failAction: 'Fall back to NTP; flag time as unverified',
    modes: ['nominal', 'hostile'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 3. COMMUNICATION AUDIT
// ═══════════════════════════════════════════════════════════

export const COMMUNICATION_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'COMM-001',
    subsystem: 'communication',
    category: 'multi_path',
    description: 'Multi-path communication: cellular, WiFi, satellite, mesh, V2X',
    severity: 'high',
    validationRule: 'System maintains connectivity via any available path; seamless handoff between paths',
    testScenarios: ['Disable cellular', 'Disable WiFi', 'All paths down except satellite'],
    passThreshold: 'Handoff completes in <2s; no message loss during transition',
    failAction: 'Queue messages; activate store-and-forward; alert user',
    modes: ['nominal', 'degraded', 'failure'],
  },
  {
    id: 'COMM-002',
    subsystem: 'communication',
    category: 'packet_loss',
    description: 'Graceful handling of up to 30% packet loss',
    severity: 'high',
    validationRule: 'Application-layer retransmission ensures 100% delivery for critical messages; best-effort for telemetry',
    testScenarios: ['10% random packet loss', '30% burst loss', '50% loss on one path'],
    passThreshold: 'Critical messages delivered 100%; telemetry gap <5s at 30% loss',
    failAction: 'Increase redundancy; switch to more reliable path; reduce telemetry rate',
    modes: ['degraded', 'overload'],
  },
  {
    id: 'COMM-003',
    subsystem: 'communication',
    category: 'replay_safety',
    description: 'Replay attack prevention with nonce and timestamp validation',
    severity: 'critical',
    validationRule: 'Every message includes monotonic nonce + timestamp; reject if nonce reused or timestamp >30s old',
    testScenarios: ['Replay captured message', 'Replay with modified timestamp', 'Out-of-order delivery'],
    passThreshold: '100% replayed messages rejected; out-of-order handled correctly',
    failAction: 'Drop message; log replay attempt; alert security',
    modes: ['hostile'],
  },
  {
    id: 'COMM-004',
    subsystem: 'communication',
    category: 'offline_recovery',
    description: 'Full offline operation with automatic sync on reconnection',
    severity: 'critical',
    validationRule: 'All critical functions work offline; CRDT-based sync resolves conflicts on reconnection',
    testScenarios: ['24h offline operation', 'Conflicting edits during offline', 'Partial sync interrupted'],
    passThreshold: 'Zero data loss after 24h offline; conflicts resolved deterministically',
    failAction: 'Preserve local state; queue all operations; sync with conflict resolution on reconnect',
    modes: ['failure', 'recovery'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 4. SYNCHRONIZATION AUDIT
// ═══════════════════════════════════════════════════════════

export const SYNCHRONIZATION_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'SYNC-001',
    subsystem: 'synchronization',
    category: 'time_consistency',
    description: 'All nodes synchronized within 10ms of reference time',
    severity: 'high',
    validationRule: 'NTP/PTP synchronization with GNSS as primary reference; drift monitoring per node',
    testScenarios: ['NTP server unreachable', 'GNSS time unavailable', 'Clock drift injection'],
    passThreshold: 'All nodes within 10ms; alert if any node drifts >50ms',
    failAction: 'Use local monotonic clock; flag events as time-uncertain',
    modes: ['nominal', 'degraded', 'failure'],
  },
  {
    id: 'SYNC-002',
    subsystem: 'synchronization',
    category: 'state_consistency',
    description: 'Eventual consistency with causal ordering for all state updates',
    severity: 'high',
    validationRule: 'Vector clocks or HLC for causal ordering; CRDT for conflict-free merge',
    testScenarios: ['Concurrent updates from 3 nodes', 'Network partition then heal', 'Rapid state changes'],
    passThreshold: 'All nodes converge to same state within 5s of partition heal',
    failAction: 'Log divergence; trigger manual reconciliation for non-CRDT state',
    modes: ['nominal', 'degraded', 'recovery'],
  },
  {
    id: 'SYNC-003',
    subsystem: 'synchronization',
    category: 'ordering',
    description: 'Strict ordering guarantees for safety-critical event sequences',
    severity: 'critical',
    validationRule: 'Safety events use total order broadcast; non-safety uses causal order',
    testScenarios: ['Concurrent safety events', 'Out-of-order delivery', 'Duplicate event delivery'],
    passThreshold: 'Safety events always in correct order; duplicates idempotently handled',
    failAction: 'Buffer and reorder; reject if ordering cannot be determined',
    modes: ['nominal', 'degraded', 'hostile'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 5. UPDATES AUDIT (OTA)
// ═══════════════════════════════════════════════════════════

export const UPDATES_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'UPD-001',
    subsystem: 'updates',
    category: 'ota',
    description: 'OTA updates with dual-partition A/B scheme and atomic rollback',
    severity: 'critical',
    validationRule: 'Update written to inactive partition; boot into new partition; rollback if health check fails within 60s',
    testScenarios: ['Corrupt update package', 'Power loss during update', 'Update that causes boot loop'],
    passThreshold: 'Corrupt updates rejected; power loss results in old partition boot; boot loop triggers rollback',
    failAction: 'Rollback to previous partition; report update failure; block retry for 1h',
    modes: ['nominal', 'failure'],
  },
  {
    id: 'UPD-002',
    subsystem: 'updates',
    category: 'rollback',
    description: 'Rollback protection prevents downgrade to known-vulnerable versions',
    severity: 'critical',
    validationRule: 'Monotonic version counter in secure element; reject any version <= current minimum',
    testScenarios: ['Attempt downgrade to v1.0', 'Attempt rollback past security fix', 'Version counter tampering'],
    passThreshold: '100% downgrade attempts rejected; counter tamper detected',
    failAction: 'Reject update; alert security; lock update mechanism for investigation',
    modes: ['hostile'],
  },
  {
    id: 'UPD-003',
    subsystem: 'updates',
    category: 'integrity',
    description: 'Update package integrity: Ed25519 signature + SHA-256 hash + reproducible build',
    severity: 'critical',
    validationRule: 'Signature verified against pinned public key; hash matches manifest; build reproducible from source',
    testScenarios: ['Modified binary', 'Valid hash but wrong signature', 'Signature from revoked key'],
    passThreshold: '100% tampered packages rejected; revoked keys rejected',
    failAction: 'Reject update; quarantine package; alert security team',
    modes: ['nominal', 'hostile'],
  },
  {
    id: 'UPD-004',
    subsystem: 'updates',
    category: 'version_compatibility',
    description: 'Cross-service version compatibility validation before deployment',
    severity: 'high',
    validationRule: 'Compatibility matrix checked before update; incompatible combinations blocked',
    testScenarios: ['Update client without server', 'Update server breaking client API', 'Partial fleet update'],
    passThreshold: 'Incompatible updates blocked; partial fleet handled with version negotiation',
    failAction: 'Block update; notify operator; suggest compatible version set',
    modes: ['nominal'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 6. AUTOMATION AUDIT
// ═══════════════════════════════════════════════════════════

export const AUTOMATION_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'AUTO-001',
    subsystem: 'automation',
    category: 'self_healing',
    description: 'Automatic recovery from detected failures without human intervention',
    severity: 'high',
    validationRule: 'Health checks every 10s; unhealthy service restarted within 30s; escalate after 3 restarts',
    testScenarios: ['Kill service process', 'Memory leak causing OOM', 'Deadlocked service'],
    passThreshold: 'Recovery within 30s for 95% of failures; escalation for persistent failures',
    failAction: 'Escalate to ops; activate standby; page on-call engineer',
    modes: ['failure', 'recovery'],
  },
  {
    id: 'AUTO-002',
    subsystem: 'automation',
    category: 'rerouting',
    description: 'Automatic traffic rerouting on service degradation',
    severity: 'high',
    validationRule: 'Load balancer detects unhealthy backend within 10s; reroutes within 5s',
    testScenarios: ['Backend returns 500s', 'Backend latency >5s', 'Backend connection refused'],
    passThreshold: 'User-visible errors <0.1% during rerouting',
    failAction: 'Remove backend from pool; alert; add back after 3 consecutive health checks pass',
    modes: ['degraded', 'failure'],
  },
  {
    id: 'AUTO-003',
    subsystem: 'automation',
    category: 'scaling',
    description: 'Horizontal auto-scaling based on CPU, memory, and request rate',
    severity: 'medium',
    validationRule: 'Scale up when CPU >70% for 2min or request queue >100; scale down when CPU <30% for 10min',
    testScenarios: ['Sudden 10x traffic spike', 'Gradual ramp to 5x', 'Traffic drop to near zero'],
    passThreshold: 'Scale-up within 2min; no request drops during scaling; scale-down within 15min',
    failAction: 'Alert if scaling fails; activate request shedding as fallback',
    modes: ['nominal', 'overload'],
  },
  {
    id: 'AUTO-004',
    subsystem: 'automation',
    category: 'idempotency',
    description: 'All automated actions are idempotent and safe to retry',
    severity: 'critical',
    validationRule: 'Every automated action has idempotency key; duplicate execution produces same result',
    testScenarios: ['Trigger same action 5 times', 'Action interrupted mid-execution then retried', 'Concurrent duplicate triggers'],
    passThreshold: 'State identical after 1 execution vs 5 executions; no side-effect duplication',
    failAction: 'Log duplicate detection; skip execution; alert if unexpected duplicate pattern',
    modes: ['nominal', 'failure', 'recovery'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 7. AUTONOMY AUDIT
// ═══════════════════════════════════════════════════════════

export const AUTONOMY_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'AUTON-001',
    subsystem: 'autonomy',
    category: 'bounded_decisions',
    description: 'All autonomous decisions bounded by policy engine with hard limits',
    severity: 'critical',
    validationRule: 'Every AI decision checked against policy constraints before execution; out-of-bounds decisions blocked',
    testScenarios: ['AI suggests route through restricted area', 'AI recommends speed above limit', 'AI action conflicts with user preference'],
    passThreshold: '100% out-of-bounds decisions blocked; user notified of overrides',
    failAction: 'Block action; fall back to default behavior; log for review',
    modes: ['nominal', 'hostile'],
  },
  {
    id: 'AUTON-002',
    subsystem: 'autonomy',
    category: 'override_safety',
    description: 'Human override always available and takes precedence over automation',
    severity: 'critical',
    validationRule: 'User can override any automated decision within 2 taps; override takes effect within 500ms',
    testScenarios: ['Override during active reroute', 'Override during emergency mode', 'Override conflicting with safety constraint'],
    passThreshold: 'Override effective within 500ms; safety constraints still enforced even on override',
    failAction: 'If override conflicts with safety, warn user but allow with acknowledgment',
    modes: ['nominal', 'degraded'],
  },
  {
    id: 'AUTON-003',
    subsystem: 'autonomy',
    category: 'explainability',
    description: 'Every autonomous decision has human-readable explanation',
    severity: 'high',
    validationRule: 'Decision log includes: input data, model used, confidence, reasoning chain, alternatives considered',
    testScenarios: ['Route change explanation', 'Alert generation explanation', 'Anomaly detection explanation'],
    passThreshold: 'Every decision has explanation accessible within 1 tap; explanation understandable by non-technical user',
    failAction: 'Flag unexplainable decisions; require human approval for low-confidence decisions',
    modes: ['nominal'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 8. AI/ML AUDIT
// ═══════════════════════════════════════════════════════════

export const AI_ML_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'AIML-001',
    subsystem: 'ai_ml',
    category: 'drift',
    description: 'Model drift detection using PSI, KL divergence, and accuracy monitoring',
    severity: 'high',
    validationRule: 'PSI >0.2 triggers alert; PSI >0.5 triggers model rollback; accuracy drop >5% triggers retraining',
    testScenarios: ['Gradual feature distribution shift', 'Sudden input pattern change', 'Label drift in feedback data'],
    passThreshold: 'Drift detected within 1h; rollback within 5min of critical drift',
    failAction: 'Rollback to previous model version; alert ML team; activate rule-based fallback',
    modes: ['nominal', 'degraded'],
  },
  {
    id: 'AIML-002',
    subsystem: 'ai_ml',
    category: 'adversarial_robustness',
    description: 'Models resilient to adversarial inputs (FGSM, PGD, C&W attacks)',
    severity: 'critical',
    validationRule: 'Model accuracy drops <5% under adversarial perturbation within epsilon=0.03',
    testScenarios: ['FGSM attack on input features', 'PGD attack on sensor data', 'Data poisoning in training set'],
    passThreshold: 'Adversarial accuracy >90% of clean accuracy; poisoned samples detected and excluded',
    failAction: 'Activate adversarial input filter; fall back to rule-based system; alert security',
    modes: ['hostile'],
  },
  {
    id: 'AIML-003',
    subsystem: 'ai_ml',
    category: 'explainability',
    description: 'All ML predictions have feature attribution and confidence scores',
    severity: 'high',
    validationRule: 'SHAP/LIME explanations available for every prediction; confidence calibrated (Brier score <0.1)',
    testScenarios: ['Explain route prediction', 'Explain anomaly detection', 'Explain ETA estimation'],
    passThreshold: 'Explanations generated in <100ms; confidence calibration error <10%',
    failAction: 'Flag low-confidence predictions; require human review for critical decisions',
    modes: ['nominal'],
  },
  {
    id: 'AIML-004',
    subsystem: 'ai_ml',
    category: 'prompt_injection',
    description: 'LLM-based components resistant to prompt injection attacks',
    severity: 'critical',
    validationRule: 'System prompts isolated from user input; output sanitized; no code execution from LLM output',
    testScenarios: ['Direct prompt injection', 'Indirect injection via data', 'Jailbreak attempts', 'Output manipulation'],
    passThreshold: '100% injection attempts blocked; no unauthorized actions from LLM output',
    failAction: 'Sanitize output; block action; log attempt; alert security',
    modes: ['hostile'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 9. RESPONSIVENESS AUDIT
// ═══════════════════════════════════════════════════════════

export const RESPONSIVENESS_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'RESP-001',
    subsystem: 'responsiveness',
    category: 'latency_p50',
    description: 'p50 latency for all user-facing operations <100ms',
    severity: 'high',
    validationRule: 'Measure end-to-end from user action to visual feedback; includes network + processing + rendering',
    testScenarios: ['Map pan/zoom', 'Route calculation', 'Search query', 'Settings change'],
    passThreshold: 'p50 <100ms for UI interactions; p50 <500ms for route calculation',
    failAction: 'Optimize hot path; add caching; precompute likely next actions',
    modes: ['nominal'],
  },
  {
    id: 'RESP-002',
    subsystem: 'responsiveness',
    category: 'latency_p99',
    description: 'p99 latency for all operations <2s; SOS <500ms',
    severity: 'critical',
    validationRule: 'No operation exceeds 2s at p99; SOS activation must complete in <500ms at p99',
    testScenarios: ['Under 10x normal load', 'With 200ms network delay', 'With degraded backend'],
    passThreshold: 'p99 <2s general; p99 <500ms SOS; p99 <1s navigation',
    failAction: 'Timeout and return cached/approximate result; never block SOS',
    modes: ['nominal', 'overload', 'degraded'],
  },
  {
    id: 'RESP-003',
    subsystem: 'responsiveness',
    category: 'degraded_mode',
    description: 'Graceful degradation: reduce features, never freeze',
    severity: 'critical',
    validationRule: 'Under resource pressure, disable non-essential features in priority order; core navigation always works',
    testScenarios: ['CPU at 95%', 'Memory at 90%', 'Network at 1kbps', 'Battery at 5%'],
    passThreshold: 'Navigation functional at all resource levels; UI responsive within 500ms',
    failAction: 'Disable: analytics → overlays → animations → 3D → AR; keep: map + navigation + SOS',
    modes: ['degraded', 'failure'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 10. EMERGENCY/SOS AUDIT
// ═══════════════════════════════════════════════════════════

export const EMERGENCY_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'SOS-001',
    subsystem: 'emergency',
    category: 'offline_mode',
    description: 'Full SOS functionality without any network connectivity',
    severity: 'critical',
    validationRule: 'SOS captures: GPS position, timestamp, device ID, user ID; stores locally; transmits when connectivity restored',
    testScenarios: ['SOS with no network', 'SOS with no GPS', 'SOS with no network and no GPS'],
    passThreshold: 'SOS always activatable; position from last known or cell tower or WiFi fingerprint',
    failAction: 'Use any available positioning; store event; retry transmission every 30s',
    modes: ['failure', 'cascading_failure'],
  },
  {
    id: 'SOS-002',
    subsystem: 'emergency',
    category: 'extreme_failure',
    description: 'SOS operational even with 90% system failure',
    severity: 'critical',
    validationRule: 'SOS module runs in isolated process with reserved memory; independent of main application',
    testScenarios: ['Main app crashed', 'Database corrupted', 'All services down except SOS', 'Device in low-power mode'],
    passThreshold: 'SOS activates within 1s regardless of system state',
    failAction: 'SOS is the last thing to fail; if SOS fails, device-level emergency (hardware button)',
    modes: ['failure', 'cascading_failure'],
  },
  {
    id: 'SOS-003',
    subsystem: 'emergency',
    category: 'abuse_prevention',
    description: 'SOS abuse detection without blocking legitimate emergencies',
    severity: 'high',
    validationRule: 'Rate limit: max 3 SOS in 10min from same user; 4th requires confirmation; never block if velocity/acceleration anomaly detected',
    testScenarios: ['Rapid repeated SOS', 'SOS during actual accident (high deceleration)', 'SOS from stationary device'],
    passThreshold: 'Abuse detected and flagged; legitimate emergencies never blocked',
    failAction: 'Flag for review; always transmit; add abuse flag to metadata',
    modes: ['nominal', 'hostile'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 11. NAVIGATION/MAPS AUDIT
// ═══════════════════════════════════════════════════════════

export const NAVIGATION_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'NAV-001',
    subsystem: 'navigation',
    category: 'routing_correctness',
    description: 'Route calculations produce valid, legal, and optimal paths',
    severity: 'critical',
    validationRule: 'Every route segment is on a valid road; respects one-way, turn restrictions, vehicle type; within 5% of optimal distance',
    testScenarios: ['Route through one-way street', 'Route avoiding toll roads', 'Route for truck with height restriction'],
    passThreshold: '100% routes legal; 95% within 5% of optimal; 0 routes through restricted areas',
    failAction: 'Recalculate with stricter constraints; alert if no valid route found',
    modes: ['nominal'],
  },
  {
    id: 'NAV-002',
    subsystem: 'navigation',
    category: 'urban_canyon',
    description: 'Accurate navigation in urban canyons with degraded GNSS',
    severity: 'high',
    validationRule: 'Fuse GNSS + IMU + map matching + visual odometry; position error <10m in urban canyon',
    testScenarios: ['Downtown with 60-story buildings', 'Covered parking structure exit', 'Bridge underpass'],
    passThreshold: 'Position error <10m; correct street identified >95% of time',
    failAction: 'Increase map matching weight; alert user of reduced accuracy; suggest alternative route',
    modes: ['degraded'],
  },
  {
    id: 'NAV-003',
    subsystem: 'navigation',
    category: 'tunnels',
    description: 'Continuous navigation through tunnels using dead reckoning',
    severity: 'high',
    validationRule: 'PDR + IMU + wheel speed (if available) maintains position; drift <2% of distance traveled',
    testScenarios: ['1km tunnel', '5km tunnel', 'Tunnel with turns', 'Tunnel with elevation change'],
    passThreshold: 'Exit position error <50m for 1km tunnel; <200m for 5km tunnel',
    failAction: 'Show estimated position with uncertainty circle; recalibrate on tunnel exit',
    modes: ['degraded', 'failure'],
  },
  {
    id: 'NAV-004',
    subsystem: 'navigation',
    category: 'map_freshness',
    description: 'Map data freshness validation and stale data detection',
    severity: 'medium',
    validationRule: 'Map tiles have version + timestamp; alert if >30 days old; critical roads checked against crowd data',
    testScenarios: ['Navigate with 90-day old map', 'Road closure not in map', 'New road not in map'],
    passThreshold: 'Stale data flagged; crowd-sourced updates integrated within 1h',
    failAction: 'Flag route as potentially inaccurate; suggest alternative; prompt map update',
    modes: ['nominal', 'degraded'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// 12. INTEGRATION AUDIT
// ═══════════════════════════════════════════════════════════

export const INTEGRATION_AUDIT: readonly AuditCriterion[] = [
  {
    id: 'INT-001',
    subsystem: 'integration',
    category: 'contracts',
    description: 'All service interfaces have versioned contracts with backward compatibility',
    severity: 'high',
    validationRule: 'Every API has OpenAPI/protobuf schema; breaking changes require major version bump; old versions supported for 6 months',
    testScenarios: ['Call v1 API after v2 deployed', 'Send v2 payload to v1 endpoint', 'Unknown fields in request'],
    passThreshold: 'v1 calls succeed after v2 deploy; unknown fields ignored (not rejected)',
    failAction: 'Version negotiation; return appropriate error with migration guide',
    modes: ['nominal'],
  },
  {
    id: 'INT-002',
    subsystem: 'integration',
    category: 'dependencies',
    description: 'No circular dependencies; dependency graph is a DAG',
    severity: 'high',
    validationRule: 'Build-time dependency analysis confirms DAG; runtime dependency injection prevents circular calls',
    testScenarios: ['Compile-time cycle detection', 'Runtime call graph analysis', 'Service A → B → A call chain'],
    passThreshold: '0 circular dependencies at compile time; runtime cycles detected and broken',
    failAction: 'Break cycle with event-based decoupling; refactor shared dependency',
    modes: ['nominal'],
  },
  {
    id: 'INT-003',
    subsystem: 'integration',
    category: 'data_consistency',
    description: 'Cross-service data consistency with saga pattern for distributed transactions',
    severity: 'critical',
    validationRule: 'Every multi-service operation uses saga with compensating transactions; no orphaned state',
    testScenarios: ['Service B fails after Service A commits', 'Timeout during saga', 'Concurrent sagas on same entity'],
    passThreshold: 'Compensating transaction executes on failure; final state consistent; no orphans',
    failAction: 'Compensate; log inconsistency; alert; manual reconciliation for edge cases',
    modes: ['nominal', 'failure'],
  },
] as const;

// ═══════════════════════════════════════════════════════════
// MASTER AUDIT CATALOG
// ═══════════════════════════════════════════════════════════

export const CORE_SYSTEM_AUDIT_CATALOG = {
  connectivity: CONNECTIVITY_AUDIT,
  gnss: GNSS_AUDIT,
  communication: COMMUNICATION_AUDIT,
  synchronization: SYNCHRONIZATION_AUDIT,
  updates: UPDATES_AUDIT,
  automation: AUTOMATION_AUDIT,
  autonomy: AUTONOMY_AUDIT,
  aiMl: AI_ML_AUDIT,
  responsiveness: RESPONSIVENESS_AUDIT,
  emergency: EMERGENCY_AUDIT,
  navigation: NAVIGATION_AUDIT,
  integration: INTEGRATION_AUDIT,
} as const;

export const ALL_AUDIT_CRITERIA: readonly AuditCriterion[] = [
  ...CONNECTIVITY_AUDIT,
  ...GNSS_AUDIT,
  ...COMMUNICATION_AUDIT,
  ...SYNCHRONIZATION_AUDIT,
  ...UPDATES_AUDIT,
  ...AUTOMATION_AUDIT,
  ...AUTONOMY_AUDIT,
  ...AI_ML_AUDIT,
  ...RESPONSIVENESS_AUDIT,
  ...EMERGENCY_AUDIT,
  ...NAVIGATION_AUDIT,
  ...INTEGRATION_AUDIT,
];

// ═══════════════════════════════════════════════════════════
// AUDIT EXECUTION FRAMEWORK
// ═══════════════════════════════════════════════════════════

export interface AuditRun {
  readonly id: string;
  readonly startTime: number;
  readonly endTime?: number;
  readonly criteria: readonly AuditCriterion[];
  readonly results: AuditResult[];
  readonly mode: AuditMode;
  readonly status: 'running' | 'completed' | 'failed';
}

export interface RemediationAction {
  readonly criterionId: string;
  readonly description: string;
  readonly priority: AuditSeverity;
  readonly estimatedEffort: string;
  readonly appliedAt?: number;
  readonly retestResult?: AuditStatus;
  readonly regressionResult?: AuditStatus;
}

export function createAuditRun(mode: AuditMode): AuditRun {
  return {
    id: `AUDIT-${Date.now()}-${mode}`,
    startTime: Date.now(),
    criteria: ALL_AUDIT_CRITERIA,
    results: [],
    mode,
    status: 'running',
  };
}

export function evaluateAuditResult(
  criterion: AuditCriterion,
  mode: AuditMode,
  evidence: string,
  passed: boolean
): AuditResult {
  return {
    criterionId: criterion.id,
    status: passed ? 'pass' : 'fail',
    mode,
    evidence,
    timestamp: Date.now(),
  };
}

export function generateRemediationPlan(failedResults: readonly AuditResult[]): RemediationAction[] {
  return failedResults.map(result => {
    const criterion = ALL_AUDIT_CRITERIA.find(c => c.id === result.criterionId);
    return {
      criterionId: result.criterionId,
      description: criterion?.failAction ?? 'Investigate and remediate',
      priority: criterion?.severity ?? 'medium',
      estimatedEffort: criterion?.severity === 'critical' ? '4-8h' : criterion?.severity === 'high' ? '2-4h' : '1-2h',
    };
  });
}

/** Summary statistics for an audit run */
export function auditSummary(run: AuditRun) {
  const total = run.results.length;
  const passed = run.results.filter(r => r.status === 'pass').length;
  const failed = run.results.filter(r => r.status === 'fail').length;
  const warnings = run.results.filter(r => r.status === 'warning').length;
  const remediated = run.results.filter(r => r.status === 'remediated').length;
  return { total, passed, failed, warnings, remediated, passRate: total > 0 ? passed / total : 0 };
}
