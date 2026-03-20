/// power fluid: level, pressure, filter, cool, check
/// Phase 1255

#[derive(Debug, Clone)]
pub struct PowerFluid {
    pub level_ok: bool,
    pub pressure_ok: bool,
    pub filter_ok: bool,
    pub cool_ok: bool,
    pub check_ok: bool,
}

impl Default for PowerFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerFluid {
    pub fn new() -> Self {
        Self {
            level_ok: true,
            pressure_ok: true,
            filter_ok: true,
            cool_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.level_ok && self.pressure_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cool_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.level_ok || !self.pressure_ok
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
        let c = PowerFluid::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PowerFluid::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PowerFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PowerFluid::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PowerFluid::new();
        c.level_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PowerFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
