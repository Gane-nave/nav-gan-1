/// WAF engine: rule, inspect, block, whitelist, log
/// Phase 1008

#[derive(Debug, Clone)]
pub struct WafEngine {
    pub rule_ok: bool,
    pub inspect_ok: bool,
    pub block_ok: bool,
    pub whitelist_ok: bool,
    pub log_ok: bool,
}

impl Default for WafEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl WafEngine {
    pub fn new() -> Self {
        Self {
            rule_ok: true,
            inspect_ok: true,
            block_ok: true,
            whitelist_ok: true,
            log_ok: true,
        }
    }

    pub fn protection_ok(&self) -> bool {
        self.rule_ok && self.inspect_ok && self.block_ok
    }

    pub fn management_ok(&self) -> bool {
        self.whitelist_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.protection_ok() && self.management_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.rule_ok || !self.inspect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.rule_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection() {
        let c = WafEngine::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_management() {
        let c = WafEngine::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WafEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = WafEngine::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_rule() {
        let mut c = WafEngine::new();
        c.rule_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = WafEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
