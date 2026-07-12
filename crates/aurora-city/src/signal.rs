//! Traffic signal management — adaptive signal control, phase timing,
//! preemption requests, green wave coordination, and SPaT (Signal Phase
//! and Timing) ingestion.

use aurora_core::infrastructure::{TrafficPhase, TrafficSignalState};
use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{debug, info, warn};

// ---------------------------------------------------------------------------
// Preemption
// ---------------------------------------------------------------------------

/// Priority level for signal preemption requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreemptionPriority {
    /// Lowest — transit signal priority.
    Transit,
    /// Medium — freight / logistics.
    Freight,
    /// High — emergency vehicle.
    Emergency,
    /// Highest — active mass-casualty or evacuation.
    Evacuation,
}

/// A request to preempt a traffic signal.
#[derive(Debug, Clone)]
pub struct PreemptionRequest {
    pub id: EntityId,
    pub signal_id: EntityId,
    pub priority: PreemptionPriority,
    pub requested_phase: TrafficPhase,
    pub vehicle_id: EntityId,
    pub requested_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Outcome of a preemption evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreemptionOutcome {
    Granted,
    Queued,
    Denied,
    Expired,
}

// ---------------------------------------------------------------------------
// Green wave
// ---------------------------------------------------------------------------

/// A green-wave corridor definition: a sequence of signals timed so that
/// a vehicle travelling at `target_speed_kmh` hits green at each one.
#[derive(Debug, Clone)]
pub struct GreenWaveCorridor {
    pub id: EntityId,
    pub name: String,
    /// Ordered list of signal IDs along the corridor.
    pub signal_ids: Vec<EntityId>,
    /// Target travel speed for the wave (km/h).
    pub target_speed_kmh: f64,
    /// Desired offset between successive signals (seconds).
    pub offsets_s: Vec<f64>,
}

/// Result of a green-wave quality assessment.
#[derive(Debug, Clone)]
pub struct GreenWaveQuality {
    pub corridor_id: EntityId,
    /// Fraction of signals currently in sync with the wave [0, 1].
    pub sync_ratio: f64,
    /// Average deviation from ideal offset (seconds).
    pub avg_offset_deviation_s: f64,
    pub assessed_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Adaptive control
// ---------------------------------------------------------------------------

/// Per-direction demand estimate fed into adaptive timing.
#[derive(Debug, Clone)]
pub struct DirectionDemand {
    pub direction: String,
    pub queue_length_m: f64,
    pub arrival_rate_veh_per_min: f64,
}

/// Adaptive timing recommendation for a single signal.
#[derive(Debug, Clone)]
pub struct AdaptiveTimingRecommendation {
    pub signal_id: EntityId,
    pub recommended_green_s: HashMap<String, f64>,
    pub recommended_cycle_s: f64,
    pub reason: String,
    pub computed_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Signal controller
// ---------------------------------------------------------------------------

/// Configuration for the signal controller.
#[derive(Debug, Clone)]
pub struct SignalControllerConfig {
    /// Minimum green time for any phase (seconds).
    pub min_green_s: f64,
    /// Maximum green time for any phase (seconds).
    pub max_green_s: f64,
    /// Default cycle duration (seconds).
    pub default_cycle_s: f64,
    /// Maximum preemption duration (seconds).
    pub max_preemption_duration_s: f64,
}

impl Default for SignalControllerConfig {
    fn default() -> Self {
        Self {
            min_green_s: 10.0,
            max_green_s: 90.0,
            default_cycle_s: 120.0,
            max_preemption_duration_s: 60.0,
        }
    }
}

/// Manages traffic signals across the network, including SPaT ingestion,
/// preemption handling, adaptive timing, and green-wave coordination.
pub struct SignalController {
    config: SignalControllerConfig,
    /// Live signal states keyed by signal ID.
    signals: HashMap<EntityId, TrafficSignalState>,
    /// Pending preemption requests, sorted by priority.
    preemption_queue: Vec<PreemptionRequest>,
    /// Active preemption grants (signal_id → request).
    active_preemptions: HashMap<EntityId, PreemptionRequest>,
    /// Green-wave corridors.
    corridors: HashMap<EntityId, GreenWaveCorridor>,
}

impl SignalController {
    pub fn new() -> Self {
        Self {
            config: SignalControllerConfig::default(),
            signals: HashMap::new(),
            preemption_queue: Vec::new(),
            active_preemptions: HashMap::new(),
            corridors: HashMap::new(),
        }
    }

