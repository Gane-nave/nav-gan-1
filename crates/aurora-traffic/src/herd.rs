//! Herd behaviour suppression — prevents navigation-induced traffic oscillations
//! by detecting when too many users are routed to the same alternative and
//! applying stochastic route allocation to distribute load.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{debug, info};

/// A route allocation request — the system decides which route variant a user gets.
#[derive(Debug, Clone)]
pub struct AllocationRequest {
    pub user_id: EntityId,
    pub route_options: Vec<RouteOption>,
}

/// A candidate route with its base score.
#[derive(Debug, Clone)]
pub struct RouteOption {
    pub route_id: EntityId,
    /// Base desirability score [0, 1] from the routing engine.
    pub base_score: f64,
    /// Segments this route uses (for load tracking).
    pub segment_ids: Vec<EntityId>,
}

/// The result of a stochastic allocation.
#[derive(Debug, Clone)]
pub struct AllocationResult {
    pub user_id: EntityId,
    pub selected_route: EntityId,
    /// Adjusted probability that was used for selection.
    pub selection_probability: f64,
    pub allocated_at: DateTime<Utc>,
}

/// Configuration for herd suppression.
#[derive(Debug, Clone)]
pub struct HerdConfig {
    /// Maximum fraction of users that can be assigned to any single route [0, 1].
    pub max_route_share: f64,
    /// How aggressively to redistribute (higher = more redistribution) [0, 1].
    pub redistribution_strength: f64,
    /// Time window for tracking allocations (seconds).
    pub tracking_window_s: i64,
    /// Minimum number of allocations before suppression kicks in.
    pub min_allocations_for_suppression: usize,
}

impl Default for HerdConfig {
    fn default() -> Self {
        Self {
            max_route_share: 0.5,
            redistribution_strength: 0.4,
            tracking_window_s: 300,
            min_allocations_for_suppression: 10,
        }
    }
}

/// Herd behaviour suppressor — tracks route allocations and redistributes
/// users to prevent oscillation and overloading of popular alternatives.
pub struct HerdSuppressor {
    config: HerdConfig,
    /// Recent allocation counts per route.
    route_allocations: HashMap<EntityId, Vec<DateTime<Utc>>>,
    /// Recent allocation timestamps per segment (for time-windowed load awareness).
    segment_load: HashMap<EntityId, Vec<DateTime<Utc>>>,
    /// Total allocations in the current window.
    total_allocations: u32,
}

impl HerdSuppressor {
    pub fn new() -> Self {
        Self {
            config: HerdConfig::default(),
            route_allocations: HashMap::new(),
            segment_load: HashMap::new(),
            total_allocations: 0,
        }
    }

