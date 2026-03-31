/// aurora-chart-gantt: chart gantt
/// Phase 2458

#[derive(Debug, Clone)]
pub struct ChartGantt {
    pub render_ok: bool,
    pub task_ok: bool,
    pub dep_ok: bool,
    pub milestone_ok: bool,
    pub zoom_ok: bool,
}

impl Default for ChartGantt {
    fn default() -> Self {
        Self::new()
    }
}

impl ChartGantt {
    pub fn new() -> Self {
        Self {
            render_ok: true,
            task_ok: true,
            dep_ok: true,
            milestone_ok: true,
            zoom_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.render_ok && self.task_ok && self.dep_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.milestone_ok && self.zoom_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.render_ok || !self.task_ok
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
        let c = ChartGantt::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ChartGantt::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChartGantt::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ChartGantt::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ChartGantt::new();
        c.render_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ChartGantt::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ChartGantt::default();
        assert!(c.all_ok());
    }
}
