/// aurora-chart-bump: chart bump
/// Phase 2464

#[derive(Debug, Clone)]
pub struct ChartBump {
    pub render_ok: bool,
    pub rank_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
    pub tooltip_ok: bool,
}

impl Default for ChartBump {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartBump {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            rank_ok: true,
            color_ok: true,
            label_ok: true,
            tooltip_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.rank_ok && self.color_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.label_ok && self.tooltip_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.rank_ok
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
        let c = ChartBump::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartBump::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartBump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartBump::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartBump::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartBump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartBump::default();
        assert!(c.all_ok());
    }
}
