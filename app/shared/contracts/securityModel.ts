/**
 * G.A.N.E — Security Model Contract
 * ====================================
 * Formal threat model, trust boundaries, key hierarchy,
 * authentication flows, and RBAC permission matrix.
 */

// ─── Trust Zones ────────────────────────────────────────

export type TrustZone = 'untrusted' | 'edge' | 'dmz' | 'internal' | 'privileged' | 'critical';

export interface TrustBoundary {
  id: string;
  name: string;
  fromZone: TrustZone;
  toZone: TrustZone;
  protocol: string;
  authRequired: boolean;
  encryptionRequired: boolean;
  rateLimited: boolean;
  validationRules: string[];
}

export const TRUST_BOUNDARIES: TrustBoundary[] = [
  {
    id: 'tb-client-api',
    name: 'Client → API Gateway',
    fromZone: 'untrusted',
    toZone: 'dmz',
    protocol: 'HTTPS/TLS 1.3',
    authRequired: false, // public endpoints exist
    encryptionRequired: true,
    rateLimited: true,
    validationRules: [
      'Payload size < 1MB',
      'Content-Type must be application/json',
      'Coordinate bounds: lat [-90,90], lng [-180,180]',
      'Device ID format: UUID v4',
      'Request rate: 100/min per IP',
    ],
  },
  {
    id: 'tb-api-auth',
    name: 'API Gateway → Auth Service',
    fromZone: 'dmz',
    toZone: 'internal',
    protocol: 'Internal HTTPS',
    authRequired: true,
    encryptionRequired: true,
    rateLimited: true,
    validationRules: [
      'JWT signature verification (RS256)',
      'Token expiry check (max 24h)',
      'Issuer validation',
      'Audience validation',
    ],
  },
  {
    id: 'tb-api-db',
    name: 'API Server → Database',
    fromZone: 'internal',
    toZone: 'critical',
    protocol: 'TLS-encrypted SQL',
    authRequired: true,
    encryptionRequired: true,
    rateLimited: true,
    validationRules: [
      'Connection pool limit: 50',
      'Query timeout: 30s',
      'Parameterized queries only (no string concatenation)',
      'Row-level security for multi-tenant queries',
    ],
  },
  {
    id: 'tb-api-ml',
    name: 'API Server → ML Service',
    fromZone: 'internal',
    toZone: 'internal',
    protocol: 'gRPC/TLS',
    authRequired: true,
    encryptionRequired: true,
    rateLimited: true,
    validationRules: [
      'Model version pinning',
      'Input feature validation',
      'Output range checking',
      'Inference timeout: 500ms',
    ],
  },
  {
    id: 'tb-v2x',
    name: 'V2X Network → Edge Processor',
    fromZone: 'untrusted',
    toZone: 'edge',
    protocol: 'DSRC/C-V2X (IEEE 1609.2)',
    authRequired: true,
    encryptionRequired: true,
    rateLimited: true,
    validationRules: [
      'PKI certificate chain validation',
      'Message freshness (< 500ms)',
      'Plausibility check on position/speed',
      'Misbehavior detection',
    ],
  },
  {
    id: 'tb-admin',
    name: 'Admin Panel → Admin API',
    fromZone: 'untrusted',
    toZone: 'privileged',
    protocol: 'HTTPS/TLS 1.3',
    authRequired: true,
    encryptionRequired: true,
    rateLimited: true,
    validationRules: [
      'Admin role verification',
      'MFA required for destructive operations',
      'Audit log for all admin actions',
      'IP allowlist (configurable)',
      'Session timeout: 1 hour',
    ],
  },
];

// ─── Threat Model (STRIDE) ─────────────────────────────

export type STRIDECategory = 'spoofing' | 'tampering' | 'repudiation' | 'information_disclosure' | 'denial_of_service' | 'elevation_of_privilege';

