/// aurora-etcd-lock: etcd lock
/// Phase 2601

#[derive(Debug, Clone)]
pub struct EtcdLock {
    pub acquire_ok: bool,
    pub release_ok: bool,
    pub renew_ok: bool,
    pub trylock_ok: bool,
    pub watch_ok: bool,
}

impl Default for EtcdLock {
    fn default() -> Self {
        Self::new()
    }
}

impl EtcdLock {
    pub fn new() -> Self {
        Self {
            acquire_ok: true,
            release_ok: true,
            renew_ok: true,
            trylock_ok: true,
            watch_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.acquire_ok && self.release_ok && self.renew_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.trylock_ok && self.watch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.acquire_ok || !self.release_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.acquire_ok {
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
        let c = EtcdLock::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EtcdLock::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EtcdLock::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EtcdLock::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EtcdLock::new();
        c.acquire_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EtcdLock::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EtcdLock::default();
        assert!(c.all_ok());
    }
}
