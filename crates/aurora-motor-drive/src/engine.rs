/// motor drive: torque, speed, field, brake, coast
/// Phase 1149

#[derive(Debug, Clone)]
pub struct MotorDrive {
    pub torque_ok: bool,
    pub speed_ok: bool,
    pub field_ok: bool,
    pub brake_ok: bool,
    pub coast_ok: bool,
}

impl Default for MotorDrive {
    fn default() -> Self {
        Self::new()
    }
}

impl MotorDrive {
    pub fn new() -> Self {
        Self {
            torque_ok: true,
            speed_ok: true,
            field_ok: true,
            brake_ok: true,
            coast_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.torque_ok && self.speed_ok && self.field_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.brake_ok && self.coast_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.torque_ok || !self.speed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.torque_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MotorDrive::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MotorDrive::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MotorDrive::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MotorDrive::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MotorDrive::new();
        c.torque_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MotorDrive::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
