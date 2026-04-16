/// aurora-metric-otel: metric otel
/// Phase 2612

#[derive(Debug, Clone)]
pub struct MetricOtel {
    pub collect_ok: bool,
    pub export_ok: bool,
    pub label_ok: bool,
    pub histogram_ok: bool,
    pub alert_ok: bool,
}

impl Default for MetricOtel {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricOtel {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            export_ok: true,
            label_ok: true,
            histogram_ok: true,
            alert_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.export_ok && self.label_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.histogram_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.export_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
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
        let c = MetricOtel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MetricOtel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MetricOtel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MetricOtel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MetricOtel::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MetricOtel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MetricOtel::default();
        assert!(c.all_ok());
    }
}
