/// aurora-chart-slope: chart slope
/// Phase 2463

#[derive(Debug, Clone)]
pub struct ChartSlope {
    pub render_ok: bool,
    pub line_ok: bool,
    pub label_ok: bool,
    pub color_ok: bool,
    pub tooltip_ok: bool,
}

impl Default for ChartSlope {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartSlope {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            line_ok: true,
            label_ok: true,
            color_ok: true,
            tooltip_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.line_ok && self.label_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.tooltip_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.line_ok
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
        let c = ChartSlope::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartSlope::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartSlope::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartSlope::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartSlope::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartSlope::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartSlope::default();
        assert!(c.all_ok());
    }
}
