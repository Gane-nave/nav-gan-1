/// Ceramic coating: hardness, hydrophobic, gloss, durability
/// Phase 771

#[derive(Debug, Clone)]
pub struct CeramicCoat {
    pub hardness_ok: bool,
    pub hydrophobic_ok: bool,
    pub gloss_ok: bool,
    pub durability_ok: bool,
    pub cured: bool,
}

impl Default for CeramicCoat {
    fn default() -> Self {
        Self::new()
    }
}

impl CeramicCoat {
    pub fn new() -> Self {
        Self {
            hardness_ok: true,
            hydrophobic_ok: true,
            gloss_ok: true,
            durability_ok: true,
            cured: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.hardness_ok && self.hydrophobic_ok
    }

    pub fn finish_ok(&self) -> bool {
        self.gloss_ok && self.durability_ok && self.cured
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.finish_ok()
    }

    pub fn needs_reapply(&self) -> bool {
        !self.hydrophobic_ok || !self.durability_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hydrophobic_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = CeramicCoat::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_finish() {
        let c = CeramicCoat::new();
        assert!(c.finish_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CeramicCoat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reapply() {
        let c = CeramicCoat::new();
        assert!(!c.needs_reapply());
    }

    #[test]
    fn test_hydro() {
        let mut c = CeramicCoat::new();
        c.hydrophobic_ok = false;
        assert!(c.needs_reapply());
    }

    #[test]
    fn test_health() {
        let c = CeramicCoat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
