/// aurora-mock-fs: mock fs
/// Phase 2487

#[derive(Debug, Clone)]
pub struct MockFs {
    pub create_ok: bool,
    pub read_ok: bool,
    pub write_ok: bool,
    pub delete_ok: bool,
    pub verify_ok: bool,
}

impl Default for MockFs {
    fn default() -> Self {
        Self::new()
    }
}

impl MockFs {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            read_ok: true,
            write_ok: true,
            delete_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.read_ok && self.write_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = MockFs::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockFs::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockFs::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockFs::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockFs::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockFs::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockFs::default();
        assert!(c.all_ok());
    }
}
