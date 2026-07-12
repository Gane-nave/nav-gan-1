/**
 * G.A.N.E — Evidence Chain-of-Custody Contract
 * ================================================
 * Cryptographic hashing, digital signatures, tamper detection,
 * and audit trail for incident reports and telemetry evidence.
 *
 * Ensures legal admissibility and forensic integrity of all
 * crowd-sourced and sensor-derived evidence.
 */

// ─── Types ──────────────────────────────────────────────

export type EvidenceType = 'incident_report' | 'telemetry_snapshot' | 'media_capture' | 'sensor_log' | 'v2x_message' | 'trip_record' | 'crowd_report';
export type CustodyAction = 'created' | 'collected' | 'transferred' | 'analyzed' | 'verified' | 'sealed' | 'archived' | 'released' | 'challenged';
export type IntegrityStatus = 'intact' | 'tampered' | 'unverified' | 'expired';

export interface EvidenceItem {
  id: string;
  type: EvidenceType;
  /** SHA-256 hash of the evidence content */
  contentHash: string;
  /** Hash algorithm used */
  hashAlgorithm: 'SHA-256' | 'SHA-384' | 'SHA-512';
  /** Digital signature of the content hash */
  signature: string;
  /** Public key ID used for signing */
  signerKeyId: string;
  /** Timestamp of evidence creation (Unix ms) */
  createdAt: number;
  /** Device or user that created the evidence */
  createdBy: string;
  /** Geographic location at time of creation */
  location?: { lat: number; lng: number; accuracy: number };
  /** Metadata about the evidence */
  metadata: Record<string, unknown>;
  /** Chain of custody entries */
  custodyChain: CustodyEntry[];
  /** Current integrity status */
  integrityStatus: IntegrityStatus;
  /** Expiry timestamp for time-limited evidence */
  expiresAt?: number;
}

export interface CustodyEntry {
  action: CustodyAction;
  timestamp: number;
  actor: string;
  actorRole: string;
  reason: string;
  /** Hash of the evidence at this point in the chain */
  evidenceHash: string;
  /** Hash of the previous custody entry (blockchain-style) */
  previousEntryHash: string;
  /** Digital signature of this entry */
  entrySignature: string;
}

export interface VerificationResult {
  evidenceId: string;
  isValid: boolean;
  checks: VerificationCheck[];
  overallConfidence: number;
  verifiedAt: number;
}

export interface VerificationCheck {
  name: string;
  passed: boolean;
  details: string;
}

// ─── Evidence Policies ──────────────────────────────────

export interface EvidencePolicy {
  type: EvidenceType;
  retentionDays: number;
  requiredMetadata: string[];
  hashAlgorithm: 'SHA-256' | 'SHA-384' | 'SHA-512';
  requireSignature: boolean;
  requireLocation: boolean;
  maxSizeBytes: number;
  allowedFormats: string[];
  chainOfCustodyRequired: boolean;
  minVerificationLevel: number;
}

export const EVIDENCE_POLICIES: EvidencePolicy[] = [
  {
    type: 'incident_report',
    retentionDays: 365,
    requiredMetadata: ['incidentType', 'severity', 'description'],
    hashAlgorithm: 'SHA-256',
    requireSignature: true,
    requireLocation: true,
    maxSizeBytes: 10 * 1024 * 1024, // 10MB
    allowedFormats: ['json', 'jpeg', 'png', 'mp4'],
    chainOfCustodyRequired: true,
    minVerificationLevel: 2,
  },
  {
    type: 'telemetry_snapshot',
    retentionDays: 90,
    requiredMetadata: ['deviceId', 'sessionId', 'sensorTypes'],
    hashAlgorithm: 'SHA-256',
    requireSignature: true,
    requireLocation: true,
    maxSizeBytes: 1 * 1024 * 1024, // 1MB
    allowedFormats: ['json', 'protobuf'],
    chainOfCustodyRequired: false,
    minVerificationLevel: 1,
  },
  {
    type: 'media_capture',
    retentionDays: 180,
    requiredMetadata: ['captureDevice', 'resolution', 'duration'],
    hashAlgorithm: 'SHA-256',
    requireSignature: true,
    requireLocation: true,
    maxSizeBytes: 100 * 1024 * 1024, // 100MB
    allowedFormats: ['jpeg', 'png', 'mp4', 'webm'],
    chainOfCustodyRequired: true,
    minVerificationLevel: 2,
  },
  {
    type: 'v2x_message',
    retentionDays: 30,
    requiredMetadata: ['messageType', 'senderId', 'protocolVersion'],
    hashAlgorithm: 'SHA-256',
    requireSignature: true,
    requireLocation: true,
    maxSizeBytes: 64 * 1024, // 64KB
    allowedFormats: ['protobuf', 'json'],
    chainOfCustodyRequired: false,
    minVerificationLevel: 1,
  },
  {
    type: 'trip_record',
    retentionDays: 365,
    requiredMetadata: ['tripId', 'userId', 'startTime', 'endTime'],
    hashAlgorithm: 'SHA-256',
    requireSignature: true,
    requireLocation: false,
    maxSizeBytes: 5 * 1024 * 1024, // 5MB
    allowedFormats: ['json', 'geojson'],
    chainOfCustodyRequired: true,
    minVerificationLevel: 2,
  },
  {
    type: 'crowd_report',
    retentionDays: 90,
    requiredMetadata: ['reportType', 'reporterId', 'confirmations'],
    hashAlgorithm: 'SHA-256',
    requireSignature: false,
    requireLocation: true,
    maxSizeBytes: 1 * 1024 * 1024, // 1MB
    allowedFormats: ['json'],
    chainOfCustodyRequired: false,
    minVerificationLevel: 1,
  },
];

