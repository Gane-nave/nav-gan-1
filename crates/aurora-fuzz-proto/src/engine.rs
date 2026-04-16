/// aurora-fuzz-proto: fuzz proto
/// Phase 2494

#[derive(Debug, Clone)]
pub struct FuzzProto {
    pub generate_ok: bool,
    pub mutate_ok: bool,
    pub inject_ok: bool,
    pub validate_ok: bool,
    pub report_ok: bool,
}

impl Default for FuzzProto {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzProto {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            mutate_ok: true,
            inject_ok: true,
            validate_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.mutate_ok && self.inject_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.mutate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = FuzzProto::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuzzProto::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuzzProto::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuzzProto::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuzzProto::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuzzProto::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = FuzzProto::default();
        assert!(c.all_ok());
    }
}
