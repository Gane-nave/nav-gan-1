/// fog compute: discover, offload, execute, collect, balance
/// Phase 1137

#[derive(Debug, Clone)]
pub struct FogCompute {
    pub discover_ok: bool,
    pub offload_ok: bool,
    pub execute_ok: bool,
    pub collect_ok: bool,
    pub balance_ok: bool,
}

impl Default for FogCompute {
    fn default() -> Self {
        Self::new()
    }
}

impl FogCompute {
    pub fn new() -> Self {
        Self {
            discover_ok: true,
            offload_ok: true,
            execute_ok: true,
            collect_ok: true,
            balance_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.discover_ok && self.offload_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.collect_ok && self.balance_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.discover_ok || !self.offload_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.discover_ok {
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
        let c = FogCompute::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FogCompute::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FogCompute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FogCompute::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FogCompute::new();
        c.discover_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FogCompute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