export interface Threat {
  id: string;
  category: STRIDECategory;
  name: string;
  description: string;
  affectedAssets: string[];
  likelihood: 'low' | 'medium' | 'high' | 'critical';
  impact: 'low' | 'medium' | 'high' | 'critical';
  riskScore: number; // 1-25
  mitigations: string[];
  residualRisk: string;
}

export const THREAT_MODEL: Threat[] = [
  {
    id: 'threat-gnss-spoofing',
    category: 'spoofing',
    name: 'GNSS Signal Spoofing',
    description: 'Attacker broadcasts fake GNSS signals to manipulate vehicle position',
    affectedAssets: ['ESKF', 'Navigation', 'RouteGraph'],
    likelihood: 'medium',
    impact: 'critical',
    riskScore: 20,
    mitigations: [
      'Multi-constellation cross-validation',
      'IMU/PDR consistency checking',
      'RAIM (Receiver Autonomous Integrity Monitoring)',
      'Authenticated GNSS signals (Galileo OSNMA)',
      'Anomaly detection on position jumps',
    ],
    residualRisk: 'Sophisticated attacks may still succeed briefly before detection',
  },
  {
    id: 'threat-replay-attack',
    category: 'tampering',
    name: 'Telemetry Replay Attack',
    description: 'Attacker replays captured telemetry to inject false traffic data',
    affectedAssets: ['TelemetryPipeline', 'CrowdIntelligence', 'TrafficPipeline'],
    likelihood: 'medium',
    impact: 'high',
    riskScore: 15,
    mitigations: [
      'Nonce-based request deduplication',
      'Timestamp freshness validation (< 60s)',
      'Device attestation tokens',
      'Rate limiting per device ID',
    ],
    residualRisk: 'Real-time replay within freshness window may pass validation',
  },
  {
    id: 'threat-data-exfiltration',
    category: 'information_disclosure',
    name: 'Location Data Exfiltration',
    description: 'Unauthorized access to user location history and trip data',
    affectedAssets: ['UserData', 'TripHistory', 'LiveSharing'],
    likelihood: 'medium',
    impact: 'critical',
    riskScore: 20,
    mitigations: [
      'Encryption at rest (AES-256-GCM)',
      'Row-level security in database',
      'Privacy engine: k-anonymity, differential privacy',
      'Data retention policies (auto-purge after 90 days)',
      'Audit logging for all data access',
    ],
    residualRisk: 'Insider threat with database access',
  },
  {
    id: 'threat-ddos',
    category: 'denial_of_service',
    name: 'DDoS on API Gateway',
    description: 'Volumetric attack overwhelming API infrastructure',
    affectedAssets: ['APIGateway', 'TelemetryPipeline', 'AllServices'],
    likelihood: 'high',
    impact: 'high',
    riskScore: 16,
    mitigations: [
      'CDN/WAF with DDoS protection',
      'Rate limiting per IP/device',
      'Backpressure controller with graceful degradation',
      'Geographic traffic filtering',
      'Auto-scaling with cost caps',
    ],
    residualRisk: 'Sustained sophisticated L7 attacks may cause brief degradation',
  },
  {
    id: 'threat-privilege-escalation',
    category: 'elevation_of_privilege',
    name: 'Admin Privilege Escalation',
    description: 'Regular user gains admin access through API manipulation',
    affectedAssets: ['AdminPanel', 'UserManagement', 'SystemConfig'],
    likelihood: 'low',
    impact: 'critical',
    riskScore: 15,
    mitigations: [
      'Server-side role verification on every admin procedure',
      'Admin role set only by owner (not self-assignable)',
      'Audit log for all role changes',
      'Separate admin procedures with role middleware',
    ],
    residualRisk: 'Compromised owner account',
  },
  {
    id: 'threat-v2x-misbehavior',
    category: 'tampering',
    name: 'V2X Misbehavior (False BSM)',
    description: 'Compromised vehicle broadcasts false Basic Safety Messages',
    affectedAssets: ['V2XEngine', 'IncidentEngine', 'DigitalTwin'],
    likelihood: 'medium',
    impact: 'high',
    riskScore: 15,
    mitigations: [
      'Misbehavior detection algorithms',
      'PKI certificate revocation',
      'Plausibility checking (physics-based)',
      'Reputation scoring for V2X peers',
      'N-of-M confirmation for safety-critical messages',
    ],
    residualRisk: 'Coordinated multi-vehicle attacks',
  },
  {
    id: 'threat-supply-chain',
    category: 'tampering',
    name: 'Supply Chain Attack (Dependencies)',
    description: 'Malicious code injected through compromised npm packages',
    affectedAssets: ['BuildPipeline', 'AllModules'],
    likelihood: 'low',
    impact: 'critical',
    riskScore: 12,
    mitigations: [
      'Dependency lockfile with integrity hashes',
      'Automated vulnerability scanning (npm audit)',
      'Minimal dependency policy',
      'SRI (Subresource Integrity) for CDN assets',
      'Regular dependency review and pruning',
    ],
    residualRisk: 'Zero-day in widely-used package before patch',
  },
  {
    id: 'threat-session-hijack',
    category: 'spoofing',
    name: 'Session Hijacking',
    description: 'Attacker steals session cookie to impersonate user',
    affectedAssets: ['Authentication', 'UserData', 'AdminPanel'],
    likelihood: 'medium',
    impact: 'high',
    riskScore: 15,
    mitigations: [
      'HttpOnly, Secure, SameSite=Strict cookies',
      'Short session TTL (24h)',
      'Session binding to device fingerprint',
      'Anomaly detection on session usage patterns',
    ],
    residualRisk: 'XSS vulnerability could bypass HttpOnly',
  },
];

