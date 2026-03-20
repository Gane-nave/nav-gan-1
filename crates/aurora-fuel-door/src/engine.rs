/// Fuel door: actuator, hinge, seal, lock
/// Phase 685

#[derive(Debug, Clone)]
pub struct FuelDoor {
    pub actuator_ok: bool,
    pub hinge_ok: bool,
    pub seal_ok: bool,
    pub lock_ok: bool,
    pub spring_ok: bool,
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
            seal_ok: true,
            lock_ok: true,
            spring_ok: true,
        }
    }

    pub fn mechanism_ok(&self) -> bool {
        self.actuator_ok && self.hinge_ok && self.spring_ok
    }

    pub fn sealing_ok(&self) -> bool {
        self.seal_ok && self.lock_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanism_ok() && self.sealing_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.actuator_ok || !self.hinge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok { return 15.0; }
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
    fn test_sealing() {
        let c = FuelDoor::new();
        assert!(c.sealing_ok());
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
