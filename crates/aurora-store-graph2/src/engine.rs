/// store graph2: add, query, update, remove, log
/// Phase 1975

#[derive(Debug, Clone)]
pub struct StoreGraph2 {
    pub add_ok: bool,
    pub query_ok: bool,
    pub update_ok: bool,
    pub remove_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreGraph2 {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreGraph2 {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            query_ok: true,
            update_ok: true,
            remove_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.query_ok && self.update_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.remove_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.query_ok
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
        let c = StoreGraph2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreGraph2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreGraph2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreGraph2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreGraph2::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreGraph2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
