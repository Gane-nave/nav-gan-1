/// Trunk lid: panel, hinge, strut, latch, seal
/// Phase 790

#[derive(Debug, Clone)]
pub struct TrunkLid {
    pub panel_ok: bool,
    pub hinge_ok: bool,
    pub strut_ok: bool,
    pub latch_ok: bool,
    pub seal_ok: bool,
}

impl Default for TrunkLid {
    fn default() -> Self {
        Self::new()
    }
}

impl TrunkLid {
    pub fn new() -> Self {
        Self {
            panel_ok: true,
            hinge_ok: true,
            strut_ok: true,
            latch_ok: true,
            seal_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.panel_ok && self.hinge_ok
    }

    pub fn mechanism_ok(&self) -> bool {
        self.strut_ok && self.latch_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.mechanism_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.latch_ok || !self.strut_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.latch_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = TrunkLid::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_mechanism() {
        let c = TrunkLid::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrunkLid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = TrunkLid::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_latch() {
        let mut c = TrunkLid::new();
        c.latch_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = TrunkLid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