    pub fn with_config(config: SignalControllerConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    // -----------------------------------------------------------------------
    // SPaT ingestion
    // -----------------------------------------------------------------------

    /// Ingest or update a signal's current state (e.g., from a SPaT feed).
    pub fn update_signal(&mut self, state: TrafficSignalState) {
        debug!(signal = %state.id, phase = ?state.current_phase, "signal updated");
        self.signals.insert(state.id, state);
    }

    /// Get the current state of a signal.
    pub fn get_signal(&self, id: &EntityId) -> Option<&TrafficSignalState> {
        self.signals.get(id)
    }

    /// Number of tracked signals.
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Predict the phase a signal will be in after `seconds` from now.
    pub fn predict_phase(&self, signal_id: &EntityId, seconds_ahead: f64) -> Option<TrafficPhase> {
        let signal = self.signals.get(signal_id)?;
        if signal.cycle_duration_s <= 0.0 {
            return Some(signal.current_phase);
        }

        let remaining = signal.time_to_change_s.unwrap_or(0.0);
        if seconds_ahead <= remaining {
            return Some(signal.current_phase);
        }

        // Simple model: after the current phase ends we cycle through
        // Green → Yellow → Red → Green.
        let time_after_change = seconds_ahead - remaining;
        let phases = [TrafficPhase::Green, TrafficPhase::Yellow, TrafficPhase::Red];
        let current_idx = phases
            .iter()
            .position(|p| *p == signal.current_phase)
            .unwrap_or(0);

        // Assume equal split across three phases.
        let phase_duration = signal.cycle_duration_s / 3.0;
        if phase_duration <= 0.0 {
            return Some(signal.current_phase);
        }
        let phases_elapsed = (time_after_change / phase_duration) as usize;
        let idx = (current_idx + 1 + phases_elapsed) % phases.len();
        Some(phases[idx])
    }

    // -----------------------------------------------------------------------
    // Preemption
    // -----------------------------------------------------------------------

    /// Submit a preemption request. Returns the outcome.
    pub fn request_preemption(&mut self, request: PreemptionRequest) -> PreemptionOutcome {
        let now = Utc::now();

        if request.expires_at <= now {
            debug!(id = %request.id, "preemption request expired on arrival");
            return PreemptionOutcome::Expired;
        }

        if !self.signals.contains_key(&request.signal_id) {
            warn!(signal = %request.signal_id, "preemption for unknown signal");
            return PreemptionOutcome::Denied;
        }

        // Check if there's already an active preemption at higher or equal priority.
        if let Some(active) = self.active_preemptions.get(&request.signal_id) {
            if active.priority >= request.priority {
                debug!(
                    signal = %request.signal_id,
                    existing = ?active.priority,
                    incoming = ?request.priority,
                    "preemption queued behind higher priority"
                );
                self.preemption_queue.push(request);
                self.preemption_queue
                    .sort_by_key(|r| std::cmp::Reverse(r.priority));
                return PreemptionOutcome::Queued;
            }
            // Incoming has higher priority — override.
            info!(
                signal = %request.signal_id,
                "overriding active preemption with higher priority"
            );
        }

        // Grant immediately.
        info!(
            signal = %request.signal_id,
            priority = ?request.priority,
            "preemption granted"
        );
        self.active_preemptions.insert(request.signal_id, request);
        PreemptionOutcome::Granted
    }

    /// Release a preemption. Promotes the next queued request if any.
    pub fn release_preemption(&mut self, signal_id: &EntityId) -> bool {
        if self.active_preemptions.remove(signal_id).is_none() {
            return false;
        }

        // Promote the highest-priority queued request for this signal.
        let best = self
            .preemption_queue
            .iter()
            .enumerate()
            .filter(|(_, r)| r.signal_id == *signal_id)
            .max_by_key(|(_, r)| r.priority)
            .map(|(i, _)| i);

        if let Some(pos) = best {
            let next = self.preemption_queue.remove(pos);
            info!(signal = %signal_id, priority = ?next.priority, "promoted queued preemption");
            self.active_preemptions.insert(*signal_id, next);
        }

        true
    }

    /// Expire stale preemptions and queue entries.
    pub fn expire_preemptions(&mut self) -> usize {
        let now = Utc::now();
        let mut expired = 0;

        // Expire active.
        let to_expire: Vec<EntityId> = self
            .active_preemptions
            .iter()
            .filter(|(_, r)| r.expires_at <= now)
            .map(|(id, _)| *id)
            .collect();

        for signal_id in to_expire {
            self.active_preemptions.remove(&signal_id);
            expired += 1;
        }

        // Expire queued.
        let before = self.preemption_queue.len();
        self.preemption_queue.retain(|r| r.expires_at > now);
        expired += before - self.preemption_queue.len();

        if expired > 0 {
            debug!(count = expired, "expired preemptions");
        }
        expired
    }

    /// Get the active preemption for a signal, if any.
    pub fn active_preemption(&self, signal_id: &EntityId) -> Option<&PreemptionRequest> {
        self.active_preemptions.get(signal_id)
    }

    /// Number of pending preemption requests in the queue.
    pub fn queued_preemption_count(&self) -> usize {
        self.preemption_queue.len()
    }

    // -----------------------------------------------------------------------
    // Adaptive timing
    // -----------------------------------------------------------------------

    /// Compute adaptive timing recommendation for a signal given current demand.
    pub fn compute_adaptive_timing(
        &self,
        signal_id: &EntityId,
        demands: &[DirectionDemand],
    ) -> Option<AdaptiveTimingRecommendation> {
        let _signal = self.signals.get(signal_id)?;

        if demands.is_empty() {
            return None;
        }

        let total_demand: f64 = demands.iter().map(|d| d.arrival_rate_veh_per_min).sum();
        if total_demand <= 0.0 {
            return None;
        }

        let mut recommended_green = HashMap::new();
        let cycle = self.config.default_cycle_s;

        for demand in demands {
            let share = demand.arrival_rate_veh_per_min / total_demand;
            // Queue pressure bonus: longer queues get extra time.
            let queue_bonus = (demand.queue_length_m / 100.0).min(0.2);
            let raw_green = cycle * (share + queue_bonus);
            let clamped = raw_green.clamp(self.config.min_green_s, self.config.max_green_s);
            recommended_green.insert(demand.direction.clone(), clamped);
        }

        // Adjust cycle if total recommended green exceeds it.
        let total_green: f64 = recommended_green.values().sum();
        let recommended_cycle = total_green.max(cycle);

        Some(AdaptiveTimingRecommendation {
            signal_id: *signal_id,
            recommended_green_s: recommended_green,
            recommended_cycle_s: recommended_cycle,
            reason: format!(
                "demand-based split across {} directions, total rate {:.1} veh/min",
                demands.len(),
                total_demand
            ),
            computed_at: Utc::now(),
        })
    }

    // -----------------------------------------------------------------------
    // Green wave
    // -----------------------------------------------------------------------

    /// Register a green-wave corridor.
    pub fn add_green_wave(&mut self, corridor: GreenWaveCorridor) {
        info!(id = %corridor.id, name = %corridor.name, "green wave registered");
        self.corridors.insert(corridor.id, corridor);
    }

    /// Assess the quality of a green-wave corridor.
    pub fn assess_green_wave(&self, corridor_id: &EntityId) -> Option<GreenWaveQuality> {
        let corridor = self.corridors.get(corridor_id)?;

        if corridor.signal_ids.len() < 2 {
            return Some(GreenWaveQuality {
                corridor_id: *corridor_id,
                sync_ratio: 1.0,
                avg_offset_deviation_s: 0.0,
                assessed_at: Utc::now(),
            });
        }

        let mut synced = 0usize;
        let mut total_deviation = 0.0;
        let mut pairs = 0usize;

        for i in 0..corridor.signal_ids.len() - 1 {
            let s1 = self.signals.get(&corridor.signal_ids[i]);
            let s2 = self.signals.get(&corridor.signal_ids[i + 1]);

            let (Some(sig1), Some(sig2)) = (s1, s2) else {
                continue;
            };

            pairs += 1;

            // Ideal: both are green and the time-to-change difference matches offset.
            let desired_offset = corridor.offsets_s.get(i).copied().unwrap_or(0.0);

            let ttc1 = sig1.time_to_change_s.unwrap_or(0.0);
            let ttc2 = sig2.time_to_change_s.unwrap_or(0.0);
            let actual_offset = (ttc2 - ttc1).abs();
            let deviation = (actual_offset - desired_offset).abs();

            total_deviation += deviation;

            // Consider in-sync if both green and deviation < 5s.
            if sig1.current_phase == TrafficPhase::Green
                && sig2.current_phase == TrafficPhase::Green
                && deviation < 5.0
            {
                synced += 1;
            }
        }

        let sync_ratio = if pairs > 0 {
            synced as f64 / pairs as f64
        } else {
            0.0
        };

        let avg_deviation = if pairs > 0 {
            total_deviation / pairs as f64
        } else {
            0.0
        };

        Some(GreenWaveQuality {
            corridor_id: *corridor_id,
            sync_ratio,
            avg_offset_deviation_s: avg_deviation,
            assessed_at: Utc::now(),
        })
    }

    /// Count of registered green-wave corridors.
    pub fn corridor_count(&self) -> usize {
        self.corridors.len()
    }
}

impl Default for SignalController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::types::GeoPosition;
    use chrono::Duration;

