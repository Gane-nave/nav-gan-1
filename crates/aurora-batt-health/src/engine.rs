/// batt health: capacity, resistance, cycle, predict, log
/// Phase 1358

#[derive(Debug, Clone)]
pub struct BattHealth {
    pub capacity_ok: bool,
    pub resistance_ok: bool,
    pub cycle_ok: bool,
    pub predict_ok: bool,
    pub log_ok: bool,
}

impl Default for BattHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl BattHealth {
    pub fn new() -> Self {
        Self {
            capacity_ok: true,
            resistance_ok: true,
            cycle_ok: true,
            predict_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capacity_ok && self.resistance_ok && self.cycle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.predict_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capacity_ok || !self.resistance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capacity_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BattHealth::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BattHealth::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BattHealth::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BattHealth::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BattHealth::new();
        c.capacity_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BattHealth::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
