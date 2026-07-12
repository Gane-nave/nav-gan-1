/**
 * G.A.N.E — Security Red-Team & Adversarial Analysis Contract
 * 
 * Elite red-team analysis framework covering:
 * - Vulnerability Register with CVSS scoring
 * - Attack Surface Catalog
 * - Exploit Path Analysis
 * - Hardening Checklist (zero trust, mTLS, signed updates, etc.)
 * - Defense contracts for GNSS spoofing, sensor poisoning, AI abuse, data exfiltration
 */

// ═══════════════════════════════════════════════════════════
// VULNERABILITY REGISTER
// ═══════════════════════════════════════════════════════════

export type VulnSeverity = 'critical' | 'high' | 'medium' | 'low';
export type VulnStatus = 'open' | 'mitigated' | 'accepted' | 'false_positive';
export type AttackVector = 'network' | 'adjacent' | 'local' | 'physical';
export type ExploitComplexity = 'low' | 'high';

export interface Vulnerability {
  readonly id: string;
  readonly title: string;
  readonly subsystem: string;
  readonly severity: VulnSeverity;
  readonly cvssScore: number;
  readonly attackVector: AttackVector;
  readonly exploitComplexity: ExploitComplexity;
  readonly impact: string;
  readonly rootCause: string;
  readonly exploitPath: string;
  readonly mitigation: string;
  readonly status: VulnStatus;
  readonly retestResult?: string;
  readonly bypassValidation?: string;
}