// ─── Key Hierarchy ──────────────────────────────────────

export interface CryptoKey {
  id: string;
  name: string;
  algorithm: string;
  purpose: string;
  rotationPeriod: string;
  storage: string;
  accessControl: string[];
}

export const KEY_HIERARCHY: CryptoKey[] = [
  {
    id: 'key-master',
    name: 'Master Encryption Key (MEK)',
    algorithm: 'AES-256-GCM',
    purpose: 'Encrypts all Data Encryption Keys (DEKs)',
    rotationPeriod: '365 days',
    storage: 'HSM / KMS',
    accessControl: ['Platform team only', 'Requires 2-of-3 key custodians'],
  },
  {
    id: 'key-dek-user',
    name: 'User Data DEK',
    algorithm: 'AES-256-GCM',
    purpose: 'Encrypts user PII and location data at rest',
    rotationPeriod: '90 days',
    storage: 'Encrypted by MEK in database',
    accessControl: ['API server process', 'Decrypted on-demand'],
  },
  {
    id: 'key-jwt-signing',
    name: 'JWT Signing Key',
    algorithm: 'RS256 (RSA 2048)',
    purpose: 'Signs authentication tokens',
    rotationPeriod: '30 days (with overlap)',
    storage: 'Environment variable (JWT_SECRET)',
    accessControl: ['Auth service only'],
  },
  {
    id: 'key-api-hmac',
    name: 'API Request HMAC Key',
    algorithm: 'HMAC-SHA256',
    purpose: 'Signs API requests for integrity verification',
    rotationPeriod: '90 days',
    storage: 'Environment variable',
    accessControl: ['API gateway', 'Client SDK'],
  },
  {
    id: 'key-v2x-pki',
    name: 'V2X PKI Certificate',
    algorithm: 'ECDSA P-256',
    purpose: 'Signs and verifies V2X messages (IEEE 1609.2)',
    rotationPeriod: '7 days (pseudonym certificates)',
    storage: 'Secure element / TPM',
    accessControl: ['V2X engine only', 'Hardware-bound'],
  },
  {
    id: 'key-offline-sync',
    name: 'Offline Sync Encryption Key',
    algorithm: 'AES-256-GCM',
    purpose: 'Encrypts offline queue data on device',
    rotationPeriod: 'Per-session',
    storage: 'Device keychain / secure storage',
    accessControl: ['App process only', 'Biometric unlock'],
  },
];

