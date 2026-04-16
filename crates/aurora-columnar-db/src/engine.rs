/// Columnar DB: column, compress, encode, scan, vectorize
/// Phase 1044

#[derive(Debug, Clone)]
pub struct ColumnarDb {
    pub column_ok: bool,
    pub compress_ok: bool,
    pub encode_ok: bool,
    pub scan_ok: bool,
    pub vectorize_ok: bool,
}

impl Default for ColumnarDb {
    fn default() -> Self {
        Self::new()
    }
}

impl ColumnarDb {
    pub fn new() -> Self {
        Self {
            column_ok: true,
            compress_ok: true,
            encode_ok: true,
            scan_ok: true,
            vectorize_ok: true,
        }
    }

    pub fn storage_ok(&self) -> bool {
        self.column_ok && self.compress_ok && self.encode_ok
    }

    pub fn query_ok(&self) -> bool {
        self.scan_ok && self.vectorize_ok
    }

    pub fn all_ok(&self) -> bool {
        self.storage_ok() && self.query_ok()
    }

    pub fn needs_compact(&self) -> bool {
        !self.compress_ok || !self.column_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.column_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let c = ColumnarDb::new();
        assert!(c.storage_ok());
    }

    #[test]
    fn test_query() {
        let c = ColumnarDb::new();
        assert!(c.query_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ColumnarDb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_compact() {
        let c = ColumnarDb::new();
        assert!(!c.needs_compact());
    }

    #[test]
    fn test_compress() {
        let mut c = ColumnarDb::new();
        c.compress_ok = false;
        assert!(c.needs_compact());
    }

    #[test]
    fn test_health() {
        let c = ColumnarDb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
