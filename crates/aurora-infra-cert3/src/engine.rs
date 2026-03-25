/// infra cert3: request, renew, revoke, monitor, log
/// Phase 2135

#[derive(Debug, Clone)]
pub struct InfraCert3 {
    pub request_ok: bool,
    pub renew_ok: bool,
    pub revoke_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for InfraCert3 {
    fn default() -> Self {
        Self::new()
    }
}

impl InfraCert3 {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            renew_ok: true,
            revoke_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.renew_ok && self.revoke_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.renew_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = InfraCert3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = InfraCert3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = InfraCert3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = InfraCert3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = InfraCert3::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = InfraCert3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
