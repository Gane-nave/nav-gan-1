/// test factory: define, build, create, sequence, log
/// Phase 2111

#[derive(Debug, Clone)]
pub struct TestFactory {
    pub define_ok: bool,
    pub build_ok: bool,
    pub create_ok: bool,
    pub sequence_ok: bool,
    pub log_ok: bool,
}

impl Default for TestFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl TestFactory {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            build_ok: true,
            create_ok: true,
            sequence_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.build_ok && self.create_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sequence_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.build_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = TestFactory::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestFactory::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestFactory::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestFactory::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestFactory::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestFactory::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
