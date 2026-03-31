/// aurora-trace-jaeger: trace jaeger
/// Phase 2608

#[derive(Debug, Clone)]
pub struct TraceJaeger {
    pub export_ok: bool,
    pub sample_ok: bool,
    pub batch_ok: bool,
    pub tag_ok: bool,
    pub search_ok: bool,
}

impl Default for TraceJaeger {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceJaeger {
    pub fn new() -> Self {
        Self {
            export_ok: true,
            sample_ok: true,
            batch_ok: true,
            tag_ok: true,
            search_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.export_ok && self.sample_ok && self.batch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tag_ok && self.search_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.export_ok || !self.sample_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.export_ok {
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
        let c = TraceJaeger::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TraceJaeger::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TraceJaeger::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TraceJaeger::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TraceJaeger::new();
        c.export_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TraceJaeger::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = TraceJaeger::default();
        assert!(c.all_ok());
    }
}
