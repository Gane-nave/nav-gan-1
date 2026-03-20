/// Carpet and floor mat: fit, anchor, wear, clean
/// Phase 775

#[derive(Debug, Clone)]
pub struct CarpetMat {
    pub fit_ok: bool,
    pub anchor_ok: bool,
    pub wear_ok: bool,
    pub clean_ok: bool,
    pub backing_ok: bool,
}

impl Default for CarpetMat {
    fn default() -> Self {
        Self::new()
    }
}

impl CarpetMat {
    pub fn new() -> Self {
        Self {
            fit_ok: true,
            anchor_ok: true,
            wear_ok: true,
            clean_ok: true,
            backing_ok: true,
        }
    }

    pub fn placement_ok(&self) -> bool {
        self.fit_ok && self.anchor_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.wear_ok && self.clean_ok && self.backing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.placement_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.wear_ok || !self.anchor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wear_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placement() {
        let c = CarpetMat::new();
        assert!(c.placement_ok());
    }

    #[test]
    fn test_condition() {
        let c = CarpetMat::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CarpetMat::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CarpetMat::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_wear() {
        let mut c = CarpetMat::new();
        c.wear_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CarpetMat::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
