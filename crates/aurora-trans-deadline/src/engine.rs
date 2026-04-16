/// trans deadline: set, propagate, check, expire, log
/// Phase 2290

#[derive(Debug, Clone)]
pub struct TransDeadline {
    pub set_ok: bool,
    pub propagate_ok: bool,
    pub check_ok: bool,
    pub expire_ok: bool,
    pub log_ok: bool,
}

impl Default for TransDeadline {
    fn default() -> Self {
        Self::new()
    }
}

impl TransDeadline {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            propagate_ok: true,
            check_ok: true,
            expire_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.propagate_ok && self.check_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.expire_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.propagate_ok
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
        let c = TransDeadline::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransDeadline::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransDeadline::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransDeadline::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransDeadline::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransDeadline::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