    pub fn with_config(config: HerdConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Allocate a route for a user using stochastic load-aware selection.
    ///
    /// Instead of always picking the "best" route, this spreads users across
    /// alternatives to prevent herd behaviour and oscillation.
    pub fn allocate(&mut self, request: &AllocationRequest) -> Option<AllocationResult> {
        if request.route_options.is_empty() {
            return None;
        }

        if request.route_options.len() == 1 {
            let route = &request.route_options[0];
            self.record_allocation(route);
            return Some(AllocationResult {
                user_id: request.user_id,
                selected_route: route.route_id,
                selection_probability: 1.0,
                allocated_at: Utc::now(),
            });
        }

        // Compute adjusted probabilities.
        let probabilities = self.compute_probabilities(&request.route_options);

        // Deterministic selection based on user_id hash for reproducibility.
        let hash = simple_hash(&request.user_id) as f64 / u64::MAX as f64;
        let mut cumulative = 0.0;
        let mut selected_idx = probabilities.len() - 1;

        for (i, &prob) in probabilities.iter().enumerate() {
            cumulative += prob;
            if hash <= cumulative {
                selected_idx = i;
                break;
            }
        }

        let selected = &request.route_options[selected_idx];
        self.record_allocation(selected);

        debug!(
            user = %request.user_id,
            route = %selected.route_id,
            probability = probabilities[selected_idx],
            "route allocated"
        );

        Some(AllocationResult {
            user_id: request.user_id,
            selected_route: selected.route_id,
            selection_probability: probabilities[selected_idx],
            allocated_at: Utc::now(),
        })
    }

    /// Compute adjusted probabilities for route options, suppressing overloaded routes.
    fn compute_probabilities(&self, options: &[RouteOption]) -> Vec<f64> {
        let total = self.total_allocations.max(1) as f64;
        let suppress =
            self.total_allocations as usize >= self.config.min_allocations_for_suppression;

        // Start with base scores.
        let mut weights: Vec<f64> = options.iter().map(|o| o.base_score.max(0.01)).collect();

        if suppress {
            // Reduce weight for routes that already have a high share.
            for (i, option) in options.iter().enumerate() {
                let route_count = self
                    .route_allocations
                    .get(&option.route_id)
                    .map(|v| v.len())
                    .unwrap_or(0) as f64;
                let share = route_count / total;

                if share > self.config.max_route_share {
                    let excess = share - self.config.max_route_share;
                    let penalty =
                        1.0 - (excess * self.config.redistribution_strength * 5.0).min(0.8);
                    weights[i] *= penalty;

                    debug!(
                        route = %option.route_id,
                        share,
                        penalty,
                        "herd suppression applied"
                    );
                }
            }
        }

        // Normalise to probabilities.
        let sum: f64 = weights.iter().sum();
        if sum <= 0.0 {
            let uniform = 1.0 / options.len() as f64;
            return vec![uniform; options.len()];
        }

        weights.iter().map(|w| w / sum).collect()
    }

    /// Record that a route was allocated.
    fn record_allocation(&mut self, option: &RouteOption) {
        let now = Utc::now();
        self.route_allocations
            .entry(option.route_id)
            .or_default()
            .push(now);
        self.total_allocations += 1;

        for seg in &option.segment_ids {
            self.segment_load.entry(*seg).or_default().push(now);
        }
    }

    /// Prune old allocations outside the tracking window.
    pub fn prune(&mut self) {
        let cutoff = Utc::now() - chrono::Duration::seconds(self.config.tracking_window_s);

        let mut total_removed = 0u32;
        for timestamps in self.route_allocations.values_mut() {
            let before = timestamps.len();
            timestamps.retain(|t| *t >= cutoff);
            total_removed += (before - timestamps.len()) as u32;
        }

        self.total_allocations = self.total_allocations.saturating_sub(total_removed);

        // Remove empty entries.
        self.route_allocations.retain(|_, v| !v.is_empty());

        // Prune segment load timestamps as well so they stay accurate.
        for timestamps in self.segment_load.values_mut() {
            timestamps.retain(|t| *t >= cutoff);
        }
        self.segment_load.retain(|_, v| !v.is_empty());

        if total_removed > 0 {
            info!(removed = total_removed, "pruned old herd allocations");
        }
    }

    /// Get the current share of allocations for each route.
    pub fn route_shares(&self) -> HashMap<EntityId, f64> {
        let total = self.total_allocations.max(1) as f64;
        self.route_allocations
            .iter()
            .map(|(id, timestamps)| (*id, timestamps.len() as f64 / total))
            .collect()
    }

    /// Check if a route is currently experiencing herd behaviour.
    pub fn is_herding(&self, route_id: &EntityId) -> bool {
        if (self.total_allocations as usize) < self.config.min_allocations_for_suppression {
            return false;
        }
        let count = self
            .route_allocations
            .get(route_id)
            .map(|v| v.len())
            .unwrap_or(0) as f64;
        let share = count / self.total_allocations.max(1) as f64;
        share > self.config.max_route_share
    }

    /// Get the load on a specific segment (count of recent allocations).
    pub fn segment_load(&self, segment_id: &EntityId) -> u32 {
        self.segment_load
            .get(segment_id)
            .map(|v| v.len() as u32)
            .unwrap_or(0)
    }

    /// Total number of allocations tracked.
    pub fn total_allocations(&self) -> u32 {
        self.total_allocations
    }
}

impl Default for HerdSuppressor {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple deterministic hash for EntityId-based selection.
fn simple_hash(id: &EntityId) -> u64 {
    let bytes = id.0.as_bytes();
    let mut hash: u64 = 14695981039346656037; // FNV offset basis
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(1099511628211); // FNV prime
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_options(count: usize) -> Vec<RouteOption> {
        (0..count)
            .map(|i| RouteOption {
                route_id: EntityId::new(),
                base_score: 0.5 + (i as f64 * 0.1),
                segment_ids: vec![EntityId::new()],
            })
            .collect()
    }

    fn make_request(options: &[RouteOption]) -> AllocationRequest {
        AllocationRequest {
            user_id: EntityId::new(),
            route_options: options.to_vec(),
        }
    }

    #[test]
    fn single_option_always_selected() {
        let mut suppressor = HerdSuppressor::new();
        let options = make_options(1);
        let request = make_request(&options);

        let result = suppressor.allocate(&request).unwrap();
        assert_eq!(result.selected_route, options[0].route_id);
        assert!((result.selection_probability - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn empty_options_returns_none() {
        let mut suppressor = HerdSuppressor::new();
        let request = AllocationRequest {
            user_id: EntityId::new(),
            route_options: Vec::new(),
        };
        assert!(suppressor.allocate(&request).is_none());
    }

    #[test]
    fn allocations_are_tracked() {
        let mut suppressor = HerdSuppressor::new();
        let options = make_options(2);

        for _ in 0..10 {
            let request = make_request(&options);
            suppressor.allocate(&request);
        }

        assert_eq!(suppressor.total_allocations(), 10);
        let shares = suppressor.route_shares();
        // At least one route should have allocations.
        assert!(!shares.is_empty());
    }

    #[test]
    fn herd_detection_works() {
        let config = HerdConfig {
            max_route_share: 0.4,
            min_allocations_for_suppression: 5,
            ..Default::default()
        };
        let mut suppressor = HerdSuppressor::with_config(config);

        let popular_route = EntityId::new();
        let other_route = EntityId::new();

        let option_popular = RouteOption {
            route_id: popular_route,
            base_score: 0.9,
            segment_ids: vec![EntityId::new()],
        };

        // Force 8 allocations to the popular route.
        for _ in 0..8 {
            suppressor.record_allocation(&option_popular);
        }

        // Add 2 to the other route.
        let option_other = RouteOption {
            route_id: other_route,
            base_score: 0.5,
            segment_ids: vec![EntityId::new()],
        };
        for _ in 0..2 {
            suppressor.record_allocation(&option_other);
        }

        assert!(suppressor.is_herding(&popular_route)); // 80% > 40%
        assert!(!suppressor.is_herding(&other_route)); // 20% < 40%
    }

    #[test]
    fn suppression_reduces_overloaded_route_probability() {
        let config = HerdConfig {
            max_route_share: 0.3,
            redistribution_strength: 0.8,
            min_allocations_for_suppression: 5,
            ..Default::default()
        };
        let mut suppressor = HerdSuppressor::with_config(config);

        let r1 = EntityId::new();
        let r2 = EntityId::new();

        let opt1 = RouteOption {
            route_id: r1,
            base_score: 0.9,
            segment_ids: vec![],
        };
        let opt2 = RouteOption {
            route_id: r2,
            base_score: 0.5,
            segment_ids: vec![],
        };

        // Force r1 to have high share.
        for _ in 0..8 {
            suppressor.record_allocation(&opt1);
        }
        for _ in 0..2 {
            suppressor.record_allocation(&opt2);
        }

        let probs = suppressor.compute_probabilities(&[opt1.clone(), opt2.clone()]);

        // After suppression, r1's probability should be reduced relative to its base score.
        // Without suppression, r1 would get 0.9/(0.9+0.5) ≈ 0.643.
        // With suppression, it should be lower.
        let base_ratio = 0.9 / (0.9 + 0.5);
        assert!(
            probs[0] < base_ratio,
            "suppressed prob {} should be < base ratio {}",
            probs[0],
            base_ratio
        );
    }

    #[test]
    fn segment_load_tracked() {
        let mut suppressor = HerdSuppressor::new();
        let seg = EntityId::new();

        let option = RouteOption {
            route_id: EntityId::new(),
            base_score: 0.8,
            segment_ids: vec![seg],
        };

        suppressor.record_allocation(&option);
        suppressor.record_allocation(&option);
        suppressor.record_allocation(&option);

        assert_eq!(suppressor.segment_load(&seg), 3);
    }
}