    fn make_signal(id: EntityId, phase: TrafficPhase, ttc: f64) -> TrafficSignalState {
        TrafficSignalState {
            id,
            position: GeoPosition {
                latitude_deg: 32.0,
                longitude_deg: 34.0,
                altitude_m: Some(0.0),
            },
            current_phase: phase,
            time_to_change_s: Some(ttc),
            cycle_duration_s: 90.0,
            spat_available: true,
            updated_at: Utc::now(),
        }
    }

    fn make_preemption(
        signal_id: EntityId,
        priority: PreemptionPriority,
        ttl_s: i64,
    ) -> PreemptionRequest {
        let now = Utc::now();
        PreemptionRequest {
            id: EntityId::new(),
            signal_id,
            priority,
            requested_phase: TrafficPhase::Green,
            vehicle_id: EntityId::new(),
            requested_at: now,
            expires_at: now + Duration::seconds(ttl_s),
        }
    }

    #[test]
    fn ingest_and_query_signal() {
        let mut ctrl = SignalController::new();
        let id = EntityId::new();
        ctrl.update_signal(make_signal(id, TrafficPhase::Green, 20.0));

        assert_eq!(ctrl.signal_count(), 1);
        let sig = ctrl.get_signal(&id).unwrap();
        assert_eq!(sig.current_phase, TrafficPhase::Green);
    }

