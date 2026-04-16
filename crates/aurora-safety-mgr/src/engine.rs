/// safety mgr: assess, mitigate, monitor, report, log
/// Phase 1482

#[derive(Debug, Clone)]
pub struct SafetyMgr {
    pub assess_ok: bool,
    pub mitigate_ok: bool,
    pub monitor_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for SafetyMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyMgr {
    pub fn new() -> Self {
        Self {
            assess_ok: true,
            mitigate_ok: true,
            monitor_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assess_ok && self.mitigate_ok && self.monitor_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assess_ok || !self.mitigate_ok
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
        let c = SafetyMgr::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SafetyMgr::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SafetyMgr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SafetyMgr::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SafetyMgr::new();
        c.assess_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SafetyMgr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
