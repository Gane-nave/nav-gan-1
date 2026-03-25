/// Intake manifold: runner length, swirl flap, vacuum
/// Phase 502

#[derive(Debug, Clone)]
pub struct IntakeManifold {
    pub vacuum_kpa: f64,
    pub swirl_flap_ok: bool,
    pub runner_ok: bool,
    pub gasket_ok: bool,
    pub leak_free: bool,
}

impl Default for IntakeManifold {
    fn default() -> Self {
        Self::new()
    }
}

impl IntakeManifold {
    pub fn new() -> Self {
        Self {
            vacuum_kpa: 50.0,
            swirl_flap_ok: true,
            runner_ok: true,
            gasket_ok: true,
            leak_free: true,
        }
    }

    pub fn vacuum_ok(&self) -> bool {
        self.vacuum_kpa > 20.0
    }

    pub fn flaps_ok(&self) -> bool {
        self.swirl_flap_ok
    }

    pub fn all_ok(&self) -> bool {
        self.vacuum_ok() && self.swirl_flap_ok && self.runner_ok && self.gasket_ok && self.leak_free
    }

    pub fn needs_service(&self) -> bool {
        !self.gasket_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacuum() {
        let c = IntakeManifold::new();
        assert!(c.vacuum_ok());
    }

    #[test]
    fn test_flaps() {
        let c = IntakeManifold::new();
        assert!(c.flaps_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntakeManifold::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = IntakeManifold::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_gasket_fail() {
        let mut c = IntakeManifold::new();
        c.gasket_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = IntakeManifold::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
