/// Wiper mechanism: linkage, motor, park position, washer system
/// Phase 419

#[derive(Debug, Clone)]
pub struct WiperMech {
    pub motor_ok: bool,
    pub linkage_ok: bool,
    pub blade_ok: bool,
    pub washer_ok: bool,
    pub park_ok: bool,
}

impl Default for WiperMech {
    fn default() -> Self {
        Self::new()
    }
}

impl WiperMech {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            linkage_ok: true,
            blade_ok: true,
            washer_ok: true,
            park_ok: true,
        }
    }

    pub fn functional(&self) -> bool {
        self.motor_ok && self.linkage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.functional() && self.blade_ok && self.washer_ok && self.park_ok
    }

    pub fn needs_blades(&self) -> bool {
        !self.blade_ok
    }

    pub fn needs_repair(&self) -> bool {
        !self.motor_ok || !self.linkage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        if !self.blade_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional() {
        let w = WiperMech::new();
        assert!(w.functional());
    }

    #[test]
    fn test_all_ok() {
        let w = WiperMech::new();
        assert!(w.all_ok());
    }

    #[test]
    fn test_no_blades() {
        let w = WiperMech::new();
        assert!(!w.needs_blades());
    }

    #[test]
    fn test_no_repair() {
        let w = WiperMech::new();
        assert!(!w.needs_repair());
    }

    #[test]
    fn test_bad_motor() {
        let mut w = WiperMech::new();
        w.motor_ok = false;
        assert!(w.needs_repair());
    }

    #[test]
    fn test_health() {
        let w = WiperMech::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
