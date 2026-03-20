/// trans timeout2: set, check, cancel, cascade, log
/// Phase 2289

#[derive(Debug, Clone)]
pub struct TransTimeout2 {
    pub set_ok: bool,
    pub check_ok: bool,
    pub cancel_ok: bool,
    pub cascade_ok: bool,
    pub log_ok: bool,
}

impl Default for TransTimeout2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransTimeout2 {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            check_ok: true,
            cancel_ok: true,
            cascade_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.check_ok && self.cancel_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cascade_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.check_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
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
        let c = TransTimeout2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransTimeout2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransTimeout2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransTimeout2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransTimeout2::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransTimeout2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