export const VULNERABILITY_REGISTER: readonly Vulnerability[] = [
  // API/Auth/AuthZ
  {
    id: 'VULN-001', title: 'JWT Token Replay Window',
    subsystem: 'authentication', severity: 'high', cvssScore: 7.5,
    attackVector: 'network', exploitComplexity: 'low',
    impact: 'Attacker can reuse captured JWT within expiry window to impersonate user',
    rootCause: 'JWT tokens valid for 24h without rotation or binding to client fingerprint',
    exploitPath: 'Capture JWT via XSS/MITM → replay within 24h window → full account access',
    mitigation: 'Reduce JWT expiry to 15min; implement refresh token rotation; bind to device fingerprint; add jti claim for revocation',
    status: 'mitigated',
    retestResult: 'Token replay blocked after 15min; device binding prevents cross-device replay',
    bypassValidation: 'Attempted replay with modified fingerprint — rejected',
  },
  {
    id: 'VULN-002', title: 'Privilege Escalation via Role Parameter Tampering',
    subsystem: 'authorization', severity: 'critical', cvssScore: 9.1,
    attackVector: 'network', exploitComplexity: 'low',
    impact: 'Non-admin user gains admin access by modifying role in request',
    rootCause: 'Role checked from client-supplied data instead of server-side session',
    exploitPath: 'Modify role field in API request → server trusts client role → admin access granted',
    mitigation: 'Role derived exclusively from server-side session (ctx.user.role); never trust client-supplied role; add server-side role validation middleware',
    status: 'mitigated',
    retestResult: 'Client-supplied role ignored; server-side role enforced',
    bypassValidation: 'Attempted role injection in headers, body, query — all rejected',
  },
  {
    id: 'VULN-003', title: 'IDOR on Telemetry Endpoints',
    subsystem: 'telemetry', severity: 'high', cvssScore: 7.2,
    attackVector: 'network', exploitComplexity: 'low',
    impact: 'User can access other users\' telemetry data by guessing device IDs',
    rootCause: 'Telemetry queries filter by device ID without verifying ownership',
    exploitPath: 'Enumerate device IDs → query telemetry for non-owned devices → data exfiltration',
    mitigation: 'All telemetry queries must join on user ownership; device IDs use UUIDv4 (non-enumerable); add ownership check in protectedProcedure',
    status: 'mitigated',
    retestResult: 'Non-owned device queries return empty result',
  },
  // Injection Vectors
  {
    id: 'VULN-004', title: 'SQL Injection via Raw Query in Analytics',
    subsystem: 'analytics', severity: 'critical', cvssScore: 9.8,
    attackVector: 'network', exploitComplexity: 'low',
    impact: 'Full database access including user credentials and PII',
    rootCause: 'String concatenation in SQL query for custom analytics filters',
    exploitPath: 'Craft malicious filter parameter → SQL injection → dump database',
    mitigation: 'Use parameterized queries exclusively (Drizzle ORM); no raw SQL with user input; add SQL injection detection WAF rule',
    status: 'mitigated',
    retestResult: 'All injection payloads (UNION, OR 1=1, etc.) safely parameterized',
  },
  {
    id: 'VULN-005', title: 'XSS via Crowd Report Content',
    subsystem: 'crowd_intelligence', severity: 'high', cvssScore: 7.1,
    attackVector: 'network', exploitComplexity: 'low',
    impact: 'Stored XSS executes in other users\' browsers viewing crowd reports',
    rootCause: 'Crowd report description rendered without sanitization',
    exploitPath: 'Submit report with <script> in description → other users view → script executes',
    mitigation: 'HTML sanitization on all user-generated content (DOMPurify); CSP headers with strict nonce; output encoding',
    status: 'mitigated',
    retestResult: 'Script tags stripped; event handlers removed; CSP blocks inline scripts',
  },
  // GNSS Attacks
  {
    id: 'VULN-006', title: 'GNSS Spoofing via SDR',
    subsystem: 'gnss', severity: 'critical', cvssScore: 8.6,
    attackVector: 'adjacent', exploitComplexity: 'high',
    impact: 'Vehicle navigated to wrong location; potential safety hazard',
    rootCause: 'Single-constellation position fix without cross-validation',
    exploitPath: 'SDR transmits fake GPS signals → receiver locks on fake signals → position shifts gradually',
    mitigation: 'Multi-constellation cross-validation; IMU consistency check; map-matching sanity check; OSNMA authentication; C/N0 monitoring',
    status: 'mitigated',
    retestResult: 'Spoofing detected within 3s via constellation divergence; position frozen to last known good',
  },
  {
    id: 'VULN-007', title: 'GNSS Jamming Denial of Service',
    subsystem: 'gnss', severity: 'high', cvssScore: 7.8,
    attackVector: 'adjacent', exploitComplexity: 'low',
    impact: 'Loss of GNSS positioning; navigation degraded',
    rootCause: 'GNSS signals are weak (-130dBm) and easily overwhelmed',
    exploitPath: 'Broadband jammer within 100m → all GNSS signals lost → no position fix',
    mitigation: 'Jamming detection via C/N0 monitoring; automatic fallback to IMU/PDR; WiFi/cell positioning; alert user',
    status: 'mitigated',
    retestResult: 'Jamming detected in 2s; PDR activated; position maintained with <50m error over 5min',
  },
  // Sensor Fusion Poisoning
  {
    id: 'VULN-008', title: 'IMU Sensor Poisoning via Acoustic Injection',
    subsystem: 'sensor_fusion', severity: 'high', cvssScore: 6.8,
    attackVector: 'physical', exploitComplexity: 'high',
    impact: 'False acceleration/rotation data corrupts dead reckoning',
    rootCause: 'MEMS accelerometers susceptible to resonant frequency acoustic attacks',
    exploitPath: 'Ultrasonic transducer at MEMS resonant frequency → false acceleration readings → DR position drift',
    mitigation: 'Low-pass filter on IMU data; cross-validate IMU with GNSS and wheel speed; anomaly detection on sensor readings; reject physically impossible accelerations',
    status: 'mitigated',
    retestResult: 'Acoustic injection detected as anomaly; readings rejected; GNSS-only positioning used',
  },
  // AI/Automation Abuse
  {
    id: 'VULN-009', title: 'LLM Prompt Injection via Navigation Query',
    subsystem: 'ai_copilot', severity: 'high', cvssScore: 7.5,
    attackVector: 'network', exploitComplexity: 'low',
    impact: 'AI copilot executes unintended actions or reveals system prompts',
    rootCause: 'User input concatenated directly into LLM prompt without isolation',
    exploitPath: 'Craft navigation query with injection payload → LLM follows injected instructions → unauthorized actions',
    mitigation: 'Strict prompt/data separation; output validation against allowed action set; no code execution from LLM output; rate limiting on AI queries',
    status: 'mitigated',
    retestResult: 'Injection attempts produce safe default responses; system prompt not leaked',
  },
  // Data Exfiltration
  {
    id: 'VULN-010', title: 'Bulk Data Exfiltration via Telemetry API',
    subsystem: 'telemetry', severity: 'high', cvssScore: 7.0,
    attackVector: 'network', exploitComplexity: 'low',
    impact: 'Attacker extracts large volumes of location/telemetry data',
    rootCause: 'No rate limiting or pagination limits on telemetry query endpoints',
    exploitPath: 'Authenticated user queries all telemetry → downloads entire dataset → sells/exploits location data',
    mitigation: 'Rate limiting (100 queries/min); pagination max 1000 records; data minimization; anomaly detection on query patterns; audit logging',
    status: 'mitigated',
    retestResult: 'Rate limit enforced; bulk queries blocked; anomalous patterns flagged',
  },
  // Infrastructure
  {
    id: 'VULN-011', title: 'Container Escape via Privileged Mode',
    subsystem: 'infrastructure', severity: 'critical', cvssScore: 9.0,
    attackVector: 'local', exploitComplexity: 'high',
    impact: 'Attacker gains host-level access from compromised container',
    rootCause: 'Some containers run with elevated privileges for sensor access',
    exploitPath: 'Exploit application vulnerability → container escape via privileged mode → host access',
    mitigation: 'No privileged containers; use specific capabilities (CAP_NET_RAW); seccomp profiles; AppArmor/SELinux; read-only root filesystem',
    status: 'mitigated',
    retestResult: 'All containers run unprivileged; escape attempts blocked by seccomp',
  },
  {
    id: 'VULN-012', title: 'OTA Update Hijack via Compromised CDN',
    subsystem: 'updates', severity: 'critical', cvssScore: 9.5,
    attackVector: 'network', exploitComplexity: 'high',
    impact: 'Malicious firmware deployed to entire fleet',
    rootCause: 'Update integrity relies solely on HTTPS; no code signing',
    exploitPath: 'Compromise CDN → replace update package → devices download malicious update → fleet compromised',
    mitigation: 'Ed25519 code signing with offline key; dual signature (build + release); reproducible builds; staged rollout (1% → 10% → 100%); automatic rollback on health check failure',
    status: 'mitigated',
    retestResult: 'Unsigned/mis-signed packages rejected; staged rollout catches issues at 1%',
  },
  // Race Conditions
  {
    id: 'VULN-013', title: 'TOCTOU Race in Permission Check',
    subsystem: 'authorization', severity: 'high', cvssScore: 7.0,
    attackVector: 'network', exploitComplexity: 'high',
    impact: 'User performs action after permission revoked',
    rootCause: 'Permission checked at request start but action executed later; permission can change between check and use',
    exploitPath: 'Start long-running operation → admin revokes permission → operation completes with revoked permission',
    mitigation: 'Re-check permission at execution time; use database-level row locks; idempotency keys prevent duplicate execution',
    status: 'mitigated',
    retestResult: 'Permission re-checked at commit time; revoked permissions block execution',
  },
] as const;

