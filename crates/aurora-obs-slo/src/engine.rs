/// obs slo: define, measure, alert, report, log
/// Phase 2160

#[derive(Debug, Clone)]
pub struct ObsSlo {
    pub define_ok: bool,
    pub measure_ok: bool,
    pub alert_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsSlo {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsSlo {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            measure_ok: true,
            alert_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.measure_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = ObsSlo::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsSlo::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsSlo::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsSlo::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsSlo::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsSlo::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
