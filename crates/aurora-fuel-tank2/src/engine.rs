/// fuel tank: level, vapor, vent, seal, check
/// Phase 1259

#[derive(Debug, Clone)]
pub struct FuelTank2 {
    pub level_ok: bool,
    pub vapor_ok: bool,
    pub vent_ok: bool,
    pub seal_ok: bool,
    pub check_ok: bool,
}

impl Default for FuelTank2 {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelTank2 {
    pub fn new() -> Self {
        Self {
            level_ok: true,
            vapor_ok: true,
            vent_ok: true,
            seal_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.level_ok && self.vapor_ok && self.vent_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.seal_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.level_ok || !self.vapor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.level_ok {
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
        let c = FuelTank2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuelTank2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelTank2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuelTank2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuelTank2::new();
        c.level_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuelTank2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