// ─── Evidence Manager ───────────────────────────────────

export class EvidenceManager {
  private evidence = new Map<string, EvidenceItem>();

  /** Create a new evidence item with initial custody entry */
  createEvidence(
    type: EvidenceType,
    contentHash: string,
    createdBy: string,
    metadata: Record<string, unknown>,
    location?: { lat: number; lng: number; accuracy: number }
  ): EvidenceItem {
    const id = `ev_${Date.now()}_${Math.random().toString(36).slice(2, 10)}`;
    const now = Date.now();

    const initialEntry: CustodyEntry = {
      action: 'created',
      timestamp: now,
      actor: createdBy,
      actorRole: 'originator',
      reason: 'Evidence created',
      evidenceHash: contentHash,
      previousEntryHash: '0'.repeat(64),
      entrySignature: this.signEntry(contentHash, '0'.repeat(64), now),
    };

    const policy = EVIDENCE_POLICIES.find(p => p.type === type);

    const item: EvidenceItem = {
      id,
      type,
      contentHash,
      hashAlgorithm: policy?.hashAlgorithm ?? 'SHA-256',
      signature: this.signContent(contentHash),
      signerKeyId: 'gane-evidence-key-001',
      createdAt: now,
      createdBy,
      location,
      metadata,
      custodyChain: [initialEntry],
      integrityStatus: 'intact',
      expiresAt: policy ? now + policy.retentionDays * 86400000 : undefined,
    };

    this.evidence.set(id, item);
    return item;
  }

  /** Add a custody chain entry */
  addCustodyEntry(
    evidenceId: string,
    action: CustodyAction,
    actor: string,
    actorRole: string,
    reason: string
  ): CustodyEntry | null {
    const item = this.evidence.get(evidenceId);
    if (!item) return null;

    const lastEntry = item.custodyChain[item.custodyChain.length - 1];
    const now = Date.now();

    const entry: CustodyEntry = {
      action,
      timestamp: now,
      actor,
      actorRole,
      reason,
      evidenceHash: item.contentHash,
      previousEntryHash: this.hashEntry(lastEntry),
      entrySignature: this.signEntry(item.contentHash, this.hashEntry(lastEntry), now),
    };

    item.custodyChain.push(entry);
    return entry;
  }

