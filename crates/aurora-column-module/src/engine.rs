/// column module: stalk, switch, cancel, cruise, check
/// Phase 1277

#[derive(Debug, Clone)]
pub struct ColumnModule {
    pub stalk_ok: bool,
    pub switch_ok: bool,
    pub cancel_ok: bool,
    pub cruise_ok: bool,
    pub check_ok: bool,
}

impl Default for ColumnModule {
    fn default() -> Self {
        Self::new()
    }
}

impl ColumnModule {
    pub fn new() -> Self {
        Self {
            stalk_ok: true,
            switch_ok: true,
            cancel_ok: true,
            cruise_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.stalk_ok && self.switch_ok && self.cancel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cruise_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.stalk_ok || !self.switch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.stalk_ok {
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
        let c = ColumnModule::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ColumnModule::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ColumnModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ColumnModule::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ColumnModule::new();
        c.stalk_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ColumnModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
