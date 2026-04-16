/// aurora-chart-candlestick: chart candlestick
/// Phase 2456

#[derive(Debug, Clone)]
pub struct ChartCandlestick {
    pub render_ok: bool,
    pub ohlc_ok: bool,
    pub color_ok: bool,
    pub tooltip_ok: bool,
    pub zoom_ok: bool,
}

impl Default for ChartCandlestick {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartCandlestick {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            ohlc_ok: true,
            color_ok: true,
            tooltip_ok: true,
            zoom_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.ohlc_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tooltip_ok && self.zoom_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.ohlc_ok
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
        let c = ChartCandlestick::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartCandlestick::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartCandlestick::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartCandlestick::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartCandlestick::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartCandlestick::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartCandlestick::default();
        assert!(c.all_ok());
    }
}
