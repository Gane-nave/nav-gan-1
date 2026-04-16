/// aurora-chart-boxplot: chart boxplot
/// Phase 2461

#[derive(Debug, Clone)]
pub struct ChartBoxplot {
    pub render_ok: bool,
    pub quartile_ok: bool,
    pub outlier_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
}

impl Default for ChartBoxplot {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartBoxplot {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            quartile_ok: true,
            outlier_ok: true,
            color_ok: true,
            label_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.quartile_ok && self.outlier_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.label_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.quartile_ok
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
        let c = ChartBoxplot::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartBoxplot::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartBoxplot::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartBoxplot::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartBoxplot::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartBoxplot::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartBoxplot::default();
        assert!(c.all_ok());
    }
}
