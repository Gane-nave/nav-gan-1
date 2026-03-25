/// obs capacity: measure, forecast, alert, plan, log
/// Phase 2158

#[derive(Debug, Clone)]
pub struct ObsCapacity {
    pub measure_ok: bool,
    pub forecast_ok: bool,
    pub alert_ok: bool,
    pub plan_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsCapacity {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsCapacity {
    pub fn new() -> Self {
        Self {
            measure_ok: true,
            forecast_ok: true,
            alert_ok: true,
            plan_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.measure_ok && self.forecast_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.plan_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.measure_ok || !self.forecast_ok
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
        let c = ObsCapacity::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsCapacity::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsCapacity::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsCapacity::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsCapacity::new();
        c.measure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsCapacity::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
