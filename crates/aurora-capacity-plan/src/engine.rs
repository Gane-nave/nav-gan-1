/// Capacity planning: forecast, provision, scale, reserve, report
/// Phase 1080

#[derive(Debug, Clone)]
pub struct CapacityPlan {
    pub forecast_ok: bool,
    pub provision_ok: bool,
    pub scale_ok: bool,
    pub reserve_ok: bool,
    pub report_ok: bool,
}

impl Default for CapacityPlan {
    fn default() -> Self {
        Self::new()
    }
}

impl CapacityPlan {
    pub fn new() -> Self {
        Self {
            forecast_ok: true,
            provision_ok: true,
            scale_ok: true,
            reserve_ok: true,
            report_ok: true,
        }
    }

    pub fn planning_ok(&self) -> bool {
        self.forecast_ok && self.provision_ok && self.scale_ok
    }

    pub fn management_ok(&self) -> bool {
        self.reserve_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.planning_ok() && self.management_ok()
    }

    pub fn needs_forecast(&self) -> bool {
        !self.forecast_ok || !self.provision_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.forecast_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning() {
        let c = CapacityPlan::new();
        assert!(c.planning_ok());
    }

    #[test]
    fn test_management() {
        let c = CapacityPlan::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CapacityPlan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_forecast() {
        let c = CapacityPlan::new();
        assert!(!c.needs_forecast());
    }

    #[test]
    fn test_forecast() {
        let mut c = CapacityPlan::new();
        c.forecast_ok = false;
        assert!(c.needs_forecast());
    }

    #[test]
    fn test_health() {
        let c = CapacityPlan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
