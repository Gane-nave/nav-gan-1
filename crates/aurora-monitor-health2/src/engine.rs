/// monitor health: check, status, dependency, recover, log
/// Phase 1586

#[derive(Debug, Clone)]
pub struct MonitorHealth2 {
    pub check_ok: bool,
    pub status_ok: bool,
    pub dependency_ok: bool,
    pub recover_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorHealth2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorHealth2 {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            status_ok: true,
            dependency_ok: true,
            recover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.status_ok && self.dependency_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recover_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.status_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = MonitorHealth2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorHealth2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorHealth2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorHealth2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorHealth2::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorHealth2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
