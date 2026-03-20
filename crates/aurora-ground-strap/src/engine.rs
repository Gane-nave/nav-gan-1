/// Ground strap: resistance, corrosion, routing, terminal
/// Phase 689

#[derive(Debug, Clone)]
pub struct GroundStrap {
    pub resistance_ok: bool,
    pub corrosion_free: bool,
    pub routing_ok: bool,
    pub terminal_ok: bool,
    pub tight: bool,
}

impl Default for GroundStrap {
    fn default() -> Self {
        Self::new()
    }
}

impl GroundStrap {
    pub fn new() -> Self {
        Self {
            resistance_ok: true,
            corrosion_free: true,
            routing_ok: true,
            terminal_ok: true,
            tight: true,
        }
    }

    pub fn electrical_ok(&self) -> bool {
        self.resistance_ok && self.corrosion_free
    }

    pub fn physical_ok(&self) -> bool {
        self.routing_ok && self.terminal_ok && self.tight
    }

    pub fn all_ok(&self) -> bool {
        self.electrical_ok() && self.physical_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.resistance_ok || !self.corrosion_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.resistance_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrical() {
        let c = GroundStrap::new();
        assert!(c.electrical_ok());
    }

    #[test]
    fn test_physical() {
        let c = GroundStrap::new();
        assert!(c.physical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GroundStrap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = GroundStrap::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_resistance() {
        let mut c = GroundStrap::new();
        c.resistance_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = GroundStrap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
