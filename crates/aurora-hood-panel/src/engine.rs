/// Hood panel: latch, hinge, strut, insulation
/// Phase 785

#[derive(Debug, Clone)]
pub struct HoodPanel {
    pub latch_ok: bool,
    pub hinge_ok: bool,
    pub strut_ok: bool,
    pub insulation_ok: bool,
    pub alignment_ok: bool,
}

impl Default for HoodPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl HoodPanel {
    pub fn new() -> Self {
        Self {
            latch_ok: true,
            hinge_ok: true,
            strut_ok: true,
            insulation_ok: true,
            alignment_ok: true,
        }
    }

    pub fn mechanism_ok(&self) -> bool {
        self.latch_ok && self.hinge_ok && self.strut_ok
    }

    pub fn fit_ok(&self) -> bool {
        self.insulation_ok && self.alignment_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanism_ok() && self.fit_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.latch_ok || !self.strut_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.latch_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanism() {
        let c = HoodPanel::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_fit() {
        let c = HoodPanel::new();
        assert!(c.fit_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HoodPanel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HoodPanel::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_latch() {
        let mut c = HoodPanel::new();
        c.latch_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HoodPanel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
