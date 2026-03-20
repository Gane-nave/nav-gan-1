/// durability sim: load, stress, fatigue, predict, log
/// Phase 1409

#[derive(Debug, Clone)]
pub struct DurabilitySim {
    pub load_ok: bool,
    pub stress_ok: bool,
    pub fatigue_ok: bool,
    pub predict_ok: bool,
    pub log_ok: bool,
}

impl Default for DurabilitySim {
    fn default() -> Self {
        Self::new()
    }
}

impl DurabilitySim {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            stress_ok: true,
            fatigue_ok: true,
            predict_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.stress_ok && self.fatigue_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.predict_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.stress_ok
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
        let c = DurabilitySim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DurabilitySim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DurabilitySim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DurabilitySim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DurabilitySim::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DurabilitySim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
