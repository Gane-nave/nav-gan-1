//! Load balancer — distributes traffic across service instances.

/// Load balancing strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalancerStrategy {
    /// Round-robin — cycles through instances.
    RoundRobin,
    /// Weighted round-robin — respects instance weights.
    WeightedRoundRobin,
    /// Least connections — picks instance with fewest active connections.
    LeastConnections,
    /// Random — picks a random instance (deterministic with seed).
    Random,
}

/// Instance state tracked by the balancer.
#[derive(Debug, Clone)]
pub struct InstanceState {
    /// Instance identifier.
    pub id: String,
    /// Instance weight (higher = more traffic).
    pub weight: u32,
    /// Active connection count.
    pub active_connections: u32,
    /// Total requests served.
    pub total_requests: u64,
    /// Total errors.
    pub total_errors: u64,
    /// Whether the instance is available.
    pub available: bool,
}

/// Load balancer.
pub struct LoadBalancer {
    strategy: BalancerStrategy,
    instances: Vec<InstanceState>,
    round_robin_index: usize,
    weighted_counter: u32,
    weighted_max: u32,
    random_seed: u64,
}

impl LoadBalancer {
    /// Create a new load balancer with a given strategy.
    pub fn new(strategy: BalancerStrategy) -> Self {
        Self {
            strategy,
            instances: Vec::new(),
            round_robin_index: 0,
            weighted_counter: 0,
            weighted_max: 0,
            random_seed: 42,
        }
    }

    /// Add an instance to the balancer.
    pub fn add_instance(&mut self, id: &str, weight: u32) {
        self.instances.push(InstanceState {
            id: id.to_string(),
            weight,
            active_connections: 0,
            total_requests: 0,
            total_errors: 0,
            available: true,
        });
        self.recalc_max_weight();
    }

    /// Remove an instance.
    pub fn remove_instance(&mut self, id: &str) -> bool {
        let before = self.instances.len();
        self.instances.retain(|i| i.id != id);
        let removed = self.instances.len() < before;
        if removed {
            self.recalc_max_weight();
        }
        removed
    }

    /// Set instance availability.
    pub fn set_available(&mut self, id: &str, available: bool) -> bool {
        if let Some(inst) = self.instances.iter_mut().find(|i| i.id == id) {
            inst.available = available;
            self.recalc_max_weight();
            true
        } else {
            false
        }
    }

    fn recalc_max_weight(&mut self) {
        self.weighted_max = self
            .instances
            .iter()
            .filter(|i| i.available)
            .map(|i| i.weight)
            .max()
            .unwrap_or(0);
    }

    /// Pick the next instance to route to.
    pub fn pick(&mut self) -> Option<String> {
        let available: Vec<usize> = self
            .instances
            .iter()
            .enumerate()
            .filter(|(_, i)| i.available)
            .map(|(idx, _)| idx)
            .collect();

        if available.is_empty() {
            return None;
        }

        let chosen_idx = match self.strategy {
            BalancerStrategy::RoundRobin => {
                let idx = self.round_robin_index % available.len();
                self.round_robin_index = self.round_robin_index.wrapping_add(1);
                available[idx]
            }
            BalancerStrategy::WeightedRoundRobin => {
                // Weighted round-robin: cycle through, skipping if counter > weight
                let mut iterations = 0usize;
                loop {
                    iterations += 1;
                    let idx = self.round_robin_index % self.instances.len();
                    self.round_robin_index = self.round_robin_index.wrapping_add(1);

                    if !self.instances[idx].available {
                        // Safety: prevent infinite loop if all weights are 0
                        if iterations > self.instances.len() * (self.weighted_max as usize + 2) {
                            break available[0];
                        }
                        continue;
                    }

                    if self.instances[idx].weight > self.weighted_counter {
                        // Every full cycle, increment counter
                        #[allow(unknown_lints, clippy::manual_is_multiple_of)]
                        if self.round_robin_index % self.instances.len() == 0 {
                            self.weighted_counter += 1;
                            if self.weighted_counter >= self.weighted_max {
                                self.weighted_counter = 0;
                            }
                        }
                        break idx;
                    }

                    // Safety: prevent infinite loop if all weights are 0
                    if iterations > self.instances.len() * (self.weighted_max as usize + 2) {
                        break available[0];
                    }
                }
            }
            BalancerStrategy::LeastConnections => *available
                .iter()
                .min_by_key(|&&idx| self.instances[idx].active_connections)
                .unwrap(),
            BalancerStrategy::Random => {
                // Simple LCG PRNG
                self.random_seed = self
                    .random_seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let r = (self.random_seed >> 33) as usize;
                available[r % available.len()]
            }
        };

        self.instances[chosen_idx].active_connections += 1;
        self.instances[chosen_idx].total_requests += 1;
        Some(self.instances[chosen_idx].id.clone())
    }

