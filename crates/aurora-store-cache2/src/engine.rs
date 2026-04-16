/// store cache2: get, set, expire, flush, log
/// Phase 1976

#[derive(Debug, Clone)]
pub struct StoreCache2 {
    pub get_ok: bool,
    pub set_ok: bool,
    pub expire_ok: bool,
    pub flush_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreCache2 {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreCache2 {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            set_ok: true,
            expire_ok: true,
            flush_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.get_ok && self.set_ok && self.expire_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flush_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.get_ok || !self.set_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.get_ok {
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
        let c = StoreCache2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreCache2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreCache2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreCache2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreCache2::new();
        c.get_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreCache2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
