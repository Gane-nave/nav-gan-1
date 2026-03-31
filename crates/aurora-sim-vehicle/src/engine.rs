/// aurora-sim-vehicle: sim vehicle
/// Phase 2524

#[derive(Debug, Clone)]
pub struct SimVehicle {
    pub position_ok: bool,
    pub speed_ok: bool,
    pub heading_ok: bool,
    pub throttle_ok: bool,
    pub brake_ok: bool,
}

impl Default for SimVehicle {
    fn default() -> Self {
        Self::new()
    }
}

impl SimVehicle {
    pub fn new() -> Self {
        Self {
            position_ok: true,
            speed_ok: true,
            heading_ok: true,
            throttle_ok: true,
            brake_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.position_ok && self.speed_ok && self.heading_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.throttle_ok && self.brake_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.position_ok || !self.speed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.position_ok {
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
        let c = SimVehicle::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimVehicle::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimVehicle::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimVehicle::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimVehicle::new();
        c.position_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimVehicle::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimVehicle::default();
        assert!(c.all_ok());
    }
}
