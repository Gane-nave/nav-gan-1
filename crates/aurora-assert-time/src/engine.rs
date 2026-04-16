/// aurora-assert-time: assert time
/// Phase 2503

#[derive(Debug, Clone)]
pub struct AssertTime {
    pub before_ok: bool,
    pub after_ok: bool,
    pub within_ok: bool,
    pub sequence_ok: bool,
    pub interval_ok: bool,
}

impl Default for AssertTime {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertTime {
    pub fn new() -> Self {
        Self {
            before_ok: true,
            after_ok: true,
            within_ok: true,
            sequence_ok: true,
            interval_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.before_ok && self.after_ok && self.within_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sequence_ok && self.interval_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.before_ok || !self.after_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.before_ok {
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
        let c = AssertTime::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertTime::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertTime::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertTime::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertTime::new();
        c.before_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertTime::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertTime::default();
        assert!(c.all_ok());
    }
}
