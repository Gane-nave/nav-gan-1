/// Fuel optimization: route, speed, idle, coast, report
/// Phase 1096

#[derive(Debug, Clone)]
pub struct FuelOptim {
    pub route_ok: bool,
    pub speed_ok: bool,
    pub idle_ok: bool,
    pub coast_ok: bool,
    pub report_ok: bool,
}

impl Default for FuelOptim {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelOptim {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            speed_ok: true,
            idle_ok: true,
            coast_ok: true,
            report_ok: true,
        }
    }

    pub fn driving_ok(&self) -> bool {
        self.route_ok && self.speed_ok && self.idle_ok
    }

    pub fn analysis_ok(&self) -> bool {
        self.coast_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.driving_ok() && self.analysis_ok()
    }

    pub fn needs_calibrate(&self) -> bool {
        !self.route_ok || !self.speed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driving() {
        let c = FuelOptim::new();
        assert!(c.driving_ok());
    }

    #[test]
    fn test_analysis() {
        let c = FuelOptim::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelOptim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_calibrate() {
        let c = FuelOptim::new();
        assert!(!c.needs_calibrate());
    }

    #[test]
    fn test_route() {
        let mut c = FuelOptim::new();
        c.route_ok = false;
        assert!(c.needs_calibrate());
    }

    #[test]
    fn test_health() {
        let c = FuelOptim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
