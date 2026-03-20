//! Cache entry with value, timestamps, and TTL support.

/// A cache entry holding a value with metadata.
#[derive(Debug, Clone)]
pub struct CacheEntry<V: Clone> {
    key: String,
    value: V,
    created_ms: u64,
    last_access_ms: u64,
    access_count: u64,
    ttl_ms: Option<u64>,
}

impl<V: Clone> CacheEntry<V> {
    /// Create a new cache entry.
    pub fn new(key: String, value: V, now_ms: u64, ttl_ms: Option<u64>) -> Self {
        Self {
            key,
            value,
            created_ms: now_ms,
            last_access_ms: now_ms,
            access_count: 0,
            ttl_ms,
        }
    }

    /// Get the key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the value.
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Update the value.
    pub fn set_value(&mut self, value: V) {
        self.value = value;
    }

    /// Record an access.
    pub fn touch(&mut self, now_ms: u64) {
        self.last_access_ms = now_ms;
        self.access_count = self.access_count.saturating_add(1);
    }

    /// Check if the entry has expired.
    pub fn is_expired(&self, now_ms: u64) -> bool {
        match self.ttl_ms {
            Some(ttl) => now_ms.saturating_sub(self.created_ms) >= ttl,
            None => false,
        }
    }

    /// Time since creation in milliseconds.
    pub fn age_ms(&self, now_ms: u64) -> u64 {
        now_ms.saturating_sub(self.created_ms)
    }

    /// Time since last access in milliseconds.
    pub fn idle_ms(&self, now_ms: u64) -> u64 {
        now_ms.saturating_sub(self.last_access_ms)
    }

    /// Total access count.
    pub fn access_count(&self) -> u64 {
        self.access_count
    }

    /// Creation timestamp.
    pub fn created_ms(&self) -> u64 {
        self.created_ms
    }

    /// Last access timestamp.
    pub fn last_access_ms(&self) -> u64 {
        self.last_access_ms
    }

    /// TTL setting.
    pub fn ttl_ms(&self) -> Option<u64> {
        self.ttl_ms
    }

    /// Remaining TTL (None if no TTL set).
    pub fn remaining_ttl_ms(&self, now_ms: u64) -> Option<u64> {
        self.ttl_ms.map(|ttl| {
            let elapsed = now_ms.saturating_sub(self.created_ms);
            ttl.saturating_sub(elapsed)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entry() {
        let e = CacheEntry::new("k1".into(), 42u32, 1000, Some(5000));
        assert_eq!(e.key(), "k1");
        assert_eq!(*e.value(), 42);
        assert_eq!(e.created_ms(), 1000);
        assert_eq!(e.access_count(), 0);
    }

    #[test]
    fn test_touch_updates_access() {
        let mut e = CacheEntry::new("k".into(), 1u32, 1000, None);
        e.touch(2000);
        assert_eq!(e.access_count(), 1);
        assert_eq!(e.last_access_ms(), 2000);
        e.touch(3000);
        assert_eq!(e.access_count(), 2);
    }

    #[test]
    fn test_expiry_with_ttl() {
        let e = CacheEntry::new("k".into(), 1u32, 1000, Some(3000));
        assert!(!e.is_expired(2000)); // 1000ms elapsed < 3000ms TTL
        assert!(!e.is_expired(3999)); // 2999ms elapsed < 3000ms TTL
        assert!(e.is_expired(4000)); // 3000ms elapsed = 3000ms TTL
        assert!(e.is_expired(5000)); // past TTL
    }

    #[test]
    fn test_no_ttl_never_expires() {
        let e = CacheEntry::new("k".into(), 1u32, 1000, None);
        assert!(!e.is_expired(u64::MAX));
    }

    #[test]
    fn test_age_and_idle() {
        let mut e = CacheEntry::new("k".into(), 1u32, 1000, None);
        e.touch(3000);
        assert_eq!(e.age_ms(5000), 4000);
        assert_eq!(e.idle_ms(5000), 2000);
    }

    #[test]
    fn test_remaining_ttl() {
        let e = CacheEntry::new("k".into(), 1u32, 1000, Some(5000));
        assert_eq!(e.remaining_ttl_ms(2000), Some(4000));
        assert_eq!(e.remaining_ttl_ms(6000), Some(0));
    }

    #[test]
    fn test_set_value() {
        let mut e = CacheEntry::new("k".into(), 10u32, 1000, None);
        e.set_value(20);
        assert_eq!(*e.value(), 20);
    }

    #[test]
    fn test_saturating_access_count() {
        let mut e = CacheEntry::new("k".into(), 1u32, 0, None);
        // Manually set to near max
        e.access_count = u64::MAX - 1;
        e.touch(1);
        assert_eq!(e.access_count(), u64::MAX);
        e.touch(2); // should saturate
        assert_eq!(e.access_count(), u64::MAX);
    }
}
