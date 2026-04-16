/// sched adaptive: measure, adjust, optimize, report, log
/// Phase 2362

#[derive(Debug, Clone)]
pub struct SchedAdaptive {
    pub measure_ok: bool,
    pub adjust_ok: bool,
    pub optimize_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedAdaptive {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedAdaptive {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            adjust_ok: true,
            optimize_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.adjust_ok && self.optimize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.adjust_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.measure_ok {
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
        let c = SchedAdaptive::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedAdaptive::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedAdaptive::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedAdaptive::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedAdaptive::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedAdaptive::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
