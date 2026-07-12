/**
 * G.A.N.E — FORMAL STATE MACHINES
 * 
 * Spec Reference: Doc 9 §2
 * 
 * Every critical service MUST have a formal state machine.
 * All transitions are typed and validated.
 * Invalid transitions throw deterministic errors.
 * Every transition emits an event for observability.
 */

// ═══════════════════════════════════════════════════════════
// GENERIC STATE MACHINE FRAMEWORK
// ═══════════════════════════════════════════════════════════

export interface StateTransition<S extends string, E extends string> {
  from: S;
  to: S;
  event: E;
  guard?: (context: Record<string, unknown>) => boolean;
  action?: (context: Record<string, unknown>) => void;
}

export interface StateMachineConfig<S extends string, E extends string> {
  id: string;
  initial: S;
  states: readonly S[];
  transitions: StateTransition<S, E>[];
}

export interface StateChangeEvent<S extends string, E extends string> {
  machine_id: string;
  from: S;
  to: S;
  event: E;
  ts_utc: string;
  context?: Record<string, unknown>;
}

export class StateMachine<S extends string, E extends string> {
  private _state: S;
  private readonly config: StateMachineConfig<S, E>;
  private readonly history: StateChangeEvent<S, E>[] = [];
  private readonly listeners: ((event: StateChangeEvent<S, E>) => void)[] = [];
  private readonly maxHistory: number;

  constructor(config: StateMachineConfig<S, E>, maxHistory: number = 1000) {
    this.config = config;
    this._state = config.initial;
    this.maxHistory = maxHistory;
  }

  get state(): S { return this._state; }
  get id(): string { return this.config.id; }
  get stateHistory(): readonly StateChangeEvent<S, E>[] { return this.history; }

  /**
   * Attempt a state transition. Returns true if successful.
   * Throws if the transition is not defined.
   */
  transition(event: E, context: Record<string, unknown> = {}): boolean {
    const validTransitions = this.config.transitions.filter(
      t => t.from === this._state && t.event === event
    );

    if (validTransitions.length === 0) {
      throw new Error(
        `[StateMachine:${this.config.id}] Invalid transition: ${this._state} --${event}--> ? ` +
        `Valid events from ${this._state}: ${this.getValidEvents().join(', ') || 'none'}`
      );
    }

    // Find first transition whose guard passes (or has no guard)
    const transition = validTransitions.find(t => !t.guard || t.guard(context));
    if (!transition) {
      return false; // Guard blocked
    }

    const from = this._state;
    this._state = transition.to;

    // Execute action
    if (transition.action) {
      transition.action(context);
    }

    // Record history
    const changeEvent: StateChangeEvent<S, E> = {
      machine_id: this.config.id,
      from,
      to: transition.to,
      event,
      ts_utc: new Date().toISOString(),
      context: { ...context },
    };

    this.history.push(changeEvent);
    if (this.history.length > this.maxHistory) {
      this.history.shift();
    }

    // Notify listeners
    for (const listener of this.listeners) {
      listener(changeEvent);
    }

    return true;
  }

  /**
   * Get all valid events from the current state
   */
  getValidEvents(): E[] {
    return Array.from(new Set(
      this.config.transitions
        .filter(t => t.from === this._state)
        .map(t => t.event)
    ));
  }

  /**
   * Check if a transition is valid without executing it
   */
  canTransition(event: E): boolean {
    return this.config.transitions.some(t => t.from === this._state && t.event === event);
  }

  /**
   * Force state (for recovery/replay only)
   */
  forceState(state: S): void {
    if (!this.config.states.includes(state)) {
      throw new Error(`[StateMachine:${this.config.id}] Invalid state: ${state}`);
    }
    this._state = state;
  }

  /**
   * Subscribe to state changes
   */
  onTransition(listener: (event: StateChangeEvent<S, E>) => void): () => void {
    this.listeners.push(listener);
    return () => {
      const idx = this.listeners.indexOf(listener);
      if (idx >= 0) this.listeners.splice(idx, 1);
    };
  }

  /**
   * Get full state machine definition for visualization
   */
  getDefinition(): StateMachineConfig<S, E> {
    return { ...this.config };
  }

