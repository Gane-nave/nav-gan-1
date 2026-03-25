/// feature sdk: init, evaluate, track, close, log
/// Phase 1787

#[derive(Debug, Clone)]
pub struct FeatureSdk {
    pub init_ok: bool,
    pub evaluate_ok: bool,
    pub track_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for FeatureSdk {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureSdk {
    pub fn new() -> Self {
        Self {
            init_ok: true,
            evaluate_ok: true,
            track_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.init_ok && self.evaluate_ok && self.track_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.init_ok || !self.evaluate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.init_ok {
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
        let c = FeatureSdk::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FeatureSdk::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FeatureSdk::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FeatureSdk::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FeatureSdk::new();
        c.init_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FeatureSdk::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
