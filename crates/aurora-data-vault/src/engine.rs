/// Data vault: hub, link, satellite, pit, bridge
/// Phase 1042

#[derive(Debug, Clone)]
pub struct DataVault {
    pub hub_ok: bool,
    pub link_ok: bool,
    pub satellite_ok: bool,
    pub pit_ok: bool,
    pub bridge_ok: bool,
}

impl Default for DataVault {
    fn default() -> Self {
        Self::new()
    }
}

impl DataVault {
    pub fn new() -> Self {
        Self {
            hub_ok: true,
            link_ok: true,
            satellite_ok: true,
            pit_ok: true,
            bridge_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.hub_ok && self.link_ok && self.satellite_ok
    }

    pub fn optimization_ok(&self) -> bool {
        self.pit_ok && self.bridge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.optimization_ok()
    }

    pub fn needs_load(&self) -> bool {
        !self.hub_ok || !self.link_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.hub_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = DataVault::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_optimization() {
        let c = DataVault::new();
        assert!(c.optimization_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataVault::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_load() {
        let c = DataVault::new();
        assert!(!c.needs_load());
    }

    #[test]
    fn test_hub() {
        let mut c = DataVault::new();
        c.hub_ok = false;
        assert!(c.needs_load());
    }

    #[test]
    fn test_health() {
        let c = DataVault::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
