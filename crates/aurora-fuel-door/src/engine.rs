/// Fuel door: release mechanism, hinge, seal, capless filler
/// Phase 416

#[derive(Debug, Clone)]
pub struct FuelDoor {
    pub release_ok: bool,
    pub hinge_ok: bool,
    pub seal_ok: bool,
    pub capless: bool,
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
            release_ok: true,
            hinge_ok: true,
            seal_ok: true,
            capless: false,
            spring_ok: true,
        }
    }

    pub fn functional(&self) -> bool {
        self.release_ok && self.hinge_ok && self.spring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.functional() && self.seal_ok
    }

    pub fn needs_repair(&self) -> bool {
        !self.release_ok || !self.hinge_ok
    }

    pub fn sealed(&self) -> bool {
        self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.release_ok {
            return 20.0;
        }
        if !self.seal_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional() {
        let f = FuelDoor::new();
        assert!(f.functional());
    }

    #[test]
    fn test_all_ok() {
        let f = FuelDoor::new();
        assert!(f.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let f = FuelDoor::new();
        assert!(!f.needs_repair());
    }

    #[test]
    fn test_sealed() {
        let f = FuelDoor::new();
        assert!(f.sealed());
    }

    #[test]
    fn test_stuck() {
        let mut f = FuelDoor::new();
        f.release_ok = false;
        assert!(f.needs_repair());
    }

    #[test]
    fn test_health() {
        let f = FuelDoor::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
