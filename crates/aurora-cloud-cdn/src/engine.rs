/// aurora-cloud-cdn: cloud cdn
/// Phase 2550

#[derive(Debug, Clone)]
pub struct CloudCdn {
    pub distribute_ok: bool,
    pub purge_ok: bool,
    pub preload_ok: bool,
    pub monitor_ok: bool,
    pub cost_ok: bool,
}

impl Default for CloudCdn {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudCdn {
    pub fn new() -> Self {
        Self {
            distribute_ok: true,
            purge_ok: true,
            preload_ok: true,
            monitor_ok: true,
            cost_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.distribute_ok && self.purge_ok && self.preload_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.cost_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.distribute_ok || !self.purge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.distribute_ok {
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
        let c = CloudCdn::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudCdn::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudCdn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudCdn::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudCdn::new();
        c.distribute_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudCdn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = CloudCdn::default();
        assert!(c.all_ok());
    }
}
