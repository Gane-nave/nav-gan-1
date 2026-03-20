/// Wiper motor: linkage, blade, washer, intermittent
/// Phase 677

#[derive(Debug, Clone)]
pub struct WiperMotor {
    pub motor_ok: bool,
    pub linkage_ok: bool,
    pub blade_ok: bool,
    pub washer_ok: bool,
    pub intermittent_ok: bool,
}

impl Default for WiperMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl WiperMotor {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            linkage_ok: true,
            blade_ok: true,
            washer_ok: true,
            intermittent_ok: true,
        }
    }

    pub fn drive_ok(&self) -> bool {
        self.motor_ok && self.linkage_ok
    }

    pub fn cleaning_ok(&self) -> bool {
        self.blade_ok && self.washer_ok
    }

    pub fn all_ok(&self) -> bool {
        self.drive_ok() && self.cleaning_ok() && self.intermittent_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.blade_ok
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
    fn test_drive() {
        let c = WiperMotor::new();
        assert!(c.drive_ok());
    }

    #[test]
    fn test_cleaning() {
        let c = WiperMotor::new();
        assert!(c.cleaning_ok());
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