  /**
   * Reset to initial state
   */
  reset(): void {
    this._state = this.config.initial;
    this.history.length = 0;
  }
}

// ═══════════════════════════════════════════════════════════
// 1. POSITIONING STATE MACHINE (Doc 9 §2.1)
// ═══════════════════════════════════════════════════════════

export type PositioningState =
  | 'INITIALIZING'
  | 'GNSS_ACQUIRING'
  | 'GNSS_LOCKED'
  | 'FUSION_ACTIVE'
  | 'DEGRADED'
  | 'LOST'
  | 'RECOVERING';

export type PositioningEvent =
  | 'SENSORS_READY'
  | 'GNSS_ACQUIRED'
  | 'FUSION_STARTED'
  | 'SIGNAL_DEGRADED'
  | 'SIGNAL_LOST'
  | 'RECOVERY_STARTED'
  | 'RECOVERY_COMPLETE'
  | 'RESET';

export const POSITIONING_STATES: readonly PositioningState[] = [
  'INITIALIZING', 'GNSS_ACQUIRING', 'GNSS_LOCKED', 'FUSION_ACTIVE', 'DEGRADED', 'LOST', 'RECOVERING'
] as const;

export const positioningMachineConfig: StateMachineConfig<PositioningState, PositioningEvent> = {
  id: 'positioning',
  initial: 'INITIALIZING',
  states: POSITIONING_STATES,
  transitions: [
    { from: 'INITIALIZING', to: 'GNSS_ACQUIRING', event: 'SENSORS_READY' },
    { from: 'GNSS_ACQUIRING', to: 'GNSS_LOCKED', event: 'GNSS_ACQUIRED' },
    { from: 'GNSS_ACQUIRING', to: 'LOST', event: 'SIGNAL_LOST' },
    { from: 'GNSS_LOCKED', to: 'FUSION_ACTIVE', event: 'FUSION_STARTED' },
    { from: 'GNSS_LOCKED', to: 'DEGRADED', event: 'SIGNAL_DEGRADED' },
    { from: 'GNSS_LOCKED', to: 'LOST', event: 'SIGNAL_LOST' },
    { from: 'FUSION_ACTIVE', to: 'DEGRADED', event: 'SIGNAL_DEGRADED' },
    { from: 'FUSION_ACTIVE', to: 'LOST', event: 'SIGNAL_LOST' },
    { from: 'DEGRADED', to: 'RECOVERING', event: 'RECOVERY_STARTED' },
    { from: 'DEGRADED', to: 'LOST', event: 'SIGNAL_LOST' },
    { from: 'LOST', to: 'RECOVERING', event: 'RECOVERY_STARTED' },
    { from: 'RECOVERING', to: 'GNSS_LOCKED', event: 'GNSS_ACQUIRED' },
    { from: 'RECOVERING', to: 'FUSION_ACTIVE', event: 'FUSION_STARTED' },
    { from: 'RECOVERING', to: 'LOST', event: 'SIGNAL_LOST' },
    // Reset from any state
    { from: 'INITIALIZING', to: 'INITIALIZING', event: 'RESET' },
    { from: 'GNSS_ACQUIRING', to: 'INITIALIZING', event: 'RESET' },
    { from: 'GNSS_LOCKED', to: 'INITIALIZING', event: 'RESET' },
    { from: 'FUSION_ACTIVE', to: 'INITIALIZING', event: 'RESET' },
    { from: 'DEGRADED', to: 'INITIALIZING', event: 'RESET' },
    { from: 'LOST', to: 'INITIALIZING', event: 'RESET' },
    { from: 'RECOVERING', to: 'INITIALIZING', event: 'RESET' },
  ],
};

// ═══════════════════════════════════════════════════════════
// 2. CONTINUITY STATE MACHINE (extends positioning)
// ═══════════════════════════════════════════════════════════

export type ContinuityState =
  | 'FULL_GNSS'
  | 'FUSION_DEGRADED'
  | 'INS_BOUNDED'
  | 'DEAD_RECKONING'
  | 'EMERGENCY_POSITIONING'
  | 'RECOVERY_PENDING'
  | 'OFFLINE_MODE';

