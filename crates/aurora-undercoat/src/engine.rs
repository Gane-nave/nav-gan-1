/// Undercoating: thickness, coverage, flexibility, adhesion
/// Phase 765

#[derive(Debug, Clone)]
pub struct Undercoat {
    pub thickness_ok: bool,
    pub coverage_ok: bool,
    pub flexibility_ok: bool,
    pub adhesion_ok: bool,
    pub cured: bool,
}

impl Default for Undercoat {
    fn default() -> Self {
        Self::new()
    }
}

impl Undercoat {
    pub fn new() -> Self {
        Self {
            thickness_ok: true,
            coverage_ok: true,
            flexibility_ok: true,
            adhesion_ok: true,
            cured: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.thickness_ok && self.coverage_ok
    }

    pub fn durability_ok(&self) -> bool {
        self.flexibility_ok && self.adhesion_ok && self.cured
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.durability_ok()
    }

    pub fn needs_reapply(&self) -> bool {
        !self.thickness_ok || !self.coverage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.thickness_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = Undercoat::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_durability() {
        let c = Undercoat::new();
        assert!(c.durability_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Undercoat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reapply() {
        let c = Undercoat::new();
        assert!(!c.needs_reapply());
    }

    #[test]
    fn test_thickness() {
        let mut c = Undercoat::new();
        c.thickness_ok = false;
        assert!(c.needs_reapply());
    }

    #[test]
    fn test_health() {
        let c = Undercoat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