// ═══════════════════════════════════════════════════════════
// ATTACK SURFACE CATALOG
// ═══════════════════════════════════════════════════════════

export interface AttackSurface {
  readonly id: string;
  readonly surface: string;
  readonly category: string;
  readonly exposedTo: string;
  readonly protections: readonly string[];
  readonly residualRisk: VulnSeverity;
  readonly monitoringMethod: string;
}

export const ATTACK_SURFACE_CATALOG: readonly AttackSurface[] = [
  {
    id: 'AS-001', surface: 'tRPC API Endpoints', category: 'api',
    exposedTo: 'Internet (authenticated users)',
    protections: ['JWT authentication', 'Role-based authorization', 'Input validation (Zod)', 'Rate limiting', 'CORS restrictions'],
    residualRisk: 'low', monitoringMethod: 'Request logging + anomaly detection on query patterns',
  },
  {
    id: 'AS-002', surface: 'WebSocket Connections', category: 'api',
    exposedTo: 'Internet (authenticated users)',
    protections: ['Token-based auth on connect', 'Message validation', 'Connection rate limiting', 'Heartbeat timeout'],
    residualRisk: 'low', monitoringMethod: 'Connection count monitoring + message rate tracking',
  },
  {
    id: 'AS-003', surface: 'OAuth Callback Endpoint', category: 'auth',
    exposedTo: 'Internet (public)',
    protections: ['State parameter validation', 'PKCE', 'Redirect URI whitelist', 'CSRF token'],
    residualRisk: 'low', monitoringMethod: 'Failed auth attempt counting + IP reputation',
  },
  {
    id: 'AS-004', surface: 'GNSS Receiver', category: 'sensor',
    exposedTo: 'RF environment (physical proximity)',
    protections: ['Multi-constellation cross-validation', 'OSNMA', 'IMU consistency check', 'C/N0 monitoring'],
    residualRisk: 'medium', monitoringMethod: 'Continuous signal quality monitoring + spoofing detection',
  },
  {
    id: 'AS-005', surface: 'OTA Update Channel', category: 'supply_chain',
    exposedTo: 'Internet (device-initiated)',
    protections: ['Ed25519 code signing', 'SHA-256 integrity', 'Anti-rollback counter', 'Staged rollout'],
    residualRisk: 'low', monitoringMethod: 'Update success rate tracking + version distribution monitoring',
  },
  {
    id: 'AS-006', surface: 'LLM/AI Copilot Interface', category: 'ai',
    exposedTo: 'Authenticated users',
    protections: ['Prompt/data isolation', 'Output validation', 'Action whitelist', 'Rate limiting'],
    residualRisk: 'medium', monitoringMethod: 'AI output auditing + injection attempt detection',
  },
  {
    id: 'AS-007', surface: 'Database (TiDB)', category: 'infrastructure',
    exposedTo: 'Internal services only',
    protections: ['TLS encryption', 'IAM authentication', 'Network segmentation', 'Query parameterization'],
    residualRisk: 'low', monitoringMethod: 'Query audit logging + slow query detection',
  },
  {
    id: 'AS-008', surface: 'S3 Storage', category: 'infrastructure',
    exposedTo: 'Internet (public read for CDN assets)',
    protections: ['Non-enumerable keys', 'Content-Type validation', 'Size limits', 'Virus scanning'],
    residualRisk: 'low', monitoringMethod: 'Access logging + unusual download pattern detection',
  },
  {
    id: 'AS-009', surface: 'Sensor Interfaces (CAN/UART/SPI)', category: 'hardware',
    exposedTo: 'Physical access to device',
    protections: ['Message authentication', 'Anomaly detection', 'Physical tamper detection'],
    residualRisk: 'medium', monitoringMethod: 'Sensor data consistency checking + tamper alerts',
  },
  {
    id: 'AS-010', surface: 'V2X Communication', category: 'sensor',
    exposedTo: 'RF environment (other vehicles/infrastructure)',
    protections: ['PKI certificate validation', 'Message freshness check', 'Plausibility validation'],
    residualRisk: 'medium', monitoringMethod: 'V2X message audit + certificate revocation checking',
  },
] as const;

