/// aurora-dash-search: dash search
/// Phase 2449

#[derive(Debug, Clone)]
pub struct DashSearch {
    pub query_ok: bool,
    pub filter_ok: bool,
    pub highlight_ok: bool,
    pub reset_ok: bool,
    pub export_ok: bool,
}

impl Default for DashSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl DashSearch {
    pub fn new() -> Self {
        Self {
            query_ok: true,
            filter_ok: true,
            highlight_ok: true,
            reset_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.query_ok && self.filter_ok && self.highlight_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.query_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.query_ok {
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
        let c = DashSearch::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashSearch::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashSearch::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashSearch::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashSearch::new();
        c.query_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashSearch::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashSearch::default();
        assert!(c.all_ok());
    }
}
