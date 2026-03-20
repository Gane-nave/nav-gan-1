/// hvac ctrl: heat, cool, vent, dehumidify, recirculate
/// Phase 1151

#[derive(Debug, Clone)]
pub struct HvacCtrl {
    pub heat_ok: bool,
    pub cool_ok: bool,
    pub vent_ok: bool,
    pub dehumidify_ok: bool,
    pub recirculate_ok: bool,
}

impl Default for HvacCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl HvacCtrl {
    pub fn new() -> Self {
        Self {
            heat_ok: true,
            cool_ok: true,
            vent_ok: true,
            dehumidify_ok: true,
            recirculate_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.heat_ok && self.cool_ok && self.vent_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dehumidify_ok && self.recirculate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.heat_ok || !self.cool_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.heat_ok {
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
        let c = HvacCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HvacCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HvacCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HvacCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HvacCtrl::new();
        c.heat_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HvacCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
