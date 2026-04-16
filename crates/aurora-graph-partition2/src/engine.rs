/// graph partition2: split, balance, migrate, verify, log
/// Phase 1901

#[derive(Debug, Clone)]
pub struct GraphPartition2 {
    pub split_ok: bool,
    pub balance_ok: bool,
    pub migrate_ok: bool,
    pub verify_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphPartition2 {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphPartition2 {
    pub fn new() -> Self {
        Self {
            split_ok: true,
            balance_ok: true,
            migrate_ok: true,
            verify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.split_ok && self.balance_ok && self.migrate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.verify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.split_ok || !self.balance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.split_ok {
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
        let c = GraphPartition2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphPartition2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphPartition2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphPartition2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphPartition2::new();
        c.split_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphPartition2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
