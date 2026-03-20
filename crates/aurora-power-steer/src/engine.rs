/// Power steering: EPS motor, torque sensor, assist map
/// Phase 482

#[derive(Debug, Clone)]
pub struct PowerSteering {
    pub assist_level_pct: f64,
    pub motor_current_a: f64,
    pub torque_sensor_ok: bool,
    pub motor_ok: bool,
    pub ecu_ok: bool,
}

impl Default for PowerSteering {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerSteering {
    pub fn new() -> Self {
        Self {
            assist_level_pct: 50.0,
            motor_current_a: 15.0,
            torque_sensor_ok: true,
            motor_ok: true,
            ecu_ok: true,
        }
    }

    pub fn assist_active(&self) -> bool {
        self.assist_level_pct > 0.0 && self.motor_ok
    }

    pub fn current_ok(&self) -> bool {
        self.motor_current_a < 80.0
    }

    pub fn all_ok(&self) -> bool {
        self.torque_sensor_ok && self.motor_ok && self.ecu_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.torque_sensor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assist() {
        let c = PowerSteering::new();
        assert!(c.assist_active());
    }

    #[test]
    fn test_current() {
        let c = PowerSteering::new();
        assert!(c.current_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PowerSteering::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = PowerSteering::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor_fail() {
        let mut c = PowerSteering::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = PowerSteering::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
