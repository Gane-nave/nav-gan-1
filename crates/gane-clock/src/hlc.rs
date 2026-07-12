//! Hybrid Logical Clock implementation.

use std::cmp::Ordering;

/// A hybrid logical clock timestamp combining physical and logical components.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HlcTimestamp {
    pub physical: u64,
    pub logical: u32,
    pub node_id: u32,
}

impl HlcTimestamp {
    pub fn new(physical: u64, logical: u32, node_id: u32) -> Self {
        Self {
            physical,
            logical,
            node_id,
        }
    }
}

impl Ord for HlcTimestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.physical
            .cmp(&other.physical)
            .then(self.logical.cmp(&other.logical))
            .then(self.node_id.cmp(&other.node_id))
    }
}

impl PartialOrd for HlcTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Hybrid Logical Clock for distributed event ordering.
pub struct HybridClock {
    node_id: u32,
    physical: u64,
    logical: u32,
    max_drift: u64,
}

impl HybridClock {
    /// Create a new hybrid clock for the given node.
    pub fn new(node_id: u32) -> Self {
        Self {
            node_id,
            physical: 0,
            logical: 0,
            max_drift: 1000,
        }
    }

    /// Create with a maximum allowed clock drift.
    pub fn with_max_drift(node_id: u32, max_drift: u64) -> Self {
        Self {
            node_id,
            physical: 0,
            logical: 0,
            max_drift,
        }
    }

    /// Generate a new timestamp for a local event.
    pub fn now(&mut self, wall_clock: u64) -> HlcTimestamp {
        if wall_clock > self.physical {
            self.physical = wall_clock;
            self.logical = 0;
        } else {
            self.logical = self.logical.saturating_add(1);
        }
        HlcTimestamp::new(self.physical, self.logical, self.node_id)
    }

    /// Update clock on receiving a remote timestamp.
    /// Returns the new local timestamp.
    pub fn receive(&mut self, wall_clock: u64, remote: &HlcTimestamp) -> Option<HlcTimestamp> {
        // Check for excessive drift
        if remote.physical > wall_clock.saturating_add(self.max_drift) {
            return None;
        }

        let max_phys = wall_clock.max(self.physical).max(remote.physical);

        if max_phys == self.physical && max_phys == remote.physical {
            self.logical = self.logical.max(remote.logical).saturating_add(1);
        } else if max_phys == self.physical {
            self.logical = self.logical.saturating_add(1);
        } else if max_phys == remote.physical {
            self.logical = remote.logical.saturating_add(1);
        } else {
            self.logical = 0;
        }
        self.physical = max_phys;

        Some(HlcTimestamp::new(self.physical, self.logical, self.node_id))
    }

    /// Get the current timestamp without advancing.
    pub fn current(&self) -> HlcTimestamp {
        HlcTimestamp::new(self.physical, self.logical, self.node_id)
    }

    /// Node ID.
    pub fn node_id(&self) -> u32 {
        self.node_id
    }

    /// Maximum allowed drift.
    pub fn max_drift(&self) -> u64 {
        self.max_drift
    }

    /// Check if a timestamp is causally before another.
    pub fn is_before(a: &HlcTimestamp, b: &HlcTimestamp) -> bool {
        a < b
    }

    /// Check if two timestamps are concurrent (neither is before the other
    /// and they are from different nodes).
    pub fn is_concurrent(a: &HlcTimestamp, b: &HlcTimestamp) -> bool {
        a.physical == b.physical && a.logical == b.logical && a.node_id != b.node_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let clock = HybridClock::new(1);
        assert_eq!(clock.node_id(), 1);
    }

    #[test]
    fn test_now_advances() {
        let mut clock = HybridClock::new(1);
        let t1 = clock.now(100);
        let t2 = clock.now(100);
        assert!(t2 > t1);
        assert_eq!(t1.physical, 100);
        assert_eq!(t2.physical, 100);
        assert_eq!(t1.logical, 0);
        assert_eq!(t2.logical, 1);
    }

    #[test]
    fn test_now_wall_clock_advance() {
        let mut clock = HybridClock::new(1);
        let t1 = clock.now(100);
        let t2 = clock.now(200);
        assert!(t2 > t1);
        assert_eq!(t2.physical, 200);
        assert_eq!(t2.logical, 0);
    }

    #[test]
    fn test_receive() {
        let mut clock = HybridClock::new(1);
        clock.now(100);
        let remote = HlcTimestamp::new(200, 5, 2);
        let result = clock.receive(150, &remote);
        assert!(result.is_some());
        let ts = result.unwrap();
        assert_eq!(ts.physical, 200);
        assert_eq!(ts.logical, 6);
    }

    #[test]
    fn test_receive_excessive_drift() {
        let mut clock = HybridClock::with_max_drift(1, 100);
        let remote = HlcTimestamp::new(10000, 0, 2);
        let result = clock.receive(50, &remote);
        assert!(result.is_none());
    }

    #[test]
    fn test_ordering() {
        let a = HlcTimestamp::new(100, 0, 1);
        let b = HlcTimestamp::new(100, 1, 1);
        let c = HlcTimestamp::new(200, 0, 1);
        assert!(a < b);
        assert!(b < c);
        assert!(a < c);
    }

    #[test]
    fn test_is_before() {
        let a = HlcTimestamp::new(100, 0, 1);
        let b = HlcTimestamp::new(200, 0, 2);
        assert!(HybridClock::is_before(&a, &b));
        assert!(!HybridClock::is_before(&b, &a));
    }

    #[test]
    fn test_is_concurrent() {
        let a = HlcTimestamp::new(100, 5, 1);
        let b = HlcTimestamp::new(100, 5, 2);
        assert!(HybridClock::is_concurrent(&a, &b));
    }

    #[test]
    fn test_current() {
        let mut clock = HybridClock::new(1);
        clock.now(100);
        let current = clock.current();
        assert_eq!(current.physical, 100);
        assert_eq!(current.node_id, 1);
    }

    #[test]
    fn test_node_id_tiebreaker() {
        let a = HlcTimestamp::new(100, 0, 1);
        let b = HlcTimestamp::new(100, 0, 2);
        assert!(a < b);
    }
}