export type ContinuityEvent =
  | 'GNSS_AVAILABLE'
  | 'GNSS_LOST'
  | 'FUSION_DEGRADED'
  | 'INS_ONLY'
  | 'DRIFT_EXCEEDED'
  | 'EMERGENCY_ACTIVATED'
  | 'NETWORK_LOST'
  | 'NETWORK_RESTORED'
  | 'RECOVERY_COMPLETE';

export const CONTINUITY_STATES: readonly ContinuityState[] = [
  'FULL_GNSS', 'FUSION_DEGRADED', 'INS_BOUNDED', 'DEAD_RECKONING',
  'EMERGENCY_POSITIONING', 'RECOVERY_PENDING', 'OFFLINE_MODE'
] as const;

export const continuityMachineConfig: StateMachineConfig<ContinuityState, ContinuityEvent> = {
  id: 'continuity',
  initial: 'FULL_GNSS',
  states: CONTINUITY_STATES,
  transitions: [
    { from: 'FULL_GNSS', to: 'FUSION_DEGRADED', event: 'FUSION_DEGRADED' },
    { from: 'FULL_GNSS', to: 'INS_BOUNDED', event: 'GNSS_LOST' },
    { from: 'FULL_GNSS', to: 'OFFLINE_MODE', event: 'NETWORK_LOST' },
    { from: 'FUSION_DEGRADED', to: 'INS_BOUNDED', event: 'INS_ONLY' },
    { from: 'FUSION_DEGRADED', to: 'FULL_GNSS', event: 'GNSS_AVAILABLE' },
    { from: 'FUSION_DEGRADED', to: 'EMERGENCY_POSITIONING', event: 'EMERGENCY_ACTIVATED' },
    { from: 'INS_BOUNDED', to: 'DEAD_RECKONING', event: 'DRIFT_EXCEEDED' },
    { from: 'INS_BOUNDED', to: 'RECOVERY_PENDING', event: 'GNSS_AVAILABLE' },
    { from: 'INS_BOUNDED', to: 'EMERGENCY_POSITIONING', event: 'EMERGENCY_ACTIVATED' },
    { from: 'DEAD_RECKONING', to: 'RECOVERY_PENDING', event: 'GNSS_AVAILABLE' },
    { from: 'DEAD_RECKONING', to: 'EMERGENCY_POSITIONING', event: 'EMERGENCY_ACTIVATED' },
    { from: 'EMERGENCY_POSITIONING', to: 'RECOVERY_PENDING', event: 'GNSS_AVAILABLE' },
    { from: 'RECOVERY_PENDING', to: 'FULL_GNSS', event: 'RECOVERY_COMPLETE' },
    { from: 'RECOVERY_PENDING', to: 'FUSION_DEGRADED', event: 'FUSION_DEGRADED' },
    { from: 'OFFLINE_MODE', to: 'FULL_GNSS', event: 'NETWORK_RESTORED' },
    { from: 'OFFLINE_MODE', to: 'INS_BOUNDED', event: 'GNSS_LOST' },
  ],
};

// ═══════════════════════════════════════════════════════════
// 3. ROUTING STATE MACHINE (Doc 9 §2.2)
// ═══════════════════════════════════════════════════════════

export type RoutingState =
  | 'IDLE'
  | 'COMPUTING'
  | 'READY'
  | 'ACTIVE'
  | 'INVALIDATED'
  | 'REROUTING'
  | 'FAILED';

export type RoutingEvent =
  | 'COMPUTE_REQUESTED'
  | 'ROUTE_COMPUTED'
  | 'NAVIGATION_STARTED'
  | 'ROUTE_INVALIDATED'
  | 'REROUTE_TRIGGERED'
  | 'REROUTE_COMPLETE'
  | 'COMPUTATION_FAILED'
  | 'NAVIGATION_ENDED'
  | 'RESET';

export const ROUTING_STATES: readonly RoutingState[] = [
  'IDLE', 'COMPUTING', 'READY', 'ACTIVE', 'INVALIDATED', 'REROUTING', 'FAILED'
] as const;

