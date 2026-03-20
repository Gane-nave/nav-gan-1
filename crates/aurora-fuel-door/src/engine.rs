/// Fuel door: actuator, hinge, spring, seal
/// Phase 555

#[derive(Debug, Clone)]
pub struct FuelDoor {
    pub actuator_ok: bool,
    pub hinge_ok: bool,
    pub spring_ok: bool,
    pub seal_ok: bool,
    pub locked: bool,
}

impl Default for FuelDoor {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelDoor {
    pub fn new() -> Self {
        Self {
            actuator_ok: true,
            hinge_ok: true,
            spring_ok: true,
            seal_ok: true,
            locked: true,
        }
    }

    pub fn mechanism_ok(&self) -> bool {
        self.actuator_ok && self.hinge_ok && self.spring_ok
    }

    pub fn sealed(&self) -> bool {
        self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanism_ok() && self.sealed()
    }

    pub fn needs_service(&self) -> bool {
        !self.actuator_ok || !self.hinge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanism() {
        let c = FuelDoor::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_sealed() {
        let c = FuelDoor::new();
        assert!(c.sealed());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelDoor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FuelDoor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_actuator() {
        let mut c = FuelDoor::new();
        c.actuator_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FuelDoor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
