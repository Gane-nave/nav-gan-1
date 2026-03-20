/// sec waf: inspect, block, allow, report, log
/// Phase 2054

#[derive(Debug, Clone)]
pub struct SecWaf {
    pub inspect_ok: bool,
    pub block_ok: bool,
    pub allow_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for SecWaf {
    fn default() -> Self {
        Self::new()
    }
}

impl SecWaf {
    pub fn new() -> Self {
        Self {
            inspect_ok: true,
            block_ok: true,
            allow_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.inspect_ok && self.block_ok && self.allow_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.inspect_ok || !self.block_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inspect_ok {
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
        let c = SecWaf::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SecWaf::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SecWaf::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SecWaf::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SecWaf::new();
        c.inspect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SecWaf::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
