/// index inverted: add, search, remove, merge, log
/// Phase 1893

#[derive(Debug, Clone)]
pub struct IndexInverted {
    pub add_ok: bool,
    pub search_ok: bool,
    pub remove_ok: bool,
    pub merge_ok: bool,
    pub log_ok: bool,
}

impl Default for IndexInverted {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexInverted {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            search_ok: true,
            remove_ok: true,
            merge_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.search_ok && self.remove_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.merge_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.search_ok
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
        let c = IndexInverted::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IndexInverted::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IndexInverted::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IndexInverted::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IndexInverted::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IndexInverted::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
