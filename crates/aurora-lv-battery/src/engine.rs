/// LV battery: cranking, reserve, CCA, state
/// Phase 723

#[derive(Debug, Clone)]
pub struct LvBattery {
    pub cranking_ok: bool,
    pub reserve_ok: bool,
    pub cca_ok: bool,
    pub state_ok: bool,
    pub terminals_ok: bool,
}

impl Default for LvBattery {
    fn default() -> Self {
        Self::new()
    }
}

impl LvBattery {
    pub fn new() -> Self {
        Self {
            cranking_ok: true,
            reserve_ok: true,
            cca_ok: true,
            state_ok: true,
            terminals_ok: true,
        }
    }

    pub fn starting_ok(&self) -> bool {
        self.cranking_ok && self.cca_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.reserve_ok && self.state_ok && self.terminals_ok
    }

    pub fn all_ok(&self) -> bool {
        self.starting_ok() && self.condition_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.cranking_ok || !self.cca_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cranking_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting() {
        let c = LvBattery::new();
        assert!(c.starting_ok());
    }

    #[test]
    fn test_condition() {
        let c = LvBattery::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LvBattery::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = LvBattery::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_cranking() {
        let mut c = LvBattery::new();
        c.cranking_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = LvBattery::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
