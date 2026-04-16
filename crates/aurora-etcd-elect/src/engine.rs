/// aurora-etcd-elect: etcd elect
/// Phase 2602

#[derive(Debug, Clone)]
pub struct EtcdElect {
    pub campaign_ok: bool,
    pub resign_ok: bool,
    pub leader_ok: bool,
    pub follow_ok: bool,
    pub watch_ok: bool,
}

impl Default for EtcdElect {
    fn default() -> Self {
        Self::new()
    }
}

impl EtcdElect {
    pub fn new() -> Self {
        Self {
            campaign_ok: true,
            resign_ok: true,
            leader_ok: true,
            follow_ok: true,
            watch_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.campaign_ok && self.resign_ok && self.leader_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.follow_ok && self.watch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.campaign_ok || !self.resign_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.campaign_ok {
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
        let c = EtcdElect::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EtcdElect::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EtcdElect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EtcdElect::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EtcdElect::new();
        c.campaign_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EtcdElect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = EtcdElect::default();
        assert!(c.all_ok());
    }
}
