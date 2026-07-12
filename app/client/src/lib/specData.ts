// G.A.N.E Specification Data
// Design Philosophy: Dark Intelligence Dashboard — Aerospace HUD aesthetic

export interface SpecSection {
  id: string;
  number: string;
  title: string;
  content: string;
  subsections?: SpecSubsection[];
}

export interface SpecSubsection {
  id: string;
  number: string;
  title: string;
  content: string;
  items?: string[];
  table?: { headers: string[]; rows: string[][] };
  code?: string;
}

export interface SpecPart {
  id: string;
  partNumber: string;
  romanNumeral: string;
  title: string;
  description: string;
  sections: SpecSection[];
  color: 'indigo' | 'teal' | 'amber' | 'green' | 'rose';
}

export const specParts: SpecPart[] = [
  {
    id: 'part-i',
    partNumber: 'I',
    romanNumeral: 'I',
    title: 'System Foundations',
    description: 'Mission, architectural principles, invariants, constraints, and failure domains.',
    color: 'indigo',
    sections: [
      {
        id: 'i-1',
        number: '1',
        title: 'System Mission & Scope',
        content: 'The Global Autonomous Navigation Ecosystem (G.A.N.E) is a decentralized, planet-scale infrastructure designed for the real-time management, prediction, control, and regulation of global traffic networks. G.A.N.E functions as a comprehensive control system — not merely a navigation application — creating a unified, intelligent mobility ecosystem that operates at continental scale with defense-grade reliability.',
        subsections: [
          {
            id: 'i-1-1',
            number: '1.1',
            title: 'Core Capability Matrix',
            content: 'The system encompasses 26 primary capability domains, each with defined interfaces and SLA tiers.',
            table: {
              headers: ['Capability Domain', 'Classification', 'SLA Tier', 'Offline Support'],
              rows: [
                ['Probabilistic Routing Core', 'Safety-Critical', 'Tier 1 (<200ms)', 'Full'],
                ['Network Congestion Orchestration', 'Safety-Critical', 'Tier 1 (<200ms)', 'Partial'],
                ['Road Discovery Engine', 'Infrastructure', 'Tier 2 (<1s)', 'Full'],
                ['Event-Driven Architecture', 'Core Platform', 'Tier 1 (<200ms)', 'Full'],
                ['Evidence-Native Trust Layer', 'Security', 'Tier 2 (<1s)', 'Full'],
                ['EMS & Fleet Operations', 'Safety-Critical', 'Tier 1 (<200ms)', 'Full'],
                ['Satellite Fallback', 'Resilience', 'Tier 3 (best-effort)', 'N/A'],
                ['Digital Twin & City Simulation', 'Analytics', 'Tier 3 (async)', 'Partial'],
                ['Risk Engine', 'Safety-Critical', 'Tier 1 (<200ms)', 'Full'],
                ['Human State Intelligence', 'Safety-Critical', 'Tier 2 (<500ms)', 'Full'],
                ['Energy & Environmental Layer', 'Optimization', 'Tier 2 (<1s)', 'Partial'],
                ['Security & Governance', 'Core Platform', 'Tier 1 (<200ms)', 'Full'],
              ]
            }
          }
        ]
      },
      {
        id: 'i-2',
        number: '2',
        title: 'Non-Negotiable Architectural Principles',
        content: 'These principles are immutable constraints on the architecture. Any design decision that violates them is rejected regardless of performance or cost implications.',
        subsections: [
          {
            id: 'i-2-principles',
            number: '2.1–2.13',
            title: 'Thirteen Foundational Principles',
            content: '',
            table: {
              headers: ['#', 'Principle', 'Enforcement Mechanism', 'Violation Consequence'],
              rows: [
                ['2.1', 'Canonical Single Source of Truth', 'Schema Registry + Entity Versioning', 'Data inconsistency → system halt'],
                ['2.2', 'Event-Driven Only Architecture', 'No direct RPC between core planes', 'Coupling → cascading failures'],
                ['2.3', 'Probabilistic Outputs', 'All ETAs carry confidence intervals', 'False certainty → user harm'],
                ['2.4', 'Network Stability Precedence', 'Flow control overrides individual routing', 'Congestion collapse'],
                ['2.5', 'Closed-Loop Control', 'Measure → Predict → Allocate → Monitor → Correct', 'Open-loop drift'],
                ['2.6', 'Evidence-First Trust', 'No trust without signed evidence chain', 'Manipulation vulnerability'],
                ['2.7', 'Partition Tolerant Design', 'CAP: AP with eventual consistency', 'Availability loss'],
                ['2.8', 'Offline-First Execution', 'Edge autonomy for all Tier 1 ops', 'Connectivity dependency'],
                ['2.9', 'Satellite Minimal Critical Channel', 'SOS + dispatch over satellite', 'Blackout vulnerability'],
                ['2.10', 'Deterministic Conflict Resolution', 'CRDT + timestamp-ordered merge', 'Non-deterministic state'],
                ['2.11', 'Schema-Strict Data Governance', 'Avro/Protobuf with registry enforcement', 'Schema drift'],
                ['2.12', 'Idempotent Processing Everywhere', 'Idempotency keys on all events', 'Duplicate operations'],
                ['2.13', 'Multi-Tier SLA Classification', 'Safety/Routing/Analytics tiers', 'Priority inversion'],
              ]
            }
          }
        ]
      },
      {
        id: 'i-3',
        number: '3',
        title: 'System Invariants',
        content: 'System invariants are fundamental properties that must hold true at all times. They are verified continuously by the monitoring subsystem and any violation triggers an immediate alert to the operations team.',
        subsections: [
          {
            id: 'i-3-inv',
            number: '3.1–3.6',
            title: 'Core Invariants',
            content: '',
            items: [
              'INV-01: All events carry a globally monotonic UTC timestamp. Clock skew tolerance: ±50ms.',
              'INV-02: The immutable event log is append-only. No event may be modified or deleted after commit.',
              'INV-03: Conservation of flow — total ingress equals total egress at any network boundary over a 60-second window.',
              'INV-04: Trust scores are monotonically non-decreasing within a session absent explicit revocation.',
              'INV-05: Every entity in the canonical model has exactly one authoritative region at any point in time.',
              'INV-06: All routing decisions carry an uncertainty bound. Deterministic ETAs are prohibited.',
            ]
          }
        ]
      },
      {
        id: 'i-4',
        number: '4',
        title: 'Global Constraints',
        content: 'Global constraints are system-wide rules enforced at the policy layer. They represent hard limits that cannot be overridden by individual subsystems.',
        subsections: [
          {
            id: 'i-4-constraints',
            number: '4.1',
            title: 'Hard Constraints vs Tunable Parameters',
            content: '',
            table: {
              headers: ['Constraint', 'Type', 'Default Value', 'Override Authority'],
              rows: [
                ['Max corridor density threshold', 'Tunable', '0.85 (85% capacity)', 'Regional Operator'],
                ['Minimum ETA confidence interval width', 'Hard', 'σ ≥ 0.05 × μ', 'None'],
                ['Event replay window', 'Tunable', '72 hours', 'Platform Admin'],
                ['Max reroute quota per corridor per minute', 'Tunable', '15%', 'Flow Orchestrator'],
                ['Data residency boundary enforcement', 'Hard', 'Per-region', 'None'],
                ['Satellite packet max size', 'Hard', '512 bytes', 'None'],
                ['Edge autonomy minimum map cache', 'Hard', '50km radius', 'None'],
                ['Trust score floor for evidence submission', 'Tunable', '0.1', 'Trust Admin'],
              ]
            }
          }
        ]
      },
      {
        id: 'i-5',
        number: '5',
        title: 'Failure Domains',
        content: 'The system is partitioned into independent failure domains. A failure in one domain must not cascade to others. Domain boundaries are enforced through bulkhead patterns, circuit breakers, and independent deployment units.',
        subsections: [
          {
            id: 'i-5-domains',
            number: '5.1',
            title: 'Domain Isolation Matrix',
            content: '',
            table: {
              headers: ['Domain', 'Blast Radius', 'Recovery Target (RTO)', 'Recovery Point (RPO)'],
              rows: [
                ['Edge Device (single)', 'Single user', '0s (local fallback)', '0s (local state)'],
                ['Edge Cluster (city)', 'City-level routing', '30s', '5s'],
                ['Regional Datacenter', 'Regional services', '5min', '30s'],
                ['Global Federation Plane', 'Cross-region sync', '15min', '5min'],
                ['Control Plane', 'Policy distribution', '10min', '1min'],
                ['Trust & Evidence Plane', 'Trust scoring', '30min', '10min'],
              ]
            }
          }
        ]
      },
      {
        id: 'i-6',
        number: '6',
        title: 'Consistency Boundaries',
        content: 'The G.A.N.E employs a tiered consistency model. Safety-critical operations require strong consistency within a region, while analytics and social data tolerate eventual consistency globally. This is a deliberate trade-off between availability and consistency, governed by the CAP theorem.',
        subsections: [
          {
            id: 'i-6-consistency',
            number: '6.1',
            title: 'Consistency Model by Data Class',
            content: '',
            table: {
              headers: ['Data Class', 'Consistency Model', 'Scope', 'Conflict Resolution'],
              rows: [
                ['Safety events', 'Linearizable', 'Regional', 'Last-writer-wins + timestamp'],
                ['Route plans', 'Sequential', 'Regional', 'Version vector merge'],
                ['Map topology', 'Causal', 'Global', 'Trust-weighted merge'],
                ['Trust scores', 'Eventual', 'Global', 'CRDT counter'],
                ['Analytics', 'Eventual', 'Global', 'Commutative merge'],
                ['Social messages', 'Eventual', 'Regional', 'Append-only'],
              ]
            }
          }
        ]
      }
    ]
  },
  {
    id: 'part-ii',
    partNumber: 'II',
    romanNumeral: 'II',
    title: 'Canonical Data Model',
    description: 'Entity definitions, field semantics, versioning, event-sourcing, and conflict resolution.',
    color: 'teal',
    sections: [
      {
        id: 'ii-1',
        number: '1',
        title: 'Entity Definitions',
        content: 'The canonical data model defines 27 core entities. Every entity carries a UUID, versioned schema, immutable change log, region metadata, and trust metadata. Entities are never deleted — they are archived with a tombstone event.',
        subsections: [
          {
            id: 'ii-1-entities',
            number: '1.1',
            title: 'Core Entity Registry',
            content: '',
            table: {
              headers: ['Entity', 'Category', 'Primary Key', 'Mutability', 'Trust Required'],
              rows: [
                ['User', 'Identity', 'UUID v4', 'Mutable (versioned)', 'N/A'],
                ['MobilityIdentity', 'Identity', 'UUID v4', 'Mutable (versioned)', 'Low'],
                ['Device', 'Infrastructure', 'UUID v4 + fingerprint', 'Mutable (versioned)', 'Medium'],
                ['VehicleProfile', 'Transport', 'UUID v4', 'Mutable (versioned)', 'Medium'],
                ['DriverState', 'Behavioral', 'UUID v4 + timestamp', 'Append-only', 'High'],
                ['RoadGraph', 'Topology', 'H3 cell + version', 'Versioned snapshot', 'High'],
                ['LaneGraph', 'Topology', 'H3 cell + lane_id', 'Versioned snapshot', 'High'],
                ['IndoorGraph', 'Topology', 'Venue UUID + floor', 'Versioned snapshot', 'Medium'],
                ['MapTile', 'Topology', 'Tile XYZ + version', 'Versioned snapshot', 'Medium'],
                ['RoutePlan', 'Navigation', 'UUID v4', 'Immutable after commit', 'Low'],
                ['Segment', 'Topology', 'H3 edge ID', 'Versioned snapshot', 'High'],
                ['Corridor', 'Topology', 'UUID v4', 'Versioned snapshot', 'High'],
                ['Incident', 'Event', 'UUID v4', 'Lifecycle-managed', 'Medium'],
                ['Evidence', 'Trust', 'UUID v4 + hash', 'Immutable', 'N/A'],
                ['RiskScore', 'Intelligence', 'Entity UUID + timestamp', 'Append-only', 'High'],
                ['TrustScore', 'Trust', 'Entity UUID + timestamp', 'Append-only', 'N/A'],
                ['StabilityScore', 'Intelligence', 'Corridor UUID + timestamp', 'Append-only', 'High'],
                ['FlowControlBias', 'Control', 'Corridor UUID + timestamp', 'Append-only', 'High'],
                ['Policy', 'Governance', 'UUID v4 + version', 'Versioned', 'Admin'],
                ['Task', 'Operations', 'UUID v4', 'Lifecycle-managed', 'Medium'],
                ['Assignment', 'Operations', 'UUID v4', 'Lifecycle-managed', 'Medium'],
                ['Dispatch', 'EMS/Fleet', 'UUID v4', 'Lifecycle-managed', 'High'],
                ['EnergyProfile', 'Environment', 'Vehicle UUID + timestamp', 'Append-only', 'Low'],
                ['CityState', 'Digital Twin', 'City UUID + timestamp', 'Append-only', 'High'],
                ['ModelArtifact', 'ML', 'UUID v4 + hash', 'Immutable', 'Admin'],
                ['AuditLog', 'Governance', 'UUID v4 + timestamp', 'Immutable', 'N/A'],
                ['ReputationLedger', 'Trust', 'User UUID', 'Append-only', 'N/A'],
              ]
            }
          }
        ]
      },
      {
        id: 'ii-2',
        number: '2',
        title: 'Field-Level Semantics',
        content: 'Every field in the canonical model has a precisely defined semantic. Ambiguity at the field level is a source of systemic inconsistency. The following conventions are enforced across all entities.',
        subsections: [
          {
            id: 'ii-2-fields',
            number: '2.1',
            title: 'Universal Field Conventions',
            content: '',
            code: `// Universal Entity Base Schema (Protobuf 3 notation)
message EntityBase {
  string  entity_id       = 1;  // UUID v4, immutable after creation
  string  entity_type     = 2;  // Fully-qualified type name
  int64   schema_version  = 3;  // Monotonic integer, per entity type
  int64   created_at_utc  = 4;  // Unix epoch microseconds
  int64   updated_at_utc  = 5;  // Unix epoch microseconds
  string  region_id       = 6;  // ISO 3166-1 alpha-2 + subdivision
  string  authority_node  = 7;  // Node ID of authoritative replica
  float   trust_weight    = 8;  // [0.0, 1.0] — data source trust
  bytes   signature       = 9;  // Ed25519 signature of payload hash
  string  idempotency_key = 10; // SHA-256 of (entity_id + event_type + timestamp)
}`
          }
        ]
      },
      {
        id: 'ii-3',
        number: '3',
        title: 'Versioning Model',
        content: 'Schema evolution follows a strict backward-compatibility policy. All schema changes are registered in the Schema Registry before deployment. Breaking changes require a major version increment and a migration period of at least 30 days.',
        subsections: [
          {
            id: 'ii-3-versioning',
            number: '3.1',
            title: 'Schema Evolution Rules',
            content: '',
            items: [
              'RULE-V1: Adding optional fields is backward-compatible (minor version increment).',
              'RULE-V2: Removing or renaming fields requires a major version increment.',
              'RULE-V3: Changing field types is always a major version increment.',
              'RULE-V4: All consumers must handle unknown fields gracefully (ignore-unknown policy).',
              'RULE-V5: The Schema Registry enforces compatibility checks on registration.',
              'RULE-V6: Dual-write period of 30 days is mandatory for major version transitions.',
            ]
          }
        ]
      },
      {
        id: 'ii-4',
        number: '4',
        title: 'Immutable Log Structure',
        content: 'The immutable event log is the system of record. All state is derived from this log. The log is partitioned by entity type and region, with a global ordering guarantee within each partition.',
        subsections: [
          {
            id: 'ii-4-log',
            number: '4.1',
            title: 'Log Partition Strategy',
            content: '',
            code: `// Log partition key construction
partition_key = hash(entity_type + ":" + region_id + ":" + h3_cell_l7)

// Retention policy by data class
SAFETY_EVENTS:    retain = 7 years  (regulatory requirement)
ROUTING_EVENTS:   retain = 90 days
ANALYTICS_EVENTS: retain = 365 days
AUDIT_LOGS:       retain = 10 years (immutable, compliance)
TELEMETRY_RAW:    retain = 7 days   (high-volume, compressed)`
          }
        ]
      },
      {
        id: 'ii-5',
        number: '5',
        title: 'Event-Sourcing Strategy',
        content: 'Current entity state is derived by replaying the event log from the last snapshot. Snapshots are taken every 1,000 events or 24 hours, whichever comes first. The snapshot + delta approach bounds replay time to O(1,000) events maximum.',
      },
      {
        id: 'ii-6',
        number: '6',
        title: 'Conflict Resolution Algorithms',
        content: 'Conflicts arise when concurrent writes occur during network partitions. The system uses a deterministic, trust-weighted resolution algorithm that produces the same result regardless of the order in which conflicting events are processed.',
        subsections: [
          {
            id: 'ii-6-algo',
            number: '6.1',
            title: 'Resolution Algorithm',
            content: '',
            code: `// Conflict resolution pseudocode
function resolve_conflict(events: Event[]): Event {
  // Step 1: Filter by schema validity
  valid = events.filter(e => validate_schema(e))
  
  // Step 2: Sort by trust weight (descending), then timestamp (ascending)
  sorted = valid.sort_by(e => (-e.trust_weight, e.timestamp_utc))
  
  // Step 3: Apply CRDT merge for commutative fields
  merged = crdt_merge(sorted.map(e => e.payload))
  
  // Step 4: Winner-takes-all for non-commutative fields
  winner = sorted[0]
  
  // Step 5: Emit ConflictResolved event with provenance
  return ConflictResolved {
    winning_event_id: winner.event_id,
    merged_payload: merged,
    losers: sorted[1:].map(e => e.event_id),
    resolution_algorithm: "TRUST_WEIGHTED_CRDT_v2"
  }
}`
          }
        ]
      },
      {
        id: 'ii-7',
        number: '7',
        title: 'Topology Merge Logic',
        content: 'Road topology data is merged from multiple sources including GPS traces, satellite imagery, authority feeds, and licensed map providers. The merge process is trust-weighted and produces a canonical topology that is versioned and auditable.',
        subsections: [
          {
            id: 'ii-7-merge',
            number: '7.1',
            title: 'Merge Pipeline Stages',
            content: '',
            items: [
              'Stage 1 — Normalize: Convert all sources to canonical H3-indexed geometry.',
              'Stage 2 — Entity Resolution: Match incoming segments to existing entities using spatial proximity + attribute similarity.',
              'Stage 3 — Conflict Detection: Identify geometric conflicts (overlapping segments, contradictory one-way flags).',
              'Stage 4 — Trust Weighting: Apply source trust scores to conflicting claims.',
              'Stage 5 — Topology Rebuild: Reconstruct the graph ensuring planarity and connectivity invariants.',
              'Stage 6 — Version Increment: Emit RoadGraph.Updated event with diff and provenance.',
            ]
          }
        ]
      }
    ]
  },
  {
    id: 'part-iii',
    partNumber: 'III',
    romanNumeral: 'III',
    title: 'Global Event Fabric',
    description: 'Event envelope, taxonomy, flow topology, replication, ordering, and replay.',
    color: 'teal',
    sections: [
      {
        id: 'iii-1',
        number: '1',
        title: 'Event Envelope Specification',
        content: 'Every event in the G.A.N.E is encapsulated in a standardized envelope. The envelope is the atomic unit of communication and is immutable once published.',
        subsections: [
          {
            id: 'iii-1-envelope',
            number: '1.1',
            title: 'Event Envelope Schema',
            content: '',
            code: `message EventEnvelope {
  string  event_id        = 1;  // UUID v4
  int64   timestamp_utc   = 2;  // Unix epoch microseconds (GPS-synchronized)
  string  origin          = 3;  // "region:node_id:component"
  string  entity_type     = 4;  // Fully-qualified entity type
  string  entity_id       = 5;  // UUID of affected entity
  string  event_type      = 6;  // See Event Taxonomy (§III.2)
  bytes   payload         = 7;  // Avro-serialized payload
  int32   schema_version  = 8;  // Payload schema version
  bytes   signature       = 9;  // Ed25519(sha256(payload), origin_key)
  string  idempotency_key = 10; // sha256(entity_id + event_type + timestamp)
  string  correlation_id  = 11; // Parent event_id for causality chains
  int32   priority        = 12; // 0=SAFETY, 1=ROUTING, 2=ANALYTICS
  string  region_id       = 13; // Data residency region
}`
          }
        ]
      },
      {
        id: 'iii-2',
        number: '2',
        title: 'Event Taxonomy',
        content: 'The complete event taxonomy covers all system operations. Events are organized into 6 categories with 21 primary event types.',
        subsections: [
          {
            id: 'iii-2-taxonomy',
            number: '2.1',
            title: 'Complete Event Type Registry',
            content: '',
            table: {
              headers: ['Event Type', 'Category', 'Priority', 'Payload Size (max)', 'Retention'],
              rows: [
                ['Telemetry', 'Sensor', 'P2', '2KB', '7 days'],
                ['RoadDiscovered', 'Topology', 'P1', '50KB', '7 years'],
                ['GeometryChanged', 'Topology', 'P1', '100KB', '7 years'],
                ['LaneUpdated', 'Topology', 'P1', '20KB', '7 years'],
                ['IncidentLifecycle', 'Safety', 'P0', '10KB', '7 years'],
                ['EvidenceAdded', 'Trust', 'P1', '500KB', '7 years'],
                ['TrustAdjusted', 'Trust', 'P1', '5KB', '7 years'],
                ['RiskRecalculated', 'Intelligence', 'P0', '10KB', '90 days'],
                ['StabilityUpdated', 'Intelligence', 'P0', '5KB', '90 days'],
                ['FlowControlUpdate', 'Control', 'P0', '5KB', '90 days'],
                ['RouteProposed', 'Navigation', 'P1', '20KB', '30 days'],
                ['RouteCommitted', 'Navigation', 'P1', '20KB', '30 days'],
                ['RerouteApplied', 'Navigation', 'P0', '10KB', '30 days'],
                ['PolicyChanged', 'Governance', 'P0', '50KB', '10 years'],
                ['OfflineSyncResolved', 'Resilience', 'P1', '100KB', '90 days'],
                ['SatellitePacket', 'Resilience', 'P0', '512B', '7 years'],
                ['DispatchIssued', 'Operations', 'P0', '10KB', '7 years'],
                ['TaskUpdated', 'Operations', 'P1', '10KB', '90 days'],
                ['MessageCreated', 'Social', 'P2', '5KB', '30 days'],
                ['VoiceSessionStarted', 'Social', 'P2', '1KB', '30 days'],
                ['ModelVersionUpdated', 'ML', 'P1', '1MB', '10 years'],
              ]
            }
          }
        ]
      },
      {
        id: 'iii-3',
        number: '3',
        title: 'Event Flow Topology',
        content: 'Events flow from edge devices through regional compute planes to the global federation plane. The topology is designed for low latency at the edge and high throughput at the regional level.',
        subsections: [
          {
            id: 'iii-3-flow',
            number: '3.1',
            title: 'Flow Path by Priority',
            content: '',
            items: [
              'P0 (Safety): Edge → Regional Kafka (P0 topic) → Stream Processor → Safety Consumer → <200ms end-to-end',
              'P1 (Routing): Edge → Regional Kafka (P1 topic) → Stream Processor → Routing Consumer → <500ms end-to-end',
              'P2 (Analytics): Edge → Regional Kafka (P2 topic) → Batch Processor → Analytics Store → async',
              'Cross-region: Regional → Global Federation Bus → Target Region → <2s end-to-end',
            ]
          }
        ]
      },
      {
        id: 'iii-4',
        number: '4',
        title: 'Multi-Region Replication Model',
        content: 'The G.A.N.E uses selective topic mirroring for cross-region replication. Not all data is replicated globally — only data that is required for cross-region operations. This minimizes bandwidth and latency while maintaining global consistency for critical data.',
        subsections: [
          {
            id: 'iii-4-replication',
            number: '4.1',
            title: 'Replication Policy Matrix',
            content: '',
            table: {
              headers: ['Topic', 'Replication Scope', 'Lag Tolerance', 'Conflict Policy'],
              rows: [
                ['safety.*', 'Global', '<2s', 'Last-writer-wins'],
                ['topology.*', 'Global', '<30s', 'Trust-weighted merge'],
                ['policy.*', 'Global', '<5s', 'Version-ordered'],
                ['routing.*', 'Regional', 'N/A', 'N/A'],
                ['analytics.*', 'Regional', 'N/A', 'N/A'],
                ['social.*', 'Regional', 'N/A', 'N/A'],
              ]
            }
          }
        ]
      },
      {
        id: 'iii-5',
        number: '5',
        title: 'Ordering Guarantees',
        content: 'The system provides different ordering guarantees per partition. Within a partition (entity_id + region), events are strictly ordered. Across partitions, only causal ordering is guaranteed via correlation_id chains.',
      },
      {
        id: 'iii-6',
        number: '6',
        title: 'Idempotency Rules',
        content: 'All event consumers must be idempotent. The idempotency_key in the event envelope is used to detect and deduplicate redelivered events. The deduplication window is 24 hours.',
        subsections: [
          {
            id: 'iii-6-idempotency',
            number: '6.1',
            title: 'Idempotency Key Construction',
            content: '',
            code: `// Idempotency key construction
idempotency_key = sha256(
  entity_id     +  // Affected entity
  event_type    +  // Operation type
  timestamp_utc +  // Microsecond precision
  origin           // Source node
)

// Consumer deduplication pattern
if dedup_store.exists(event.idempotency_key):
  log.info("Duplicate event detected, skipping", event_id=event.event_id)
  ack(event)  // Acknowledge to prevent redelivery
  return

process(event)
dedup_store.set(event.idempotency_key, ttl=24h)`
          }
        ]
      },
      {
        id: 'iii-7',
        number: '7',
        title: 'Replay & Recovery Model',
        content: 'The event-sourced architecture enables full state reconstruction from the log. Recovery procedures are classified by scope and urgency.',
        subsections: [
          {
            id: 'iii-7-replay',
            number: '7.1',
            title: 'Recovery Procedures',
            content: '',
            table: {
              headers: ['Recovery Type', 'Trigger', 'Procedure', 'Max Duration'],
              rows: [
                ['Consumer restart', 'Process crash', 'Resume from last committed offset', '<30s'],
                ['State store rebuild', 'Corruption detected', 'Replay from last snapshot + delta', '<5min'],
                ['Regional failover', 'Datacenter outage', 'Promote replica + replay lag', '<15min'],
                ['Global reconciliation', 'Partition healed', 'Merge diverged states via CRDT', '<30min'],
                ['Full region rebuild', 'Catastrophic loss', 'Replay from global log mirror', '<4h'],
              ]
            }
          }
        ]
      },
      {
        id: 'iii-8',
        number: '8',
        title: 'Latency Budgets',
        content: 'End-to-end latency budgets are allocated across subsystems. The total budget for a P0 safety event is 200ms from edge generation to consumer action.',
        subsections: [
          {
            id: 'iii-8-latency',
            number: '8.1',
            title: 'P0 Safety Event Latency Allocation',
            content: '',
            table: {
              headers: ['Stage', 'Budget', 'P99 Target', 'Notes'],
              rows: [
                ['Edge serialization', '5ms', '8ms', 'Protobuf encoding'],
                ['Edge → Regional network', '20ms', '35ms', 'LTE/5G uplink'],
                ['Kafka ingestion', '5ms', '10ms', 'Regional cluster'],
                ['Stream processing', '50ms', '80ms', 'Flink operator chain'],
                ['Safety consumer action', '100ms', '150ms', 'Routing + dispatch'],
                ['Response to edge', '20ms', '35ms', 'Downlink'],
                ['TOTAL', '200ms', '318ms', 'P99 within SLA'],
              ]
            }
          }
        ]
      }
    ]
  },
  {
    id: 'part-iv',
    partNumber: 'IV',
    romanNumeral: 'IV',
    title: 'Core Intelligence Engines',
    description: 'Probabilistic routing, network stability, risk engine, road discovery, and human state.',
    color: 'indigo',
    sections: [
      {
        id: 'iv-a',
        number: 'A',
        title: 'Probabilistic Routing Engine',
        content: 'The Probabilistic Routing Engine (PRE) is the primary intelligence component of the G.A.N.E. It replaces deterministic shortest-path algorithms with a multi-objective probabilistic optimization that explicitly models uncertainty.',
        subsections: [
          {
            id: 'iv-a-math',
            number: 'A.1',
            title: 'Mathematical ETA Model',
            content: 'The ETA for a route R = {s₁, s₂, ..., sₙ} is modeled as a sum of segment travel time distributions:',
            code: `// ETA Distribution Model
ETA(R) = Σᵢ T(sᵢ) where T(sᵢ) ~ LogNormal(μᵢ, σᵢ²)

// Segment travel time parameters
μᵢ = log(length(sᵢ) / v̄ᵢ)  // Mean log-travel-time
σᵢ = f(density, weather, incident_proximity, time_of_day)

// Route ETA distribution (Central Limit Theorem approximation for n > 10)
ETA(R) ~ Normal(Σμᵢ, Σσᵢ²)

// Confidence interval output
ETA_output = {
  p50: exp(Σμᵢ),
  p10: quantile(ETA_dist, 0.10),
  p90: quantile(ETA_dist, 0.90),
  confidence_index: 1 - (p90 - p10) / p50,
  volatility_index: σ_total / μ_total
}`
          },
          {
            id: 'iv-a-optim',
            number: 'A.2',
            title: 'Multi-Objective Optimization Vector',
            content: '',
            table: {
              headers: ['Objective', 'Weight (default)', 'Tunable Range', 'Unit'],
              rows: [
                ['Travel time (E[ETA])', '0.40', '[0.0, 1.0]', 'seconds'],
                ['Risk score', '0.25', '[0.0, 1.0]', 'normalized [0,1]'],
                ['Network stability impact', '0.15', '[0.0, 1.0]', 'normalized [0,1]'],
                ['Energy consumption', '0.10', '[0.0, 1.0]', 'kWh or liters'],
                ['Cognitive load', '0.05', '[0.0, 1.0]', 'normalized [0,1]'],
                ['Emissions', '0.05', '[0.0, 1.0]', 'gCO₂eq'],
              ]
            }
          },
          {
            id: 'iv-a-worstcase',
            number: 'A.3',
            title: 'Worst-Case Routing',
            content: '',
            code: `// Robust routing with worst-case bound
function robust_route(origin, destination, risk_tolerance):
  candidates = enumerate_k_shortest_paths(origin, destination, k=20)
  
  for route in candidates:
    route.eta_p50   = compute_eta(route, quantile=0.50)
    route.eta_p95   = compute_eta(route, quantile=0.95)  // worst-case
    route.risk      = aggregate_risk(route.segments)
    route.stability = flow_impact_score(route)
  
  // Pareto-optimal selection
  pareto_front = compute_pareto_front(candidates, objectives=[
    minimize(eta_p50), minimize(eta_p95), minimize(risk), minimize(stability_impact)
  ])
  
  return select_by_user_profile(pareto_front, risk_tolerance)`
          }
        ]
      },
      {
        id: 'iv-b',
        number: 'B',
        title: 'Network Stability & Flow Orchestration',
        content: 'The Network Stability Engine (NSE) prevents the classic "routing convergence" problem where all navigation systems simultaneously reroute users to the same alternative path, creating a new congestion point. It operates as a closed-loop control system.',
        subsections: [
          {
            id: 'iv-b-state',
            number: 'B.1',
            title: 'State Variables & Control Equations',
            content: '',
            code: `// Network state variables
ρ(c, t)  = vehicle density on corridor c at time t  [veh/km]
q(c, t)  = flow rate on corridor c at time t        [veh/min]
v(c, t)  = mean speed on corridor c at time t       [km/h]
H(t)     = network entropy at time t                [bits]

// Fundamental diagram (Greenshields model)
v(c,t) = v_free × (1 - ρ(c,t)/ρ_jam)
q(c,t) = ρ(c,t) × v(c,t)

// Network entropy (disorder measure)
H(t) = -Σc P(c,t) × log₂(P(c,t))
where P(c,t) = q(c,t) / Σc q(c,t)

// Flow control objective: minimize entropy increase rate
minimize: dH/dt subject to: q(c,t) ≤ q_max(c) ∀c

// Reroute quota allocation (prevents oscillation)
quota(c,t) = min(α × (q_max(c) - q(c,t)), β × q(c,t))
where α=0.15 (max reroute fraction), β=0.05 (damping factor)`
          },
          {
            id: 'iv-b-collapse',
            number: 'B.2',
            title: 'Collapse Prediction',
            content: '',
            code: `// Collapse risk score
CollapseRisk(t) = w₁ × ρ_normalized + w₂ × dρ/dt + w₃ × incident_density
                + w₄ × weather_severity + w₅ × H_normalized

// Threshold-based alert levels
if CollapseRisk > 0.85: CRITICAL — activate evacuation routing
if CollapseRisk > 0.70: HIGH     — reduce reroute quotas by 50%
if CollapseRisk > 0.50: ELEVATED — increase monitoring frequency`
          }
        ]
      },
      {
        id: 'iv-c',
        number: 'C',
        title: 'Risk Engine',
        content: 'The Risk Engine provides real-time, multi-dimensional risk scores for segments, routes, and the network as a whole. Risk scores feed directly into the routing engine and the flow orchestrator.',
        subsections: [
          {
            id: 'iv-c-features',
            number: 'C.1',
            title: 'Feature Pipeline',
            content: '',
            table: {
              headers: ['Feature', 'Source', 'Update Frequency', 'Weight (default)'],
              rows: [
                ['Accident density (7-day rolling)', 'Incident events', '1min', '0.25'],
                ['Hard braking density', 'Telemetry', '30s', '0.15'],
                ['Weather severity index', 'Weather API', '5min', '0.15'],
                ['Visibility score', 'Weather + camera', '5min', '0.10'],
                ['Road geometry complexity', 'LaneGraph', 'Static', '0.10'],
                ['Infrastructure health index', 'InfrastructureState', '1h', '0.10'],
                ['Human error probability', 'DriverState aggregate', '1min', '0.15'],
              ]
            }
          }
        ]
      },
      {
        id: 'iv-d',
        number: 'D',
        title: 'Road Discovery Engine',
        content: 'The Road Discovery Engine (RDE) automatically detects new roads, geometry changes, and topology updates from GPS traces and other data sources. It operates continuously and feeds updates into the canonical topology.',
        subsections: [
          {
            id: 'iv-d-algo',
            number: 'D.1',
            title: 'GPS Clustering Algorithm',
            content: '',
            code: `// Road detection from GPS traces
function detect_roads(traces: GPSTrace[], region: H3Cell):
  
  // Step 1: Filter and clean traces
  clean = traces
    .filter(t => t.accuracy < 10m)  // GPS accuracy threshold
    .filter(t => t.speed > 2 km/h)  // Remove stationary points
  
  // Step 2: DBSCAN clustering on trace density
  clusters = DBSCAN(
    points = clean,
    epsilon = 5m,      // Max distance between points
    min_samples = 10   // Min traces to form a road
  )
  
  // Step 3: Centerline extraction (medial axis transform)
  centerlines = clusters.map(c => medial_axis(c.points))
  
  // Step 4: Topology inference
  graph = build_topology(centerlines, snap_tolerance=3m)
  
  // Step 5: Adversarial filtering
  graph = filter_adversarial(graph, min_trust_sources=3)
  
  return graph`
          }
        ]
      },
      {
        id: 'iv-e',
        number: 'E',
        title: 'Human State Intelligence',
        content: 'The Human State Intelligence (HSI) engine models driver cognitive and physical state to enable safety-adaptive routing and UI adaptation. It operates entirely on-device to protect privacy.',
        subsections: [
          {
            id: 'iv-e-fatigue',
            number: 'E.1',
            title: 'Fatigue Detection Model',
            content: '',
            code: `// Fatigue score computation (on-device)
FatigueScore = w₁ × driving_duration_normalized
             + w₂ × time_of_day_factor        // Circadian rhythm
             + w₃ × lane_deviation_frequency  // From vehicle sensors
             + w₄ × reaction_time_delta       // Vs. baseline
             + w₅ × micro_sleep_events        // Camera-based (optional)

// Adaptive routing thresholds
if FatigueScore > 0.8: suggest_rest_stop, simplify_route
if FatigueScore > 0.6: reduce_route_complexity, increase_warnings
if FatigueScore > 0.4: enable_enhanced_guidance

// UI adaptation rules
if CognitiveLoad > 0.7: suppress_non_critical_alerts
if FatigueScore > 0.7:  voice_only_mode, disable_text_input`
          }
        ]
      }
    ]
  },
  {
    id: 'part-v',
    partNumber: 'V',
    romanNumeral: 'V',
    title: 'Evidence & Trust Layer',
    description: 'Signed metadata, chain-of-custody, reputation ledger, and anti-collusion detection.',
    color: 'amber',
    sections: [
      {
        id: 'v-1',
        number: '1',
        title: 'Signed Metadata Model',
        content: 'All evidence submitted to the system is accompanied by cryptographically signed metadata. The signature covers the content hash, location, timestamp, and device fingerprint. This creates a tamper-evident chain from capture to consumption.',
        subsections: [
          {
            id: 'v-1-schema',
            number: '1.1',
            title: 'Evidence Metadata Schema',
            content: '',
            code: `message EvidenceMetadata {
  string  evidence_id      = 1;  // UUID v4
  bytes   content_hash     = 2;  // SHA-256 of raw content
  string  content_type     = 3;  // PHOTO | VIDEO | VOICE | SENSOR
  int64   captured_at_utc  = 4;  // GPS-synchronized timestamp
  double  latitude         = 5;  // WGS-84
  double  longitude        = 6;  // WGS-84
  float   location_accuracy= 7;  // Meters (GPS accuracy)
  string  device_id        = 8;  // Registered device UUID
  bytes   device_signature = 9;  // Ed25519(content_hash, device_key)
  bytes   pre_roll_hash    = 10; // Hash of 30s pre-event buffer
  float   trust_weight     = 11; // Source trust at capture time
  bool    offline_captured = 12; // Was device offline at capture?
}`
          }
        ]
      },
      {
        id: 'v-2',
        number: '2',
        title: 'Chain-of-Custody',
        content: 'Every evidence item maintains a complete chain-of-custody from capture through processing to final disposition. The chain is immutable and auditable.',
      },
      {
        id: 'v-3',
        number: '3',
        title: 'Secure Offline Vault',
        content: 'Evidence captured while offline is stored in a secure, encrypted vault on the edge device. The vault uses AES-256-GCM encryption with a device-bound key. Evidence is uploaded and verified when connectivity is restored.',
      },
      {
        id: 'v-4',
        number: '4',
        title: 'Cross-Source Correlation',
        content: 'Multiple independent evidence sources reporting the same event increase confidence. The correlation algorithm uses spatial proximity, temporal overlap, and content similarity to identify corroborating evidence.',
        subsections: [
          {
            id: 'v-4-correlation',
            number: '4.1',
            title: 'Correlation Score Formula',
            content: '',
            code: `// Evidence correlation score
CorrelationScore(E₁, E₂) = 
  w₁ × spatial_overlap(E₁.location, E₂.location, radius=100m) +
  w₂ × temporal_overlap(E₁.timestamp, E₂.timestamp, window=120s) +
  w₃ × content_similarity(E₁.content_hash, E₂.content_hash) +
  w₄ × source_independence(E₁.device_id, E₂.device_id)

// Confidence boost from corroboration
IncidentConfidence = 1 - Π(1 - TrustScore(Eᵢ)) for all corroborating Eᵢ`
          }
        ]
      },
      {
        id: 'v-5',
        number: '5',
        title: 'Reputation Ledger',
        content: 'The Reputation Ledger maintains a long-term trust score for each user and device. The score is updated based on the quality and accuracy of submitted evidence, verified against ground truth when available.',
      },
      {
        id: 'v-6',
        number: '6',
        title: 'Anti-Collusion Detection',
        content: 'The system detects coordinated false reporting through social graph analysis and statistical anomaly detection. Suspicious clusters of correlated reports from socially connected users are flagged for human review.',
      },
      {
        id: 'v-7',
        number: '7',
        title: 'Manipulation Prevention',
        content: 'The evidence-first trust model, combined with cryptographic signing, spatial validation, and anti-collusion detection, creates a multi-layer defense against manipulation. No single actor can significantly influence the system without a large, coordinated attack that would be statistically detectable.',
      }
    ]
  },
  {
    id: 'part-vi',
    partNumber: 'VI',
    romanNumeral: 'VI',
    title: 'Operational Layers',
    description: 'EMS dispatch, fleet optimization, VRP, SLA monitoring, and scenario simulation.',
    color: 'green',
    sections: [
      {
        id: 'vi-1',
        number: '1',
        title: 'EMS Dispatch Architecture',
        content: 'Emergency Medical Services dispatch is a Tier 0 operation with the highest priority in the system. The EMS dispatch architecture provides sub-second routing decisions and preemptive signal control.',
        subsections: [
          {
            id: 'vi-1-dispatch',
            number: '1.1',
            title: 'Dispatch Decision Pipeline',
            content: '',
            items: [
              'Step 1: Incident received → classify severity (P0/P1/P2) within 100ms.',
              'Step 2: Identify available units within 10km radius.',
              'Step 3: Compute ETA for each candidate unit using PRE with EMS vehicle profile.',
              'Step 4: Apply assignment optimization (minimize max ETA, consider unit specialization).',
              'Step 5: Issue Dispatch event → signal preemption request → route commitment.',
              'Step 6: Continuous ETA monitoring → reroute if deviation > 20% from committed ETA.',
            ]
          }
        ]
      },
      {
        id: 'vi-2',
        number: '2',
        title: 'Fleet & VRP Optimization',
        content: 'The Vehicle Routing Problem (VRP) solver handles multi-stop optimization for fleet operations. It supports a rich constraint model including time windows, vehicle capacities, driver hours, and hazmat restrictions.',
        subsections: [
          {
            id: 'vi-2-vrp',
            number: '2.1',
            title: 'VRP Constraint Model',
            content: '',
            table: {
              headers: ['Constraint Type', 'Classification', 'Violation Handling'],
              rows: [
                ['Time windows (hard)', 'Hard', 'Route rejected'],
                ['Vehicle capacity', 'Hard', 'Route rejected'],
                ['Driver hours of service', 'Hard', 'Route rejected'],
                ['Hazmat routing restrictions', 'Hard', 'Route rejected'],
                ['Height/weight restrictions', 'Hard', 'Route rejected'],
                ['Time windows (soft)', 'Soft', 'Penalty in objective'],
                ['Preferred delivery windows', 'Soft', 'Penalty in objective'],
                ['Fuel efficiency preference', 'Soft', 'Penalty in objective'],
              ]
            }
          }
        ]
      },
      {
        id: 'vi-3',
        number: '3',
        title: 'Time Window Constraints',
        content: 'Time windows are enforced at two levels: hard constraints that cannot be violated and soft constraints that incur a penalty in the optimization objective. The solver uses a branch-and-bound algorithm with a 5-second time limit for real-time dispatch.',
      },
      {
        id: 'vi-4',
        number: '4',
        title: 'SLA Monitoring',
        content: 'SLA monitoring operates in real-time against all defined service levels. Breaches trigger automated escalation procedures and are recorded in the immutable audit log.',
      },
      {
        id: 'vi-5',
        number: '5',
        title: 'Scenario Simulation',
        content: 'The scenario simulation engine allows operators to test the system response to hypothetical events before they occur. Simulations run against a shadow copy of the digital twin and do not affect the live system.',
      }
    ]
  },
  {
    id: 'part-vii',
    partNumber: 'VII',
    romanNumeral: 'VII',
    title: 'Digital Twin & City Layer',
    description: 'Real-time simulation, SPaT/GLOSA integration, reversible lanes, and evacuation modeling.',
    color: 'teal',
    sections: [
      {
        id: 'vii-1',
        number: '1',
        title: 'Real-Time Simulation Engine',
        content: 'The Digital Twin maintains a real-time, high-fidelity simulation of the urban transportation network. It ingests all G.A.N.E events and maintains a shadow state that is 500ms behind real-time (processing lag). The twin is used for prediction, planning, and what-if analysis.',
      },
      {
        id: 'vii-2',
        number: '2',
        title: 'SPaT / GLOSA Integration',
        content: 'Signal Phase and Timing (SPaT) data from connected infrastructure is ingested as SignalState events. The Green Light Optimal Speed Advisory (GLOSA) algorithm computes the optimal approach speed to minimize stops at signalized intersections.',
        subsections: [
          {
            id: 'vii-2-glosa',
            number: '2.1',
            title: 'GLOSA Speed Advisory',
            content: '',
            code: `// GLOSA advisory computation
function compute_glosa(vehicle, signal):
  distance = haversine(vehicle.position, signal.position)
  current_phase = signal.current_phase  // RED | GREEN | YELLOW
  time_to_green = signal.time_to_next_green
  
  // Optimal speed to arrive at green
  v_optimal = distance / time_to_green
  
  // Constrain to legal and safe bounds
  v_advisory = clamp(v_optimal, v_min=20km/h, v_max=speed_limit)
  
  // Only issue advisory if within advisory horizon
  if distance < 500m and abs(v_advisory - vehicle.speed) > 5km/h:
    emit GlosaAdvisory(speed=v_advisory, confidence=signal.spat_confidence)`
          }
        ]
      },
      {
        id: 'vii-3',
        number: '3',
        title: 'Reversible Lane Modeling',
        content: 'Reversible lanes are modeled as time-variant edges in the LaneGraph. The system ingests lane control signals from infrastructure and updates the routing graph within 200ms of a lane direction change.',
      },
      {
        id: 'vii-4',
        number: '4',
        title: 'Evacuation Mode',
        content: 'Evacuation mode is a special operating mode activated by authorized operators or automatically triggered when CollapseRisk exceeds 0.95. In evacuation mode, all routing is reconfigured to maximize outbound flow from the affected area.',
      },
      {
        id: 'vii-5',
        number: '5',
        title: 'Urban Evolution Forecasting',
        content: 'The digital twin supports long-range forecasting by simulating the effects of urban planning decisions on the transportation network. Forecasts are generated using agent-based simulation with calibrated demand models.',
      }
    ]
  },
  {
    id: 'part-viii',
    partNumber: 'VIII',
    romanNumeral: 'VIII',
    title: 'Energy & Environment',
    description: 'Emission-aware routing, charging load forecasting, and environmental impact scoring.',
    color: 'green',
    sections: [
      {
        id: 'viii-1',
        number: '1',
        title: 'Emission-Aware Routing',
        content: 'The routing engine incorporates a vehicle-specific emission model that accounts for speed profile, road gradient, stop frequency, and vehicle type. Emission-optimized routes minimize total gCO₂eq while maintaining a maximum ETA penalty of 15%.',
        subsections: [
          {
            id: 'viii-1-model',
            number: '1.1',
            title: 'Emission Model',
            content: '',
            code: `// Instantaneous emission rate (COPERT model)
E(v, grade, vehicle_type) = 
  base_rate(vehicle_type) × 
  speed_factor(v) × 
  grade_factor(grade) × 
  load_factor

// Route total emissions
Emissions(R) = Σᵢ E(v̄(sᵢ), grade(sᵢ), vehicle_type) × length(sᵢ)

// Emission-aware objective weight
if user.preference == ECO:
  w_emissions = 0.30  // Elevated from default 0.05
  w_time = 0.25       // Reduced from default 0.40`
          }
        ]
      },
      {
        id: 'viii-2',
        number: '2',
        title: 'Charging Load Forecasting',
        content: 'EV charging demand is forecast using a combination of fleet telemetry (state of charge, location, trip patterns) and historical charging behavior. Forecasts are provided to grid operators 4 hours ahead with 15-minute resolution.',
      },
      {
        id: 'viii-3',
        number: '3',
        title: 'Energy Load Balancing',
        content: 'The system coordinates EV charging to minimize peak grid demand. Drivers are offered incentives to shift charging to off-peak periods. The load balancing algorithm respects driver range requirements as a hard constraint.',
      },
      {
        id: 'viii-4',
        number: '4',
        title: 'Environmental Impact Scoring',
        content: 'Each route is assigned an Environmental Impact Score (EIS) that combines emissions, noise pollution, and air quality impact. The EIS is displayed to users and can be used as a routing objective.',
      }
    ]
  },
  {
    id: 'part-ix',
    partNumber: 'IX',
    romanNumeral: 'IX',
    title: 'Social & Communication Layer',
    description: 'Context-only channels, role-based visibility, driving safety gate, and trust-weighted exposure.',
    color: 'indigo',
    sections: [
      {
        id: 'ix-1',
        number: '1',
        title: 'Context-Only Channels',
        content: 'Communication is organized into context-bound channels that are automatically created and destroyed based on system events. There are no general-purpose feeds or infinite scroll interfaces. All channels have a defined lifecycle tied to a mobility event.',
        subsections: [
          {
            id: 'ix-1-channels',
            number: '1.1',
            title: 'Channel Types',
            content: '',
            table: {
              headers: ['Channel Type', 'Lifecycle', 'Participants', 'Content Types'],
              rows: [
                ['TripChannel', 'Duration of trip', 'Driver + invited contacts', 'ETA, location, voice'],
                ['IncidentChannel', 'Incident lifecycle', 'Nearby users + responders', 'Hazard alerts, updates'],
                ['FleetChannel', 'Shift duration', 'Fleet members + dispatcher', 'Tasks, voice, status'],
                ['ConvoyChannel', 'Convoy duration', 'Convoy members', 'Spacing, speed, voice'],
                ['HazardWatchChannel', 'Hazard active', 'Subscribed users in radius', 'Hazard updates'],
              ]
            }
          }
        ]
      },
      {
        id: 'ix-2',
        number: '2',
        title: 'Role-Based Visibility',
        content: 'Location and status data is shared only within the defined scope of each channel. Users control their visibility level per channel. The system enforces visibility boundaries at the data layer, not just the UI layer.',
      },
      {
        id: 'ix-3',
        number: '3',
        title: 'Driving Safety Gate',
        content: 'When a vehicle is in motion above 10 km/h, the system enforces a driving safety gate that restricts text input and complex UI interactions. Voice-only mode is automatically engaged. The gate cannot be overridden by the user.',
      },
      {
        id: 'ix-4',
        number: '4',
        title: 'Meta-Event Isolation',
        content: 'The social bus is architecturally isolated from the core G.A.N.E event fabric. Social events are processed on a separate cluster with lower priority. A failure in the social layer cannot propagate to safety-critical systems.',
      },
      {
        id: 'ix-5',
        number: '5',
        title: 'Trust-Weighted Exposure',
        content: 'Information shared in channels is filtered by the trust score of the source. Low-trust sources are labeled and their content is given lower prominence. High-trust sources (verified authorities, high-reputation users) receive elevated visibility.',
      }
    ]
  },
  {
    id: 'part-x',
    partNumber: 'X',
    romanNumeral: 'X',
    title: 'Offline / Edge / Satellite Continuity',
    description: 'Edge autonomy, deterministic sync, partition survival, and satellite minimal protocol.',
    color: 'teal',
    sections: [
      {
        id: 'x-1',
        number: '1',
        title: 'Edge Autonomy Scope',
        content: 'Edge devices maintain sufficient local state to provide full Tier 1 functionality for at least 72 hours without connectivity. The local state includes a map cache, risk cache, routing engine, and evidence vault.',
        subsections: [
          {
            id: 'x-1-scope',
            number: '1.1',
            title: 'Edge Capability Matrix',
            content: '',
            table: {
              headers: ['Capability', 'Offline Support', 'Degradation', 'Cache TTL'],
              rows: [
                ['Turn-by-turn routing', 'Full', 'None', '72h'],
                ['Risk scoring', 'Full (cached)', 'Stale after 1h', '1h'],
                ['Incident reporting', 'Full (queued)', 'Delayed upload', 'Unlimited'],
                ['Evidence capture', 'Full (vaulted)', 'Delayed upload', 'Unlimited'],
                ['ETA calculation', 'Full (local model)', 'Higher uncertainty', '72h'],
                ['EMS dispatch receipt', 'Full (satellite)', 'Satellite latency', 'N/A'],
                ['Fleet coordination', 'Partial (mesh)', 'Mesh-only', 'N/A'],
                ['Social channels', 'None', 'Unavailable', 'N/A'],
              ]
            }
          }
        ]
      },
      {
        id: 'x-2',
        number: '2',
        title: 'Deterministic Sync Reconciliation',
        content: 'When an edge device reconnects, it uploads its local event queue and downloads the delta since its last sync. The reconciliation algorithm is deterministic and produces the same result regardless of the order of operations.',
        subsections: [
          {
            id: 'x-2-algo',
            number: '2.1',
            title: 'Sync Reconciliation Algorithm',
            content: '',
            code: `// Deterministic sync reconciliation
function reconcile(local_queue: Event[], server_delta: Event[]):
  
  // Step 1: Merge all events by timestamp
  all_events = merge_sorted(local_queue, server_delta)
  
  // Step 2: Deduplicate by idempotency_key
  unique_events = deduplicate(all_events)
  
  // Step 3: Resolve conflicts (trust-weighted)
  resolved = resolve_conflicts(unique_events)
  
  // Step 4: Apply to local state
  for event in resolved:
    apply_to_state(event)
  
  // Step 5: Upload local-only events to server
  upload(local_queue.filter(e => !server_delta.contains(e.idempotency_key)))`
          }
        ]
      },
      {
        id: 'x-3',
        number: '3',
        title: 'Partition Survival Guarantees',
        content: 'During a network partition, edge devices continue to operate autonomously. The system guarantees that no safety-critical data is lost during a partition of up to 72 hours. Data is queued locally and uploaded when connectivity is restored.',
      },
      {
        id: 'x-4',
        number: '4',
        title: 'Satellite Minimal Protocol',
        content: 'The satellite protocol is designed for extreme bandwidth constraints (512 bytes/packet). It supports only the most critical operations: SOS, incident reporting, and EMS dispatch.',
        subsections: [
          {
            id: 'x-4-protocol',
            number: '4.1',
            title: 'Satellite Packet Format',
            content: '',
            code: `// Satellite minimal packet (512 bytes max)
struct SatellitePacket {
  uint8   version;        // Protocol version (1 byte)
  uint8   packet_type;    // SOS|INCIDENT|DISPATCH|HEARTBEAT (1 byte)
  uint32  device_id;      // Compressed device ID (4 bytes)
  int32   latitude;       // Fixed-point, 6 decimal places (4 bytes)
  int32   longitude;      // Fixed-point, 6 decimal places (4 bytes)
  uint32  timestamp;      // Unix epoch seconds (4 bytes)
  uint8   priority;       // 0-255 (1 byte)
  uint8   payload_len;    // Payload length (1 byte)
  uint8   payload[492];   // Compressed payload (up to 492 bytes)
  uint16  checksum;       // CRC-16 (2 bytes)
}  // Total: 514 bytes → compress to ≤512 bytes`
          }
        ]
      },
      {
        id: 'x-5',
        number: '5',
        title: 'Store-and-Forward Mesh',
        content: 'In areas with no connectivity, edge devices form a store-and-forward mesh using Bluetooth and WiFi Direct. Data hops through the mesh until it reaches a device with connectivity. The mesh uses epidemic routing with a TTL of 24 hours.',
      }
    ]
  },
  {
    id: 'part-xi',
    partNumber: 'XI',
    romanNumeral: 'XI',
    title: 'Control Plane',
    description: 'Layer switchboard, context presets, policy engine, and privacy sovereignty controls.',
    color: 'indigo',
    sections: [
      {
        id: 'xi-1',
        number: '1',
        title: 'Layer Switchboard',
        content: 'The Layer Switchboard provides a centralized interface for managing the operational state of all G.A.N.E planes. Each plane can be independently enabled, disabled, or placed in degraded mode without affecting other planes.',
      },
      {
        id: 'xi-2',
        number: '2',
        title: 'Context Presets',
        content: 'Context presets are pre-validated configurations for common operating scenarios. They are tested against the simulation engine before deployment.',
        subsections: [
          {
            id: 'xi-2-presets',
            number: '2.1',
            title: 'Standard Context Presets',
            content: '',
            table: {
              headers: ['Preset', 'Trigger', 'Key Changes', 'Authority'],
              rows: [
                ['RUSH_HOUR', 'Scheduled / density threshold', 'Increased reroute quotas, reduced ETA confidence threshold', 'Automated'],
                ['SPECIAL_EVENT', 'Manual / calendar', 'Custom corridor weights, expanded EMS priority zones', 'Regional Operator'],
                ['EMERGENCY', 'Manual / CollapseRisk > 0.85', 'Evacuation routing, signal preemption, satellite priority', 'Emergency Authority'],
                ['MAINTENANCE', 'Scheduled', 'Reduced capacity on affected corridors, advance rerouting', 'Infrastructure Operator'],
                ['ADVERSE_WEATHER', 'Weather API trigger', 'Reduced speed limits, increased safety margins, EV range adjustment', 'Automated'],
                ['NIGHT_MODE', 'Time-based', 'Reduced analytics frequency, increased fatigue monitoring', 'Automated'],
              ]
            }
          }
        ]
      },
      {
        id: 'xi-3',
        number: '3',
        title: 'Policy Engine',
        content: 'The Policy Engine evaluates routing and operational decisions against a set of active policies. Policies are versioned, auditable, and can be scoped to specific regions, vehicle types, or time windows.',
      },
      {
        id: 'xi-4',
        number: '4',
        title: 'Privacy Sovereignty Controls',
        content: 'Users have granular control over their data. Privacy controls are enforced at the data layer and cannot be bypassed by application code. The system supports three privacy tiers: Full Participation, Anonymous Contribution, and Receive-Only.',
      },
      {
        id: 'xi-5',
        number: '5',
        title: 'Performance Modes',
        content: 'Performance modes allow operators to trade off between latency, accuracy, and resource consumption. In degraded infrastructure scenarios, the system can operate in a reduced-accuracy mode that consumes 70% less compute.',
      }
    ]
  },
  {
    id: 'part-xii',
    partNumber: 'XII',
    romanNumeral: 'XII',
    title: 'Security & Governance',
    description: 'Zero-trust architecture, RBAC, data residency, immutable audit, and adversarial simulation.',
    color: 'rose',
    sections: [
      {
        id: 'xii-1',
        number: '1',
        title: 'Zero-Trust Architecture',
        content: 'No implicit trust is granted to any entity, regardless of network location. Every request must be authenticated, authorized, and encrypted. Lateral movement within the system is prevented by micro-segmentation.',
        subsections: [
          {
            id: 'xii-1-zt',
            number: '1.1',
            title: 'Zero-Trust Enforcement Points',
            content: '',
            items: [
              'Device authentication: mTLS with device certificates, rotated every 24 hours.',
              'User authentication: FIDO2/WebAuthn + short-lived JWT tokens (15-minute expiry).',
              'Service-to-service: SPIFFE/SPIRE identity framework with workload attestation.',
              'Data access: Attribute-Based Access Control (ABAC) on all data reads.',
              'Network: All traffic encrypted with TLS 1.3 minimum; no plaintext allowed.',
              'Quantum resilience: CRYSTALS-Kyber key encapsulation for long-lived secrets.',
            ]
          }
        ]
      },
      {
        id: 'xii-2',
        number: '2',
        title: 'RBAC Model',
        content: 'The Role-Based Access Control model defines 12 system roles with fine-grained permissions. Roles are hierarchical and can be composed. All role assignments are logged in the immutable audit system.',
        subsections: [
          {
            id: 'xii-2-roles',
            number: '2.1',
            title: 'System Role Hierarchy',
            content: '',
            table: {
              headers: ['Role', 'Scope', 'Key Permissions', 'Audit Level'],
              rows: [
                ['System Admin', 'Global', 'Full system access', 'Full'],
                ['Regional Operator', 'Regional', 'Policy management, incident response', 'Full'],
                ['Emergency Authority', 'Regional', 'Evacuation mode, signal preemption', 'Full'],
                ['Fleet Manager', 'Fleet', 'Fleet routing, driver assignment', 'Standard'],
                ['EMS Dispatcher', 'Regional', 'EMS dispatch, priority routing', 'Full'],
                ['City Planner', 'Regional', 'Digital twin, simulation', 'Standard'],
                ['Trust Auditor', 'Global', 'Evidence review, trust adjustment', 'Full'],
                ['Developer', 'Sandbox', 'API access, simulation only', 'Standard'],
                ['End User', 'Personal', 'Own data, routing, social', 'Minimal'],
                ['Anonymous', 'None', 'Public map tiles only', 'None'],
              ]
            }
          }
        ]
      },
      {
        id: 'xii-3',
        number: '3',
        title: 'Data Residency Handling',
        content: 'Data residency is enforced at the storage and processing layers. Personal data is stored in the user\'s designated region and is not transferred without explicit consent or legal requirement. The system supports GDPR, CCPA, and equivalent regulations.',
      },
      {
        id: 'xii-4',
        number: '4',
        title: 'Immutable Audit System',
        content: 'All system actions are recorded in an immutable audit log. The log is cryptographically chained (each entry includes the hash of the previous entry) to prevent tampering. The audit log is retained for 10 years.',
      },
      {
        id: 'xii-5',
        number: '5',
        title: 'Model Drift Governance',
        content: 'Machine learning models are continuously monitored for performance drift. When a model\'s accuracy degrades below a threshold, it is automatically flagged for retraining. Model updates require approval from the Model Governance Board before deployment.',
      },
      {
        id: 'xii-6',
        number: '6',
        title: 'Adversarial Simulation',
        content: 'The system undergoes continuous adversarial simulation using a red team of automated attack agents. Attack scenarios include GPS spoofing, evidence fabrication, collusion attacks, and denial-of-service. Results feed into the security hardening backlog.',
      }
    ]
  },
  {
    id: 'part-xiii',
    partNumber: 'XIII',
    romanNumeral: 'XIII',
    title: 'Performance Engineering',
    description: 'SLA definitions, latency allocation, throughput, scaling model, and chaos scenarios.',
    color: 'amber',
    sections: [
      {
        id: 'xiii-1',
        number: '1',
        title: 'SLA Definitions',
        content: 'Service Level Agreements are defined across three tiers based on safety criticality. All SLAs are measured at the 99th percentile (P99) unless otherwise specified.',
        subsections: [
          {
            id: 'xiii-1-slas',
            number: '1.1',
            title: 'SLA Tier Matrix',
            content: '',
            table: {
              headers: ['Operation', 'Tier', 'P50 Target', 'P99 Target', 'Availability', 'Error Budget'],
              rows: [
                ['Safety routing decision', 'T1', '80ms', '200ms', '99.999%', '5.26 min/year'],
                ['EMS dispatch', 'T1', '100ms', '200ms', '99.999%', '5.26 min/year'],
                ['Standard routing', 'T2', '200ms', '500ms', '99.99%', '52.6 min/year'],
                ['Reroute decision', 'T2', '400ms', '1,000ms', '99.99%', '52.6 min/year'],
                ['Event propagation', 'T1', '80ms', '200ms', '99.999%', '5.26 min/year'],
                ['Map tile delivery', 'T2', '100ms', '300ms', '99.99%', '52.6 min/year'],
                ['Analytics queries', 'T3', '1s', '5s', '99.9%', '8.76 h/year'],
                ['Digital twin sync', 'T3', '500ms', '2s', '99.9%', '8.76 h/year'],
              ]
            }
          }
        ]
      },
      {
        id: 'xiii-2',
        number: '2',
        title: 'Latency Allocation per Subsystem',
        content: 'The end-to-end latency budget is allocated across subsystems using a waterfall model. Each subsystem has a hard latency budget that it must not exceed.',
      },
      {
        id: 'xiii-3',
        number: '3',
        title: 'Throughput Assumptions',
        content: 'System throughput is sized for continental-scale deployment with the following baseline assumptions.',
        subsections: [
          {
            id: 'xiii-3-throughput',
            number: '3.1',
            title: 'Throughput Baseline',
            content: '',
            table: {
              headers: ['Data Stream', 'Peak Rate', 'Daily Volume', 'Compression Ratio'],
              rows: [
                ['Telemetry events', '10M events/s', '864B events/day', '8:1'],
                ['Routing requests', '500K req/s', '43B req/day', 'N/A'],
                ['Map tile requests', '2M req/s', '173B req/day', '4:1'],
                ['Evidence uploads', '100K/s', '8.6B/day', '3:1'],
                ['Social messages', '1M/s', '86B/day', '5:1'],
                ['Audit log entries', '5M/s', '432B/day', '10:1'],
              ]
            }
          }
        ]
      },
      {
        id: 'xiii-4',
        number: '4',
        title: 'Scaling Model',
        content: 'The system scales horizontally at all layers. The scaling model is based on consistent hashing with virtual nodes to minimize rebalancing overhead during scale events. Auto-scaling triggers are defined per component.',
      },
      {
        id: 'xiii-5',
        number: '5',
        title: 'Chaos Failure Scenarios',
        content: 'Chaos engineering is practiced continuously. The following failure scenarios are tested in production using controlled blast radius.',
        subsections: [
          {
            id: 'xiii-5-chaos',
            number: '5.1',
            title: 'Chaos Scenario Library',
            content: '',
            table: {
              headers: ['Scenario', 'Blast Radius', 'Expected Behavior', 'Recovery Target'],
              rows: [
                ['Single Kafka broker failure', 'Partition lag', 'Automatic leader election', '<30s'],
                ['Regional datacenter loss', 'Regional services', 'Failover to replica', '<5min'],
                ['GPS spoofing attack', 'Affected devices', 'Anomaly detection + quarantine', '<60s'],
                ['50% edge device disconnect', 'Routing accuracy', 'Degrade to cached data', 'Immediate'],
                ['Routing engine OOM', 'Routing service', 'Restart + fallback to simple routing', '<30s'],
                ['Network partition (2 regions)', 'Cross-region sync', 'Autonomous operation + reconcile on heal', 'Auto'],
                ['DDoS on API gateway', 'Public API', 'Rate limiting + circuit breaker', '<10s'],
              ]
            }
          }
        ]
      },
      {
        id: 'xiii-6',
        number: '6',
        title: 'Graceful Degradation Modes',
        content: 'The system degrades gracefully through four defined modes: Full Capability, Reduced Accuracy, Essential Services Only, and Emergency Minimum. Each mode has defined capability sets and automatic triggers.',
      }
    ]
  },
  {
    id: 'part-xiv',
    partNumber: 'XIV',
    romanNumeral: 'XIV',
    title: 'Strategic Defensibility',
    description: 'Network effects, data moat, control-theoretic moat, regulatory moat, and infrastructure lock-in.',
    color: 'amber',
    sections: [
      {
        id: 'xiv-1',
        number: '1',
        title: 'Network Effects',
        content: 'The G.A.N.E exhibits strong multi-sided network effects. Each additional user improves routing accuracy for all users, increases evidence density, and strengthens the trust network. The value of the network scales super-linearly with participation.',
        subsections: [
          {
            id: 'xiv-1-effects',
            number: '1.1',
            title: 'Network Effect Quantification',
            content: '',
            code: `// Network value model (Metcalfe's Law variant)
V(n) = k × n^α × data_density^β

where:
  n           = number of active users
  data_density = events per km² per hour
  α           = 1.5  (super-linear user scaling)
  β           = 0.8  (data density scaling)
  k           = calibration constant

// Routing accuracy improvement with density
ETA_accuracy(d) = 1 - exp(-λ × d)
where d = GPS traces per km per hour, λ = 0.1`
          }
        ]
      },
      {
        id: 'xiv-2',
        number: '2',
        title: 'Data Moat',
        content: 'The canonical data model accumulates a proprietary dataset that grows in value over time. The road discovery engine creates a continuously updated map that cannot be replicated without equivalent data density. Historical behavioral data enables model training that improves with time.',
      },
      {
        id: 'xiv-3',
        number: '3',
        title: 'Control-Theoretic Moat',
        content: 'The network stability engine creates a defensible position that is difficult to replicate. A competitor without flow orchestration will cause congestion when their users share corridors with G.A.N.E users, creating a negative externality that incentivizes migration.',
      },
      {
        id: 'xiv-4',
        number: '4',
        title: 'Regulatory Moat',
        content: 'The evidence-native trust layer and immutable audit system are designed to meet and exceed regulatory requirements for transportation data. Early compliance creates a barrier for competitors who must build equivalent compliance infrastructure.',
      },
      {
        id: 'xiv-5',
        number: '5',
        title: 'Infrastructure Lock-In',
        content: 'Deep integration with smart city infrastructure (SPaT, signal preemption, reversible lane control) creates switching costs for municipalities. The digital twin becomes a critical planning tool that is difficult to replace once integrated into city operations.',
      }
    ]
  },
  {
    id: 'part-xv',
    partNumber: 'XV',
    romanNumeral: 'XV',
    title: 'Deployment Roadmap',
    description: 'Four-phase deployment from research core to full strategic layer.',
    color: 'green',
    sections: [
      {
        id: 'xv-0',
        number: '0',
        title: 'Phase 0 — Research Core',
        content: 'Establish the foundational research and engineering capabilities required for the G.A.N.E. This phase produces no user-facing product but creates the intellectual and technical foundation for all subsequent phases.',
        subsections: [
          {
            id: 'xv-0-details',
            number: '0.1',
            title: 'Phase 0 Details',
            content: '',
            table: {
              headers: ['Dimension', 'Details'],
              rows: [
                ['Duration', '12 months'],
                ['Team Size', '50 engineers + 20 researchers'],
                ['Deliverables', 'PRE prototype, NSE prototype, RDE prototype, canonical data model v1, event fabric prototype'],
                ['Dependencies', 'Research team, compute infrastructure, GPS trace dataset (>1B traces)'],
                ['Primary Risk', 'Probabilistic routing accuracy below target in high-density urban environments'],
                ['Validation Metrics', 'ETA MAPE < 15% in simulation; NSE reduces simulated congestion by > 30%'],
              ]
            }
          }
        ]
      },
      {
        id: 'xv-1',
        number: '1',
        title: 'Phase 1 — Probabilistic Routing MVP',
        content: 'Deploy a production-grade probabilistic routing engine in a single metropolitan area. Validate the core value proposition with real users.',
        subsections: [
          {
            id: 'xv-1-details',
            number: '1.1',
            title: 'Phase 1 Details',
            content: '',
            table: {
              headers: ['Dimension', 'Details'],
              rows: [
                ['Duration', '6 months post-Phase 0'],
                ['Team Size', '150 engineers'],
                ['Target Geography', 'Single metro area, 5M+ population'],
                ['Deliverables', 'Mobile app, routing API, edge SDK, evidence system, trust layer v1'],
                ['Dependencies', 'Phase 0 completion, mobile platform partnerships, map data license'],
                ['Primary Risk', 'User adoption below critical mass for network effects'],
                ['Validation Metrics', 'ETA MAPE < 12% live; 100K DAU; NPS > 40'],
              ]
            }
          }
        ]
      },
      {
        id: 'xv-2',
        number: '2',
        title: 'Phase 2 — Network Stability Engine',
        content: 'Integrate the Network Stability Engine and demonstrate measurable reduction in urban congestion. Expand to 5 metropolitan areas.',
        subsections: [
          {
            id: 'xv-2-details',
            number: '1.1',
            title: 'Phase 2 Details',
            content: '',
            table: {
              headers: ['Dimension', 'Details'],
              rows: [
                ['Duration', '9 months post-Phase 1'],
                ['Team Size', '300 engineers'],
                ['Target Geography', '5 metro areas across 2 continents'],
                ['Deliverables', 'NSE production, flow orchestration, digital twin v1, EMS integration, fleet API'],
                ['Dependencies', 'Phase 1 at 500K+ DAU, municipal partnerships for SPaT data'],
                ['Primary Risk', 'NSE effectiveness limited by market share below 15% in target corridors'],
                ['Validation Metrics', 'Congestion reduction > 15% in instrumented corridors; 1M DAU; EMS response time -10%'],
              ]
            }
          }
        ]
      },
      {
        id: 'xv-3',
        number: '3',
        title: 'Phase 3 — Global Integration',
        content: 'Scale to continental coverage across all major markets. Integrate satellite fallback, full offline capability, and the complete evidence trust layer.',
        subsections: [
          {
            id: 'xv-3-details',
            number: '1.1',
            title: 'Phase 3 Details',
            content: '',
            table: {
              headers: ['Dimension', 'Details'],
              rows: [
                ['Duration', '18 months post-Phase 2'],
                ['Team Size', '500+ engineers'],
                ['Target Geography', '50+ cities, 5 continents'],
                ['Deliverables', 'Global federation plane, satellite integration, full offline, multi-modal, regulatory compliance suite'],
                ['Dependencies', 'Phase 2 success, satellite provider partnerships, regulatory approvals in target markets'],
                ['Primary Risk', 'Data residency compliance complexity across 50+ jurisdictions'],
                ['Validation Metrics', '50M DAU; 99.99% uptime; road discovery accuracy > 95%'],
              ]
            }
          }
        ]
      },
      {
        id: 'xv-4',
        number: '4',
        title: 'Phase 4 — Full Strategic Layer',
        content: 'Complete the strategic defensibility layer including payments, marketplace, developer platform, and full smart city integration. Establish G.A.N.E as the canonical global mobility infrastructure.',
        subsections: [
          {
            id: 'xv-4-details',
            number: '1.1',
            title: 'Phase 4 Details',
            content: '',
            table: {
              headers: ['Dimension', 'Details'],
              rows: [
                ['Duration', '24 months post-Phase 3'],
                ['Team Size', '1,000+ engineers'],
                ['Target Geography', 'Global'],
                ['Deliverables', 'Payments platform, developer marketplace, full digital twin, quantum-resilient security, autonomous vehicle integration'],
                ['Dependencies', 'Phase 3 at 50M+ DAU, regulatory approvals for payments, AV partnerships'],
                ['Primary Risk', 'Regulatory fragmentation preventing global payments layer'],
                ['Validation Metrics', '500M DAU; >100 city digital twin integrations; $1B+ GMV on marketplace'],
              ]
            }
          }
        ]
      }
    ]
  }
];

export const totalSections = specParts.reduce((acc, part) => acc + part.sections.length, 0);
export const totalParts = specParts.length;
