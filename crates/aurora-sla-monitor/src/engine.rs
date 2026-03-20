/// SLA monitor: define, measure, alert, report, escalate
/// Phase 1081

#[derive(Debug, Clone)]
pub struct SlaMonitor {
    pub define_ok: bool,
    pub measure_ok: bool,
    pub alert_ok: bool,
    pub report_ok: bool,
    pub escalate_ok: bool,
}

impl Default for SlaMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SlaMonitor {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            measure_ok: true,
            alert_ok: true,
            report_ok: true,
            escalate_ok: true,
        }
    }

    pub fn tracking_ok(&self) -> bool {
        self.define_ok && self.measure_ok && self.alert_ok
    }

    pub fn reporting_ok(&self) -> bool {
        self.report_ok && self.escalate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tracking_ok() && self.reporting_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.define_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracking() {
        let c = SlaMonitor::new();
        assert!(c.tracking_ok());
    }

    #[test]
    fn test_reporting() {
        let c = SlaMonitor::new();
        assert!(c.reporting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SlaMonitor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = SlaMonitor::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_define() {
        let mut c = SlaMonitor::new();
        c.define_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = SlaMonitor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
