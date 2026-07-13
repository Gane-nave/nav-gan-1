//! Oscillation suppression — detects and dampens traffic oscillations caused
//! by navigation systems repeatedly rerouting users between the same corridors.

use chrono::{DateTime, Duration, Utc};
use gane_core::types::EntityId;
use std::collections::HashMap;
use tracing::{debug, info};

/// A reroute event observed in the network.
#[derive(Debug, Clone)]
pub struct RerouteEvent {
    pub user_id: EntityId,
    pub from_route: EntityId,
    pub to_route: EntityId,
    pub timestamp: DateTime<Utc>,
}

/// An oscillation pattern detected between two routes.
#[derive(Debug, Clone)]
pub struct OscillationPattern {
    pub route_a: EntityId,
    pub route_b: EntityId,
    /// Number of flip-flops observed.
    pub flip_count: u32,
    /// Fraction of reroutes that are part of this oscillation.
    pub severity: f64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// Dampening action to suppress an oscillation.
#[derive(Debug, Clone, PartialEq)]
pub struct DampeningAction {
    pub route_a: EntityId,
    pub route_b: EntityId,
    /// Cooldown period: do not reroute between these routes for this duration.
    pub cooldown_s: u64,
    /// Weight penalty applied to the "from" route to prevent flip-back.
    pub penalty: f64,
    pub issued_at: DateTime<Utc>,
}

/// Configuration for oscillation detection.
#[derive(Debug, Clone)]
pub struct OscillationConfig {
    /// Time window for detecting oscillations (seconds).
    pub detection_window_s: i64,
    /// Minimum flip-flops to classify as oscillation.
    pub min_flip_count: u32,
    /// Cooldown applied when oscillation is detected (seconds).
    pub cooldown_s: u64,
    /// Weight penalty for suppressed route pair.
    pub penalty: f64,
    /// Maximum number of active dampenings.
    pub max_active_dampenings: usize,
}

impl Default for OscillationConfig {
    fn default() -> Self {
        Self {
            detection_window_s: 600,
            min_flip_count: 3,
            cooldown_s: 180,
            penalty: 0.3,
            max_active_dampenings: 50,
        }
    }
}

/// Oscillation detector and suppressor.
pub struct OscillationSuppressor {
    config: OscillationConfig,
    /// Recent reroute events.
    events: Vec<RerouteEvent>,
    /// Active dampening actions (route_pair → action).
    dampenings: HashMap<(EntityId, EntityId), DampeningAction>,
}

impl OscillationSuppressor {
    pub fn new() -> Self {
        Self {
            config: OscillationConfig::default(),
            events: Vec::new(),
            dampenings: HashMap::new(),
        }
    }

