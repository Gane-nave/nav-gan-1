/// Starter motor: solenoid, armature, drive gear, flywheel ring
/// Phase 725

#[derive(Debug, Clone)]
pub struct StarterMotor {
    pub solenoid_ok: bool,
    pub armature_ok: bool,
    pub drive_ok: bool,
    pub ring_gear_ok: bool,
    pub current_ok: bool,
}

impl Default for StarterMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl StarterMotor {
    pub fn new() -> Self {
        Self {
            solenoid_ok: true,
            armature_ok: true,
            drive_ok: true,
            ring_gear_ok: true,
            current_ok: true,
        }
    }

    pub fn engagement_ok(&self) -> bool {
        self.solenoid_ok && self.drive_ok && self.ring_gear_ok
    }

    pub fn motor_ok(&self) -> bool {
        self.armature_ok && self.current_ok
    }

    pub fn all_ok(&self) -> bool {
        self.engagement_ok() && self.motor_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.solenoid_ok || !self.armature_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.armature_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engagement() {
        let c = StarterMotor::new();
        assert!(c.engagement_ok());
    }

    #[test]
    fn test_motor() {
        let c = StarterMotor::new();
        assert!(c.motor_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StarterMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = StarterMotor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_solenoid() {
        let mut c = StarterMotor::new();
        c.solenoid_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = StarterMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
