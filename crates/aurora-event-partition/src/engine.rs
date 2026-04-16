/// event partition: assign, rebalance, commit, seek, log
/// Phase 1869

#[derive(Debug, Clone)]
pub struct EventPartition {
    pub assign_ok: bool,
    pub rebalance_ok: bool,
    pub commit_ok: bool,
    pub seek_ok: bool,
    pub log_ok: bool,
}

impl Default for EventPartition {
    fn default() -> Self {
        Self::new()
    }
}

impl EventPartition {
    pub fn new() -> Self {
        Self {
            assign_ok: true,
            rebalance_ok: true,
            commit_ok: true,
            seek_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assign_ok && self.rebalance_ok && self.commit_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.seek_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assign_ok || !self.rebalance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assign_ok {
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
        let c = EventPartition::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventPartition::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventPartition::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventPartition::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventPartition::new();
        c.assign_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventPartition::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
