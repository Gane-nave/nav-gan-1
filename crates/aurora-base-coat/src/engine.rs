/// Base coat: color match, coverage, metallic, pearl
/// Phase 763

#[derive(Debug, Clone)]
pub struct BaseCoat {
    pub color_ok: bool,
    pub coverage_ok: bool,
    pub metallic_ok: bool,
    pub pearl_ok: bool,
    pub adhesion_ok: bool,
}

impl Default for BaseCoat {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseCoat {
    pub fn new() -> Self {
        Self {
            color_ok: true,
            coverage_ok: true,
            metallic_ok: true,
            pearl_ok: true,
            adhesion_ok: true,
        }
    }

    pub fn finish_ok(&self) -> bool {
        self.color_ok && self.coverage_ok
    }

    pub fn effect_ok(&self) -> bool {
        self.metallic_ok && self.pearl_ok && self.adhesion_ok
    }

    pub fn all_ok(&self) -> bool {
        self.finish_ok() && self.effect_ok()
    }

    pub fn needs_refinish(&self) -> bool {
        !self.color_ok || !self.coverage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.color_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finish() {
        let c = BaseCoat::new();
        assert!(c.finish_ok());
    }

    #[test]
    fn test_effect() {
        let c = BaseCoat::new();
        assert!(c.effect_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BaseCoat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refinish() {
        let c = BaseCoat::new();
        assert!(!c.needs_refinish());
    }

    #[test]
    fn test_color() {
        let mut c = BaseCoat::new();
        c.color_ok = false;
        assert!(c.needs_refinish());
    }

    #[test]
    fn test_health() {
        let c = BaseCoat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
