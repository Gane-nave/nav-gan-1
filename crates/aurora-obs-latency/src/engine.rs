/// obs latency: measure, percentile, alert, trend, log
/// Phase 2155

#[derive(Debug, Clone)]
pub struct ObsLatency {
    pub measure_ok: bool,
    pub percentile_ok: bool,
    pub alert_ok: bool,
    pub trend_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsLatency {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsLatency {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            percentile_ok: true,
            alert_ok: true,
            trend_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.percentile_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.trend_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.percentile_ok
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
        let c = ObsLatency::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsLatency::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsLatency::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsLatency::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsLatency::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsLatency::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
