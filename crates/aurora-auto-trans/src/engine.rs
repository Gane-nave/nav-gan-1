/// auto trans: shift, lock, unlock, adapt, report
/// Phase 1212

#[derive(Debug, Clone)]
pub struct AutoTrans {
    pub shift_ok: bool,
    pub lock_ok: bool,
    pub unlock_ok: bool,
    pub adapt_ok: bool,
    pub report_ok: bool,
}

impl Default for AutoTrans {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoTrans {
    pub fn new() -> Self {
        Self {
            shift_ok: true,
            lock_ok: true,
            unlock_ok: true,
            adapt_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.shift_ok && self.lock_ok && self.unlock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.shift_ok || !self.lock_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.shift_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = AutoTrans::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AutoTrans::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AutoTrans::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AutoTrans::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AutoTrans::new();
        c.shift_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AutoTrans::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
