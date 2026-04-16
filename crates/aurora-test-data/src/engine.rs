/// aurora-test-data: test data
/// Phase 2476

#[derive(Debug, Clone)]
pub struct TestData {
    pub generate_ok: bool,
    pub load_ok: bool,
    pub save_ok: bool,
    pub reset_ok: bool,
    pub validate_ok: bool,
}

impl Default for TestData {
    fn default() -> Self {
        Self::new()
    }
}

impl TestData {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            load_ok: true,
            save_ok: true,
            reset_ok: true,
            validate_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.load_ok && self.save_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.load_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = TestData::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestData::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestData::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestData::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestData::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestData::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TestData::default();
        assert!(c.all_ok());
    }
}
