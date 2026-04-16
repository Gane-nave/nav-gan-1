/// aurora-chart-waterfall: chart waterfall
/// Phase 2457

#[derive(Debug, Clone)]
pub struct ChartWaterfall {
    pub render_ok: bool,
    pub bar_ok: bool,
    pub connector_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
}

impl Default for ChartWaterfall {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartWaterfall {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            bar_ok: true,
            connector_ok: true,
            color_ok: true,
            label_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.bar_ok && self.connector_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.label_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.bar_ok
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
        let c = ChartWaterfall::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartWaterfall::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartWaterfall::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartWaterfall::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartWaterfall::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartWaterfall::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartWaterfall::default();
        assert!(c.all_ok());
    }
}