    pub fn with_config(config: OscillationConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Record a reroute event.
    pub fn record_reroute(&mut self, event: RerouteEvent) {
        debug!(
            user = %event.user_id,
            from = %event.from_route,
            to = %event.to_route,
            "reroute event recorded"
        );
        self.events.push(event);
    }

    /// Detect oscillation patterns in the recent reroute history.
    pub fn detect(&self) -> Vec<OscillationPattern> {
        let cutoff = Utc::now() - Duration::seconds(self.config.detection_window_s);
        let recent: Vec<&RerouteEvent> = self
            .events
            .iter()
            .filter(|e| e.timestamp >= cutoff)
            .collect();

        if recent.is_empty() {
            return Vec::new();
        }

        // Count flip-flops between route pairs.
        let mut pair_counts: HashMap<(EntityId, EntityId), Vec<&RerouteEvent>> = HashMap::new();
        for event in &recent {
            let key = normalise_pair(event.from_route, event.to_route);
            pair_counts.entry(key).or_default().push(event);
        }

        let mut patterns = Vec::new();
        let total_reroutes = recent.len() as f64;

        for ((a, b), events) in &pair_counts {
            // Count actual flip-flops (A→B followed by B→A or vice versa).
            let flip_count = count_flips(events, *a, *b);

            if flip_count >= self.config.min_flip_count {
                let first_seen = events.iter().map(|e| e.timestamp).min().unwrap();
                let last_seen = events.iter().map(|e| e.timestamp).max().unwrap();

                patterns.push(OscillationPattern {
                    route_a: *a,
                    route_b: *b,
                    flip_count,
                    severity: events.len() as f64 / total_reroutes,
                    first_seen,
                    last_seen,
                });
            }
        }

        patterns
    }

    /// Run detection and apply dampening to any oscillations found.
    pub fn suppress(&mut self) -> Vec<DampeningAction> {
        let patterns = self.detect();
        let now = Utc::now();
        let mut new_actions = Vec::new();

        for pattern in &patterns {
            let key = normalise_pair(pattern.route_a, pattern.route_b);

            // Don't duplicate existing dampening.
            if self.dampenings.contains_key(&key) {
                continue;
            }

            let action = DampeningAction {
                route_a: pattern.route_a,
                route_b: pattern.route_b,
                cooldown_s: self.config.cooldown_s,
                penalty: self.config.penalty,
                issued_at: now,
            };

            info!(
                route_a = %pattern.route_a,
                route_b = %pattern.route_b,
                flips = pattern.flip_count,
                "oscillation dampening applied"
            );

            self.dampenings.insert(key, action.clone());
            new_actions.push(action);

            if self.dampenings.len() >= self.config.max_active_dampenings {
                break;
            }
        }

        new_actions
    }

    /// Check if a reroute between two routes is currently dampened.
    pub fn is_dampened(&self, from_route: &EntityId, to_route: &EntityId) -> bool {
        let key = normalise_pair(*from_route, *to_route);
        if let Some(action) = self.dampenings.get(&key) {
            let elapsed = (Utc::now() - action.issued_at).num_seconds() as u64;
            elapsed < action.cooldown_s
        } else {
            false
        }
    }

    /// Get the penalty for a specific route pair (0.0 if not dampened).
    pub fn get_penalty(&self, from_route: &EntityId, to_route: &EntityId) -> f64 {
        let key = normalise_pair(*from_route, *to_route);
        if let Some(action) = self.dampenings.get(&key) {
            let elapsed = (Utc::now() - action.issued_at).num_seconds() as u64;
            if elapsed < action.cooldown_s {
                return action.penalty;
            }
        }
        0.0
    }

    /// Expire old dampenings that have passed their cooldown.
    pub fn expire_dampenings(&mut self) {
        let now = Utc::now();
        let before = self.dampenings.len();
        self.dampenings.retain(|_, action| {
            let elapsed = (now - action.issued_at).num_seconds() as u64;
            elapsed < action.cooldown_s
        });
        let expired = before - self.dampenings.len();
        if expired > 0 {
            info!(expired, "expired oscillation dampenings");
        }
    }

    /// Prune old reroute events outside the detection window.
    pub fn prune_events(&mut self) {
        let cutoff = Utc::now() - Duration::seconds(self.config.detection_window_s);
        self.events.retain(|e| e.timestamp >= cutoff);
    }

    /// Number of active dampenings.
    pub fn active_dampening_count(&self) -> usize {
        self.dampenings.len()
    }

    /// Number of recorded reroute events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

impl Default for OscillationSuppressor {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalise a pair of route IDs so (A, B) == (B, A).
fn normalise_pair(a: EntityId, b: EntityId) -> (EntityId, EntityId) {
    if a.0.as_bytes() <= b.0.as_bytes() {
        (a, b)
    } else {
        (b, a)
    }
}

/// Count the number of flip-flops (direction changes) between two routes.
fn count_flips(events: &[&RerouteEvent], a: EntityId, b: EntityId) -> u32 {
    if events.len() < 2 {
        return 0;
    }

    // Sort by timestamp.
    let mut sorted: Vec<&&RerouteEvent> = events.iter().collect();
    sorted.sort_by_key(|e| e.timestamp);

    let mut flips = 0u32;
    let mut last_direction: Option<bool> = None; // true = A→B, false = B→A

    for event in sorted {
        let direction = if event.from_route == a && event.to_route == b {
            Some(true)
        } else if event.from_route == b && event.to_route == a {
            Some(false)
        } else {
            continue;
        };

        if let (Some(prev), Some(curr)) = (last_direction, direction) {
            if prev != curr {
                flips += 1;
            }
        }
        last_direction = direction;
    }

    flips
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reroute(from: EntityId, to: EntityId, offset_s: i64) -> RerouteEvent {
        RerouteEvent {
            user_id: EntityId::new(),
            from_route: from,
            to_route: to,
            timestamp: Utc::now() + Duration::seconds(offset_s),
        }
    }

    #[test]
    fn no_oscillation_without_flips() {
        let mut suppressor = OscillationSuppressor::new();
        let a = EntityId::new();
        let b = EntityId::new();

        // All reroutes in the same direction — no flip.
        suppressor.record_reroute(reroute(a, b, 0));
        suppressor.record_reroute(reroute(a, b, 10));

        let patterns = suppressor.detect();
        assert!(patterns.is_empty());
    }

    #[test]
    fn oscillation_detected_on_flip_flops() {
        let config = OscillationConfig {
            min_flip_count: 2,
            detection_window_s: 600,
            ..Default::default()
        };
        let mut suppressor = OscillationSuppressor::with_config(config);

        let a = EntityId::new();
        let b = EntityId::new();

        // A→B, B→A, A→B, B→A — 3 flips.
        suppressor.record_reroute(reroute(a, b, 0));
        suppressor.record_reroute(reroute(b, a, 10));
        suppressor.record_reroute(reroute(a, b, 20));
        suppressor.record_reroute(reroute(b, a, 30));

        let patterns = suppressor.detect();
        assert!(!patterns.is_empty());
        assert!(patterns[0].flip_count >= 2);
    }

    #[test]
    fn dampening_applied_on_suppress() {
        let config = OscillationConfig {
            min_flip_count: 2,
            cooldown_s: 300,
            penalty: 0.4,
            ..Default::default()
        };
        let mut suppressor = OscillationSuppressor::with_config(config);

        let a = EntityId::new();
        let b = EntityId::new();

        suppressor.record_reroute(reroute(a, b, 0));
        suppressor.record_reroute(reroute(b, a, 10));
        suppressor.record_reroute(reroute(a, b, 20));
        suppressor.record_reroute(reroute(b, a, 30));

        let actions = suppressor.suppress();
        assert!(!actions.is_empty());

        // The route pair should now be dampened.
        assert!(suppressor.is_dampened(&a, &b));
        assert!(suppressor.is_dampened(&b, &a)); // symmetric
        assert!((suppressor.get_penalty(&a, &b) - 0.4).abs() < f64::EPSILON);
    }

    #[test]
    fn undampened_routes_have_zero_penalty() {
        let suppressor = OscillationSuppressor::new();
        let a = EntityId::new();
        let b = EntityId::new();

        assert!(!suppressor.is_dampened(&a, &b));
        assert!((suppressor.get_penalty(&a, &b)).abs() < f64::EPSILON);
    }

    #[test]
    fn duplicate_dampening_not_applied() {
        let config = OscillationConfig {
            min_flip_count: 2,
            ..Default::default()
        };
        let mut suppressor = OscillationSuppressor::with_config(config);

        let a = EntityId::new();
        let b = EntityId::new();

        suppressor.record_reroute(reroute(a, b, 0));
        suppressor.record_reroute(reroute(b, a, 10));
        suppressor.record_reroute(reroute(a, b, 20));

        let actions1 = suppressor.suppress();
        let actions2 = suppressor.suppress();

        // Second suppress should not create duplicate dampenings.
        assert!(!actions1.is_empty());
        assert!(actions2.is_empty());
    }

    #[test]
    fn event_count_tracked() {
        let mut suppressor = OscillationSuppressor::new();
        let a = EntityId::new();
        let b = EntityId::new();

        assert_eq!(suppressor.event_count(), 0);

        suppressor.record_reroute(reroute(a, b, 0));
        suppressor.record_reroute(reroute(b, a, 10));

        assert_eq!(suppressor.event_count(), 2);
    }

    #[test]
    fn normalise_pair_symmetric() {
        let a = EntityId::new();
        let b = EntityId::new();

        assert_eq!(normalise_pair(a, b), normalise_pair(b, a));
    }
}
