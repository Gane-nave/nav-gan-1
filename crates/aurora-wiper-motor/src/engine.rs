/// Wiper motor: speed control, park position, linkage
/// Phase 534

#[derive(Debug, Clone)]
pub struct WiperMotor {
    pub speed_rpm: f64,
    pub park_ok: bool,
    pub linkage_ok: bool,
    pub motor_ok: bool,
    pub washer_ok: bool,
}

impl Default for WiperMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl WiperMotor {
    pub fn new() -> Self {
        Self {
            speed_rpm: 45.0,
            park_ok: true,
            linkage_ok: true,
            motor_ok: true,
            washer_ok: true,
        }
    }

    pub fn speed_ok(&self) -> bool {
        self.speed_rpm > 20.0
    }

    pub fn mechanical_ok(&self) -> bool {
        self.park_ok && self.linkage_ok && self.motor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.speed_ok() && self.mechanical_ok() && self.washer_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.linkage_ok
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
    fn test_speed() {
        let c = WiperMotor::new();
        assert!(c.speed_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = WiperMotor::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WiperMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = WiperMotor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = WiperMotor::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = WiperMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
