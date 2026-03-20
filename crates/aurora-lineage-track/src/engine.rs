/// Data lineage: trace, graph, impact, audit, visualize
/// Phase 1039

#[derive(Debug, Clone)]
pub struct LineageTrack {
    pub trace_ok: bool,
    pub graph_ok: bool,
    pub impact_ok: bool,
    pub audit_ok: bool,
    pub visualize_ok: bool,
}

impl Default for LineageTrack {
    fn default() -> Self {
        Self::new()
    }
}

impl LineageTrack {
    pub fn new() -> Self {
        Self {
            trace_ok: true,
            graph_ok: true,
            impact_ok: true,
            audit_ok: true,
            visualize_ok: true,
        }
    }

    pub fn tracking_ok(&self) -> bool {
        self.trace_ok && self.graph_ok && self.impact_ok
    }

    pub fn reporting_ok(&self) -> bool {
        self.audit_ok && self.visualize_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tracking_ok() && self.reporting_ok()
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.graph_ok || !self.trace_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.trace_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracking() {
        let c = LineageTrack::new();
        assert!(c.tracking_ok());
    }

    #[test]
    fn test_reporting() {
        let c = LineageTrack::new();
        assert!(c.reporting_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LineageTrack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebuild() {
        let c = LineageTrack::new();
        assert!(!c.needs_rebuild());
    }

    #[test]
    fn test_graph() {
        let mut c = LineageTrack::new();
        c.graph_ok = false;
        assert!(c.needs_rebuild());
    }

    #[test]
    fn test_health() {
        let c = LineageTrack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