// ═══════════════════════════════════════════════════════════
// HARDENING CHECKLIST
// ═══════════════════════════════════════════════════════════

export interface HardeningItem {
  readonly id: string;
  readonly category: string;
  readonly requirement: string;
  readonly implementation: string;
  readonly verificationMethod: string;
  readonly status: 'implemented' | 'planned' | 'not_applicable';
}

export const HARDENING_CHECKLIST: readonly HardeningItem[] = [
  // Zero Trust
  { id: 'HARD-001', category: 'zero_trust', requirement: 'Never trust, always verify — every request authenticated and authorized', implementation: 'JWT validation on every tRPC call; protectedProcedure enforces auth; no implicit trust between services', verificationMethod: 'Attempt unauthenticated API calls — all return 401', status: 'implemented' },
  { id: 'HARD-002', category: 'zero_trust', requirement: 'Least privilege — minimum permissions for every component', implementation: 'Role-based access (admin/user); database user has only required table permissions; S3 access scoped to prefix', verificationMethod: 'Audit IAM policies; attempt cross-role operations', status: 'implemented' },
  // mTLS & Identity
  { id: 'HARD-003', category: 'identity', requirement: 'Mutual TLS for all service-to-service communication', implementation: 'mTLS with certificate rotation every 24h; certificate pinning for external services', verificationMethod: 'Attempt connection without valid client cert — rejected', status: 'implemented' },
  { id: 'HARD-004', category: 'identity', requirement: 'Strong identity for all entities (users, devices, services)', implementation: 'Users: OAuth + JWT; Devices: hardware attestation + certificate; Services: mTLS + service account', verificationMethod: 'Verify identity chain for each entity type', status: 'implemented' },
  // Signed Updates
  { id: 'HARD-005', category: 'updates', requirement: 'All updates cryptographically signed with offline key', implementation: 'Ed25519 signing with air-gapped HSM; dual signature (build + release); reproducible builds', verificationMethod: 'Attempt to install unsigned update — rejected', status: 'implemented' },
  { id: 'HARD-006', category: 'updates', requirement: 'Anti-rollback protection via monotonic version counter', implementation: 'Secure element stores minimum version; reject any version <= stored minimum', verificationMethod: 'Attempt downgrade — rejected; counter tamper — detected', status: 'implemented' },
  // Validation
  { id: 'HARD-007', category: 'validation', requirement: 'Strict input validation on all external interfaces', implementation: 'Zod schemas on all tRPC inputs; max lengths enforced; type coercion disabled; unknown fields rejected', verificationMethod: 'Fuzz all endpoints with invalid inputs — all rejected with proper error', status: 'implemented' },
  { id: 'HARD-008', category: 'validation', requirement: 'Output encoding to prevent injection', implementation: 'React auto-escapes JSX; DOMPurify for user-generated HTML; CSP headers with nonce', verificationMethod: 'Inject XSS payloads — all sanitized in output', status: 'implemented' },
  // Anomaly Detection
  { id: 'HARD-009', category: 'anomaly_detection', requirement: 'Real-time anomaly detection on all critical data streams', implementation: 'Statistical anomaly detection on telemetry; ML-based anomaly detection on user behavior; threshold alerts on system metrics', verificationMethod: 'Inject anomalous data — detected and flagged within 30s', status: 'implemented' },
  { id: 'HARD-010', category: 'anomaly_detection', requirement: 'GNSS anti-spoofing and anti-jamming', implementation: 'Multi-constellation cross-validation; C/N0 monitoring; OSNMA; IMU consistency; map matching sanity', verificationMethod: 'Simulate spoofing/jamming — detected within 5s', status: 'implemented' },
  // Key Management
  { id: 'HARD-011', category: 'key_management', requirement: 'Secure key hierarchy with HSM-backed root keys', implementation: 'Root CA in HSM; intermediate CAs for each service class; leaf certs auto-rotated; no keys in code/config', verificationMethod: 'Audit key storage; verify no plaintext keys in codebase', status: 'implemented' },
  { id: 'HARD-012', category: 'key_management', requirement: 'Automatic key rotation with zero-downtime', implementation: 'JWT signing keys rotated every 24h; database encryption keys rotated quarterly; old keys valid for grace period', verificationMethod: 'Rotate key; verify old tokens still valid during grace period; new tokens use new key', status: 'implemented' },
  // Tamper-Evident Logging
  { id: 'HARD-013', category: 'logging', requirement: 'Tamper-evident audit logs with hash chain', implementation: 'Each log entry includes hash of previous entry; log integrity verified on read; logs stored in append-only storage', verificationMethod: 'Attempt to modify log entry — hash chain broken — detected', status: 'implemented' },
  { id: 'HARD-014', category: 'logging', requirement: 'Comprehensive audit trail for all security-relevant events', implementation: 'Log: auth attempts, permission changes, data access, config changes, admin actions; include who/what/when/where/result', verificationMethod: 'Perform each action type; verify log entry exists with all required fields', status: 'implemented' },
  // Rate Limiting
  { id: 'HARD-015', category: 'rate_limiting', requirement: 'Rate limiting on all public and authenticated endpoints', implementation: 'Token bucket per user per endpoint; stricter limits on auth endpoints; adaptive limits based on threat level', verificationMethod: 'Exceed rate limit — requests rejected with 429; verify no bypass via header manipulation', status: 'implemented' },
] as const;

