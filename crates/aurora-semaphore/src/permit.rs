//! Semaphore permit — represents acquired access to a shared resource.

/// A permit representing acquired access to a semaphore slot.
#[derive(Debug, Clone)]
pub struct Permit {
    /// Unique permit identifier.
    id: u64,
    /// Timestamp when the permit was acquired (ms).
    acquired_at_ms: u64,
    /// Optional TTL in milliseconds (0 = no expiry).
    ttl_ms: u64,
    /// Whether this permit has been released.
    released: bool,
}

impl Permit {
    /// Create a new permit.
    pub fn new(id: u64, acquired_at_ms: u64, ttl_ms: u64) -> Self {
        Self {
            id,
            acquired_at_ms,
            ttl_ms,
            released: false,
        }
    }

    /// Get the permit ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the acquisition timestamp.
    pub fn acquired_at_ms(&self) -> u64 {
        self.acquired_at_ms
    }

    /// Get the TTL.
    pub fn ttl_ms(&self) -> u64 {
        self.ttl_ms
    }

    /// Check if the permit has expired at the given time.
    pub fn is_expired(&self, now_ms: u64) -> bool {
        if self.ttl_ms == 0 {
            return false;
        }
        now_ms.saturating_sub(self.acquired_at_ms) >= self.ttl_ms
    }

    /// Check if the permit has been released.
    pub fn is_released(&self) -> bool {
        self.released
    }

    /// Release the permit.
    pub fn release(&mut self) {
        self.released = true;
    }

    /// Duration the permit has been held (ms).
    pub fn held_duration_ms(&self, now_ms: u64) -> u64 {
        now_ms.saturating_sub(self.acquired_at_ms)
    }

    /// Remaining TTL at the given time.
    pub fn remaining_ttl_ms(&self, now_ms: u64) -> u64 {
        if self.ttl_ms == 0 {
            return u64::MAX;
        }
        let elapsed = now_ms.saturating_sub(self.acquired_at_ms);
        self.ttl_ms.saturating_sub(elapsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permit_creation() {
        let p = Permit::new(1, 1000, 5000);
        assert_eq!(p.id(), 1);
        assert_eq!(p.acquired_at_ms(), 1000);
        assert_eq!(p.ttl_ms(), 5000);
        assert!(!p.is_released());
    }

    #[test]
    fn test_permit_expiry() {
        let p = Permit::new(1, 1000, 5000);
        assert!(!p.is_expired(3000)); // 2s elapsed < 5s TTL
        assert!(!p.is_expired(5999)); // 4.999s < 5s
        assert!(p.is_expired(6000)); // 5s = TTL
        assert!(p.is_expired(10000)); // 9s > 5s
    }

    #[test]
    fn test_permit_no_ttl_never_expires() {
        let p = Permit::new(1, 1000, 0);
        assert!(!p.is_expired(u64::MAX));
    }

    #[test]
    fn test_permit_release() {
        let mut p = Permit::new(1, 1000, 5000);
        assert!(!p.is_released());
        p.release();
        assert!(p.is_released());
    }

    #[test]
    fn test_permit_held_duration() {
        let p = Permit::new(1, 1000, 5000);
        assert_eq!(p.held_duration_ms(3000), 2000);
        assert_eq!(p.held_duration_ms(1000), 0);
    }

    #[test]
    fn test_permit_remaining_ttl() {
        let p = Permit::new(1, 1000, 5000);
        assert_eq!(p.remaining_ttl_ms(3000), 3000);
        assert_eq!(p.remaining_ttl_ms(6000), 0);
        assert_eq!(p.remaining_ttl_ms(8000), 0);
    }

    #[test]
    fn test_permit_no_ttl_remaining() {
        let p = Permit::new(1, 1000, 0);
        assert_eq!(p.remaining_ttl_ms(5000), u64::MAX);
    }

    #[test]
    fn test_permit_overflow_protection() {
        let p = Permit::new(1, u64::MAX - 10, 100);
        assert_eq!(p.held_duration_ms(u64::MAX), 10);
        assert_eq!(p.remaining_ttl_ms(u64::MAX), 90);
    }
}
