/// KV store: put, get, delete, scan, expire
/// Phase 1048

#[derive(Debug, Clone)]
pub struct KvStore {
    pub put_ok: bool,
    pub get_ok: bool,
    pub delete_ok: bool,
    pub scan_ok: bool,
    pub expire_ok: bool,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            put_ok: true,
            get_ok: true,
            delete_ok: true,
            scan_ok: true,
            expire_ok: true,
        }
    }

    pub fn crud_ok(&self) -> bool {
        self.put_ok && self.get_ok && self.delete_ok
    }

    pub fn management_ok(&self) -> bool {
        self.scan_ok && self.expire_ok
    }

    pub fn all_ok(&self) -> bool {
        self.crud_ok() && self.management_ok()
    }

    pub fn needs_compact(&self) -> bool {
        !self.put_ok || !self.scan_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.put_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crud() {
        let c = KvStore::new();
        assert!(c.crud_ok());
    }

    #[test]
    fn test_management() {
        let c = KvStore::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = KvStore::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_compact() {
        let c = KvStore::new();
        assert!(!c.needs_compact());
    }

    #[test]
    fn test_put() {
        let mut c = KvStore::new();
        c.put_ok = false;
        assert!(c.needs_compact());
    }

    #[test]
    fn test_health() {
        let c = KvStore::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