    #[test]
    fn predict_phase_within_current() {
        let mut ctrl = SignalController::new();
        let id = EntityId::new();
        ctrl.update_signal(make_signal(id, TrafficPhase::Green, 20.0));

        assert_eq!(ctrl.predict_phase(&id, 10.0), Some(TrafficPhase::Green));
    }

    #[test]
    fn predict_phase_cycles_correctly() {
        let mut ctrl = SignalController::new();
        let id = EntityId::new();
        ctrl.update_signal(make_signal(id, TrafficPhase::Green, 5.0));

        // 5s remaining → after 5s, next phase (Yellow).
        assert_eq!(ctrl.predict_phase(&id, 6.0), Some(TrafficPhase::Yellow));
    }

    #[test]
    fn preemption_grant_for_known_signal() {
        let mut ctrl = SignalController::new();
        let sig_id = EntityId::new();
        ctrl.update_signal(make_signal(sig_id, TrafficPhase::Red, 30.0));

        let req = make_preemption(sig_id, PreemptionPriority::Emergency, 60);
        assert_eq!(ctrl.request_preemption(req), PreemptionOutcome::Granted);
        assert!(ctrl.active_preemption(&sig_id).is_some());
    }

    #[test]
    fn preemption_denied_for_unknown_signal() {
        let mut ctrl = SignalController::new();
        let unknown = EntityId::new();
        let req = make_preemption(unknown, PreemptionPriority::Emergency, 60);
        assert_eq!(ctrl.request_preemption(req), PreemptionOutcome::Denied);
    }