    /// Report request completion (decrements active connections).
    pub fn report_complete(&mut self, id: &str, success: bool) {
        if let Some(inst) = self.instances.iter_mut().find(|i| i.id == id) {
            inst.active_connections = inst.active_connections.saturating_sub(1);
            if !success {
                inst.total_errors += 1;
            }
        }
    }

    /// Get instance count.
    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }

    /// Get available instance count.
    pub fn available_count(&self) -> usize {
        self.instances.iter().filter(|i| i.available).count()
    }

    /// Get strategy.
    pub fn strategy(&self) -> BalancerStrategy {
        self.strategy
    }

    /// Get instance stats.
    pub fn get_stats(&self, id: &str) -> Option<&InstanceState> {
        self.instances.iter().find(|i| i.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin() {
        let mut lb = LoadBalancer::new(BalancerStrategy::RoundRobin);
        lb.add_instance("a", 1);
        lb.add_instance("b", 1);
        lb.add_instance("c", 1);

        assert_eq!(lb.pick(), Some("a".to_string()));
        assert_eq!(lb.pick(), Some("b".to_string()));
        assert_eq!(lb.pick(), Some("c".to_string()));
        assert_eq!(lb.pick(), Some("a".to_string())); // wraps around
    }

    #[test]
    fn test_round_robin_skips_unavailable() {
        let mut lb = LoadBalancer::new(BalancerStrategy::RoundRobin);
        lb.add_instance("a", 1);
        lb.add_instance("b", 1);
        lb.add_instance("c", 1);
        lb.set_available("b", false);

        // Should only cycle through a and c
        let mut seen = std::collections::HashSet::new();
        for _ in 0..4 {
            seen.insert(lb.pick().unwrap());
        }
        assert!(seen.contains("a"));
        assert!(seen.contains("c"));
        assert!(!seen.contains("b"));
    }

    #[test]
    fn test_least_connections() {
        let mut lb = LoadBalancer::new(BalancerStrategy::LeastConnections);
        lb.add_instance("a", 1);
        lb.add_instance("b", 1);

        // First request goes to a (both at 0)
        let first = lb.pick().unwrap();
        // Second should go to b (a has 1 connection now)
        let second = lb.pick().unwrap();
        assert_ne!(first, second);

        // Complete first request
        lb.report_complete(&first, true);
        // Third should go to first again (both at 1, then first completed to 0)
        let third = lb.pick().unwrap();
        assert_eq!(third, first);
    }

    #[test]
    fn test_no_available_instances() {
        let mut lb = LoadBalancer::new(BalancerStrategy::RoundRobin);
        assert_eq!(lb.pick(), None);

        lb.add_instance("a", 1);
        lb.set_available("a", false);
        assert_eq!(lb.pick(), None);
    }

    #[test]
    fn test_remove_instance() {
        let mut lb = LoadBalancer::new(BalancerStrategy::RoundRobin);
        lb.add_instance("a", 1);
        lb.add_instance("b", 1);
        assert!(lb.remove_instance("a"));
        assert_eq!(lb.instance_count(), 1);
        assert_eq!(lb.pick(), Some("b".to_string()));
    }

    #[test]
    fn test_report_complete_tracks_errors() {
        let mut lb = LoadBalancer::new(BalancerStrategy::RoundRobin);
        lb.add_instance("a", 1);
        lb.pick().unwrap(); // a
        lb.report_complete("a", false); // error
        assert_eq!(lb.get_stats("a").unwrap().total_errors, 1);
        assert_eq!(lb.get_stats("a").unwrap().active_connections, 0);
    }

    #[test]
    fn test_random_distributes() {
        let mut lb = LoadBalancer::new(BalancerStrategy::Random);
        lb.add_instance("a", 1);
        lb.add_instance("b", 1);
        lb.add_instance("c", 1);

        let mut counts = std::collections::HashMap::new();
        for _ in 0..100 {
            let picked = lb.pick().unwrap();
            lb.report_complete(&picked, true);
            *counts.entry(picked).or_insert(0u32) += 1;
        }
        // All three should have been picked at least once
        assert!(counts.contains_key("a"));
        assert!(counts.contains_key("b"));
        assert!(counts.contains_key("c"));
    }

    #[test]
    fn test_available_count() {
        let mut lb = LoadBalancer::new(BalancerStrategy::RoundRobin);
        lb.add_instance("a", 1);
        lb.add_instance("b", 1);
        lb.add_instance("c", 1);
        lb.set_available("b", false);
        assert_eq!(lb.available_count(), 2);
        assert_eq!(lb.instance_count(), 3);
    }
}
