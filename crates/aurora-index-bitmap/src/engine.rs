/// index bitmap: set, test, clear, count, log
/// Phase 1892

#[derive(Debug, Clone)]
pub struct IndexBitmap {
    pub set_ok: bool,
    pub test_ok: bool,
    pub clear_ok: bool,
    pub count_ok: bool,
    pub log_ok: bool,
}

impl Default for IndexBitmap {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexBitmap {
    pub fn new() -> Self {
        Self {
            set_ok: true,
            test_ok: true,
            clear_ok: true,
            count_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.set_ok && self.test_ok && self.clear_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.count_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.set_ok || !self.test_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.set_ok {
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
        let c = IndexBitmap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IndexBitmap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IndexBitmap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IndexBitmap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IndexBitmap::new();
        c.set_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IndexBitmap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
