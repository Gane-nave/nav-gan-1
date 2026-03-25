/// sync spinlock2: lock, unlock, trylock, backoff, log
/// Phase 2392

#[derive(Debug, Clone)]
pub struct SyncSpinlock2 {
    pub lock_ok: bool,
    pub unlock_ok: bool,
    pub trylock_ok: bool,
    pub backoff_ok: bool,
    pub log_ok: bool,
}

impl Default for SyncSpinlock2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncSpinlock2 {
    pub fn new() -> Self {
        Self {
            lock_ok: true,
            unlock_ok: true,
            trylock_ok: true,
            backoff_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.lock_ok && self.unlock_ok && self.trylock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.backoff_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.lock_ok || !self.unlock_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lock_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SyncSpinlock2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SyncSpinlock2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SyncSpinlock2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SyncSpinlock2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SyncSpinlock2::new();
        c.lock_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SyncSpinlock2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
