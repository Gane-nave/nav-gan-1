//! Counting semaphore — manages concurrent access to a fixed number of resource slots.

use crate::permit::Permit;

/// Outcome of an acquire attempt.
#[derive(Debug, PartialEq)]
pub enum AcquireResult {
    /// Successfully acquired a permit.
    Acquired { permit_id: u64 },
    /// No permits available.
    Unavailable,
    /// Semaphore is closed.
    Closed,
}

/// A counting semaphore that limits concurrent access.
pub struct CountingSemaphore {
    /// Maximum number of concurrent permits.
    max_permits: u64,
    /// Currently active (non-released, non-expired) permits.
    active_permits: Vec<Permit>,
    /// Next permit ID.
    next_id: u64,
    /// Default TTL for new permits (0 = no expiry).
    default_ttl_ms: u64,
    /// Whether the semaphore is closed.
    closed: bool,
    /// Total permits ever acquired.
    total_acquired: u64,
    /// Total permits released.
    total_released: u64,
    /// Total acquire attempts rejected.
    total_rejected: u64,
}

impl CountingSemaphore {
    /// Create a new counting semaphore.
    pub fn new(max_permits: u64, default_ttl_ms: u64) -> Self {
        Self {
            max_permits,
            active_permits: Vec::new(),
            next_id: 1,
            default_ttl_ms,
            closed: false,
            total_acquired: 0,
            total_released: 0,
            total_rejected: 0,
        }
    }

    /// Try to acquire a permit at the given timestamp.
    pub fn try_acquire(&mut self, now_ms: u64) -> AcquireResult {
        if self.closed {
            return AcquireResult::Closed;
        }

        // Clean up expired permits first
        self.cleanup_expired(now_ms);

        if self.active_permits.len() as u64 >= self.max_permits {
            self.total_rejected += 1;
            return AcquireResult::Unavailable;
        }

        let id = self.next_id;
        self.next_id += 1;
        let permit = Permit::new(id, now_ms, self.default_ttl_ms);
        self.active_permits.push(permit);
        self.total_acquired += 1;

        AcquireResult::Acquired { permit_id: id }
    }

    /// Release a permit by ID.
    pub fn release(&mut self, permit_id: u64) -> bool {
        if let Some(pos) = self.active_permits.iter().position(|p| p.id() == permit_id) {
            self.active_permits.remove(pos);
            self.total_released += 1;
            true
        } else {
            false
        }
    }

    /// Close the semaphore — no new permits can be acquired.
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Reopen a closed semaphore.
    pub fn reopen(&mut self) {
        self.closed = false;
    }

    /// Check if the semaphore is closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Number of currently active permits.
    pub fn active_count(&self) -> u64 {
        self.active_permits.len() as u64
    }

    /// Number of available permits.
    pub fn available(&self, now_ms: u64) -> u64 {
        let expired = self
            .active_permits
            .iter()
            .filter(|p| p.is_expired(now_ms))
            .count() as u64;
        let active = self.active_permits.len() as u64 - expired;
        self.max_permits.saturating_sub(active)
    }

    /// Maximum permits.
    pub fn max_permits(&self) -> u64 {
        self.max_permits
    }

    /// Total acquired (lifetime).
    pub fn total_acquired(&self) -> u64 {
        self.total_acquired
    }

    /// Total released (lifetime).
    pub fn total_released(&self) -> u64 {
        self.total_released
    }

    /// Total rejected (lifetime).
    pub fn total_rejected(&self) -> u64 {
        self.total_rejected
    }

    /// Utilization ratio (0.0 to 1.0).
    pub fn utilization(&self, now_ms: u64) -> f64 {
        if self.max_permits == 0 {
            return 1.0;
        }
        let active = self.max_permits.saturating_sub(self.available(now_ms));
        active as f64 / self.max_permits as f64
    }

    /// Remove expired permits.
    fn cleanup_expired(&mut self, now_ms: u64) {
        let before = self.active_permits.len();
        self.active_permits.retain(|p| !p.is_expired(now_ms));
        let removed = before - self.active_permits.len();
        self.total_released += removed as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acquire_and_release() {
        let mut sem = CountingSemaphore::new(3, 0);
        let r1 = sem.try_acquire(100);
        assert_eq!(r1, AcquireResult::Acquired { permit_id: 1 });
        let r2 = sem.try_acquire(200);
        assert_eq!(r2, AcquireResult::Acquired { permit_id: 2 });
        assert_eq!(sem.active_count(), 2);

        assert!(sem.release(1));
        assert_eq!(sem.active_count(), 1);
    }

    #[test]
    fn test_acquire_at_capacity() {
        let mut sem = CountingSemaphore::new(2, 0);
        sem.try_acquire(100);
        sem.try_acquire(200);
        let r3 = sem.try_acquire(300);
        assert_eq!(r3, AcquireResult::Unavailable);
        assert_eq!(sem.total_rejected(), 1);
    }

    #[test]
    fn test_acquire_after_release() {
        let mut sem = CountingSemaphore::new(1, 0);
        sem.try_acquire(100);
        assert_eq!(sem.try_acquire(200), AcquireResult::Unavailable);
        sem.release(1);
        assert_eq!(
            sem.try_acquire(300),
            AcquireResult::Acquired { permit_id: 2 }
        );
    }

    #[test]
    fn test_closed_semaphore() {
        let mut sem = CountingSemaphore::new(5, 0);
        sem.close();
        assert_eq!(sem.try_acquire(100), AcquireResult::Closed);
        assert!(sem.is_closed());
        sem.reopen();
        assert!(!sem.is_closed());
        assert_eq!(
            sem.try_acquire(200),
            AcquireResult::Acquired { permit_id: 1 }
        );
    }

    #[test]
    fn test_ttl_expiry() {
        let mut sem = CountingSemaphore::new(1, 5000);
        sem.try_acquire(1000); // expires at 6000
        assert_eq!(sem.try_acquire(3000), AcquireResult::Unavailable);
        // At t=7000, permit expired
        assert_eq!(
            sem.try_acquire(7000),
            AcquireResult::Acquired { permit_id: 2 }
        );
    }

    #[test]
    fn test_available_count() {
        let mut sem = CountingSemaphore::new(3, 5000);
        assert_eq!(sem.available(100), 3);
        sem.try_acquire(100);
        sem.try_acquire(200);
        assert_eq!(sem.available(300), 1);
        // After expiry of first permit at t=5100
        assert_eq!(sem.available(5100), 2);
    }

    #[test]
    fn test_utilization() {
        let mut sem = CountingSemaphore::new(4, 0);
        assert!((sem.utilization(100) - 0.0).abs() < f64::EPSILON);
        sem.try_acquire(100);
        sem.try_acquire(200);
        assert!((sem.utilization(300) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_release_nonexistent() {
        let mut sem = CountingSemaphore::new(5, 0);
        assert!(!sem.release(999));
    }

    #[test]
    fn test_stats() {
        let mut sem = CountingSemaphore::new(2, 0);
        sem.try_acquire(100);
        sem.try_acquire(200);
        sem.try_acquire(300); // rejected
        sem.release(1);
        assert_eq!(sem.total_acquired(), 2);
        assert_eq!(sem.total_released(), 1);
        assert_eq!(sem.total_rejected(), 1);
    }

    #[test]
    fn test_zero_max_permits() {
        let mut sem = CountingSemaphore::new(0, 0);
        assert_eq!(sem.try_acquire(100), AcquireResult::Unavailable);
        assert!((sem.utilization(100) - 1.0).abs() < f64::EPSILON);
    }
}
