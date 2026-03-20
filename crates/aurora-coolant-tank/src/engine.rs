/// coolant tank: fill, level, degas, overflow, check
/// Phase 1250

#[derive(Debug, Clone)]
pub struct CoolantTank {
    pub fill_ok: bool,
    pub level_ok: bool,
    pub degas_ok: bool,
    pub overflow_ok: bool,
    pub check_ok: bool,
}

impl Default for CoolantTank {
    fn default() -> Self {
        Self::new()
    }
}

impl CoolantTank {
    pub fn new() -> Self {
        Self {
            fill_ok: true,
            level_ok: true,
            degas_ok: true,
            overflow_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fill_ok && self.level_ok && self.degas_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.overflow_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fill_ok || !self.level_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fill_ok {
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
        let c = CoolantTank::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CoolantTank::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CoolantTank::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CoolantTank::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CoolantTank::new();
        c.fill_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CoolantTank::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
