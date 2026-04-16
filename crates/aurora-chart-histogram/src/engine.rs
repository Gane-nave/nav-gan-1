/// aurora-chart-histogram: chart histogram
/// Phase 2460

#[derive(Debug, Clone)]
pub struct ChartHistogram {
    pub render_ok: bool,
    pub bin_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
    pub density_ok: bool,
}

impl Default for ChartHistogram {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartHistogram {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            bin_ok: true,
            color_ok: true,
            label_ok: true,
            density_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.bin_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.label_ok && self.density_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.bin_ok
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
        let c = ChartHistogram::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartHistogram::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartHistogram::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartHistogram::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartHistogram::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartHistogram::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartHistogram::default();
        assert!(c.all_ok());
    }
}
