/// aurora-grpc-lb: grpc lb
/// Phase 2588

#[derive(Debug, Clone)]
pub struct GrpcLb {
    pub route_ok: bool,
    pub health_ok: bool,
    pub weight_ok: bool,
    pub sticky_ok: bool,
    pub report_ok: bool,
}

impl Default for GrpcLb {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcLb {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            health_ok: true,
            weight_ok: true,
            sticky_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.health_ok && self.weight_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sticky_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.health_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok {
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
        let c = GrpcLb::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GrpcLb::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GrpcLb::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GrpcLb::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GrpcLb::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GrpcLb::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = GrpcLb::default();
        assert!(c.all_ok());
    }
}
