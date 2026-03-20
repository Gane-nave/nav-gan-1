/// E-axle: motor, gearbox, inverter, cooling, housing
/// Phase 862

#[derive(Debug, Clone)]
pub struct EAxle {
    pub motor_ok: bool,
    pub gearbox_ok: bool,
    pub inverter_ok: bool,
    pub cooling_ok: bool,
    pub housing_ok: bool,
}

impl Default for EAxle {
    fn default() -> Self {
        Self::new()
    }
}

impl EAxle {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            gearbox_ok: true,
            inverter_ok: true,
            cooling_ok: true,
            housing_ok: true,
        }
    }

    pub fn powertrain_ok(&self) -> bool {
        self.motor_ok && self.gearbox_ok && self.inverter_ok
    }

    pub fn support_ok(&self) -> bool {
        self.cooling_ok && self.housing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.powertrain_ok() && self.support_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.cooling_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powertrain() {
        let c = EAxle::new();
        assert!(c.powertrain_ok());
    }

    #[test]
    fn test_support() {
        let c = EAxle::new();
        assert!(c.support_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EAxle::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EAxle::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = EAxle::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EAxle::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
