/// Data lake: ingest, catalog, query, partition, lifecycle
/// Phase 1033

#[derive(Debug, Clone)]
pub struct DataLake {
    pub ingest_ok: bool,
    pub catalog_ok: bool,
    pub query_ok: bool,
    pub partition_ok: bool,
    pub lifecycle_ok: bool,
}

impl Default for DataLake {
    fn default() -> Self {
        Self::new()
    }
}

impl DataLake {
    pub fn new() -> Self {
        Self {
            ingest_ok: true,
            catalog_ok: true,
            query_ok: true,
            partition_ok: true,
            lifecycle_ok: true,
        }
    }

    pub fn storage_ok(&self) -> bool {
        self.ingest_ok && self.catalog_ok && self.partition_ok
    }

    pub fn access_ok(&self) -> bool {
        self.query_ok && self.lifecycle_ok
    }

    pub fn all_ok(&self) -> bool {
        self.storage_ok() && self.access_ok()
    }

    pub fn needs_reindex(&self) -> bool {
        !self.catalog_ok || !self.partition_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ingest_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let c = DataLake::new();
        assert!(c.storage_ok());
    }

    #[test]
    fn test_access() {
        let c = DataLake::new();
        assert!(c.access_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataLake::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reindex() {
        let c = DataLake::new();
        assert!(!c.needs_reindex());
    }

    #[test]
    fn test_catalog() {
        let mut c = DataLake::new();
        c.catalog_ok = false;
        assert!(c.needs_reindex());
    }

    #[test]
    fn test_health() {
        let c = DataLake::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
