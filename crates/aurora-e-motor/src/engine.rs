/// Electric motor: stator, rotor, bearing, resolver
/// Phase 720

#[derive(Debug, Clone)]
pub struct EMotor {
    pub stator_ok: bool,
    pub rotor_ok: bool,
    pub bearing_ok: bool,
    pub resolver_ok: bool,
    pub cooling_ok: bool,
}

impl Default for EMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl EMotor {
    pub fn new() -> Self {
        Self {
            stator_ok: true,
            rotor_ok: true,
            bearing_ok: true,
            resolver_ok: true,
            cooling_ok: true,
        }
    }

    pub fn electromagnetic_ok(&self) -> bool {
        self.stator_ok && self.rotor_ok
    }

    pub fn mechanical_ok(&self) -> bool {
        self.bearing_ok && self.resolver_ok && self.cooling_ok
    }

    pub fn all_ok(&self) -> bool {
        self.electromagnetic_ok() && self.mechanical_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.stator_ok || !self.bearing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.stator_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electromagnetic() {
        let c = EMotor::new();
        assert!(c.electromagnetic_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = EMotor::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EMotor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_stator() {
        let mut c = EMotor::new();
        c.stator_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
