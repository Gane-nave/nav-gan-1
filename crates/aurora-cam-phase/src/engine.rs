/// cam phase: advance, retard, hold, adapt, check
/// Phase 1235

#[derive(Debug, Clone)]
pub struct CamPhase {
    pub advance_ok: bool,
    pub retard_ok: bool,
    pub hold_ok: bool,
    pub adapt_ok: bool,
    pub check_ok: bool,
}

impl Default for CamPhase {
    fn default() -> Self {
        Self::new()
    }
}

impl CamPhase {
    pub fn new() -> Self {
        Self {
            advance_ok: true,
            retard_ok: true,
            hold_ok: true,
            adapt_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.advance_ok && self.retard_ok && self.hold_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.advance_ok || !self.retard_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.advance_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CamPhase::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CamPhase::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CamPhase::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CamPhase::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CamPhase::new();
        c.advance_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CamPhase::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
