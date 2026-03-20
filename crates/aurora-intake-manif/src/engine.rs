/// Intake manifold: runner, flap, vacuum, gasket
/// Phase 609

#[derive(Debug, Clone)]
pub struct IntakeManifold {
    pub runner_ok: bool,
    pub flap_ok: bool,
    pub vacuum_ok: bool,
    pub gasket_ok: bool,
    pub clean: bool,
}

impl Default for IntakeManifold {
    fn default() -> Self {
        Self::new()
    }
}

impl IntakeManifold {
    pub fn new() -> Self {
        Self {
            runner_ok: true,
            flap_ok: true,
            vacuum_ok: true,
            gasket_ok: true,
            clean: true,
        }
    }

    pub fn airflow_ok(&self) -> bool {
        self.runner_ok && self.flap_ok && self.vacuum_ok
    }

    pub fn sealing_ok(&self) -> bool {
        self.gasket_ok
    }

    pub fn all_ok(&self) -> bool {
        self.airflow_ok() && self.sealing_ok() && self.clean
    }

    pub fn needs_service(&self) -> bool {
        !self.gasket_ok || !self.flap_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gasket_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airflow() {
        let c = IntakeManifold::new();
        assert!(c.airflow_ok());
    }

    #[test]
    fn test_sealing() {
        let c = IntakeManifold::new();
        assert!(c.sealing_ok());
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
    fn test_gasket() {
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
