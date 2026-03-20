/// Quality gate: inspect, measure, pass/fail, report, trend
/// Phase 969

#[derive(Debug, Clone)]
pub struct QualityGate {
    pub inspect_ok: bool,
    pub measure_ok: bool,
    pub pass_fail_ok: bool,
    pub report_ok: bool,
    pub trend_ok: bool,
}

impl Default for QualityGate {
    fn default() -> Self {
        Self::new()
    }
}

impl QualityGate {
    pub fn new() -> Self {
        Self {
            inspect_ok: true,
            measure_ok: true,
            pass_fail_ok: true,
            report_ok: true,
            trend_ok: true,
        }
    }

    pub fn inspection_ok(&self) -> bool {
        self.inspect_ok && self.measure_ok && self.pass_fail_ok
    }

    pub fn analytics_ok(&self) -> bool {
        self.report_ok && self.trend_ok
    }

    pub fn all_ok(&self) -> bool {
        self.inspection_ok() && self.analytics_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.measure_ok || !self.inspect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inspect_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspection() {
        let c = QualityGate::new();
        assert!(c.inspection_ok());
    }

    #[test]
    fn test_analytics() {
        let c = QualityGate::new();
        assert!(c.analytics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = QualityGate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = QualityGate::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_measure() {
        let mut c = QualityGate::new();
        c.measure_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = QualityGate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