export const routingMachineConfig: StateMachineConfig<RoutingState, RoutingEvent> = {
  id: 'routing',
  initial: 'IDLE',
  states: ROUTING_STATES,
  transitions: [
    { from: 'IDLE', to: 'COMPUTING', event: 'COMPUTE_REQUESTED' },
    { from: 'COMPUTING', to: 'READY', event: 'ROUTE_COMPUTED' },
    { from: 'COMPUTING', to: 'FAILED', event: 'COMPUTATION_FAILED' },
    { from: 'READY', to: 'ACTIVE', event: 'NAVIGATION_STARTED' },
    { from: 'READY', to: 'IDLE', event: 'RESET' },
    { from: 'ACTIVE', to: 'INVALIDATED', event: 'ROUTE_INVALIDATED' },
    { from: 'ACTIVE', to: 'REROUTING', event: 'REROUTE_TRIGGERED' },
    { from: 'ACTIVE', to: 'IDLE', event: 'NAVIGATION_ENDED' },
    { from: 'INVALIDATED', to: 'REROUTING', event: 'REROUTE_TRIGGERED' },
    { from: 'INVALIDATED', to: 'IDLE', event: 'RESET' },
    { from: 'REROUTING', to: 'ACTIVE', event: 'REROUTE_COMPLETE' },
    { from: 'REROUTING', to: 'FAILED', event: 'COMPUTATION_FAILED' },
    { from: 'FAILED', to: 'COMPUTING', event: 'COMPUTE_REQUESTED' },
    { from: 'FAILED', to: 'IDLE', event: 'RESET' },
  ],
};

// ═══════════════════════════════════════════════════════════
// 4. INCIDENT TRUST STATE MACHINE
// ═══════════════════════════════════════════════════════════

export type IncidentTrustState =
  | 'REPORTED'
  | 'EVIDENCE_ACCUMULATING'
  | 'CROSS_VALIDATING'
  | 'VALIDATED'
  | 'REJECTED'
  | 'RESOLVED'
  | 'EXPIRED';

export type IncidentTrustEvent =
  | 'REPORT_RECEIVED'
  | 'EVIDENCE_ADDED'
  | 'THRESHOLD_MET'
  | 'CROSS_VALIDATION_STARTED'
  | 'VALIDATION_PASSED'
  | 'VALIDATION_FAILED'
  | 'MANUALLY_RESOLVED'
  | 'TTL_EXPIRED'
  | 'COUNTER_EVIDENCE';

export const INCIDENT_TRUST_STATES: readonly IncidentTrustState[] = [
  'REPORTED', 'EVIDENCE_ACCUMULATING', 'CROSS_VALIDATING', 'VALIDATED', 'REJECTED', 'RESOLVED', 'EXPIRED'
] as const;

export const incidentTrustMachineConfig: StateMachineConfig<IncidentTrustState, IncidentTrustEvent> = {
  id: 'incident_trust',
  initial: 'REPORTED',
  states: INCIDENT_TRUST_STATES,
  transitions: [
    { from: 'REPORTED', to: 'EVIDENCE_ACCUMULATING', event: 'EVIDENCE_ADDED' },
    { from: 'REPORTED', to: 'EXPIRED', event: 'TTL_EXPIRED' },
    { from: 'EVIDENCE_ACCUMULATING', to: 'CROSS_VALIDATING', event: 'THRESHOLD_MET' },
    { from: 'EVIDENCE_ACCUMULATING', to: 'EVIDENCE_ACCUMULATING', event: 'EVIDENCE_ADDED' },
    { from: 'EVIDENCE_ACCUMULATING', to: 'EXPIRED', event: 'TTL_EXPIRED' },
    { from: 'CROSS_VALIDATING', to: 'VALIDATED', event: 'VALIDATION_PASSED' },
    { from: 'CROSS_VALIDATING', to: 'REJECTED', event: 'VALIDATION_FAILED' },
    { from: 'VALIDATED', to: 'RESOLVED', event: 'MANUALLY_RESOLVED' },
    { from: 'VALIDATED', to: 'REJECTED', event: 'COUNTER_EVIDENCE' },
    { from: 'VALIDATED', to: 'EXPIRED', event: 'TTL_EXPIRED' },
    { from: 'REJECTED', to: 'EVIDENCE_ACCUMULATING', event: 'EVIDENCE_ADDED' },
  ],
};

