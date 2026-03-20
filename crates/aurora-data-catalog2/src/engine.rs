/// data catalog2: register, discover, lineage, govern, log
/// Phase 2208

#[derive(Debug, Clone)]
pub struct DataCatalog2 {
    pub register_ok: bool,
    pub discover_ok: bool,
    pub lineage_ok: bool,
    pub govern_ok: bool,
    pub log_ok: bool,
}

impl Default for DataCatalog2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataCatalog2 {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            discover_ok: true,
            lineage_ok: true,
            govern_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.discover_ok && self.lineage_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.govern_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.discover_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok {
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
        let c = DataCatalog2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataCatalog2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataCatalog2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataCatalog2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataCatalog2::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataCatalog2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
