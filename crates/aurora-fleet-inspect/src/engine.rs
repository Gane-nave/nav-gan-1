/// fleet inspect: schedule, examine, grade, report, log
/// Phase 1425

#[derive(Debug, Clone)]
pub struct FleetInspect {
    pub schedule_ok: bool,
    pub examine_ok: bool,
    pub grade_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetInspect {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetInspect {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            examine_ok: true,
            grade_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schedule_ok && self.examine_ok && self.grade_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schedule_ok || !self.examine_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schedule_ok {
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
        let c = FleetInspect::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetInspect::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetInspect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetInspect::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetInspect::new();
        c.schedule_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetInspect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
