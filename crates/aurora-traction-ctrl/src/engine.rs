/// Traction control: wheel slip, throttle intervention
/// Phase 490

#[derive(Debug, Clone)]
pub struct TractionControl {
    pub slip_pct: f64,
    pub max_slip_pct: f64,
    pub tc_active: bool,
    pub intervening: bool,
    pub sensor_ok: bool,
}

impl Default for TractionControl {
    fn default() -> Self {
        Self::new()
    }
}

impl TractionControl {
    pub fn new() -> Self {
        Self {
            slip_pct: 2.0,
            max_slip_pct: 10.0,
            tc_active: true,
            intervening: false,
            sensor_ok: true,
        }
    }

    pub fn slip_ok(&self) -> bool {
        self.slip_pct < self.max_slip_pct
    }

    pub fn system_ok(&self) -> bool {
        self.tc_active && self.sensor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.slip_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slip() {
        let c = TractionControl::new();
        assert!(c.slip_ok());
    }

    #[test]
    fn test_system() {
        let c = TractionControl::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TractionControl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TractionControl::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_sensor_fail() {
        let mut c = TractionControl::new();
        c.sensor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TractionControl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