// ─── RBAC Permission Matrix ─────────────────────────────

export type Role = 'anonymous' | 'user' | 'fleet_operator' | 'admin' | 'owner';
export type Permission =
  | 'read:map'
  | 'read:traffic'
  | 'read:weather'
  | 'write:incident'
  | 'read:incident'
  | 'write:crowd_report'
  | 'read:crowd_data'
  | 'write:trip'
  | 'read:trip'
  | 'read:own_analytics'
  | 'write:live_share'
  | 'read:live_share'
  | 'write:route'
  | 'read:route'
  | 'manage:fleet'
  | 'read:fleet_analytics'
  | 'manage:users'
  | 'manage:content'
  | 'manage:config'
  | 'manage:feature_flags'
  | 'read:admin_analytics'
  | 'manage:system'
  | 'execute:ai_commands'
  | 'manage:moderation'
  | 'read:audit_log'
  | 'manage:keys'
  | 'manage:billing';

export const RBAC_MATRIX: Record<Role, Permission[]> = {
  anonymous: [
    'read:map',
    'read:traffic',
    'read:weather',
    'read:incident',
  ],
  user: [
    'read:map',
    'read:traffic',
    'read:weather',
    'write:incident',
    'read:incident',
    'write:crowd_report',
    'read:crowd_data',
    'write:trip',
    'read:trip',
    'read:own_analytics',
    'write:live_share',
    'read:live_share',
    'write:route',
    'read:route',
  ],
  fleet_operator: [
    'read:map',
    'read:traffic',
    'read:weather',
    'write:incident',
    'read:incident',
    'write:crowd_report',
    'read:crowd_data',
    'write:trip',
    'read:trip',
    'read:own_analytics',
    'write:live_share',
    'read:live_share',
    'write:route',
    'read:route',
    'manage:fleet',
    'read:fleet_analytics',
  ],
  admin: [
    'read:map',
    'read:traffic',
    'read:weather',
    'write:incident',
    'read:incident',
    'write:crowd_report',
    'read:crowd_data',
    'write:trip',
    'read:trip',
    'read:own_analytics',
    'write:live_share',
    'read:live_share',
    'write:route',
    'read:route',
    'manage:fleet',
    'read:fleet_analytics',
    'manage:users',
    'manage:content',
    'manage:config',
    'manage:feature_flags',
    'read:admin_analytics',
    'manage:moderation',
    'read:audit_log',
  ],
  owner: [
    'read:map',
    'read:traffic',
    'read:weather',
    'write:incident',
    'read:incident',
    'write:crowd_report',
    'read:crowd_data',
    'write:trip',
    'read:trip',
    'read:own_analytics',
    'write:live_share',
    'read:live_share',
    'write:route',
    'read:route',
    'manage:fleet',
    'read:fleet_analytics',
    'manage:users',
    'manage:content',
    'manage:config',
    'manage:feature_flags',
    'read:admin_analytics',
    'manage:system',
    'execute:ai_commands',
    'manage:moderation',
    'read:audit_log',
    'manage:keys',
    'manage:billing',
  ],
};

// ─── Auth Flow Definitions ──────────────────────────────

export interface AuthFlow {
  id: string;
  name: string;
  steps: string[];
  tokenType: string;
  sessionDuration: string;
  refreshStrategy: string;
}

