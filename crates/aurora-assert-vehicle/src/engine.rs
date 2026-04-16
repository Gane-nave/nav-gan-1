/// aurora-assert-vehicle: assert vehicle
/// Phase 2508

#[derive(Debug, Clone)]
pub struct AssertVehicle {
    pub speed_ok: bool,
    pub heading_ok: bool,
    pub gear_ok: bool,
    pub throttle_ok: bool,
    pub brake_ok: bool,
}

impl Default for AssertVehicle {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertVehicle {
    pub fn new() -> Self {
        Self {
            speed_ok: true,
            heading_ok: true,
            gear_ok: true,
            throttle_ok: true,
            brake_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.speed_ok && self.heading_ok && self.gear_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.throttle_ok && self.brake_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.speed_ok || !self.heading_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.speed_ok {
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
        let c = AssertVehicle::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertVehicle::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertVehicle::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertVehicle::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertVehicle::new();
        c.speed_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertVehicle::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertVehicle::default();
        assert!(c.all_ok());
    }
}
