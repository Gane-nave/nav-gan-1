/// trans fluid: level, temp, pressure, filter, check
/// Phase 1253

#[derive(Debug, Clone)]
pub struct TransFluid {
    pub level_ok: bool,
    pub temp_ok: bool,
    pub pressure_ok: bool,
    pub filter_ok: bool,
    pub check_ok: bool,
}

impl Default for TransFluid {
    fn default() -> Self {
        Self::new()
    }
}

impl TransFluid {
    pub fn new() -> Self {
        Self {
            level_ok: true,
            temp_ok: true,
            pressure_ok: true,
            filter_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.level_ok && self.temp_ok && self.pressure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.filter_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.level_ok || !self.temp_ok
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
        let c = TransFluid::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransFluid::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransFluid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransFluid::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransFluid::new();
        c.level_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransFluid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
