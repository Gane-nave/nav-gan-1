/// Wiper inspection: blade, arm, washer, motor
/// Phase 832

#[derive(Debug, Clone)]
pub struct WiperInspect {
    pub blade_ok: bool,
    pub arm_ok: bool,
    pub washer_ok: bool,
    pub motor_ok: bool,
    pub streak_free: bool,
}

impl Default for WiperInspect {
    fn default() -> Self {
        Self::new()
    }
}

impl WiperInspect {
    pub fn new() -> Self {
        Self {
            blade_ok: true,
            arm_ok: true,
            washer_ok: true,
            motor_ok: true,
            streak_free: true,
        }
    }

    pub fn cleaning_ok(&self) -> bool {
        self.blade_ok && self.washer_ok && self.streak_free
    }

    pub fn mechanism_ok(&self) -> bool {
        self.arm_ok && self.motor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.cleaning_ok() && self.mechanism_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.blade_ok || !self.streak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.blade_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleaning() {
        let c = WiperInspect::new();
        assert!(c.cleaning_ok());
    }

    #[test]
    fn test_mechanism() {
        let c = WiperInspect::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WiperInspect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WiperInspect::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_blade() {
        let mut c = WiperInspect::new();
        c.blade_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WiperInspect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
