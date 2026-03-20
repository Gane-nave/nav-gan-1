/// Battery preconditioning: heating, target temp, route aware
/// Phase 877

#[derive(Debug, Clone)]
pub struct BatteryPrecon {
    pub heating_ok: bool,
    pub target_ok: bool,
    pub route_ok: bool,
    pub timer_ok: bool,
    pub efficiency_ok: bool,
}

impl Default for BatteryPrecon {
    fn default() -> Self {
        Self::new()
    }
}

impl BatteryPrecon {
    pub fn new() -> Self {
        Self {
            heating_ok: true,
            target_ok: true,
            route_ok: true,
            timer_ok: true,
            efficiency_ok: true,
        }
    }

    pub fn thermal_ok(&self) -> bool {
        self.heating_ok && self.target_ok
    }

    pub fn planning_ok(&self) -> bool {
        self.route_ok && self.timer_ok && self.efficiency_ok
    }

    pub fn all_ok(&self) -> bool {
        self.thermal_ok() && self.planning_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.heating_ok || !self.target_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.heating_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal() {
        let c = BatteryPrecon::new();
        assert!(c.thermal_ok());
    }

    #[test]
    fn test_planning() {
        let c = BatteryPrecon::new();
        assert!(c.planning_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BatteryPrecon::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = BatteryPrecon::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_heating() {
        let mut c = BatteryPrecon::new();
        c.heating_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = BatteryPrecon::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
