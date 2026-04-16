/// data shard2: route, rebalance, migrate, monitor, log
/// Phase 2216

#[derive(Debug, Clone)]
pub struct DataShard2 {
    pub route_ok: bool,
    pub rebalance_ok: bool,
    pub migrate_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for DataShard2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataShard2 {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            rebalance_ok: true,
            migrate_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.rebalance_ok && self.migrate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.rebalance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok {
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
        let c = DataShard2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataShard2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataShard2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataShard2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataShard2::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataShard2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