// ═══════════════════════════════════════════════════════════
// 5. ALERTING STATE MACHINE (Doc 9 §2.3)
// ═══════════════════════════════════════════════════════════

export type AlertingState =
  | 'CREATED'
  | 'QUEUED'
  | 'SENDING'
  | 'SENT'
  | 'DELIVERED'
  | 'FAILED'
  | 'RETRYING'
  | 'ABANDONED'
  | 'SUPPRESSED';

export type AlertingEvent =
  | 'ENQUEUED'
  | 'DISPATCH_STARTED'
  | 'DISPATCH_SUCCEEDED'
  | 'DELIVERY_CONFIRMED'
  | 'DISPATCH_FAILED'
  | 'RETRY_SCHEDULED'
  | 'MAX_RETRIES_REACHED'
  | 'DUPLICATE_DETECTED';

export const ALERTING_STATES: readonly AlertingState[] = [
  'CREATED', 'QUEUED', 'SENDING', 'SENT', 'DELIVERED', 'FAILED', 'RETRYING', 'ABANDONED', 'SUPPRESSED'
] as const;

export const alertingMachineConfig: StateMachineConfig<AlertingState, AlertingEvent> = {
  id: 'alerting',
  initial: 'CREATED',
  states: ALERTING_STATES,
  transitions: [
    { from: 'CREATED', to: 'QUEUED', event: 'ENQUEUED' },
    { from: 'CREATED', to: 'SUPPRESSED', event: 'DUPLICATE_DETECTED' },
    { from: 'QUEUED', to: 'SENDING', event: 'DISPATCH_STARTED' },
    { from: 'SENDING', to: 'SENT', event: 'DISPATCH_SUCCEEDED' },
    { from: 'SENDING', to: 'FAILED', event: 'DISPATCH_FAILED' },
    { from: 'SENT', to: 'DELIVERED', event: 'DELIVERY_CONFIRMED' },
    { from: 'SENT', to: 'FAILED', event: 'DISPATCH_FAILED' },
    { from: 'FAILED', to: 'RETRYING', event: 'RETRY_SCHEDULED' },
    { from: 'FAILED', to: 'ABANDONED', event: 'MAX_RETRIES_REACHED' },
    { from: 'RETRYING', to: 'SENDING', event: 'DISPATCH_STARTED' },
  ],
};

// ═══════════════════════════════════════════════════════════
// 6. PAYMENT STATE MACHINE
// ═══════════════════════════════════════════════════════════

export type PaymentState =
  | 'INITIATED'
  | 'PENDING'
  | 'AUTHORIZED'
  | 'CAPTURED'
  | 'FAILED'
  | 'REFUNDED'
  | 'DISPUTED'
  | 'CANCELLED';

export type PaymentEvent =
  | 'PAYMENT_SUBMITTED'
  | 'AUTHORIZATION_RECEIVED'
  | 'CAPTURE_CONFIRMED'
  | 'PAYMENT_FAILED'
  | 'REFUND_REQUESTED'
  | 'REFUND_PROCESSED'
  | 'DISPUTE_OPENED'
  | 'DISPUTE_RESOLVED'
  | 'PAYMENT_CANCELLED';

export const PAYMENT_STATES: readonly PaymentState[] = [
  'INITIATED', 'PENDING', 'AUTHORIZED', 'CAPTURED', 'FAILED', 'REFUNDED', 'DISPUTED', 'CANCELLED'
] as const;

