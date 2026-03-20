/// throttle body: open, close, idle, adapt, check
/// Phase 1231

#[derive(Debug, Clone)]
pub struct ThrottleBody {
    pub open_ok: bool,
    pub close_ok: bool,
    pub idle_ok: bool,
    pub adapt_ok: bool,
    pub check_ok: bool,
}

impl Default for ThrottleBody {
    fn default() -> Self {
        Self::new()
    }
}

impl ThrottleBody {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            close_ok: true,
            idle_ok: true,
            adapt_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.close_ok && self.idle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.close_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ThrottleBody::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ThrottleBody::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ThrottleBody::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ThrottleBody::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ThrottleBody::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ThrottleBody::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
