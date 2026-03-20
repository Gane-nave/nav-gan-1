//! Lease manager — manages leases on multiple resources.

use crate::grant::{LeaseGrant, LeaseStatus};

/// Result of a lease acquisition attempt.
#[derive(Debug, PartialEq)]
pub enum LeaseResult {
    /// Lease granted successfully.
    Granted { lease_id: u64 },
    /// Resource is already leased by another holder.
    Conflict { current_holder: String },
    /// Resource lease expired and was replaced.
    Replaced { lease_id: u64 },
}

/// Manages leases across multiple resources.
pub struct LeaseManager {
    /// Active leases indexed by resource name.
    leases: Vec<LeaseGrant>,
    /// Next lease ID.
    next_id: u64,
    /// Maximum renewals per lease (0 = unlimited).
    max_renewals: u32,
    /// Total leases granted (lifetime).
    total_granted: u64,
    /// Total leases expired (lifetime).
    total_expired: u64,
    /// Total leases revoked (lifetime).
    total_revoked: u64,
    /// Total conflicts (lifetime).
    total_conflicts: u64,
}

impl LeaseManager {
    /// Create a new lease manager.
    pub fn new(max_renewals: u32) -> Self {
        Self {
            leases: Vec::new(),
            next_id: 1,
            max_renewals,
            total_granted: 0,
            total_expired: 0,
            total_revoked: 0,
            total_conflicts: 0,
        }
    }