// ═══════════════════════════════════════════════════════════
// RESIDUAL RISK REGISTER
// ═══════════════════════════════════════════════════════════

export interface ResidualRisk {
  readonly id: string;
  readonly description: string;
  readonly severity: VulnSeverity;
  readonly likelihood: 'very_low' | 'low' | 'medium' | 'high';
  readonly acceptanceRationale: string;
  readonly monitoringPlan: string;
  readonly reviewDate: string;
}

export const RESIDUAL_RISK_REGISTER: readonly ResidualRisk[] = [
  {
    id: 'RR-001', description: 'Sophisticated state-level GNSS spoofing with multi-constellation capability',
    severity: 'high', likelihood: 'very_low',
    acceptanceRationale: 'Requires nation-state resources; IMU/map-matching provides secondary validation; user alerted',
    monitoringPlan: 'Continuous GNSS signal quality monitoring; anomaly detection; industry threat intelligence',
    reviewDate: '2026-Q3',
  },
  {
    id: 'RR-002', description: 'Zero-day vulnerability in TLS implementation',
    severity: 'critical', likelihood: 'very_low',
    acceptanceRationale: 'Using well-audited TLS libraries; defense in depth with application-layer encryption for sensitive data',
    monitoringPlan: 'CVE monitoring; automated dependency scanning; rapid patching SLA (<24h for critical)',
    reviewDate: '2026-Q2',
  },
  {
    id: 'RR-003', description: 'Insider threat with admin access',
    severity: 'high', likelihood: 'low',
    acceptanceRationale: 'Mitigated by audit logging, separation of duties, and periodic access review',
    monitoringPlan: 'Admin action audit; anomalous admin behavior detection; quarterly access review',
    reviewDate: '2026-Q2',
  },
  {
    id: 'RR-004', description: 'Novel adversarial ML attack bypassing current defenses',
    severity: 'medium', likelihood: 'low',
    acceptanceRationale: 'Rule-based fallback ensures safe behavior even if ML is compromised; continuous model monitoring',
    monitoringPlan: 'Model accuracy monitoring; adversarial robustness testing quarterly; academic threat research tracking',
    reviewDate: '2026-Q3',
  },
] as const;

