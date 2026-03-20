/// Crash simulation: impact, deform, occupant, structure
/// Phase 961

#[derive(Debug, Clone)]
pub struct CrashSim {
    pub impact_ok: bool,
    pub deform_ok: bool,
    pub occupant_ok: bool,
    pub structure_ok: bool,
    pub validate_ok: bool,
}

impl Default for CrashSim {
    fn default() -> Self {
        Self::new()
    }
}

impl CrashSim {
    pub fn new() -> Self {
        Self {
            impact_ok: true,
            deform_ok: true,
            occupant_ok: true,
            structure_ok: true,
            validate_ok: true,
        }
    }

    pub fn physics_ok(&self) -> bool {
        self.impact_ok && self.deform_ok && self.structure_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.occupant_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.physics_ok() && self.safety_ok()
    }

    pub fn needs_review(&self) -> bool {
        !self.validate_ok || !self.impact_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.impact_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics() {
        let c = CrashSim::new();
        assert!(c.physics_ok());
    }

    #[test]
    fn test_safety() {
        let c = CrashSim::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CrashSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_review() {
        let c = CrashSim::new();
        assert!(!c.needs_review());
    }

    #[test]
    fn test_validate() {
        let mut c = CrashSim::new();
        c.validate_ok = false;
        assert!(c.needs_review());
    }

    #[test]
    fn test_health() {
        let c = CrashSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
