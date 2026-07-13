//! Distributed locking — lease-based locks with expiry.

use std::collections::HashMap;

/// Lock holder identifier.
pub type HolderId = String;

/// Lock state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockState {
    /// Lock is free.
    Free,
    /// Lock is held by a specific holder.
    Held {
        holder: HolderId,
        acquired_ms: u64,
        expires_ms: u64,
    },
}

/// A distributed lock with lease-based expiry.
#[derive(Debug, Clone)]
pub struct DistributedLock {
    /// Lock name/resource.
    name: String,
    /// Current state.
    state: LockState,
    /// Fencing token (monotonically increasing).
    fencing_token: u64,
}

impl DistributedLock {
    /// Create a new free lock.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: LockState::Free,
            fencing_token: 0,
        }
    }

    /// Try to acquire the lock. Returns the fencing token on success.
    pub fn try_acquire(&mut self, holder: &str, now_ms: u64, lease_ms: u64) -> Option<u64> {
        self.expire_if_needed(now_ms);

        match &self.state {
            LockState::Free => {
                self.fencing_token += 1;
                self.state = LockState::Held {
                    holder: holder.to_string(),
                    acquired_ms: now_ms,
                    expires_ms: now_ms + lease_ms,
                };
                Some(self.fencing_token)
            }
            LockState::Held { holder: h, .. } if h == holder => {
                // Re-entrant: same holder can re-acquire (extend lease)
                self.fencing_token += 1;
                self.state = LockState::Held {
                    holder: holder.to_string(),
                    acquired_ms: now_ms,
                    expires_ms: now_ms + lease_ms,
                };
                Some(self.fencing_token)
            }
            _ => None,
        }
    }

    /// Release the lock. Only the holder can release.
    pub fn release(&mut self, holder: &str) -> bool {
        if let LockState::Held { holder: h, .. } = &self.state {
            if h == holder {
                self.state = LockState::Free;
                return true;
            }
        }
        false
    }

    /// Extend the lease (renew). Returns new expiry on success.
    pub fn extend(&mut self, holder: &str, now_ms: u64, additional_ms: u64) -> Option<u64> {
        if let LockState::Held {
            holder: h,
            expires_ms,
            ..
        } = &mut self.state
        {
            if h == holder && *expires_ms > now_ms {
                *expires_ms = now_ms + additional_ms;
                return Some(*expires_ms);
            }
        }
        None
    }

    /// Check if the lock is currently held.
    pub fn is_held(&self, now_ms: u64) -> bool {
        match &self.state {
            LockState::Free => false,
            LockState::Held { expires_ms, .. } => now_ms < *expires_ms,
        }
    }

    /// Get the current holder.
    pub fn holder(&self) -> Option<&str> {
        if let LockState::Held { holder, .. } = &self.state {
            Some(holder)
        } else {
            None
        }
    }

    /// Current fencing token.
    pub fn fencing_token(&self) -> u64 {
        self.fencing_token
    }

    /// Lock name.
    pub fn name(&self) -> &str {
        &self.name
    }

    fn expire_if_needed(&mut self, now_ms: u64) {
        if let LockState::Held { expires_ms, .. } = &self.state {
            if now_ms >= *expires_ms {
                self.state = LockState::Free;
            }
        }
    }
}

/// Manager for multiple distributed locks.
pub struct LockManager {
    locks: HashMap<String, DistributedLock>,
    /// Total acquisitions.
    acquisitions: u64,
    /// Total releases.
    releases: u64,
    /// Total contention events (failed acquisitions).
    contentions: u64,
}

impl LockManager {
    /// Create a new lock manager.
    pub fn new() -> Self {
        Self {
            locks: HashMap::new(),
            acquisitions: 0,
            releases: 0,
            contentions: 0,
        }
    }

    /// Try to acquire a named lock. Returns fencing token on success.
    pub fn acquire(
        &mut self,
        lock_name: &str,
        holder: &str,
        now_ms: u64,
        lease_ms: u64,
    ) -> Option<u64> {
        let lock = self
            .locks
            .entry(lock_name.to_string())
            .or_insert_with(|| DistributedLock::new(lock_name));

        match lock.try_acquire(holder, now_ms, lease_ms) {
            Some(token) => {
                self.acquisitions += 1;
                Some(token)
            }
            None => {
                self.contentions += 1;
                None
            }
        }
    }