export const AUTH_FLOWS: AuthFlow[] = [
  {
    id: 'flow-oauth',
    name: 'Manus OAuth 2.0 (Primary)',
    steps: [
      '1. Client redirects to VITE_OAUTH_PORTAL_URL with state=origin+returnPath',
      '2. User authenticates on Manus OAuth portal',
      '3. OAuth portal redirects to /api/oauth/callback with authorization code',
      '4. Server exchanges code for access token via OAUTH_SERVER_URL',
      '5. Server fetches user profile, upserts in database',
      '6. Server issues signed JWT session cookie (HttpOnly, Secure, SameSite)',
      '7. Client reads auth state via trpc.auth.me.useQuery()',
    ],
    tokenType: 'JWT (RS256)',
    sessionDuration: '24 hours',
    refreshStrategy: 'Silent re-auth on expiry; redirect to login if refresh fails',
  },
  {
    id: 'flow-device',
    name: 'Device Attestation (IoT/Fleet)',
    steps: [
      '1. Device generates keypair in secure element',
      '2. Device sends CSR to enrollment endpoint',
      '3. Server validates device identity and issues certificate',
      '4. Device uses mTLS for all subsequent API calls',
      '5. Certificate renewal every 30 days',
    ],
    tokenType: 'X.509 Certificate (ECDSA P-256)',
    sessionDuration: '30 days (certificate lifetime)',
    refreshStrategy: 'Automatic certificate renewal 7 days before expiry',
  },
  {
    id: 'flow-anonymous',
    name: 'Anonymous Session',
    steps: [
      '1. Client generates random session ID',
      '2. Server assigns anonymous role with limited permissions',
      '3. Rate limiting enforced per session ID',
      '4. No PII stored; session data ephemeral',
    ],
    tokenType: 'Ephemeral session token',
    sessionDuration: '1 hour',
    refreshStrategy: 'New session on expiry; no state preservation',
  },
  {
    id: 'flow-v2x',
    name: 'V2X PKI Authentication',
    steps: [
      '1. Vehicle obtains pseudonym certificates from SCMS',
      '2. Each BSM signed with current pseudonym certificate',
      '3. Receiver validates certificate chain and CRL',
      '4. Pseudonym rotation every 5 minutes for privacy',
    ],
    tokenType: 'IEEE 1609.2 Certificate',
    sessionDuration: '5 minutes (pseudonym rotation)',
    refreshStrategy: 'Pre-fetched certificate pool; rotate on schedule',
  },
];

// ─── Identity Model ─────────────────────────────────────

export type IdentityType = 'anonymous' | 'authenticated_user' | 'fleet_device' | 'fleet_operator' | 'service_account' | 'admin' | 'owner';

export interface Identity {
  type: IdentityType;
  description: string;
  authMethod: string;
  dataIsolation: string;
  quotas: Record<string, number>;
  capabilities: string[];
}

