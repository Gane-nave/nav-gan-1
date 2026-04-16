/// aurora-chart-radar2: chart radar2
/// Phase 2459

#[derive(Debug, Clone)]
pub struct ChartRadar2 {
    pub render_ok: bool,
    pub axis_ok: bool,
    pub value_ok: bool,
    pub color_ok: bool,
    pub label_ok: bool,
}

impl Default for ChartRadar2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartRadar2 {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            axis_ok: true,
            value_ok: true,
            color_ok: true,
            label_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.axis_ok && self.value_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.color_ok && self.label_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.axis_ok
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
        let c = ChartRadar2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartRadar2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartRadar2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartRadar2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartRadar2::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartRadar2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartRadar2::default();
        assert!(c.all_ok());
    }
}
