/// Boost controller: turbo, wastegate, target, overboost
/// Phase 954

#[derive(Debug, Clone)]
pub struct BoostCtrl {
    pub turbo_ok: bool,
    pub wastegate_ok: bool,
    pub target_ok: bool,
    pub overboost_ok: bool,
    pub sensor_ok: bool,
}

impl Default for BoostCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl BoostCtrl {
    pub fn new() -> Self {
        Self {
            turbo_ok: true,
            wastegate_ok: true,
            target_ok: true,
            overboost_ok: true,
            sensor_ok: true,
        }
    }

    pub fn regulation_ok(&self) -> bool {
        self.turbo_ok && self.wastegate_ok && self.sensor_ok
    }

    pub fn performance_ok(&self) -> bool {
        self.target_ok && self.overboost_ok
    }

    pub fn all_ok(&self) -> bool {
        self.regulation_ok() && self.performance_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.sensor_ok || !self.turbo_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regulation() {
        let c = BoostCtrl::new();
        assert!(c.regulation_ok());
    }

    #[test]
    fn test_performance() {
        let c = BoostCtrl::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BoostCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = BoostCtrl::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_sensor() {
        let mut c = BoostCtrl::new();
        c.sensor_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = BoostCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
