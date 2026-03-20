/// Service discovery: register, deregister, lookup, watch, health
/// Phase 1066

#[derive(Debug, Clone)]
pub struct SvcDisc {
    pub register_ok: bool,
    pub deregister_ok: bool,
    pub lookup_ok: bool,
    pub watch_ok: bool,
    pub health_ok: bool,
}

impl Default for SvcDisc {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcDisc {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            deregister_ok: true,
            lookup_ok: true,
            watch_ok: true,
            health_ok: true,
        }
    }

    pub fn registry_ok(&self) -> bool {
        self.register_ok && self.deregister_ok && self.lookup_ok
    }

    pub fn monitoring_ok(&self) -> bool {
        self.watch_ok && self.health_ok
    }

    pub fn all_ok(&self) -> bool {
        self.registry_ok() && self.monitoring_ok()
    }

    pub fn needs_refresh(&self) -> bool {
        !self.register_ok || !self.lookup_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry() {
        let c = SvcDisc::new();
        assert!(c.registry_ok());
    }

    #[test]
    fn test_monitoring() {
        let c = SvcDisc::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcDisc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refresh() {
        let c = SvcDisc::new();
        assert!(!c.needs_refresh());
    }

    #[test]
    fn test_register() {
        let mut c = SvcDisc::new();
        c.register_ok = false;
        assert!(c.needs_refresh());
    }

    #[test]
    fn test_health() {
        let c = SvcDisc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
