/// brake fluid: level, moisture, boil, bleed, check
/// Phase 1254

#[derive(Debug, Clone)]
pub struct BrakeFluid {
    pub level_ok: bool,
    pub moisture_ok: bool,
    pub boil_ok: bool,
    pub bleed_ok: bool,
    pub check_ok: bool,
}

impl Default for BrakeFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeFluid {
    pub fn new() -> Self {
        Self {
            level_ok: true,
            moisture_ok: true,
            boil_ok: true,
            bleed_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.level_ok && self.moisture_ok && self.boil_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.bleed_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.level_ok || !self.moisture_ok
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
        let c = BrakeFluid::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BrakeFluid::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BrakeFluid::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BrakeFluid::new();
        c.level_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BrakeFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
