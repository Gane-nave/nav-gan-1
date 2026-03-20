/// sec scan2: scan, report, remediate, schedule, log
/// Phase 2056

#[derive(Debug, Clone)]
pub struct SecScan2 {
    pub scan_ok: bool,
    pub report_ok: bool,
    pub remediate_ok: bool,
    pub schedule_ok: bool,
    pub log_ok: bool,
}

impl Default for SecScan2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SecScan2 {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            report_ok: true,
            remediate_ok: true,
            schedule_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.report_ok && self.remediate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.schedule_ok && self.log_ok
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
        let c = SecScan2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecScan2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecScan2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecScan2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecScan2::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecScan2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
