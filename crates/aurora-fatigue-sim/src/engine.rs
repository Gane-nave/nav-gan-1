/// fatigue sim: load, cycle, crack, predict, log
/// Phase 1402

#[derive(Debug, Clone)]
pub struct FatigueSim {
    pub load_ok: bool,
    pub cycle_ok: bool,
    pub crack_ok: bool,
    pub predict_ok: bool,
    pub log_ok: bool,
}

impl Default for FatigueSim {
    fn default() -> Self {
        Self::new()
    }
}

impl FatigueSim {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            cycle_ok: true,
            crack_ok: true,
            predict_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.cycle_ok && self.crack_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.predict_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.cycle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FatigueSim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FatigueSim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FatigueSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FatigueSim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FatigueSim::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FatigueSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
