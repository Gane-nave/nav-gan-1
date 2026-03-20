/// index bloom2: add, test, clear, estimate, log
/// Phase 1895

#[derive(Debug, Clone)]
pub struct IndexBloom2 {
    pub add_ok: bool,
    pub test_ok: bool,
    pub clear_ok: bool,
    pub estimate_ok: bool,
    pub log_ok: bool,
}

impl Default for IndexBloom2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexBloom2 {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            test_ok: true,
            clear_ok: true,
            estimate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.test_ok && self.clear_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.estimate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.test_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.add_ok {
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
        let c = IndexBloom2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IndexBloom2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IndexBloom2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IndexBloom2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IndexBloom2::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IndexBloom2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
