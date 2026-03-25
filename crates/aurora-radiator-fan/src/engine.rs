/// Radiator fan: electric fan speed, temperature trigger, dual fan control
/// Phase 301

#[derive(Debug, Clone)]
pub struct RadiatorFan {
    pub fan1_running: bool,
    pub fan2_running: bool,
    pub speed_pct: f64,
    pub coolant_temp_c: f64,
    pub trigger_temp_c: f64,
    pub motor_ok: bool,
}

impl Default for RadiatorFan {
    fn default() -> Self {
        Self::new()
    }
}

impl RadiatorFan {
    pub fn new() -> Self {
        Self {
            fan1_running: false,
            fan2_running: false,
            speed_pct: 0.0,
            coolant_temp_c: 80.0,
            trigger_temp_c: 95.0,
            motor_ok: true,
        }
    }

    pub fn any_running(&self) -> bool {
        self.fan1_running || self.fan2_running
    }

    pub fn should_activate(&self) -> bool {
        self.coolant_temp_c >= self.trigger_temp_c
    }

    pub fn dual_fan(&self) -> bool {
        self.fan1_running && self.fan2_running
    }

    pub fn overtemp(&self) -> bool {
        self.coolant_temp_c > 110.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        if self.overtemp() {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_running() {
        let r = RadiatorFan::new();
        assert!(!r.any_running());
    }

    #[test]
    fn test_no_activate() {
        let r = RadiatorFan::new();
        assert!(!r.should_activate());
    }

    #[test]
    fn test_no_dual() {
        let r = RadiatorFan::new();
        assert!(!r.dual_fan());
    }

    #[test]
    fn test_no_overtemp() {
        let r = RadiatorFan::new();
        assert!(!r.overtemp());
    }

    #[test]
    fn test_hot_coolant() {
        let mut r = RadiatorFan::new();
        r.coolant_temp_c = 100.0;
        assert!(r.should_activate());
    }

    #[test]
    fn test_health() {
        let r = RadiatorFan::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
