/// wf metric: collect, aggregate, alert, report, log
/// Phase 2236

#[derive(Debug, Clone)]
pub struct WfMetric {
    pub collect_ok: bool,
    pub aggregate_ok: bool,
    pub alert_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for WfMetric {
    fn default() -> Self {
        Self::new()
    }
}

impl WfMetric {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            aggregate_ok: true,
            alert_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.aggregate_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.aggregate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
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
        let c = WfMetric::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfMetric::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfMetric::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfMetric::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfMetric::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfMetric::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
