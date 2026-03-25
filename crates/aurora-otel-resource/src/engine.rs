/// otel resource: detect, merge, attrs, schema, log
/// Phase 1759

#[derive(Debug, Clone)]
pub struct OtelResource {
    pub detect_ok: bool,
    pub merge_ok: bool,
    pub attrs_ok: bool,
    pub schema_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelResource {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelResource {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            merge_ok: true,
            attrs_ok: true,
            schema_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.merge_ok && self.attrs_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.schema_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.merge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = OtelResource::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelResource::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelResource::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelResource::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelResource::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelResource::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
