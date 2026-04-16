/// aurora-viz-gauge: viz gauge
/// Phase 2426

#[derive(Debug, Clone)]
pub struct VizGauge {
    pub render_ok: bool,
    pub value_ok: bool,
    pub min_ok: bool,
    pub max_ok: bool,
    pub threshold_ok: bool,
}

impl Default for VizGauge {
    fn default() -> Self {
        Self::new()
    }
}

impl VizGauge {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            value_ok: true,
            min_ok: true,
            max_ok: true,
            threshold_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.value_ok && self.min_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.max_ok && self.threshold_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.value_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.render_ok {
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
        let c = VizGauge::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VizGauge::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VizGauge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VizGauge::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VizGauge::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VizGauge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = VizGauge::default();
        assert!(c.all_ok());
    }
}
