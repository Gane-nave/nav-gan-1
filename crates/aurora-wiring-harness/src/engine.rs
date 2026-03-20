/// Wiring harness: connector, insulation, routing, shield
/// Phase 726

#[derive(Debug, Clone)]
pub struct WiringHarness {
    pub connector_ok: bool,
    pub insulation_ok: bool,
    pub routing_ok: bool,
    pub shield_ok: bool,
    pub continuity_ok: bool,
}

impl Default for WiringHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl WiringHarness {
    pub fn new() -> Self {
        Self {
            connector_ok: true,
            insulation_ok: true,
            routing_ok: true,
            shield_ok: true,
            continuity_ok: true,
        }
    }

    pub fn electrical_ok(&self) -> bool {
        self.connector_ok && self.continuity_ok
    }

    pub fn physical_ok(&self) -> bool {
        self.insulation_ok && self.routing_ok && self.shield_ok
    }

    pub fn all_ok(&self) -> bool {
        self.electrical_ok() && self.physical_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.connector_ok || !self.insulation_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.continuity_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrical() {
        let c = WiringHarness::new();
        assert!(c.electrical_ok());
    }

    #[test]
    fn test_physical() {
        let c = WiringHarness::new();
        assert!(c.physical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WiringHarness::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = WiringHarness::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_connector() {
        let mut c = WiringHarness::new();
        c.connector_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = WiringHarness::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