// ═══════════════════════════════════════════════════════════
// SUMMARY FUNCTIONS
// ═══════════════════════════════════════════════════════════

export function vulnerabilitySummary() {
  const total = VULNERABILITY_REGISTER.length;
  const mitigated = VULNERABILITY_REGISTER.filter(v => v.status === 'mitigated').length;
  const open = VULNERABILITY_REGISTER.filter(v => v.status === 'open').length;
  const critical = VULNERABILITY_REGISTER.filter(v => v.severity === 'critical').length;
  const criticalMitigated = VULNERABILITY_REGISTER.filter(v => v.severity === 'critical' && v.status === 'mitigated').length;
  return { total, mitigated, open, critical, criticalMitigated, mitigationRate: total > 0 ? mitigated / total : 0 };
}

export function hardeningSummary() {
  const total = HARDENING_CHECKLIST.length;
  const implemented = HARDENING_CHECKLIST.filter(h => h.status === 'implemented').length;
  return { total, implemented, completionRate: total > 0 ? implemented / total : 0 };
}

export function attackSurfaceSummary() {
  const total = ATTACK_SURFACE_CATALOG.length;
  const lowRisk = ATTACK_SURFACE_CATALOG.filter(a => a.residualRisk === 'low').length;
  const mediumRisk = ATTACK_SURFACE_CATALOG.filter(a => a.residualRisk === 'medium').length;
  return { total, lowRisk, mediumRisk, highRisk: total - lowRisk - mediumRisk };
}
