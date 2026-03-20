/// Federated ML: aggregate, distribute, privacy, compress, sync
/// Phase 1021

#[derive(Debug, Clone)]
pub struct FederatedMl {
    pub aggregate_ok: bool,
    pub distribute_ok: bool,
    pub privacy_ok: bool,
    pub compress_ok: bool,
    pub sync_ok: bool,
}

impl Default for FederatedMl {
    fn default() -> Self {
        Self::new()
    }
}

impl FederatedMl {
    pub fn new() -> Self {
        Self {
            aggregate_ok: true,
            distribute_ok: true,
            privacy_ok: true,
            compress_ok: true,
            sync_ok: true,
        }
    }

    pub fn coordination_ok(&self) -> bool {
        self.aggregate_ok && self.distribute_ok && self.sync_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.privacy_ok && self.compress_ok
    }

    pub fn all_ok(&self) -> bool {
        self.coordination_ok() && self.protection_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.sync_ok || !self.aggregate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.aggregate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordination() {
        let c = FederatedMl::new();
        assert!(c.coordination_ok());
    }

    #[test]
    fn test_protection() {
        let c = FederatedMl::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FederatedMl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = FederatedMl::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_sync() {
        let mut c = FederatedMl::new();
        c.sync_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = FederatedMl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
