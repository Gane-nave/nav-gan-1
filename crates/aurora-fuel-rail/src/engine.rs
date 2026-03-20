/// Fuel rail: common rail pressure, damper, regulator
/// Phase 500

#[derive(Debug, Clone)]
pub struct FuelRail {
    pub pressure_bar: f64,
    pub target_pressure_bar: f64,
    pub damper_ok: bool,
    pub regulator_ok: bool,
    pub leak_free: bool,
}

impl Default for FuelRail {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelRail {
    pub fn new() -> Self {
        Self {
            pressure_bar: 200.0,
            target_pressure_bar: 200.0,
            damper_ok: true,
            regulator_ok: true,
            leak_free: true,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        (self.pressure_bar - self.target_pressure_bar).abs() < 20.0
    }

    pub fn damper_functional(&self) -> bool {
        self.damper_ok
    }

    pub fn all_ok(&self) -> bool {
        self.pressure_ok() && self.damper_ok && self.regulator_ok && self.leak_free
    }

    pub fn needs_service(&self) -> bool {
        !self.regulator_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let c = FuelRail::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_damper() {
        let c = FuelRail::new();
        assert!(c.damper_functional());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelRail::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FuelRail::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = FuelRail::new();
        c.leak_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FuelRail::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
