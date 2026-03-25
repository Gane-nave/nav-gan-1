/// mem limit: set, check, enforce, report, log
/// Phase 2380

#[derive(Debug, Clone)]
pub struct MemLimit {
    pub set_ok: bool,
    pub check_ok: bool,
    pub enforce_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for MemLimit {
    fn default() -> Self {
        Self::new()
    }
}

impl MemLimit {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            check_ok: true,
            enforce_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.check_ok && self.enforce_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
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
        let c = MemLimit::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemLimit::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemLimit::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemLimit::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemLimit::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemLimit::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
