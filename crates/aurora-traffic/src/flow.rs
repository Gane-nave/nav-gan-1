//! Traffic flow controller — measures, balances, and controls traffic flow across
//! the road network. Implements stochastic route allocation, corridor throttling,
//! dynamic reroute limits, and residential protection zones.

use aurora_core::infrastructure::LevelOfService;
use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Real-time flow observation for a road segment.
#[derive(Debug, Clone)]
pub struct FlowObservation {
    pub segment_id: EntityId,
    pub timestamp: DateTime<Utc>,
    /// Current speed in km/h.
    pub speed_kmh: f64,
    /// Free-flow speed in km/h (the speed under ideal uncongested conditions).
    pub free_flow_speed_kmh: f64,
    /// Vehicle density (vehicles per km).
    pub density_veh_per_km: f64,
    /// Flow rate (vehicles per hour).
    pub flow_veh_per_hour: f64,
}

/// A corridor is a named group of segments that can be throttled together.
#[derive(Debug, Clone)]
pub struct Corridor {
    pub id: EntityId,
    pub name: String,
    pub segment_ids: Vec<EntityId>,
    /// Maximum share of total network traffic this corridor should carry [0, 1].
    pub max_traffic_share: f64,
}

/// Throttle decision for a corridor or segment.
#[derive(Debug, Clone, PartialEq)]
pub struct ThrottleDecision {
    pub entity_id: EntityId,
    pub reason: ThrottleReason,
    /// Fraction of traffic to divert away [0, 1].
    pub diversion_fraction: f64,
    pub issued_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThrottleReason {
    CorridorOverloaded,
    ResidentialProtection,
    CongestionSpillover,
    FairnessRebalance,
}

/// Configuration for the flow controller.
#[derive(Debug, Clone)]
pub struct FlowControlConfig {
    /// Speed ratio below which a segment is considered congested.
    pub congestion_speed_ratio: f64,
    /// Maximum number of reroutes per user per trip.
    pub max_reroutes_per_trip: u32,
    /// Residential zone maximum flow (vehicles per hour).
    pub residential_max_flow: f64,
    /// Weight given to fairness vs efficiency [0, 1].
    pub fairness_weight: f64,
}

impl Default for FlowControlConfig {
    fn default() -> Self {
        Self {
            congestion_speed_ratio: 0.4,
            max_reroutes_per_trip: 3,
            residential_max_flow: 200.0,
            fairness_weight: 0.3,
        }
    }
}

/// The closed-loop traffic flow controller.
///
/// Cycle: Measure → Predict → Allocate → Monitor → Correct.
pub struct FlowController {
    config: FlowControlConfig,
    /// Latest flow observations per segment.
    observations: HashMap<EntityId, FlowObservation>,
    /// Defined corridors.
    corridors: HashMap<EntityId, Corridor>,
    /// Segments marked as residential protection zones.
    residential_zones: HashMap<EntityId, f64>, // segment_id → max flow
    /// Active throttle decisions.
    active_throttles: Vec<ThrottleDecision>,
    /// Per-user reroute count for the current trip window.
    reroute_counts: HashMap<EntityId, u32>,
}

impl FlowController {
    pub fn new() -> Self {
        Self {
            config: FlowControlConfig::default(),
            observations: HashMap::new(),
            corridors: HashMap::new(),
            residential_zones: HashMap::new(),
            active_throttles: Vec::new(),
            reroute_counts: HashMap::new(),
        }
    }

