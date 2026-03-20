/// Blend door actuator: motor, position, calibration
/// Phase 629

#[derive(Debug, Clone)]
pub struct BlendDoor {
    pub motor_ok: bool,
    pub position_ok: bool,
    pub calibrated: bool,
    pub linkage_ok: bool,
    pub signal_ok: bool,
}

impl Default for BlendDoor {
    fn default() -> Self {
        Self::new()
    }
}

impl BlendDoor {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            position_ok: true,
            calibrated: true,
            linkage_ok: true,
            signal_ok: true,
        }
    }

    pub fn actuator_ok(&self) -> bool {
        self.motor_ok && self.linkage_ok
    }

    pub fn control_ok(&self) -> bool {
        self.position_ok && self.signal_ok && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.actuator_ok() && self.control_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.linkage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuator() {
        let c = BlendDoor::new();
        assert!(c.actuator_ok());
    }

    #[test]
    fn test_control() {
        let c = BlendDoor::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BlendDoor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BlendDoor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = BlendDoor::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BlendDoor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
