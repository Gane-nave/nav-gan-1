/// aurora-svc-registry: svc registry
/// Phase 2565

#[derive(Debug, Clone)]
pub struct SvcRegistry {
    pub register_ok: bool,
    pub deregister_ok: bool,
    pub query_ok: bool,
    pub watch_ok: bool,
    pub health_ok: bool,
}

impl Default for SvcRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SvcRegistry {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            deregister_ok: true,
            query_ok: true,
            watch_ok: true,
            health_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.deregister_ok && self.query_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.watch_ok && self.health_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.deregister_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok {
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
        let c = SvcRegistry::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SvcRegistry::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SvcRegistry::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SvcRegistry::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SvcRegistry::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SvcRegistry::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SvcRegistry::default();
        assert!(c.all_ok());
    }
}
