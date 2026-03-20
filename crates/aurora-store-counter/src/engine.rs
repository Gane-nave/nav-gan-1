/// store counter: incr, decr, get, reset, log
/// Phase 1984

#[derive(Debug, Clone)]
pub struct StoreCounter {
    pub incr_ok: bool,
    pub decr_ok: bool,
    pub get_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreCounter {
    pub fn new() -> Self {
        Self {
            incr_ok: true,
            decr_ok: true,
            get_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.incr_ok && self.decr_ok && self.get_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.incr_ok || !self.decr_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.incr_ok {
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
        let c = StoreCounter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreCounter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreCounter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreCounter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreCounter::new();
        c.incr_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreCounter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
