/// db rocksdb: open, get, put, delete, log
/// Phase 1625

#[derive(Debug, Clone)]
pub struct DbRocksdb {
    pub open_ok: bool,
    pub get_ok: bool,
    pub put_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for DbRocksdb {
    fn default() -> Self {
        Self::new()
    }
}

impl DbRocksdb {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            get_ok: true,
            put_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.get_ok && self.put_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.get_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok {
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
        let c = DbRocksdb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DbRocksdb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DbRocksdb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DbRocksdb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DbRocksdb::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DbRocksdb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
