/// monitor budget: define, track, alert, forecast, log
/// Phase 1588

#[derive(Debug, Clone)]
pub struct MonitorBudget {
    pub define_ok: bool,
    pub track_ok: bool,
    pub alert_ok: bool,
    pub forecast_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorBudget {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorBudget {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            track_ok: true,
            alert_ok: true,
            forecast_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.track_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.forecast_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.track_ok
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
        let c = MonitorBudget::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorBudget::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorBudget::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorBudget::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorBudget::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorBudget::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