export const IDENTITY_MODEL: Identity[] = [
  {
    type: 'anonymous',
    description: 'Unauthenticated user with read-only access to public data',
    authMethod: 'None (ephemeral session)',
    dataIsolation: 'No persistent data; session-scoped only',
    quotas: { requestsPerMinute: 30, maxTrips: 0, maxShares: 0 },
    capabilities: ['View map', 'View traffic', 'View weather', 'View incidents'],
  },
  {
    type: 'authenticated_user',
    description: 'Logged-in individual user with full personal features',
    authMethod: 'Manus OAuth 2.0',
    dataIsolation: 'Row-level security; own data only',
    quotas: { requestsPerMinute: 100, maxTrips: 1000, maxShares: 50, maxIncidents: 20 },
    capabilities: ['All anonymous', 'Create trips', 'Report incidents', 'Live sharing', 'Route planning', 'Analytics'],
  },
  {
    type: 'fleet_device',
    description: 'IoT device (vehicle, sensor) reporting telemetry',
    authMethod: 'mTLS with device certificate',
    dataIsolation: 'Tenant-scoped; fleet data only',
    quotas: { requestsPerMinute: 300, telemetryEventsPerSecond: 10 },
    capabilities: ['Send telemetry', 'Receive commands', 'Report position', 'V2X messaging'],
  },
  {
    type: 'fleet_operator',
    description: 'Fleet management user with access to fleet-wide data',
    authMethod: 'Manus OAuth 2.0 + fleet role',
    dataIsolation: 'Fleet-scoped; all devices in fleet',
    quotas: { requestsPerMinute: 200, maxDevices: 10000 },
    capabilities: ['All user', 'Fleet management', 'Fleet analytics', 'Device provisioning', 'Geofence management'],
  },
  {
    type: 'service_account',
    description: 'Backend service for inter-service communication',
    authMethod: 'API key + HMAC',
    dataIsolation: 'Service-scoped; defined by service permissions',
    quotas: { requestsPerMinute: 10000 },
    capabilities: ['Internal API access', 'Data pipeline operations', 'ML model serving'],
  },
  {
    type: 'admin',
    description: 'Platform administrator with management capabilities',
    authMethod: 'Manus OAuth 2.0 + admin role',
    dataIsolation: 'Cross-tenant read access; write restricted to admin operations',
    quotas: { requestsPerMinute: 500 },
    capabilities: ['All user', 'User management', 'Content moderation', 'Config management', 'Analytics dashboard'],
  },
  {
    type: 'owner',
    description: 'Platform owner with full system control',
    authMethod: 'Manus OAuth 2.0 + owner verification',
    dataIsolation: 'Full access to all data',
    quotas: { requestsPerMinute: 1000 },
    capabilities: ['All admin', 'System management', 'AI command bot', 'Key management', 'Billing', 'Feature flags'],
  },
];

// ─── Multi-Tenant Isolation ─────────────────────────────

export interface TenantIsolation {
  layer: string;
  mechanism: string;
  enforcement: string;
  bypassConditions: string[];
}

export const TENANT_ISOLATION: TenantIsolation[] = [
  {
    layer: 'Database',
    mechanism: 'Row-level security with tenant_id column',
    enforcement: 'All queries filtered by ctx.user.tenantId; enforced in Drizzle query helpers',
    bypassConditions: ['Admin cross-tenant queries with audit log', 'System maintenance operations'],
  },
  {
    layer: 'API',
    mechanism: 'Tenant context injected in tRPC middleware',
    enforcement: 'protectedProcedure validates tenant scope; cross-tenant access denied',
    bypassConditions: ['Admin procedures with explicit tenant parameter'],
  },
  {
    layer: 'Storage',
    mechanism: 'S3 key prefix: tenants/{tenantId}/...',
    enforcement: 'Storage helpers prepend tenant prefix; IAM policies enforce prefix isolation',
    bypassConditions: ['Platform-level backup operations'],
  },
  {
    layer: 'Cache',
    mechanism: 'Cache key prefix: {tenantId}:{resourceType}:{resourceId}',
    enforcement: 'Cache helpers enforce tenant prefix; no cross-tenant cache reads',
    bypassConditions: ['Global cache entries (weather, traffic) shared across tenants'],
  },
  {
    layer: 'Rate Limiting',
    mechanism: 'Per-tenant quota pools',
    enforcement: 'Backpressure controller tracks quotas per tenant; independent exhaustion',
    bypassConditions: ['Emergency override by owner'],
  },
  {
    layer: 'Logging',
    mechanism: 'Tenant ID in all log entries and trace spans',
    enforcement: 'Observability middleware injects tenantId into every span',
    bypassConditions: ['System-level logs without tenant context'],
  },
];

// ─── Helper: Check Permission ───────────────────────────

export function hasPermission(role: Role, permission: Permission): boolean {
  return RBAC_MATRIX[role]?.includes(permission) ?? false;
}

export function getPermissions(role: Role): Permission[] {
  return RBAC_MATRIX[role] ?? [];
}

export function getRiskScore(threatId: string): number {
  return THREAT_MODEL.find(t => t.id === threatId)?.riskScore ?? 0;
}

export function getCriticalThreats(): Threat[] {
  return THREAT_MODEL.filter(t => t.riskScore >= 15);
}
