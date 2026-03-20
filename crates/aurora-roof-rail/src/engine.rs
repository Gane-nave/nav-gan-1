/// Roof rail: crossbar, clamp, load rating, aero
/// Phase 758

#[derive(Debug, Clone)]
pub struct RoofRail {
    pub crossbar_ok: bool,
    pub clamp_ok: bool,
    pub load_ok: bool,
    pub aero_ok: bool,
    pub seal_ok: bool,
}

impl Default for RoofRail {
    fn default() -> Self {
        Self::new()
    }
}

impl RoofRail {
    pub fn new() -> Self {
        Self {
            crossbar_ok: true,
            clamp_ok: true,
            load_ok: true,
            aero_ok: true,
            seal_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.crossbar_ok && self.clamp_ok && self.load_ok
    }

    pub fn finish_ok(&self) -> bool {
        self.aero_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.finish_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.crossbar_ok || !self.clamp_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.crossbar_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = RoofRail::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_finish() {
        let c = RoofRail::new();
        assert!(c.finish_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoofRail::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = RoofRail::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_crossbar() {
        let mut c = RoofRail::new();
        c.crossbar_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = RoofRail::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
