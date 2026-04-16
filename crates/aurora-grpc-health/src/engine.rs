/// aurora-grpc-health: grpc health
/// Phase 2589

#[derive(Debug, Clone)]
pub struct GrpcHealth {
    pub check_ok: bool,
    pub watch_ok: bool,
    pub report_ok: bool,
    pub timeout_ok: bool,
    pub retry_ok: bool,
}

impl Default for GrpcHealth {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcHealth {
    pub fn new() -> Self {
        Self {
            check_ok: true,
            watch_ok: true,
            report_ok: true,
            timeout_ok: true,
            retry_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.check_ok && self.watch_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.timeout_ok && self.retry_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.check_ok || !self.watch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.check_ok {
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
        let c = GrpcHealth::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GrpcHealth::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GrpcHealth::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GrpcHealth::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GrpcHealth::new();
        c.check_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GrpcHealth::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = GrpcHealth::default();
        assert!(c.all_ok());
    }
}
