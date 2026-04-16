/// aurora-chart-marimekko: chart marimekko
/// Phase 2465

#[derive(Debug, Clone)]
pub struct ChartMarimekko {
    pub render_ok: bool,
    pub segment_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
    pub tooltip_ok: bool,
}

impl Default for ChartMarimekko {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartMarimekko {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            segment_ok: true,
            color_ok: true,
            label_ok: true,
            tooltip_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.segment_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.label_ok && self.tooltip_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.segment_ok
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
        let c = ChartMarimekko::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartMarimekko::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartMarimekko::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartMarimekko::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartMarimekko::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartMarimekko::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartMarimekko::default();
        assert!(c.all_ok());
    }
}
