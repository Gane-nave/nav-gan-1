/// Rim check: runout, cracks, curb damage, balance
/// Phase 547

#[derive(Debug, Clone)]
pub struct RimCheck {
    pub runout_mm: f64,
    pub max_runout_mm: f64,
    pub cracked: bool,
    pub curb_damaged: bool,
    pub balanced: bool,
}

impl Default for RimCheck {
    fn default() -> Self {
        Self::new()
    }
}

impl RimCheck {
    pub fn new() -> Self {
        Self {
            runout_mm: 0.3,
            max_runout_mm: 1.0,
            cracked: false,
            curb_damaged: false,
            balanced: true,
        }
    }

    pub fn runout_ok(&self) -> bool {
        self.runout_mm < self.max_runout_mm
    }

    pub fn structural_ok(&self) -> bool {
        !self.cracked && !self.curb_damaged
    }

    pub fn all_ok(&self) -> bool {
        self.runout_ok() && self.structural_ok() && self.balanced
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runout() {
        let c = RimCheck::new();
        assert!(c.runout_ok());
    }

    #[test]
    fn test_structural() {
        let c = RimCheck::new();
        assert!(c.structural_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RimCheck::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = RimCheck::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_cracked() {
        let mut c = RimCheck::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = RimCheck::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