export const paymentMachineConfig: StateMachineConfig<PaymentState, PaymentEvent> = {
  id: 'payment',
  initial: 'INITIATED',
  states: PAYMENT_STATES,
  transitions: [
    { from: 'INITIATED', to: 'PENDING', event: 'PAYMENT_SUBMITTED' },
    { from: 'INITIATED', to: 'CANCELLED', event: 'PAYMENT_CANCELLED' },
    { from: 'PENDING', to: 'AUTHORIZED', event: 'AUTHORIZATION_RECEIVED' },
    { from: 'PENDING', to: 'FAILED', event: 'PAYMENT_FAILED' },
    { from: 'PENDING', to: 'CANCELLED', event: 'PAYMENT_CANCELLED' },
    { from: 'AUTHORIZED', to: 'CAPTURED', event: 'CAPTURE_CONFIRMED' },
    { from: 'AUTHORIZED', to: 'FAILED', event: 'PAYMENT_FAILED' },
    { from: 'AUTHORIZED', to: 'CANCELLED', event: 'PAYMENT_CANCELLED' },
    { from: 'CAPTURED', to: 'REFUNDED', event: 'REFUND_PROCESSED' },
    { from: 'CAPTURED', to: 'DISPUTED', event: 'DISPUTE_OPENED' },
    { from: 'DISPUTED', to: 'CAPTURED', event: 'DISPUTE_RESOLVED' },
    { from: 'DISPUTED', to: 'REFUNDED', event: 'REFUND_PROCESSED' },
    { from: 'FAILED', to: 'PENDING', event: 'PAYMENT_SUBMITTED' },
  ],
};

// ═══════════════════════════════════════════════════════════
// 7. SYNC STATE MACHINE (Doc 9 §2.4)
// ═══════════════════════════════════════════════════════════

export type SyncState =
  | 'IN_SYNC'
  | 'OUT_OF_SYNC'
  | 'SYNCING'
  | 'CONFLICT'
  | 'RESOLVING'
  | 'FAILED';

export type SyncEvent =
  | 'LOCAL_CHANGE'
  | 'REMOTE_CHANGE'
  | 'SYNC_STARTED'
  | 'SYNC_COMPLETE'
  | 'CONFLICT_DETECTED'
  | 'CONFLICT_RESOLVED'
  | 'SYNC_FAILED'
  | 'RETRY';

export const SYNC_STATES: readonly SyncState[] = [
  'IN_SYNC', 'OUT_OF_SYNC', 'SYNCING', 'CONFLICT', 'RESOLVING', 'FAILED'
] as const;

export const syncMachineConfig: StateMachineConfig<SyncState, SyncEvent> = {
  id: 'sync',
  initial: 'IN_SYNC',
  states: SYNC_STATES,
  transitions: [
    { from: 'IN_SYNC', to: 'OUT_OF_SYNC', event: 'LOCAL_CHANGE' },
    { from: 'IN_SYNC', to: 'OUT_OF_SYNC', event: 'REMOTE_CHANGE' },
    { from: 'OUT_OF_SYNC', to: 'SYNCING', event: 'SYNC_STARTED' },
    { from: 'SYNCING', to: 'IN_SYNC', event: 'SYNC_COMPLETE' },
    { from: 'SYNCING', to: 'CONFLICT', event: 'CONFLICT_DETECTED' },
    { from: 'SYNCING', to: 'FAILED', event: 'SYNC_FAILED' },
    { from: 'CONFLICT', to: 'RESOLVING', event: 'CONFLICT_RESOLVED' },
    { from: 'RESOLVING', to: 'IN_SYNC', event: 'SYNC_COMPLETE' },
    { from: 'RESOLVING', to: 'FAILED', event: 'SYNC_FAILED' },
    { from: 'FAILED', to: 'SYNCING', event: 'RETRY' },
  ],
};

// ═══════════════════════════════════════════════════════════
// 8. OFFLINE PACKAGE STATE MACHINE (Doc 9 §2.5)
// ═══════════════════════════════════════════════════════════

export type OfflinePackageState =
  | 'NOT_PRESENT'
  | 'DOWNLOADING'
  | 'VALIDATING'
  | 'READY'
  | 'STALE'
  | 'INVALID'
  | 'REPLACING'
  | 'DELETING';

export type OfflinePackageEvent =
  | 'DOWNLOAD_STARTED'
  | 'DOWNLOAD_COMPLETE'
  | 'VALIDATION_PASSED'
  | 'VALIDATION_FAILED'
  | 'UPDATE_AVAILABLE'
  | 'REPLACEMENT_STARTED'
  | 'REPLACEMENT_COMPLETE'
  | 'DELETE_REQUESTED'
  | 'DELETE_COMPLETE'
  | 'CORRUPTION_DETECTED';

