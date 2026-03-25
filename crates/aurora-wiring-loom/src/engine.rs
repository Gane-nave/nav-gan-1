/// Wiring loom: harness integrity, insulation, connectors
/// Phase 522

#[derive(Debug, Clone)]
pub struct WiringLoom {
    pub total_circuits: u32,
    pub damaged_count: u32,
    pub insulation_ok: bool,
    pub connectors_ok: bool,
    pub chafe_free: bool,
}

impl Default for WiringLoom {
    fn default() -> Self {
        Self::new()
    }
}

impl WiringLoom {
    pub fn new() -> Self {
        Self {
            total_circuits: 200,
            damaged_count: 0,
            insulation_ok: true,
            connectors_ok: true,
            chafe_free: true,
        }
    }

    pub fn all_circuits_ok(&self) -> bool {
        self.damaged_count == 0
    }

    pub fn insulation_good(&self) -> bool {
        self.insulation_ok && self.chafe_free
    }

    pub fn all_ok(&self) -> bool {
        self.all_circuits_ok() && self.insulation_good() && self.connectors_ok
    }

    pub fn needs_repair(&self) -> bool {
        self.damaged_count > 0 || !self.insulation_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.damaged_count > 0 {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuits() {
        let c = WiringLoom::new();
        assert!(c.all_circuits_ok());
    }

    #[test]
    fn test_insulation() {
        let c = WiringLoom::new();
        assert!(c.insulation_good());
    }

    #[test]
    fn test_all_ok() {
        let c = WiringLoom::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = WiringLoom::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_damaged() {
        let mut c = WiringLoom::new();
        c.damaged_count = 3;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = WiringLoom::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
