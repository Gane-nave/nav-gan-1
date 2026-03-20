/// Clear coat: UV protection, gloss, scratch resistance, ceramic coating
/// Phase 389

#[derive(Debug, Clone)]
pub struct ClearCoat {
    pub thickness_um: f64,
    pub gloss_gu: f64,
    pub scratch_resistant: bool,
    pub uv_protection_ok: bool,
    pub ceramic_applied: bool,
}

impl Default for ClearCoat {
    fn default() -> Self {
        Self::new()
    }
}

impl ClearCoat {
    pub fn new() -> Self {
        Self {
            thickness_um: 45.0,
            gloss_gu: 90.0,
            scratch_resistant: true,
            uv_protection_ok: true,
            ceramic_applied: false,
        }
    }

    pub fn gloss_ok(&self) -> bool {
        self.gloss_gu > 70.0
    }

    pub fn showroom_quality(&self) -> bool {
        self.gloss_gu > 85.0 && self.scratch_resistant
    }

    pub fn needs_polish(&self) -> bool {
        self.gloss_gu < 60.0
    }

    pub fn protected(&self) -> bool {
        self.uv_protection_ok && self.thickness_um > 30.0
    }

    pub fn health_score(&self) -> f64 {
        if self.needs_polish() {
            return 30.0;
        }
        if !self.uv_protection_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gloss() {
        let c = ClearCoat::new();
        assert!(c.gloss_ok());
    }

    #[test]
    fn test_showroom() {
        let c = ClearCoat::new();
        assert!(c.showroom_quality());
    }

    #[test]
    fn test_no_polish() {
        let c = ClearCoat::new();
        assert!(!c.needs_polish());
    }

    #[test]
    fn test_protected() {
        let c = ClearCoat::new();
        assert!(c.protected());
    }

    #[test]
    fn test_dull() {
        let mut c = ClearCoat::new();
        c.gloss_gu = 50.0;
        assert!(c.needs_polish());
    }

    #[test]
    fn test_health() {
        let c = ClearCoat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
