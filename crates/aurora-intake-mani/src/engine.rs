/// intake mani: flow, swirl, tumble, tune, check
/// Phase 1230

#[derive(Debug, Clone)]
pub struct IntakeMani {
    pub flow_ok: bool,
    pub swirl_ok: bool,
    pub tumble_ok: bool,
    pub tune_ok: bool,
    pub check_ok: bool,
}

impl Default for IntakeMani {
    fn default() -> Self {
        Self::new()
    }
}

impl IntakeMani {
    pub fn new() -> Self {
        Self {
            flow_ok: true,
            swirl_ok: true,
            tumble_ok: true,
            tune_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.flow_ok && self.swirl_ok && self.tumble_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tune_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.flow_ok || !self.swirl_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flow_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = IntakeMani::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntakeMani::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntakeMani::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntakeMani::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntakeMani::new();
        c.flow_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntakeMani::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
