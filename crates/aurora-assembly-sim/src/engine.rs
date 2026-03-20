/// Assembly simulation: sequence, torque, fit, tolerance
/// Phase 967

#[derive(Debug, Clone)]
pub struct AssemblySim {
    pub sequence_ok: bool,
    pub torque_ok: bool,
    pub fit_ok: bool,
    pub tolerance_ok: bool,
    pub validate_ok: bool,
}

impl Default for AssemblySim {
    fn default() -> Self {
        Self::new()
    }
}

impl AssemblySim {
    pub fn new() -> Self {
        Self {
            sequence_ok: true,
            torque_ok: true,
            fit_ok: true,
            tolerance_ok: true,
            validate_ok: true,
        }
    }

    pub fn process_ok(&self) -> bool {
        self.sequence_ok && self.torque_ok && self.fit_ok
    }

    pub fn quality_ok(&self) -> bool {
        self.tolerance_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.process_ok() && self.quality_ok()
    }

    pub fn needs_review(&self) -> bool {
        !self.validate_ok || !self.tolerance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sequence_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let c = AssemblySim::new();
        assert!(c.process_ok());
    }

    #[test]
    fn test_quality() {
        let c = AssemblySim::new();
        assert!(c.quality_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssemblySim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_review() {
        let c = AssemblySim::new();
        assert!(!c.needs_review());
    }

    #[test]
    fn test_validate() {
        let mut c = AssemblySim::new();
        c.validate_ok = false;
        assert!(c.needs_review());
    }

    #[test]
    fn test_health() {
        let c = AssemblySim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
