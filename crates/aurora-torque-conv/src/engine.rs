/// Torque converter: stall speed, lock-up clutch, fluid coupling
/// Phase 326

#[derive(Debug, Clone)]
pub struct TorqueConverter {
    pub stall_speed_rpm: f64,
    pub lockup_engaged: bool,
    pub slip_pct: f64,
    pub fluid_temp_c: f64,
    pub shudder_detected: bool,
}

impl Default for TorqueConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl TorqueConverter {
    pub fn new() -> Self {
        Self {
            stall_speed_rpm: 2200.0,
            lockup_engaged: false,
            slip_pct: 5.0,
            fluid_temp_c: 80.0,
            shudder_detected: false,
        }
    }

    pub fn locked_up(&self) -> bool {
        self.lockup_engaged && self.slip_pct < 1.0
    }

    pub fn slip_ok(&self) -> bool {
        self.slip_pct < 10.0
    }

    pub fn temp_ok(&self) -> bool {
        self.fluid_temp_c < 120.0
    }

    pub fn needs_service(&self) -> bool {
        self.shudder_detected || !self.slip_ok()
    }

    pub fn health_score(&self) -> f64 {
        if self.shudder_detected {
            return 20.0;
        }
        if !self.temp_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_locked() {
        let t = TorqueConverter::new();
        assert!(!t.locked_up());
    }

    #[test]
    fn test_slip_ok() {
        let t = TorqueConverter::new();
        assert!(t.slip_ok());
    }

    #[test]
    fn test_temp_ok() {
        let t = TorqueConverter::new();
        assert!(t.temp_ok());
    }

    #[test]
    fn test_no_service() {
        let t = TorqueConverter::new();
        assert!(!t.needs_service());
    }

    #[test]
    fn test_shudder() {
        let mut t = TorqueConverter::new();
        t.shudder_detected = true;
        assert!(t.needs_service());
    }

    #[test]
    fn test_health() {
        let t = TorqueConverter::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
