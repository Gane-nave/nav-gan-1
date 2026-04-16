/// aurora-grpc-reflect: grpc reflect
/// Phase 2590

#[derive(Debug, Clone)]
pub struct GrpcReflect {
    pub describe_ok: bool,
    pub list_ok: bool,
    pub query_ok: bool,
    pub export_ok: bool,
    pub cache_ok: bool,
}

impl Default for GrpcReflect {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcReflect {
    pub fn new() -> Self {
        Self {
            describe_ok: true,
            list_ok: true,
            query_ok: true,
            export_ok: true,
            cache_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.describe_ok && self.list_ok && self.query_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.cache_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.describe_ok || !self.list_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.describe_ok {
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
        let c = GrpcReflect::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GrpcReflect::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GrpcReflect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GrpcReflect::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GrpcReflect::new();
        c.describe_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GrpcReflect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = GrpcReflect::default();
        assert!(c.all_ok());
    }
}
