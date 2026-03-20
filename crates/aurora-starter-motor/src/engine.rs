/// Starter motor: cranking speed, solenoid, bendix
/// Phase 517

#[derive(Debug, Clone)]
pub struct StarterMotor {
    pub cranking_rpm: f64,
    pub min_cranking_rpm: f64,
    pub solenoid_ok: bool,
    pub bendix_ok: bool,
    pub brush_ok: bool,
}

impl Default for StarterMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl StarterMotor {
    pub fn new() -> Self {
        Self {
            cranking_rpm: 200.0,
            min_cranking_rpm: 100.0,
            solenoid_ok: true,
            bendix_ok: true,
            brush_ok: true,
        }
    }

    pub fn cranking_ok(&self) -> bool {
        self.cranking_rpm > self.min_cranking_rpm
    }

    pub fn mechanical_ok(&self) -> bool {
        self.solenoid_ok && self.bendix_ok && self.brush_ok
    }

    pub fn all_ok(&self) -> bool {
        self.cranking_ok() && self.mechanical_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.solenoid_ok || !self.brush_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.solenoid_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cranking() {
        let c = StarterMotor::new();
        assert!(c.cranking_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = StarterMotor::new();
        assert!(c.mechanical_ok());
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
