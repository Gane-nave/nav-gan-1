/// Roof panel: structure, drip rail, antenna mount
/// Phase 792

#[derive(Debug, Clone)]
pub struct RoofPanel {
    pub structure_ok: bool,
    pub drip_rail_ok: bool,
    pub antenna_ok: bool,
    pub seal_ok: bool,
    pub reinforcement_ok: bool,
}

impl Default for RoofPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl RoofPanel {
    pub fn new() -> Self {
        Self {
            structure_ok: true,
            drip_rail_ok: true,
            antenna_ok: true,
            seal_ok: true,
            reinforcement_ok: true,
        }
    }

    pub fn body_ok(&self) -> bool {
        self.structure_ok && self.reinforcement_ok
    }

    pub fn features_ok(&self) -> bool {
        self.drip_rail_ok && self.antenna_ok && self.seal_ok
    }

    pub fn all_ok(&self) -> bool {
        self.body_ok() && self.features_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.structure_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.structure_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body() {
        let c = RoofPanel::new();
        assert!(c.body_ok());
    }

    #[test]
    fn test_features() {
        let c = RoofPanel::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoofPanel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = RoofPanel::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_structure() {
        let mut c = RoofPanel::new();
        c.structure_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = RoofPanel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
