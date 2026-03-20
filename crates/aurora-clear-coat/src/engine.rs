/// Clear coat: thickness, gloss, scratch resist, UV
/// Phase 762

#[derive(Debug, Clone)]
pub struct ClearCoat {
    pub thickness_ok: bool,
    pub gloss_ok: bool,
    pub scratch_ok: bool,
    pub uv_ok: bool,
    pub adhesion_ok: bool,
}

impl Default for ClearCoat {
    fn default() -> Self {
        Self::new()
    }
}

impl ClearCoat {
    pub fn new() -> Self {
        Self {
            thickness_ok: true,
            gloss_ok: true,
            scratch_ok: true,
            uv_ok: true,
            adhesion_ok: true,
        }
    }

    pub fn appearance_ok(&self) -> bool {
        self.gloss_ok && self.scratch_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.uv_ok && self.adhesion_ok && self.thickness_ok
    }

    pub fn all_ok(&self) -> bool {
        self.appearance_ok() && self.protection_ok()
    }

    pub fn needs_refinish(&self) -> bool {
        !self.gloss_ok || !self.thickness_ok
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
    fn test_appearance() {
        let c = ClearCoat::new();
        assert!(c.appearance_ok());
    }

    #[test]
    fn test_protection() {
        let c = ClearCoat::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ClearCoat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refinish() {
        let c = ClearCoat::new();
        assert!(!c.needs_refinish());
    }

    #[test]
    fn test_thickness() {
        let mut c = ClearCoat::new();
        c.thickness_ok = false;
        assert!(c.needs_refinish());
    }

    #[test]
    fn test_health() {
        let c = ClearCoat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
