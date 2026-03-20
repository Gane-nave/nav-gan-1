/// cloud scale: monitor, evaluate, provision, balance, log
/// Phase 1451

#[derive(Debug, Clone)]
pub struct CloudScale {
    pub monitor_ok: bool,
    pub evaluate_ok: bool,
    pub provision_ok: bool,
    pub balance_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudScale {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudScale {
    pub fn new() -> Self {
        Self {
            monitor_ok: true,
            evaluate_ok: true,
            provision_ok: true,
            balance_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.monitor_ok && self.evaluate_ok && self.provision_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.balance_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.monitor_ok || !self.evaluate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.monitor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CloudScale::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudScale::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudScale::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudScale::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudScale::new();
        c.monitor_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudScale::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
