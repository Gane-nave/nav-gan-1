/// data partition2: strategy, execute, balance, monitor, log
/// Phase 2215

#[derive(Debug, Clone)]
pub struct DataPartition2 {
    pub strategy_ok: bool,
    pub execute_ok: bool,
    pub balance_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for DataPartition2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataPartition2 {
    pub fn new() -> Self {
        Self {
            strategy_ok: true,
            execute_ok: true,
            balance_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.strategy_ok && self.execute_ok && self.balance_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.strategy_ok || !self.execute_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.strategy_ok {
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
        let c = DataPartition2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataPartition2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataPartition2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataPartition2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataPartition2::new();
        c.strategy_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataPartition2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