    pub fn with_config(config: FlowControlConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    // -----------------------------------------------------------------------
    // Measure
    // -----------------------------------------------------------------------

    /// Ingest a flow observation.
    pub fn observe(&mut self, obs: FlowObservation) {
        debug!(segment = %obs.segment_id, speed = obs.speed_kmh, "flow observation");
        self.observations.insert(obs.segment_id, obs);
    }

    /// Get the latest observation for a segment.
    pub fn get_observation(&self, segment_id: &EntityId) -> Option<&FlowObservation> {
        self.observations.get(segment_id)
    }

    // -----------------------------------------------------------------------
    // Classify
    // -----------------------------------------------------------------------

    /// Classify a segment's level of service from current observations.
    pub fn classify_los(&self, segment_id: &EntityId) -> LevelOfService {
        let Some(obs) = self.observations.get(segment_id) else {
            return LevelOfService::A; // no data ⇒ assume free flow
        };

        if obs.free_flow_speed_kmh <= 0.0 {
            return LevelOfService::A;
        }

        let ratio = obs.speed_kmh / obs.free_flow_speed_kmh;
        match ratio {
            r if r >= 0.85 => LevelOfService::A,
            r if r >= 0.70 => LevelOfService::B,
            r if r >= 0.55 => LevelOfService::C,
            r if r >= 0.40 => LevelOfService::D,
            r if r >= 0.25 => LevelOfService::E,
            _ => LevelOfService::F,
        }
    }

    /// Check whether a segment is congested.
    pub fn is_congested(&self, segment_id: &EntityId) -> bool {
        let Some(obs) = self.observations.get(segment_id) else {
            return false;
        };
        if obs.free_flow_speed_kmh <= 0.0 {
            return false;
        }
        (obs.speed_kmh / obs.free_flow_speed_kmh) < self.config.congestion_speed_ratio
    }

    /// Compute congestion ratio for a segment [0, 1] where 1 = fully congested.
    pub fn congestion_ratio(&self, segment_id: &EntityId) -> f64 {
        let Some(obs) = self.observations.get(segment_id) else {
            return 0.0;
        };
        if obs.free_flow_speed_kmh <= 0.0 {
            return 0.0;
        }
        (1.0 - obs.speed_kmh / obs.free_flow_speed_kmh).clamp(0.0, 1.0)
    }

    // -----------------------------------------------------------------------
    // Corridors & zones
    // -----------------------------------------------------------------------

    /// Register a corridor.
    pub fn add_corridor(&mut self, corridor: Corridor) {
        info!(corridor_id = %corridor.id, name = %corridor.name, "corridor registered");
        self.corridors.insert(corridor.id, corridor);
    }

    /// Mark a segment as a residential protection zone.
    pub fn add_residential_zone(&mut self, segment_id: EntityId, max_flow: Option<f64>) {
        let limit = max_flow.unwrap_or(self.config.residential_max_flow);
        self.residential_zones.insert(segment_id, limit);
    }

    // -----------------------------------------------------------------------
    // Allocate & Control
    // -----------------------------------------------------------------------

    /// Run the full control cycle: check corridors, residential zones, and congestion.
    /// Returns new throttle decisions.
    pub fn run_control_cycle(&mut self) -> Vec<ThrottleDecision> {
        let now = Utc::now();
        let mut decisions = Vec::new();

        // 1. Corridor overload detection.
        let corridor_decisions = self.check_corridors(now);
        decisions.extend(corridor_decisions);

        // 2. Residential protection zone enforcement.
        let residential_decisions = self.check_residential_zones(now);
        decisions.extend(residential_decisions);

        // 3. Congestion spillover detection.
        let spillover_decisions = self.check_congestion_spillover(now);
        decisions.extend(spillover_decisions);

        self.active_throttles = decisions.clone();
        decisions
    }

    /// Check corridors for overloading.
    fn check_corridors(&self, now: DateTime<Utc>) -> Vec<ThrottleDecision> {
        let mut decisions = Vec::new();

        // Compute total network flow.
        let total_flow: f64 = self
            .observations
            .values()
            .map(|o| o.flow_veh_per_hour)
            .sum();

        if total_flow <= 0.0 {
            return decisions;
        }

        for corridor in self.corridors.values() {
            let corridor_flow: f64 = corridor
                .segment_ids
                .iter()
                .filter_map(|id| self.observations.get(id))
                .map(|o| o.flow_veh_per_hour)
                .sum();

            let share = corridor_flow / total_flow;
            if share > corridor.max_traffic_share {
                let excess = share - corridor.max_traffic_share;
                let diversion = (excess / share).clamp(0.0, 0.5);

                debug!(
                    corridor = %corridor.id,
                    share,
                    max = corridor.max_traffic_share,
                    diversion,
                    "corridor overloaded"
                );

                decisions.push(ThrottleDecision {
                    entity_id: corridor.id,
                    reason: ThrottleReason::CorridorOverloaded,
                    diversion_fraction: diversion,
                    issued_at: now,
                });
            }
        }

        decisions
    }

    /// Check residential protection zones for excessive flow.
    fn check_residential_zones(&self, now: DateTime<Utc>) -> Vec<ThrottleDecision> {
        let mut decisions = Vec::new();

        for (&segment_id, &max_flow) in &self.residential_zones {
            if let Some(obs) = self.observations.get(&segment_id) {
                if obs.flow_veh_per_hour > max_flow {
                    let excess_ratio = (obs.flow_veh_per_hour - max_flow) / obs.flow_veh_per_hour;

                    warn!(
                        segment = %segment_id,
                        flow = obs.flow_veh_per_hour,
                        max_flow,
                        "residential zone flow exceeded"
                    );

                    decisions.push(ThrottleDecision {
                        entity_id: segment_id,
                        reason: ThrottleReason::ResidentialProtection,
                        diversion_fraction: excess_ratio.clamp(0.0, 0.8),
                        issued_at: now,
                    });
                }
            }
        }

        decisions
    }

    /// Detect segments where congestion is spilling over into neighbours.
    fn check_congestion_spillover(&self, now: DateTime<Utc>) -> Vec<ThrottleDecision> {
        let mut decisions = Vec::new();

        // Simple heuristic: if a segment has LOS F and high density, flag it.
        for (segment_id, obs) in &self.observations {
            if obs.free_flow_speed_kmh <= 0.0 {
                continue;
            }

            let ratio = obs.speed_kmh / obs.free_flow_speed_kmh;
            if ratio < 0.15 && obs.density_veh_per_km > 80.0 {
                decisions.push(ThrottleDecision {
                    entity_id: *segment_id,
                    reason: ThrottleReason::CongestionSpillover,
                    diversion_fraction: 0.4,
                    issued_at: now,
                });
            }
        }

        decisions
    }

    // -----------------------------------------------------------------------
    // Reroute limiting
    // -----------------------------------------------------------------------

    /// Check if a user is allowed another reroute in the current trip.
    pub fn can_reroute(&self, user_id: &EntityId) -> bool {
        let count = self.reroute_counts.get(user_id).copied().unwrap_or(0);
        count < self.config.max_reroutes_per_trip
    }

    /// Record a reroute for a user. Returns false if limit reached.
    pub fn record_reroute(&mut self, user_id: EntityId) -> bool {
        let count = self.reroute_counts.entry(user_id).or_insert(0);
        if *count >= self.config.max_reroutes_per_trip {
            return false;
        }
        *count += 1;
        true
    }

    /// Reset reroute counts (e.g., at trip start).
    pub fn reset_reroute_counts(&mut self) {
        self.reroute_counts.clear();
    }

    // -----------------------------------------------------------------------
    // Queries
    // -----------------------------------------------------------------------

    /// Get active throttle decisions.
    pub fn active_throttles(&self) -> &[ThrottleDecision] {
        &self.active_throttles
    }

    /// Number of observed segments.
    pub fn observed_count(&self) -> usize {
        self.observations.len()
    }

    /// Count of congested segments.
    pub fn congested_count(&self) -> usize {
        self.observations
            .keys()
            .filter(|id| self.is_congested(id))
            .count()
    }
}

impl Default for FlowController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs(id: EntityId, speed: f64, free_flow: f64, density: f64, flow: f64) -> FlowObservation {
        FlowObservation {
            segment_id: id,
            timestamp: Utc::now(),
            speed_kmh: speed,
            free_flow_speed_kmh: free_flow,
            density_veh_per_km: density,
            flow_veh_per_hour: flow,
        }
    }