    /// Release a named lock.
    pub fn release(&mut self, lock_name: &str, holder: &str) -> bool {
        if let Some(lock) = self.locks.get_mut(lock_name) {
            if lock.release(holder) {
                self.releases += 1;
                return true;
            }
        }
        false
    }

    /// Extend a lock's lease.
    pub fn extend(
        &mut self,
        lock_name: &str,
        holder: &str,
        now_ms: u64,
        additional_ms: u64,
    ) -> Option<u64> {
        self.locks
            .get_mut(lock_name)
            .and_then(|l| l.extend(holder, now_ms, additional_ms))
    }

    /// Check if a lock is held.
    pub fn is_held(&self, lock_name: &str, now_ms: u64) -> bool {
        self.locks.get(lock_name).is_some_and(|l| l.is_held(now_ms))
    }

    /// Number of tracked locks.
    pub fn lock_count(&self) -> usize {
        self.locks.len()
    }

    /// Total acquisitions.
    pub fn total_acquisitions(&self) -> u64 {
        self.acquisitions
    }

    /// Total contentions.
    pub fn total_contentions(&self) -> u64 {
        self.contentions
    }

    /// Expire all stale locks.
    pub fn expire_stale(&mut self, now_ms: u64) -> usize {
        let mut expired = 0;
        for lock in self.locks.values_mut() {
            if lock.is_held(0) && !lock.is_held(now_ms) {
                lock.expire_if_needed(now_ms);
                expired += 1;
            }
        }
        expired
    }
}

impl Default for LockManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acquire_and_release() {
        let mut lock = DistributedLock::new("resource-1");
        let token = lock.try_acquire("holder-a", 1000, 5000).unwrap();
        assert_eq!(token, 1);
        assert!(lock.is_held(1000));
        assert_eq!(lock.holder(), Some("holder-a"));

        assert!(lock.release("holder-a"));
        assert!(!lock.is_held(1000));
    }

    #[test]
    fn test_contention() {
        let mut lock = DistributedLock::new("resource-1");
        lock.try_acquire("holder-a", 1000, 5000).unwrap();
        assert!(lock.try_acquire("holder-b", 2000, 5000).is_none());
    }

    #[test]
    fn test_lease_expiry() {
        let mut lock = DistributedLock::new("resource-1");
        lock.try_acquire("holder-a", 1000, 5000).unwrap();
        assert!(lock.is_held(5999));
        assert!(!lock.is_held(6000));

        // Another holder can acquire after expiry
        let token = lock.try_acquire("holder-b", 6000, 5000).unwrap();
        assert_eq!(token, 2);
    }

    #[test]
    fn test_reentrant() {
        let mut lock = DistributedLock::new("resource-1");
        let t1 = lock.try_acquire("holder-a", 1000, 5000).unwrap();
        let t2 = lock.try_acquire("holder-a", 2000, 5000).unwrap();
        assert!(t2 > t1); // New fencing token
    }

    #[test]
    fn test_extend_lease() {
        let mut lock = DistributedLock::new("resource-1");
        lock.try_acquire("holder-a", 1000, 5000).unwrap();
        let new_expiry = lock.extend("holder-a", 3000, 10000).unwrap();
        assert_eq!(new_expiry, 13000);
        assert!(lock.is_held(12000));
    }

    #[test]
    fn test_wrong_holder_cannot_release() {
        let mut lock = DistributedLock::new("resource-1");
        lock.try_acquire("holder-a", 1000, 5000).unwrap();
        assert!(!lock.release("holder-b"));
    }

    #[test]
    fn test_fencing_token_monotonic() {
        let mut lock = DistributedLock::new("resource-1");
        let t1 = lock.try_acquire("a", 1000, 1000).unwrap();
        lock.release("a");
        let t2 = lock.try_acquire("b", 2000, 1000).unwrap();
        assert!(t2 > t1);
    }

    #[test]
    fn test_lock_manager() {
        let mut mgr = LockManager::new();
        let token = mgr.acquire("res-1", "holder-a", 1000, 5000).unwrap();
        assert!(token > 0);
        assert!(mgr.is_held("res-1", 1000));
        assert!(mgr.release("res-1", "holder-a"));
        assert!(!mgr.is_held("res-1", 1000));
    }

    #[test]
    fn test_lock_manager_contention() {
        let mut mgr = LockManager::new();
        mgr.acquire("res-1", "holder-a", 1000, 5000).unwrap();
        assert!(mgr.acquire("res-1", "holder-b", 2000, 5000).is_none());
        assert_eq!(mgr.total_contentions(), 1);
    }
}