export const OFFLINE_PACKAGE_STATES: readonly OfflinePackageState[] = [
  'NOT_PRESENT', 'DOWNLOADING', 'VALIDATING', 'READY', 'STALE', 'INVALID', 'REPLACING', 'DELETING'
] as const;

export const offlinePackageMachineConfig: StateMachineConfig<OfflinePackageState, OfflinePackageEvent> = {
  id: 'offline_package',
  initial: 'NOT_PRESENT',
  states: OFFLINE_PACKAGE_STATES,
  transitions: [
    { from: 'NOT_PRESENT', to: 'DOWNLOADING', event: 'DOWNLOAD_STARTED' },
    { from: 'DOWNLOADING', to: 'VALIDATING', event: 'DOWNLOAD_COMPLETE' },
    { from: 'VALIDATING', to: 'READY', event: 'VALIDATION_PASSED' },
    { from: 'VALIDATING', to: 'INVALID', event: 'VALIDATION_FAILED' },
    { from: 'READY', to: 'STALE', event: 'UPDATE_AVAILABLE' },
    { from: 'READY', to: 'INVALID', event: 'CORRUPTION_DETECTED' },
    { from: 'READY', to: 'DELETING', event: 'DELETE_REQUESTED' },
    { from: 'STALE', to: 'REPLACING', event: 'REPLACEMENT_STARTED' },
    { from: 'STALE', to: 'DELETING', event: 'DELETE_REQUESTED' },
    { from: 'REPLACING', to: 'VALIDATING', event: 'REPLACEMENT_COMPLETE' },
    { from: 'INVALID', to: 'DOWNLOADING', event: 'DOWNLOAD_STARTED' },
    { from: 'INVALID', to: 'DELETING', event: 'DELETE_REQUESTED' },
    { from: 'DELETING', to: 'NOT_PRESENT', event: 'DELETE_COMPLETE' },
  ],
};

// ═══════════════════════════════════════════════════════════
// FACTORY — Create state machines from configs
// ═══════════════════════════════════════════════════════════

export function createPositioningMachine(): StateMachine<PositioningState, PositioningEvent> {
  return new StateMachine(positioningMachineConfig);
}

export function createContinuityMachine(): StateMachine<ContinuityState, ContinuityEvent> {
  return new StateMachine(continuityMachineConfig);
}

export function createRoutingMachine(): StateMachine<RoutingState, RoutingEvent> {
  return new StateMachine(routingMachineConfig);
}

export function createIncidentTrustMachine(): StateMachine<IncidentTrustState, IncidentTrustEvent> {
  return new StateMachine(incidentTrustMachineConfig);
}

export function createAlertingMachine(): StateMachine<AlertingState, AlertingEvent> {
  return new StateMachine(alertingMachineConfig);
}

export function createPaymentMachine(): StateMachine<PaymentState, PaymentEvent> {
  return new StateMachine(paymentMachineConfig);
}

export function createSyncMachine(): StateMachine<SyncState, SyncEvent> {
  return new StateMachine(syncMachineConfig);
}

export function createOfflinePackageMachine(): StateMachine<OfflinePackageState, OfflinePackageEvent> {
  return new StateMachine(offlinePackageMachineConfig);
}

/**
 * Registry of all state machines for a navigation session
 */
export interface SessionStateMachines {
  positioning: StateMachine<PositioningState, PositioningEvent>;
  continuity: StateMachine<ContinuityState, ContinuityEvent>;
  routing: StateMachine<RoutingState, RoutingEvent>;
  alerting: StateMachine<AlertingState, AlertingEvent>;
  sync: StateMachine<SyncState, SyncEvent>;
}

export function createSessionMachines(): SessionStateMachines {
  return {
    positioning: createPositioningMachine(),
    continuity: createContinuityMachine(),
    routing: createRoutingMachine(),
    alerting: createAlertingMachine(),
    sync: createSyncMachine(),
  };
}
