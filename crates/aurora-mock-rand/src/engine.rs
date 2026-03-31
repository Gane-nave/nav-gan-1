/// aurora-mock-rand: mock rand
/// Phase 2489

#[derive(Debug, Clone)]
pub struct MockRand {
    pub seed_ok: bool,
    pub next_ok: bool,
    pub range_ok: bool,
    pub bool_ok: bool,
    pub verify_ok: bool,
}

impl Default for MockRand {
    fn default() -> Self {
        Self::new()
    }
}

impl MockRand {
    pub fn new() -> Self {
        Self {
            seed_ok: true,
            next_ok: true,
            range_ok: true,
            bool_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.seed_ok && self.next_ok && self.range_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.bool_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.seed_ok || !self.next_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.seed_ok {
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
        let c = MockRand::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockRand::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockRand::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockRand::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockRand::new();
        c.seed_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockRand::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockRand::default();
        assert!(c.all_ok());
    }
}