    #[test]
    fn lower_priority_queued_behind_active() {
        let mut ctrl = SignalController::new();
        let sig_id = EntityId::new();
        ctrl.update_signal(make_signal(sig_id, TrafficPhase::Red, 30.0));

        let high = make_preemption(sig_id, PreemptionPriority::Evacuation, 60);
        let low = make_preemption(sig_id, PreemptionPriority::Transit, 60);

        assert_eq!(ctrl.request_preemption(high), PreemptionOutcome::Granted);
        assert_eq!(ctrl.request_preemption(low), PreemptionOutcome::Queued);
        assert_eq!(ctrl.queued_preemption_count(), 1);
    }

    #[test]
    fn higher_priority_overrides_active() {
        let mut ctrl = SignalController::new();
        let sig_id = EntityId::new();
        ctrl.update_signal(make_signal(sig_id, TrafficPhase::Red, 30.0));

        let low = make_preemption(sig_id, PreemptionPriority::Transit, 60);
        let high = make_preemption(sig_id, PreemptionPriority::Evacuation, 60);

        assert_eq!(ctrl.request_preemption(low), PreemptionOutcome::Granted);
        assert_eq!(ctrl.request_preemption(high), PreemptionOutcome::Granted);

        let active = ctrl.active_preemption(&sig_id).unwrap();
        assert_eq!(active.priority, PreemptionPriority::Evacuation);
    }

    #[test]
    fn release_promotes_queued() {
        let mut ctrl = SignalController::new();
        let sig_id = EntityId::new();
        ctrl.update_signal(make_signal(sig_id, TrafficPhase::Red, 30.0));

        let high = make_preemption(sig_id, PreemptionPriority::Emergency, 60);
        let low = make_preemption(sig_id, PreemptionPriority::Transit, 60);

        ctrl.request_preemption(high);
        ctrl.request_preemption(low);

        assert!(ctrl.release_preemption(&sig_id));
        let active = ctrl.active_preemption(&sig_id).unwrap();
        assert_eq!(active.priority, PreemptionPriority::Transit);
        assert_eq!(ctrl.queued_preemption_count(), 0);
    }

    #[test]
    fn expire_stale_preemptions() {
        let mut ctrl = SignalController::new();
        let sig_id = EntityId::new();
        ctrl.update_signal(make_signal(sig_id, TrafficPhase::Red, 30.0));

        // Create an already-expired request by setting expires_at in the past.
        let now = Utc::now();
        let expired_req = PreemptionRequest {
            id: EntityId::new(),
            signal_id: sig_id,
            priority: PreemptionPriority::Transit,
            requested_phase: TrafficPhase::Green,
            vehicle_id: EntityId::new(),
            requested_at: now - Duration::seconds(120),
            expires_at: now - Duration::seconds(1),
        };
        // Directly insert as active to simulate an expiry scenario.
        ctrl.active_preemptions.insert(sig_id, expired_req);

        let count = ctrl.expire_preemptions();
        assert_eq!(count, 1);
        assert!(ctrl.active_preemption(&sig_id).is_none());
    }