    #[test]
    fn classify_los_levels() {
        let mut ctrl = FlowController::new();
        let seg = EntityId::new();

        ctrl.observe(obs(seg, 90.0, 100.0, 10.0, 900.0));
        assert_eq!(ctrl.classify_los(&seg), LevelOfService::A);

        ctrl.observe(obs(seg, 60.0, 100.0, 30.0, 1800.0));
        assert_eq!(ctrl.classify_los(&seg), LevelOfService::C);

        ctrl.observe(obs(seg, 20.0, 100.0, 80.0, 1600.0));
        assert_eq!(ctrl.classify_los(&seg), LevelOfService::F);
    }

    #[test]
    fn congestion_detection() {
        let mut ctrl = FlowController::new();
        let seg = EntityId::new();

        ctrl.observe(obs(seg, 80.0, 100.0, 15.0, 1200.0));
        assert!(!ctrl.is_congested(&seg));
        assert!(ctrl.congestion_ratio(&seg) < 0.5);

        ctrl.observe(obs(seg, 20.0, 100.0, 70.0, 1400.0));
        assert!(ctrl.is_congested(&seg));
        assert!(ctrl.congestion_ratio(&seg) > 0.5);
    }

    #[test]
    fn corridor_throttling() {
        let mut ctrl = FlowController::new();

        let s1 = EntityId::new();
        let s2 = EntityId::new();
        let s3 = EntityId::new();

        // Corridor contains s1 and s2, set max share 30%.
        let corridor = Corridor {
            id: EntityId::new(),
            name: "Main Highway".into(),
            segment_ids: vec![s1, s2],
            max_traffic_share: 0.3,
        };
        let corridor_id = corridor.id;
        ctrl.add_corridor(corridor);

        // s1+s2 carry 800 out of 1000 total (80% share > 30% limit).
        ctrl.observe(obs(s1, 50.0, 100.0, 40.0, 500.0));
        ctrl.observe(obs(s2, 50.0, 100.0, 20.0, 300.0));
        ctrl.observe(obs(s3, 60.0, 100.0, 10.0, 200.0));

        let decisions = ctrl.run_control_cycle();
        let corridor_decision = decisions
            .iter()
            .find(|d| d.entity_id == corridor_id && d.reason == ThrottleReason::CorridorOverloaded);
        assert!(corridor_decision.is_some());
        assert!(corridor_decision.unwrap().diversion_fraction > 0.0);
    }

