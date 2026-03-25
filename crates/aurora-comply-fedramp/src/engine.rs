/// comply fedramp: assess, implement, authorize, monitor, log
/// Phase 1494

#[derive(Debug, Clone)]
pub struct ComplyFedramp {
    pub assess_ok: bool,
    pub implement_ok: bool,
    pub authorize_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for ComplyFedramp {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplyFedramp {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            implement_ok: true,
            authorize_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assess_ok && self.implement_ok && self.authorize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assess_ok || !self.implement_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assess_ok {
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
        let c = ComplyFedramp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ComplyFedramp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ComplyFedramp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ComplyFedramp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ComplyFedramp::new();
        c.assess_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ComplyFedramp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
