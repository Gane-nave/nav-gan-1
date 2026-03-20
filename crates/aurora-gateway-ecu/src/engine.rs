/// gateway ecu: translate, route, filter, log, check
/// Phase 1273

#[derive(Debug, Clone)]
pub struct GatewayEcu {
    pub translate_ok: bool,
    pub route_ok: bool,
    pub filter_ok: bool,
    pub log_ok: bool,
    pub check_ok: bool,
}

impl Default for GatewayEcu {
    fn default() -> Self {
        Self::new()
    }
}

impl GatewayEcu {
    pub fn new() -> Self {
        Self {
            translate_ok: true,
            route_ok: true,
            filter_ok: true,
            log_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.translate_ok && self.route_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.log_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.translate_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.translate_ok {
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
        let c = GatewayEcu::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GatewayEcu::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GatewayEcu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GatewayEcu::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GatewayEcu::new();
        c.translate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GatewayEcu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