    #[test]
    fn residential_zone_protection() {
        let mut ctrl = FlowController::new();
        let seg = EntityId::new();

        ctrl.add_residential_zone(seg, Some(100.0));
        ctrl.observe(obs(seg, 30.0, 50.0, 20.0, 250.0)); // flow 250 > limit 100

        let decisions = ctrl.run_control_cycle();
        let residential = decisions
            .iter()
            .find(|d| d.reason == ThrottleReason::ResidentialProtection);
        assert!(residential.is_some());
        assert!(residential.unwrap().diversion_fraction > 0.3);
    }

    #[test]
    fn congestion_spillover_detected() {
        let mut ctrl = FlowController::new();
        let seg = EntityId::new();

        // Very low speed ratio + very high density.
        ctrl.observe(obs(seg, 10.0, 100.0, 100.0, 1000.0));

        let decisions = ctrl.run_control_cycle();
        let spillover = decisions
            .iter()
            .find(|d| d.reason == ThrottleReason::CongestionSpillover);
        assert!(spillover.is_some());
    }

    #[test]
    fn reroute_limiting() {
        let config = FlowControlConfig {
            max_reroutes_per_trip: 2,
            ..Default::default()
        };
        let mut ctrl = FlowController::with_config(config);
        let user = EntityId::new();

        assert!(ctrl.can_reroute(&user));
        assert!(ctrl.record_reroute(user));
        assert!(ctrl.record_reroute(user));
        assert!(!ctrl.record_reroute(user)); // limit reached
        assert!(!ctrl.can_reroute(&user));

        ctrl.reset_reroute_counts();
        assert!(ctrl.can_reroute(&user));
    }

    #[test]
    fn no_throttle_for_balanced_corridors() {
        let mut ctrl = FlowController::new();

        let s1 = EntityId::new();
        let s2 = EntityId::new();

        let corridor = Corridor {
            id: EntityId::new(),
            name: "Balanced Route".into(),
            segment_ids: vec![s1],
            max_traffic_share: 0.6,
        };
        ctrl.add_corridor(corridor);

        // s1 carries 400 out of 1000 (40% < 60% limit) — no throttle.
        ctrl.observe(obs(s1, 80.0, 100.0, 15.0, 400.0));
        ctrl.observe(obs(s2, 80.0, 100.0, 25.0, 600.0));

        let decisions = ctrl.run_control_cycle();
        assert!(decisions
            .iter()
            .all(|d| d.reason != ThrottleReason::CorridorOverloaded));
    }

    #[test]
    fn congested_count_tracks_segments() {
        let mut ctrl = FlowController::new();

        let s1 = EntityId::new();
        let s2 = EntityId::new();
        let s3 = EntityId::new();

        ctrl.observe(obs(s1, 80.0, 100.0, 10.0, 800.0)); // not congested
        ctrl.observe(obs(s2, 20.0, 100.0, 60.0, 1200.0)); // congested
        ctrl.observe(obs(s3, 15.0, 100.0, 70.0, 1050.0)); // congested

        assert_eq!(ctrl.observed_count(), 3);
        assert_eq!(ctrl.congested_count(), 2);
    }
}
