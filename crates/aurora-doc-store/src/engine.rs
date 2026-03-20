/// Document store: insert, query, index, aggregate, replicate
/// Phase 1047

#[derive(Debug, Clone)]
pub struct DocStore {
    pub insert_ok: bool,
    pub query_ok: bool,
    pub index_ok: bool,
    pub aggregate_ok: bool,
    pub replicate_ok: bool,
}

impl Default for DocStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DocStore {
    pub fn new() -> Self {
        Self {
            insert_ok: true,
            query_ok: true,
            index_ok: true,
            aggregate_ok: true,
            replicate_ok: true,
        }
    }

    pub fn operations_ok(&self) -> bool {
        self.insert_ok && self.query_ok && self.index_ok
    }

    pub fn scalability_ok(&self) -> bool {
        self.aggregate_ok && self.replicate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.operations_ok() && self.scalability_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.index_ok || !self.replicate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.insert_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        let c = DocStore::new();
        assert!(c.operations_ok());
    }

    #[test]
    fn test_scalability() {
        let c = DocStore::new();
        assert!(c.scalability_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DocStore::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = DocStore::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_index() {
        let mut c = DocStore::new();
        c.index_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = DocStore::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
