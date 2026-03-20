/// fleet cost: track, categorize, allocate, forecast, log
/// Phase 1420

#[derive(Debug, Clone)]
pub struct FleetCost {
    pub track_ok: bool,
    pub categorize_ok: bool,
    pub allocate_ok: bool,
    pub forecast_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetCost {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetCost {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            categorize_ok: true,
            allocate_ok: true,
            forecast_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.track_ok && self.categorize_ok && self.allocate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.forecast_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.track_ok || !self.categorize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.track_ok {
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
        let c = FleetCost::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetCost::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetCost::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetCost::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetCost::new();
        c.track_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetCost::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
