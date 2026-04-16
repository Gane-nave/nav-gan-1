/// fleet driver: assign, score, train, certify, log
/// Phase 1414

#[derive(Debug, Clone)]
pub struct FleetDriver {
    pub assign_ok: bool,
    pub score_ok: bool,
    pub train_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetDriver {
    pub fn new() -> Self {
        Self {
            assign_ok: true,
            score_ok: true,
            train_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assign_ok && self.score_ok && self.train_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.certify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assign_ok || !self.score_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assign_ok {
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
        let c = FleetDriver::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetDriver::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetDriver::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetDriver::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetDriver::new();
        c.assign_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetDriver::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