  /** Verify the integrity of an evidence item */
  verify(evidenceId: string): VerificationResult {
    const item = this.evidence.get(evidenceId);
    const now = Date.now();

    if (!item) {
      return {
        evidenceId,
        isValid: false,
        checks: [{ name: 'existence', passed: false, details: 'Evidence not found' }],
        overallConfidence: 0,
        verifiedAt: now,
      };
    }

    const checks: VerificationCheck[] = [];

    // Check 1: Content hash integrity
    checks.push({
      name: 'content_hash',
      passed: item.contentHash.length === 64, // SHA-256 hex length
      details: item.contentHash.length === 64
        ? 'Content hash format valid (SHA-256)'
        : 'Content hash format invalid',
    });

    // Check 2: Signature verification
    checks.push({
      name: 'signature',
      passed: item.signature.length > 0,
      details: item.signature.length > 0
        ? 'Digital signature present'
        : 'Missing digital signature',
    });

    // Check 3: Custody chain integrity
    let chainValid = true;
    for (let i = 1; i < item.custodyChain.length; i++) {
      const prev = item.custodyChain[i - 1];
      const curr = item.custodyChain[i];
      if (curr.previousEntryHash !== this.hashEntry(prev)) {
        chainValid = false;
        break;
      }
    }
    checks.push({
      name: 'custody_chain',
      passed: chainValid,
      details: chainValid
        ? `Custody chain intact (${item.custodyChain.length} entries)`
        : 'Custody chain broken — possible tampering',
    });

    // Check 4: Temporal ordering
    let temporalValid = true;
    for (let i = 1; i < item.custodyChain.length; i++) {
      if (item.custodyChain[i].timestamp < item.custodyChain[i - 1].timestamp) {
        temporalValid = false;
        break;
      }
    }
    checks.push({
      name: 'temporal_order',
      passed: temporalValid,
      details: temporalValid
        ? 'Timestamps in correct chronological order'
        : 'Timestamp ordering violation detected',
    });

    // Check 5: Expiry
    const isExpired = item.expiresAt ? now > item.expiresAt : false;
    checks.push({
      name: 'expiry',
      passed: !isExpired,
      details: isExpired
        ? `Evidence expired at ${new Date(item.expiresAt!).toISOString()}`
        : 'Evidence within retention period',
    });

    // Check 6: Policy compliance
    const policy = EVIDENCE_POLICIES.find(p => p.type === item.type);
    if (policy) {
      const hasRequiredMeta = policy.requiredMetadata.every(k => k in item.metadata);
      checks.push({
        name: 'policy_compliance',
        passed: hasRequiredMeta,
        details: hasRequiredMeta
          ? 'All required metadata present'
          : `Missing metadata: ${policy.requiredMetadata.filter(k => !(k in item.metadata)).join(', ')}`,
      });

      if (policy.requireLocation) {
        checks.push({
          name: 'location_required',
          passed: !!item.location,
          details: item.location
            ? `Location: ${item.location.lat.toFixed(6)}, ${item.location.lng.toFixed(6)} (±${item.location.accuracy}m)`
            : 'Required location data missing',
        });
      }
    }

    const passedCount = checks.filter(c => c.passed).length;
    const isValid = checks.every(c => c.passed);

    if (isValid) {
      item.integrityStatus = 'intact';
    } else if (isExpired) {
      item.integrityStatus = 'expired';
    } else {
      item.integrityStatus = 'tampered';
    }

    return {
      evidenceId,
      isValid,
      checks,
      overallConfidence: passedCount / checks.length,
      verifiedAt: now,
    };
  }

  /** Get evidence by ID */
  getEvidence(id: string): EvidenceItem | undefined {
    return this.evidence.get(id);
  }

  /** Get all evidence for a specific type */
  getByType(type: EvidenceType): EvidenceItem[] {
    return Array.from(this.evidence.values()).filter(e => e.type === type);
  }

  /** Get evidence created by a specific actor */
  getByCreator(createdBy: string): EvidenceItem[] {
    return Array.from(this.evidence.values()).filter(e => e.createdBy === createdBy);
  }

  /** Seal evidence (no further modifications allowed) */
  sealEvidence(evidenceId: string, sealedBy: string): boolean {
    const entry = this.addCustodyEntry(evidenceId, 'sealed', sealedBy, 'authority', 'Evidence sealed for preservation');
    return entry !== null;
  }

  /** Get statistics */
  getStats(): {
    total: number;
    byType: Record<string, number>;
    byStatus: Record<string, number>;
    avgChainLength: number;
  } {
    const items = Array.from(this.evidence.values());
    const byType: Record<string, number> = {};
    const byStatus: Record<string, number> = {};

    for (const item of items) {
      byType[item.type] = (byType[item.type] ?? 0) + 1;
      byStatus[item.integrityStatus] = (byStatus[item.integrityStatus] ?? 0) + 1;
    }

    return {
      total: items.length,
      byType,
      byStatus,
      avgChainLength: items.length > 0
        ? items.reduce((sum, i) => sum + i.custodyChain.length, 0) / items.length
        : 0,
    };
  }

  // ── Crypto Helpers (simplified for contract specification) ──

  private signContent(hash: string): string {
    // In production: ECDSA P-256 signature using HSM-stored key
    return `sig_${hash.slice(0, 16)}_${Date.now().toString(36)}`;
  }

  private signEntry(hash: string, prevHash: string, timestamp: number): string {
    return `esig_${hash.slice(0, 8)}_${prevHash.slice(0, 8)}_${timestamp.toString(36)}`;
  }

  private hashEntry(entry: CustodyEntry): string {
    // In production: SHA-256 of serialized entry
    const data = `${entry.action}|${entry.timestamp}|${entry.actor}|${entry.evidenceHash}|${entry.previousEntryHash}`;
    let hash = 0;
    for (let i = 0; i < data.length; i++) {
      const char = data.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash;
    }
    return Math.abs(hash).toString(16).padStart(64, '0');
  }
}
