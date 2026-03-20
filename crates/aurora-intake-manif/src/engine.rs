/// Intake manifold: runner length, plenum pressure, IMRC control
/// Phase 318

#[derive(Debug, Clone)]
pub struct IntakeManifold {
    pub pressure_kpa: f64,
    pub temp_c: f64,
    pub runner_flap_ok: bool,
    pub vacuum_ok: bool,
    pub leak_detected: bool,
}

impl Default for IntakeManifold {
    fn default() -> Self {
        Self::new()
    }
}

impl IntakeManifold {
    pub fn new() -> Self {
        Self {
            pressure_kpa: 95.0,
            temp_c: 35.0,
            runner_flap_ok: true,
            vacuum_ok: true,
            leak_detected: false,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        self.pressure_kpa > 20.0 && self.pressure_kpa < 105.0
    }

    pub fn all_ok(&self) -> bool {
        self.runner_flap_ok && self.vacuum_ok && !self.leak_detected
    }

    pub fn boost_present(&self) -> bool {
        self.pressure_kpa > 101.3
    }

    pub fn needs_service(&self) -> bool {
        self.leak_detected || !self.runner_flap_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.leak_detected {
            return 20.0;
        }
        if !self.runner_flap_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let i = IntakeManifold::new();
        assert!(i.pressure_ok());
    }

    #[test]
    fn test_all_ok() {
        let i = IntakeManifold::new();
        assert!(i.all_ok());
    }

    #[test]
    fn test_no_boost() {
        let i = IntakeManifold::new();
        assert!(!i.boost_present());
    }

    #[test]
    fn test_no_service() {
        let i = IntakeManifold::new();
        assert!(!i.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut i = IntakeManifold::new();
        i.leak_detected = true;
        assert!(i.needs_service());
    }

    #[test]
    fn test_health() {
        let i = IntakeManifold::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
