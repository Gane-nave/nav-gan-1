/// Engine mount: rubber, hydraulic, bracket, isolator
/// Phase 799

#[derive(Debug, Clone)]
pub struct EngineMount {
    pub rubber_ok: bool,
    pub hydraulic_ok: bool,
    pub bracket_ok: bool,
    pub isolator_ok: bool,
    pub alignment_ok: bool,
}

impl Default for EngineMount {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineMount {
    pub fn new() -> Self {
        Self {
            rubber_ok: true,
            hydraulic_ok: true,
            bracket_ok: true,
            isolator_ok: true,
            alignment_ok: true,
        }
    }

    pub fn damping_ok(&self) -> bool {
        self.rubber_ok && self.hydraulic_ok
    }

    pub fn structure_ok(&self) -> bool {
        self.bracket_ok && self.isolator_ok && self.alignment_ok
    }

    pub fn all_ok(&self) -> bool {
        self.damping_ok() && self.structure_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.rubber_ok || !self.hydraulic_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rubber_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damping() {
        let c = EngineMount::new();
        assert!(c.damping_ok());
    }

    #[test]
    fn test_structure() {
        let c = EngineMount::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EngineMount::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = EngineMount::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_rubber() {
        let mut c = EngineMount::new();
        c.rubber_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = EngineMount::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
