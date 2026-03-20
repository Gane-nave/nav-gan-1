/// readiness mon: check, track, report, reset, log
/// Phase 1379

#[derive(Debug, Clone)]
pub struct ReadinessMon {
    pub check_ok: bool,
    pub track_ok: bool,
    pub report_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for ReadinessMon {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadinessMon {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            track_ok: true,
            report_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.track_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.track_ok
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
        let c = ReadinessMon::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReadinessMon::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReadinessMon::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReadinessMon::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReadinessMon::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReadinessMon::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
