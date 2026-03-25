/// turbo boost: spool, regulate, wastegate, surge, check
/// Phase 1236

#[derive(Debug, Clone)]
pub struct TurboBoost {
    pub spool_ok: bool,
    pub regulate_ok: bool,
    pub wastegate_ok: bool,
    pub surge_ok: bool,
    pub check_ok: bool,
}

impl Default for TurboBoost {
    fn default() -> Self {
        Self::new()
    }
}

impl TurboBoost {
    pub fn new() -> Self {
        Self {
            spool_ok: true,
            regulate_ok: true,
            wastegate_ok: true,
            surge_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.spool_ok && self.regulate_ok && self.wastegate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.surge_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.spool_ok || !self.regulate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spool_ok {
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
        let c = TurboBoost::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TurboBoost::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TurboBoost::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TurboBoost::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TurboBoost::new();
        c.spool_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TurboBoost::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
