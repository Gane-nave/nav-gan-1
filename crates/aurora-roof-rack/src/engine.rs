/// Roof rack: crossbar, foot, clamp, load rating
/// Phase 848

#[derive(Debug, Clone)]
pub struct RoofRack {
    pub crossbar_ok: bool,
    pub foot_ok: bool,
    pub clamp_ok: bool,
    pub load_ok: bool,
    pub aero_ok: bool,
}

impl Default for RoofRack {
    fn default() -> Self {
        Self::new()
    }
}

impl RoofRack {
    pub fn new() -> Self {
        Self {
            crossbar_ok: true,
            foot_ok: true,
            clamp_ok: true,
            load_ok: true,
            aero_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.crossbar_ok && self.foot_ok && self.clamp_ok
    }

    pub fn performance_ok(&self) -> bool {
        self.load_ok && self.aero_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.performance_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.clamp_ok || !self.crossbar_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.clamp_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = RoofRack::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_performance() {
        let c = RoofRack::new();
        assert!(c.performance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoofRack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = RoofRack::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_clamp() {
        let mut c = RoofRack::new();
        c.clamp_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = RoofRack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
