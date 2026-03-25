/// HV contactor: main, precharge, auxiliary, weld detect
/// Phase 859

#[derive(Debug, Clone)]
pub struct HvContactor {
    pub main_ok: bool,
    pub precharge_ok: bool,
    pub aux_ok: bool,
    pub weld_ok: bool,
    pub cycle_ok: bool,
}

impl Default for HvContactor {
    fn default() -> Self {
        Self::new()
    }
}

impl HvContactor {
    pub fn new() -> Self {
        Self {
            main_ok: true,
            precharge_ok: true,
            aux_ok: true,
            weld_ok: true,
            cycle_ok: true,
        }
    }

    pub fn switching_ok(&self) -> bool {
        self.main_ok && self.precharge_ok && self.aux_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.weld_ok && self.cycle_ok
    }

    pub fn all_ok(&self) -> bool {
        self.switching_ok() && self.safety_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.main_ok || !self.weld_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.main_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switching() {
        let c = HvContactor::new();
        assert!(c.switching_ok());
    }

    #[test]
    fn test_safety() {
        let c = HvContactor::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HvContactor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = HvContactor::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_main() {
        let mut c = HvContactor::new();
        c.main_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = HvContactor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
