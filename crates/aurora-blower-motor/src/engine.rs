/// Blower motor: fan speed, resistor, relay
/// Phase 627

#[derive(Debug, Clone)]
pub struct BlowerMotor {
    pub motor_ok: bool,
    pub resistor_ok: bool,
    pub relay_ok: bool,
    pub fan_ok: bool,
    pub noise_ok: bool,
}

impl Default for BlowerMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl BlowerMotor {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            resistor_ok: true,
            relay_ok: true,
            fan_ok: true,
            noise_ok: true,
        }
    }

    pub fn motor_good(&self) -> bool {
        self.motor_ok && self.fan_ok
    }

    pub fn electronics_ok(&self) -> bool {
        self.resistor_ok && self.relay_ok
    }

    pub fn all_ok(&self) -> bool {
        self.motor_good() && self.electronics_ok() && self.noise_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.motor_ok || !self.resistor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motor() {
        let c = BlowerMotor::new();
        assert!(c.motor_good());
    }

    #[test]
    fn test_electronics() {
        let c = BlowerMotor::new();
        assert!(c.electronics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BlowerMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BlowerMotor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_motor_fail() {
        let mut c = BlowerMotor::new();
        c.motor_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BlowerMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
