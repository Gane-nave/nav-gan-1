/// data warehouse2: model, load, query, optimize, log
/// Phase 2218

#[derive(Debug, Clone)]
pub struct DataWarehouse2 {
    pub model_ok: bool,
    pub load_ok: bool,
    pub query_ok: bool,
    pub optimize_ok: bool,
    pub log_ok: bool,
}

impl Default for DataWarehouse2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataWarehouse2 {
    pub fn new() -> Self {
        Self {
            model_ok: true,
            load_ok: true,
            query_ok: true,
            optimize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.model_ok && self.load_ok && self.query_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.optimize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.model_ok || !self.load_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.model_ok {
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
        let c = DataWarehouse2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataWarehouse2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataWarehouse2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataWarehouse2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataWarehouse2::new();
        c.model_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataWarehouse2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
