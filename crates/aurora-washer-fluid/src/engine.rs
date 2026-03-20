/// washer fluid: level, pump, heat, spray, check
/// Phase 1256

#[derive(Debug, Clone)]
pub struct WasherFluid {
    pub level_ok: bool,
    pub pump_ok: bool,
    pub heat_ok: bool,
    pub spray_ok: bool,
    pub check_ok: bool,
}

impl Default for WasherFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl WasherFluid {
    pub fn new() -> Self {
        Self {
            level_ok: true,
            pump_ok: true,
            heat_ok: true,
            spray_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.level_ok && self.pump_ok && self.heat_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.spray_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.level_ok || !self.pump_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.level_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = WasherFluid::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WasherFluid::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WasherFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WasherFluid::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WasherFluid::new();
        c.level_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WasherFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
