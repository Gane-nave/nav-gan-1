/// devops lint2: scan, report, fix, enforce, log
/// Phase 2169

#[derive(Debug, Clone)]
pub struct DevopsLint2 {
    pub scan_ok: bool,
    pub report_ok: bool,
    pub fix_ok: bool,
    pub enforce_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsLint2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsLint2 {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            report_ok: true,
            fix_ok: true,
            enforce_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.report_ok && self.fix_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.enforce_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.report_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok {
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
        let c = DevopsLint2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsLint2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsLint2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsLint2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsLint2::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsLint2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
