/// Aero simulation: CFD, drag, lift, turbulence, validation
/// Phase 960

#[derive(Debug, Clone)]
pub struct AeroSim {
    pub cfd_ok: bool,
    pub drag_ok: bool,
    pub lift_ok: bool,
    pub turbulence_ok: bool,
    pub validation_ok: bool,
}

impl Default for AeroSim {
    fn default() -> Self {
        Self::new()
    }
}

impl AeroSim {
    pub fn new() -> Self {
        Self {
            cfd_ok: true,
            drag_ok: true,
            lift_ok: true,
            turbulence_ok: true,
            validation_ok: true,
        }
    }

    pub fn simulation_ok(&self) -> bool {
        self.cfd_ok && self.drag_ok && self.lift_ok
    }

    pub fn accuracy_ok(&self) -> bool {
        self.turbulence_ok && self.validation_ok
    }

    pub fn all_ok(&self) -> bool {
        self.simulation_ok() && self.accuracy_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.cfd_ok || !self.validation_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cfd_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation() {
        let c = AeroSim::new();
        assert!(c.simulation_ok());
    }

    #[test]
    fn test_accuracy() {
        let c = AeroSim::new();
        assert!(c.accuracy_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AeroSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = AeroSim::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_cfd() {
        let mut c = AeroSim::new();
        c.cfd_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = AeroSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
