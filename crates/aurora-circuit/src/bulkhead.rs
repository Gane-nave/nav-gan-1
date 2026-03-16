//! Bulkhead isolation — limits concurrent access to protect resources.

/// A bulkhead that limits concurrent access to a resource.
#[derive(Debug)]
pub struct Bulkhead {
    /// Maximum concurrent requests allowed.
    max_concurrent: u32,
    /// Current number of active requests.
    active: u32,
    /// Maximum queue depth (0 = no queuing).
    max_queue: u32,
    /// Current queue depth.
    queued: u32,
    /// Total requests accepted.
    total_accepted: u64,
    /// Total requests rejected.
    total_rejected: u64,
}

impl Bulkhead {
    /// Create a new bulkhead with the given concurrency limit.
    pub fn new(max_concurrent: u32) -> Self {
        Self {
            max_concurrent,
            active: 0,
            max_queue: 0,
            queued: 0,
            total_accepted: 0,
            total_rejected: 0,
        }
    }

    /// Create a bulkhead with queuing support.
    pub fn with_queue(max_concurrent: u32, max_queue: u32) -> Self {
        Self {
            max_concurrent,
            active: 0,
            max_queue,
            queued: 0,
            total_accepted: 0,
            total_rejected: 0,
        }
    }

    /// Try to acquire a permit. Returns the acquisition result.
    pub fn try_acquire(&mut self) -> AcquireResult {
        if self.active < self.max_concurrent {
            self.active += 1;
            self.total_accepted += 1;
            AcquireResult::Acquired
        } else if self.queued < self.max_queue {
            self.queued += 1;
            self.total_accepted += 1;
            AcquireResult::Queued
        } else {
            self.total_rejected += 1;
            AcquireResult::Rejected
        }
    }

    /// Release a permit (request completed).
    pub fn release(&mut self) {
        if self.active > 0 {
            self.active -= 1;
            // Promote from queue if available
            if self.queued > 0 {
                self.queued -= 1;
                self.active += 1;
            }
        }
    }

    /// Release a queued request (cancelled before execution).
    pub fn release_queued(&mut self) {
        if self.queued > 0 {
            self.queued -= 1;
        }
    }

    /// Current number of active requests.
    pub fn active(&self) -> u32 {
        self.active
    }

    /// Current queue depth.
    pub fn queued(&self) -> u32 {
        self.queued
    }

    /// Available permits.
    pub fn available(&self) -> u32 {
        self.max_concurrent.saturating_sub(self.active)
    }

    /// Utilization as a fraction (0.0 to 1.0).
    pub fn utilization(&self) -> f64 {
        if self.max_concurrent == 0 {
            return 0.0;
        }
        self.active as f64 / self.max_concurrent as f64
    }

    /// Get statistics.
    pub fn stats(&self) -> BulkheadStats {
        BulkheadStats {
            max_concurrent: self.max_concurrent,
            active: self.active,
            queued: self.queued,
            total_accepted: self.total_accepted,
            total_rejected: self.total_rejected,
        }
    }
}

/// Result of trying to acquire a bulkhead permit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcquireResult {
    /// Request can proceed immediately.
    Acquired,
    /// Request is queued, waiting for a slot.
    Queued,
    /// Request is rejected (no capacity).
    Rejected,
}

/// Bulkhead statistics.
#[derive(Debug, Clone)]
pub struct BulkheadStats {
    pub max_concurrent: u32,
    pub active: u32,
    pub queued: u32,
    pub total_accepted: u64,
    pub total_rejected: u64,
}

/// Manages multiple named bulkheads.
pub struct BulkheadRegistry {
    bulkheads: std::collections::HashMap<String, Bulkhead>,
}

impl BulkheadRegistry {
    /// Create a new registry.
    pub fn new() -> Self {
        Self {
            bulkheads: std::collections::HashMap::new(),
        }
    }

    /// Register a bulkhead with the given name and limits.
    pub fn register(&mut self, name: String, max_concurrent: u32) {
        self.bulkheads.insert(name, Bulkhead::new(max_concurrent));
    }

