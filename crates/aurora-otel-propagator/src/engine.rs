/// otel propagator: inject, extract, fields, carrier, log
/// Phase 1760

#[derive(Debug, Clone)]
pub struct OtelPropagator {
    pub inject_ok: bool,
    pub extract_ok: bool,
    pub fields_ok: bool,
    pub carrier_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelPropagator {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelPropagator {
    pub fn new() -> Self {
        Self {
            inject_ok: true,
            extract_ok: true,
            fields_ok: true,
            carrier_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.inject_ok && self.extract_ok && self.fields_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.carrier_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.inject_ok || !self.extract_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inject_ok {
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
        let c = OtelPropagator::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelPropagator::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelPropagator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelPropagator::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelPropagator::new();
        c.inject_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelPropagator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
