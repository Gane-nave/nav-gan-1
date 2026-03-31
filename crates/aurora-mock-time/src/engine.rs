/// aurora-mock-time: mock time
/// Phase 2488

#[derive(Debug, Clone)]
pub struct MockTime {
    pub freeze_ok: bool,
    pub advance_ok: bool,
    pub reset_ok: bool,
    pub tick_ok: bool,
    pub verify_ok: bool,
}

impl Default for MockTime {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTime {
    pub fn new() -> Self {
        Self {
            freeze_ok: true,
            advance_ok: true,
            reset_ok: true,
            tick_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.freeze_ok && self.advance_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tick_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.freeze_ok || !self.advance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.freeze_ok {
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
        let c = MockTime::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockTime::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockTime::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockTime::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockTime::new();
        c.freeze_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockTime::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockTime::default();
        assert!(c.all_ok());
    }
}
