/// Transmission mount: rubber, bracket, crossmember
/// Phase 800

#[derive(Debug, Clone)]
pub struct TransMount {
    pub rubber_ok: bool,
    pub bracket_ok: bool,
    pub crossmember_ok: bool,
    pub bolt_ok: bool,
    pub alignment_ok: bool,
}

impl Default for TransMount {
    fn default() -> Self {
        Self::new()
    }
}

impl TransMount {
    pub fn new() -> Self {
        Self {
            rubber_ok: true,
            bracket_ok: true,
            crossmember_ok: true,
            bolt_ok: true,
            alignment_ok: true,
        }
    }

    pub fn damping_ok(&self) -> bool {
        self.rubber_ok && self.alignment_ok
    }

    pub fn structure_ok(&self) -> bool {
        self.bracket_ok && self.crossmember_ok && self.bolt_ok
    }

    pub fn all_ok(&self) -> bool {
        self.damping_ok() && self.structure_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.rubber_ok || !self.bolt_ok
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
        let c = TransMount::new();
        assert!(c.damping_ok());
    }

    #[test]
    fn test_structure() {
        let c = TransMount::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransMount::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TransMount::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_rubber() {
        let mut c = TransMount::new();
        c.rubber_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TransMount::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
