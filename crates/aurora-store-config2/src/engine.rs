/// store config2: load, save, watch, validate, log
/// Phase 1980

#[derive(Debug, Clone)]
pub struct StoreConfig2 {
    pub load_ok: bool,
    pub save_ok: bool,
    pub watch_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreConfig2 {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreConfig2 {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            save_ok: true,
            watch_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.save_ok && self.watch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.save_ok
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
        let c = StoreConfig2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreConfig2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreConfig2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreConfig2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreConfig2::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreConfig2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
