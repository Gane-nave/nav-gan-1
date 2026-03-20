//! Lease grant — represents an acquired lease on a resource.

/// Status of a lease.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LeaseStatus {
    /// Lease is active and valid.
    Active,
    /// Lease has expired.
    Expired,
    /// Lease was explicitly revoked.
    Revoked,
    /// Lease was renewed (previous grant superseded).
    Renewed,
}

/// A lease grant on a named resource.
#[derive(Debug, Clone)]
pub struct LeaseGrant {
    /// Unique lease ID.
    id: u64,
    /// Resource name this lease covers.
    resource: String,
    /// Holder identity.
    holder: String,
    /// Timestamp when the lease was granted (ms).
    granted_at_ms: u64,
    /// Timestamp when the lease was originally granted (ms) — never changes.
    original_granted_at_ms: u64,
    /// Time-to-live in milliseconds.
    ttl_ms: u64,
    /// Number of times this lease has been renewed.
    renewals: u32,
    /// Current status.
    status: LeaseStatus,
}

impl LeaseGrant {
    /// Create a new lease grant.
    pub fn new(id: u64, resource: &str, holder: &str, granted_at_ms: u64, ttl_ms: u64) -> Self {
        Self {
            id,
            resource: resource.to_string(),
            holder: holder.to_string(),
            granted_at_ms,
            original_granted_at_ms: granted_at_ms,
            ttl_ms,
            renewals: 0,
            status: LeaseStatus::Active,
        }
    }

    /// Get the lease ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the resource name.
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// Get the holder identity.
    pub fn holder(&self) -> &str {
        &self.holder
    }

    /// Get the grant timestamp.
    pub fn granted_at_ms(&self) -> u64 {
        self.granted_at_ms
    }

    /// Get the TTL.
    pub fn ttl_ms(&self) -> u64 {
        self.ttl_ms
    }

    /// Get the number of renewals.
    pub fn renewals(&self) -> u32 {
        self.renewals
    }

    /// Get the current status.
    pub fn status(&self) -> LeaseStatus {
        self.status
    }

    /// Check if the lease is expired at the given time.
    /// A ttl_ms of 0 means "no expiry" — the lease never expires.
    pub fn is_expired(&self, now_ms: u64) -> bool {
        if self.status == LeaseStatus::Revoked {
            return true;
        }
        if self.ttl_ms == 0 {
            return false;
        }
        now_ms.saturating_sub(self.granted_at_ms) >= self.ttl_ms
    }

    /// Check if the lease is active at the given time.
    pub fn is_active(&self, now_ms: u64) -> bool {
        self.status == LeaseStatus::Active && !self.is_expired(now_ms)
    }

    /// Remaining TTL at the given time.
    /// A ttl_ms of 0 means "no expiry" — returns u64::MAX.
    pub fn remaining_ms(&self, now_ms: u64) -> u64 {
        if self.is_expired(now_ms) {
            return 0;
        }
        if self.ttl_ms == 0 {
            return u64::MAX;
        }
        let elapsed = now_ms.saturating_sub(self.granted_at_ms);
        self.ttl_ms.saturating_sub(elapsed)
    }

    /// Renew the lease with a new TTL from the given timestamp.
    pub fn renew(&mut self, now_ms: u64, new_ttl_ms: u64) -> bool {
        if self.status != LeaseStatus::Active {
            return false;
        }
        if self.is_expired(now_ms) {
            self.status = LeaseStatus::Expired;
            return false;
        }
        self.granted_at_ms = now_ms;
        self.ttl_ms = new_ttl_ms;
        self.renewals += 1;
        true
    }

    /// Revoke the lease.
    pub fn revoke(&mut self) {
        self.status = LeaseStatus::Revoked;
    }

    /// Mark as expired.
    pub fn mark_expired(&mut self) {
        self.status = LeaseStatus::Expired;
    }

    /// Total duration held from original grant to now.
    pub fn held_duration_ms(&self, now_ms: u64) -> u64 {
        now_ms.saturating_sub(self.original_granted_at_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_creation() {
        let g = LeaseGrant::new(1, "sensor.gps", "node-a", 1000, 5000);
        assert_eq!(g.id(), 1);
        assert_eq!(g.resource(), "sensor.gps");
        assert_eq!(g.holder(), "node-a");
        assert_eq!(g.ttl_ms(), 5000);
        assert_eq!(g.status(), LeaseStatus::Active);
    }

    #[test]
    fn test_grant_expiry() {
        let g = LeaseGrant::new(1, "r", "h", 1000, 5000);
        assert!(!g.is_expired(3000));
        assert!(g.is_expired(6000));
        assert!(g.is_active(3000));
        assert!(!g.is_active(6000));
    }

    #[test]
    fn test_grant_remaining() {
        let g = LeaseGrant::new(1, "r", "h", 1000, 5000);
        assert_eq!(g.remaining_ms(3000), 3000);
        assert_eq!(g.remaining_ms(6000), 0);
    }

    #[test]
    fn test_grant_renew() {
        let mut g = LeaseGrant::new(1, "r", "h", 1000, 5000);
        assert!(g.renew(4000, 5000)); // renew at t=4000 with 5s TTL
        assert_eq!(g.renewals(), 1);
        assert_eq!(g.remaining_ms(6000), 3000); // 4000+5000-6000=3000
        assert!(g.is_active(6000));
        assert!(g.is_expired(9000)); // 4000+5000=9000
    }

    #[test]
    fn test_grant_renew_after_expiry_fails() {
        let mut g = LeaseGrant::new(1, "r", "h", 1000, 5000);
        assert!(!g.renew(7000, 5000)); // expired at t=6000
        assert_eq!(g.status(), LeaseStatus::Expired);
    }

    #[test]
    fn test_grant_revoke() {
        let mut g = LeaseGrant::new(1, "r", "h", 1000, 5000);
        g.revoke();
        assert_eq!(g.status(), LeaseStatus::Revoked);
        assert!(g.is_expired(2000)); // revoked = expired
        assert!(!g.is_active(2000));
    }

    #[test]
    fn test_renew_revoked_fails() {
        let mut g = LeaseGrant::new(1, "r", "h", 1000, 5000);
        g.revoke();
        assert!(!g.renew(2000, 5000));
    }

    #[test]
    fn test_held_duration() {
        let g = LeaseGrant::new(1, "r", "h", 1000, 5000);
        assert_eq!(g.held_duration_ms(3000), 2000);
    }

    #[test]
    fn test_held_duration_after_renewal() {
        let mut g = LeaseGrant::new(1, "r", "h", 1000, 5000);
        g.renew(4000, 5000); // renew at t=4000
        // held_duration should be from original grant (1000), not renewal time (4000)
        assert_eq!(g.held_duration_ms(6000), 5000); // 6000 - 1000 = 5000
    }

    #[test]
    fn test_zero_ttl_never_expires() {
        let g = LeaseGrant::new(1, "r", "h", 1000, 0);
        // ttl_ms=0 means "no expiry"
        assert!(!g.is_expired(1000)); // at grant time
        assert!(!g.is_expired(999_999)); // far future
        assert!(g.is_active(999_999));
        // remaining_ms returns u64::MAX for zero-TTL leases
        assert_eq!(g.remaining_ms(1000), u64::MAX);
        assert_eq!(g.remaining_ms(999_999), u64::MAX);
        // But revoke still works
        let mut g2 = LeaseGrant::new(2, "r", "h", 1000, 0);
        g2.revoke();
        assert!(g2.is_expired(1000));
        assert!(!g2.is_active(1000));
        assert_eq!(g2.remaining_ms(1000), 0); // revoked = 0 remaining
    }
}
