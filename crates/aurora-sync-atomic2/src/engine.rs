/// sync atomic2: load, store, cas, fetch, log
/// Phase 2391

#[derive(Debug, Clone)]
pub struct SyncAtomic2 {
    pub load_ok: bool,
    pub store_ok: bool,
    pub cas_ok: bool,
    pub fetch_ok: bool,
    pub log_ok: bool,
}

impl Default for SyncAtomic2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncAtomic2 {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            store_ok: true,
            cas_ok: true,
            fetch_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.store_ok && self.cas_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.fetch_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.store_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
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
        let c = SyncAtomic2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SyncAtomic2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SyncAtomic2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SyncAtomic2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SyncAtomic2::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SyncAtomic2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
