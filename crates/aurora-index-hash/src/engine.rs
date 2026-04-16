/// index hash: insert, lookup, delete, rehash, log
/// Phase 1891

#[derive(Debug, Clone)]
pub struct IndexHash {
    pub insert_ok: bool,
    pub lookup_ok: bool,
    pub delete_ok: bool,
    pub rehash_ok: bool,
    pub log_ok: bool,
}

impl Default for IndexHash {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexHash {
    pub fn new() -> Self {
        Self {
            insert_ok: true,
            lookup_ok: true,
            delete_ok: true,
            rehash_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.insert_ok && self.lookup_ok && self.delete_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.rehash_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.insert_ok || !self.lookup_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.insert_ok {
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
        let c = IndexHash::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IndexHash::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IndexHash::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IndexHash::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IndexHash::new();
        c.insert_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IndexHash::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