    /// Get a bulkhead by name.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Bulkhead> {
        self.bulkheads.get_mut(name)
    }

    /// Get a bulkhead by name (read-only).
    pub fn get(&self, name: &str) -> Option<&Bulkhead> {
        self.bulkheads.get(name)
    }

    /// Total active requests across all bulkheads.
    pub fn total_active(&self) -> u32 {
        self.bulkheads.values().map(|b| b.active()).sum()
    }

    /// List overloaded bulkheads (utilization > threshold).
    pub fn overloaded(&self, threshold: f64) -> Vec<(&str, f64)> {
        self.bulkheads
            .iter()
            .filter(|(_, b)| b.utilization() > threshold)
            .map(|(n, b)| (n.as_str(), b.utilization()))
            .collect()
    }
}

impl Default for BulkheadRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acquire_within_limit() {
        let mut bh = Bulkhead::new(3);
        assert_eq!(bh.try_acquire(), AcquireResult::Acquired);
        assert_eq!(bh.try_acquire(), AcquireResult::Acquired);
        assert_eq!(bh.try_acquire(), AcquireResult::Acquired);
        assert_eq!(bh.active(), 3);
        assert_eq!(bh.available(), 0);
    }

    #[test]
    fn test_reject_over_limit() {
        let mut bh = Bulkhead::new(1);
        assert_eq!(bh.try_acquire(), AcquireResult::Acquired);
        assert_eq!(bh.try_acquire(), AcquireResult::Rejected);
    }

    #[test]
    fn test_release_frees_slot() {
        let mut bh = Bulkhead::new(1);
        assert_eq!(bh.try_acquire(), AcquireResult::Acquired);
        bh.release();
        assert_eq!(bh.active(), 0);
        assert_eq!(bh.try_acquire(), AcquireResult::Acquired);
    }

    #[test]
    fn test_queue_support() {
        let mut bh = Bulkhead::with_queue(1, 2);
        assert_eq!(bh.try_acquire(), AcquireResult::Acquired);
        assert_eq!(bh.try_acquire(), AcquireResult::Queued);
        assert_eq!(bh.try_acquire(), AcquireResult::Queued);
        assert_eq!(bh.try_acquire(), AcquireResult::Rejected); // queue full
        assert_eq!(bh.queued(), 2);
    }

    #[test]
    fn test_release_promotes_from_queue() {
        let mut bh = Bulkhead::with_queue(1, 2);
        bh.try_acquire(); // active
        bh.try_acquire(); // queued
        assert_eq!(bh.active(), 1);
        assert_eq!(bh.queued(), 1);

        bh.release(); // promotes queued to active
        assert_eq!(bh.active(), 1);
        assert_eq!(bh.queued(), 0);
    }

    #[test]
    fn test_utilization() {
        let mut bh = Bulkhead::new(4);
        bh.try_acquire();
        bh.try_acquire();
        assert!((bh.utilization() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_stats() {
        let mut bh = Bulkhead::new(2);
        bh.try_acquire();
        bh.try_acquire();
        bh.try_acquire(); // rejected

        let stats = bh.stats();
        assert_eq!(stats.total_accepted, 2);
        assert_eq!(stats.total_rejected, 1);
    }

    #[test]
    fn test_release_queued() {
        let mut bh = Bulkhead::with_queue(1, 3);
        bh.try_acquire();
        bh.try_acquire(); // queued
        bh.try_acquire(); // queued
        assert_eq!(bh.queued(), 2);

        bh.release_queued();
        assert_eq!(bh.queued(), 1);
    }

    #[test]
    fn test_registry() {
        let mut reg = BulkheadRegistry::new();
        reg.register("db".into(), 10);
        reg.register("api".into(), 5);

        let db = reg.get_mut("db").unwrap();
        db.try_acquire();
        db.try_acquire();

        assert_eq!(reg.total_active(), 2);
    }

    #[test]
    fn test_registry_overloaded() {
        let mut reg = BulkheadRegistry::new();
        reg.register("db".into(), 2);
        reg.register("api".into(), 10);

        let db = reg.get_mut("db").unwrap();
        db.try_acquire();
        db.try_acquire(); // 100% utilization

        let overloaded = reg.overloaded(0.5);
        assert_eq!(overloaded.len(), 1);
    }
}