    #[test]
    fn adaptive_timing_proportional_to_demand() {
        let mut ctrl = SignalController::new();
        let sig_id = EntityId::new();
        ctrl.update_signal(make_signal(sig_id, TrafficPhase::Green, 20.0));

        let demands = vec![
            DirectionDemand {
                direction: "north".into(),
                queue_length_m: 50.0,
                arrival_rate_veh_per_min: 10.0,
            },
            DirectionDemand {
                direction: "east".into(),
                queue_length_m: 0.0,
                arrival_rate_veh_per_min: 5.0,
            },
        ];

        let rec = ctrl.compute_adaptive_timing(&sig_id, &demands).unwrap();
        let north_green = rec.recommended_green_s["north"];
        let east_green = rec.recommended_green_s["east"];

        // North has higher demand + queue → should get more green time.
        assert!(north_green > east_green);
    }

    #[test]
    fn adaptive_timing_clamps_green() {
        let config = SignalControllerConfig {
            min_green_s: 15.0,
            max_green_s: 60.0,
            ..Default::default()
        };
        let mut ctrl = SignalController::with_config(config);
        let sig_id = EntityId::new();
        ctrl.update_signal(make_signal(sig_id, TrafficPhase::Green, 20.0));

        let demands = vec![DirectionDemand {
            direction: "south".into(),
            queue_length_m: 0.0,
            arrival_rate_veh_per_min: 1.0,
        }];

        let rec = ctrl.compute_adaptive_timing(&sig_id, &demands).unwrap();
        let green = rec.recommended_green_s["south"];
        assert!(green >= 15.0);
        assert!(green <= 60.0);
    }

    #[test]
    fn green_wave_single_signal_perfect() {
        let mut ctrl = SignalController::new();
        let sig_id = EntityId::new();
        ctrl.update_signal(make_signal(sig_id, TrafficPhase::Green, 20.0));

        let corridor = GreenWaveCorridor {
            id: EntityId::new(),
            name: "Single".into(),
            signal_ids: vec![sig_id],
            target_speed_kmh: 50.0,
            offsets_s: vec![],
        };
        let cid = corridor.id;
        ctrl.add_green_wave(corridor);

        let quality = ctrl.assess_green_wave(&cid).unwrap();
        assert!((quality.sync_ratio - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn green_wave_synced_pair() {
        let mut ctrl = SignalController::new();
        let s1 = EntityId::new();
        let s2 = EntityId::new();

        ctrl.update_signal(make_signal(s1, TrafficPhase::Green, 20.0));
        ctrl.update_signal(make_signal(s2, TrafficPhase::Green, 15.0));

        let corridor = GreenWaveCorridor {
            id: EntityId::new(),
            name: "Pair".into(),
            signal_ids: vec![s1, s2],
            target_speed_kmh: 50.0,
            offsets_s: vec![5.0], // desired offset matches |20-15|=5
        };
        let cid = corridor.id;
        ctrl.add_green_wave(corridor);

        let quality = ctrl.assess_green_wave(&cid).unwrap();
        assert!((quality.sync_ratio - 1.0).abs() < f64::EPSILON);
        assert!(quality.avg_offset_deviation_s < 1.0);
    }

    #[test]
    fn green_wave_desynced() {
        let mut ctrl = SignalController::new();
        let s1 = EntityId::new();
        let s2 = EntityId::new();

        ctrl.update_signal(make_signal(s1, TrafficPhase::Green, 20.0));
        ctrl.update_signal(make_signal(s2, TrafficPhase::Red, 10.0)); // not green

        let corridor = GreenWaveCorridor {
            id: EntityId::new(),
            name: "Desynced".into(),
            signal_ids: vec![s1, s2],
            target_speed_kmh: 50.0,
            offsets_s: vec![5.0],
        };
        let cid = corridor.id;
        ctrl.add_green_wave(corridor);

        let quality = ctrl.assess_green_wave(&cid).unwrap();
        assert!(quality.sync_ratio < 1.0);
    }
}
