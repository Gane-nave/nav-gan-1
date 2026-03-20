/// flywheel: store, release, balance, damp, check
/// Phase 1223

#[derive(Debug, Clone)]
pub struct Flywheel {
    pub store_ok: bool,
    pub release_ok: bool,
    pub balance_ok: bool,
    pub damp_ok: bool,
    pub check_ok: bool,
}

impl Default for Flywheel {
    fn default() -> Self {
        Self::new()
    }
}

impl Flywheel {
    pub fn new() -> Self {
        Self {
            store_ok: true,
            release_ok: true,
            balance_ok: true,
            damp_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.store_ok && self.release_ok && self.balance_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.damp_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.store_ok || !self.release_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.store_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = Flywheel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Flywheel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Flywheel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Flywheel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Flywheel::new();
        c.store_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Flywheel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
