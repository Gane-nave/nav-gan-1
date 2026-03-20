/// otel metric: record, gauge, histogram, export, log
/// Phase 1753

#[derive(Debug, Clone)]
pub struct OtelMetric {
    pub record_ok: bool,
    pub gauge_ok: bool,
    pub histogram_ok: bool,
    pub export_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelMetric {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelMetric {
    pub fn new() -> Self {
        Self {
            record_ok: true,
            gauge_ok: true,
            histogram_ok: true,
            export_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.record_ok && self.gauge_ok && self.histogram_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.record_ok || !self.gauge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.record_ok {
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
        let c = OtelMetric::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelMetric::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelMetric::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelMetric::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelMetric::new();
        c.record_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelMetric::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
