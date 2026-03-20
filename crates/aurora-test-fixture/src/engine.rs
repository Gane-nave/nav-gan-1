/// test fixture: create, populate, isolate, cleanup, log
/// Phase 1529

#[derive(Debug, Clone)]
pub struct TestFixture {
    pub create_ok: bool,
    pub populate_ok: bool,
    pub isolate_ok: bool,
    pub cleanup_ok: bool,
    pub log_ok: bool,
}

impl Default for TestFixture {
    fn default() -> Self {
        Self::new()
    }
}

impl TestFixture {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            populate_ok: true,
            isolate_ok: true,
            cleanup_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.populate_ok && self.isolate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cleanup_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.populate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TestFixture::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestFixture::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestFixture::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestFixture::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestFixture::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestFixture::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
