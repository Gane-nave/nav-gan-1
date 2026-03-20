/// ignition sys: charge, fire, advance, dwell, check
/// Phase 1233

#[derive(Debug, Clone)]
pub struct IgnitionSys {
    pub charge_ok: bool,
    pub fire_ok: bool,
    pub advance_ok: bool,
    pub dwell_ok: bool,
    pub check_ok: bool,
}

impl Default for IgnitionSys {
    fn default() -> Self {
        Self::new()
    }
}

impl IgnitionSys {
    pub fn new() -> Self {
        Self {
            charge_ok: true,
            fire_ok: true,
            advance_ok: true,
            dwell_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.charge_ok && self.fire_ok && self.advance_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dwell_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.charge_ok || !self.fire_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.charge_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = IgnitionSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IgnitionSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IgnitionSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IgnitionSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IgnitionSys::new();
        c.charge_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IgnitionSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