    /// Try to acquire a lease on a resource.
    pub fn acquire(
        &mut self,
        resource: &str,
        holder: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> LeaseResult {
        // Check for existing lease on this resource
        if let Some(existing) = self.leases.iter_mut().find(|l| l.resource() == resource) {
            if existing.is_active(now_ms) {
                if existing.holder() == holder {
                    // Same holder — renew
                    if existing.renew(now_ms, ttl_ms) {
                        return LeaseResult::Granted {
                            lease_id: existing.id(),
                        };
                    }
                }
                self.total_conflicts += 1;
                return LeaseResult::Conflict {
                    current_holder: existing.holder().to_string(),
                };
            }
            // Expired — replace
            existing.mark_expired();
            self.total_expired += 1;
        }

        // Remove any expired lease for this resource
        self.leases.retain(|l| {
            !(l.resource() == resource
                && (l.status() == LeaseStatus::Expired || l.status() == LeaseStatus::Revoked))
        });

        let id = self.next_id;
        self.next_id += 1;
        self.leases
            .push(LeaseGrant::new(id, resource, holder, now_ms, ttl_ms));
        self.total_granted += 1;

        LeaseResult::Granted { lease_id: id }
    }

    /// Renew an existing lease by ID.
    pub fn renew(&mut self, lease_id: u64, now_ms: u64, new_ttl_ms: u64) -> bool {
        if let Some(lease) = self.leases.iter_mut().find(|l| l.id() == lease_id) {
            if self.max_renewals > 0 && lease.renewals() >= self.max_renewals {
                return false;
            }
            return lease.renew(now_ms, new_ttl_ms);
        }
        false
    }

    /// Revoke a lease by ID.
    pub fn revoke(&mut self, lease_id: u64) -> bool {
        if let Some(lease) = self.leases.iter_mut().find(|l| l.id() == lease_id) {
            lease.revoke();
            self.total_revoked += 1;
            true
        } else {
            false
        }
    }

    /// Get a lease by resource name.
    pub fn get_lease(&self, resource: &str) -> Option<&LeaseGrant> {
        self.leases.iter().find(|l| l.resource() == resource)
    }

    /// Check if a resource is currently leased.
    pub fn is_leased(&self, resource: &str, now_ms: u64) -> bool {
        self.leases
            .iter()
            .any(|l| l.resource() == resource && l.is_active(now_ms))
    }

    /// Get the holder of a resource lease.
    pub fn holder_of(&self, resource: &str, now_ms: u64) -> Option<&str> {
        self.leases
            .iter()
            .find(|l| l.resource() == resource && l.is_active(now_ms))
            .map(|l| l.holder())
    }

    /// Number of active leases.
    pub fn active_count(&self, now_ms: u64) -> usize {
        self.leases.iter().filter(|l| l.is_active(now_ms)).count()
    }

    /// Total number of leases (including expired/revoked).
    pub fn total_leases(&self) -> usize {
        self.leases.len()
    }

    /// Clean up expired and revoked leases.
    pub fn cleanup(&mut self, now_ms: u64) {
        let before = self.leases.len();
        self.leases.retain(|l| l.is_active(now_ms));
        let removed = before - self.leases.len();
        self.total_expired += removed as u64;
    }

    /// Total granted (lifetime).
    pub fn total_granted(&self) -> u64 {
        self.total_granted
    }

    /// Total expired (lifetime).
    pub fn total_expired(&self) -> u64 {
        self.total_expired
    }

    /// Total revoked (lifetime).
    pub fn total_revoked(&self) -> u64 {
        self.total_revoked
    }

    /// Total conflicts (lifetime).
    pub fn total_conflicts(&self) -> u64 {
        self.total_conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acquire_lease() {
        let mut mgr = LeaseManager::new(0);
        let result = mgr.acquire("sensor.gps", "node-a", 1000, 5000);
        assert_eq!(result, LeaseResult::Granted { lease_id: 1 });
        assert!(mgr.is_leased("sensor.gps", 2000));
        assert_eq!(mgr.holder_of("sensor.gps", 2000), Some("node-a"));
    }

    #[test]
    fn test_lease_conflict() {
        let mut mgr = LeaseManager::new(0);
        mgr.acquire("sensor.gps", "node-a", 1000, 5000);
        let result = mgr.acquire("sensor.gps", "node-b", 2000, 5000);
        assert_eq!(
            result,
            LeaseResult::Conflict {
                current_holder: "node-a".to_string()
            }
        );
        assert_eq!(mgr.total_conflicts(), 1);
    }

    #[test]
    fn test_lease_after_expiry() {
        let mut mgr = LeaseManager::new(0);
        mgr.acquire("sensor.gps", "node-a", 1000, 5000);
        // At t=7000, lease expired
        let result = mgr.acquire("sensor.gps", "node-b", 7000, 5000);
        assert_eq!(result, LeaseResult::Granted { lease_id: 2 });
        assert_eq!(mgr.holder_of("sensor.gps", 8000), Some("node-b"));
    }

    #[test]
    fn test_renew_by_same_holder() {
        let mut mgr = LeaseManager::new(0);
        mgr.acquire("sensor.gps", "node-a", 1000, 5000);
        let result = mgr.acquire("sensor.gps", "node-a", 4000, 5000);
        assert_eq!(result, LeaseResult::Granted { lease_id: 1 });
        assert!(mgr.is_leased("sensor.gps", 8000)); // renewed to 4000+5000=9000
    }

    #[test]
    fn test_renew_by_id() {
        let mut mgr = LeaseManager::new(0);
        mgr.acquire("sensor.gps", "node-a", 1000, 5000);
        assert!(mgr.renew(1, 4000, 3000));
        let lease = mgr.get_lease("sensor.gps").unwrap();
        assert_eq!(lease.renewals(), 1); // one renewal via mgr.renew()
    }

    #[test]
    fn test_max_renewals() {
        let mut mgr = LeaseManager::new(2);
        mgr.acquire("r", "h", 1000, 5000);
        assert!(mgr.renew(1, 2000, 5000));
        assert!(mgr.renew(1, 3000, 5000));
        assert!(!mgr.renew(1, 4000, 5000)); // exceeded max_renewals=2
    }

    #[test]
    fn test_revoke() {
        let mut mgr = LeaseManager::new(0);
        mgr.acquire("r", "h", 1000, 5000);
        assert!(mgr.revoke(1));
        assert!(!mgr.is_leased("r", 2000));
        assert_eq!(mgr.total_revoked(), 1);
    }

    #[test]
    fn test_revoke_nonexistent() {
        let mut mgr = LeaseManager::new(0);
        assert!(!mgr.revoke(999));
    }

    #[test]
    fn test_active_count() {
        let mut mgr = LeaseManager::new(0);
        mgr.acquire("r1", "h", 1000, 5000);
        mgr.acquire("r2", "h", 1000, 3000);
        mgr.acquire("r3", "h", 1000, 10000);
        assert_eq!(mgr.active_count(2000), 3);
        assert_eq!(mgr.active_count(5000), 2); // r2 expired at 4000
        assert_eq!(mgr.active_count(7000), 1); // r1 expired at 6000
    }

    #[test]
    fn test_cleanup() {
        let mut mgr = LeaseManager::new(0);
        mgr.acquire("r1", "h", 1000, 2000);
        mgr.acquire("r2", "h", 1000, 10000);
        assert_eq!(mgr.total_leases(), 2);
        mgr.cleanup(5000);
        assert_eq!(mgr.total_leases(), 1);
    }

    #[test]
    fn test_multiple_resources() {
        let mut mgr = LeaseManager::new(0);
        mgr.acquire("gps", "a", 1000, 5000);
        mgr.acquire("imu", "b", 1000, 5000);
        assert_eq!(mgr.holder_of("gps", 2000), Some("a"));
        assert_eq!(mgr.holder_of("imu", 2000), Some("b"));
        assert_eq!(mgr.holder_of("lidar", 2000), None);
    }
}
