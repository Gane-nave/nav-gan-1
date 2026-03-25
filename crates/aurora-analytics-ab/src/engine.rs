/// analytics ab: create, assign, measure, conclude, log
/// Phase 2190

#[derive(Debug, Clone)]
pub struct AnalyticsAb {
    pub create_ok: bool,
    pub assign_ok: bool,
    pub measure_ok: bool,
    pub conclude_ok: bool,
    pub log_ok: bool,
}

impl Default for AnalyticsAb {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyticsAb {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            assign_ok: true,
            measure_ok: true,
            conclude_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.assign_ok && self.measure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.conclude_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.assign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = AnalyticsAb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AnalyticsAb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AnalyticsAb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AnalyticsAb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AnalyticsAb::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AnalyticsAb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
