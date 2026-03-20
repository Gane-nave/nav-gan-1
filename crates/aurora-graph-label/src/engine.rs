/// graph label: assign, remove, query, count, log
/// Phase 1912

#[derive(Debug, Clone)]
pub struct GraphLabel {
    pub assign_ok: bool,
    pub remove_ok: bool,
    pub query_ok: bool,
    pub count_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphLabel {
    pub fn new() -> Self {
        Self {
            assign_ok: true,
            remove_ok: true,
            query_ok: true,
            count_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.assign_ok && self.remove_ok && self.query_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.count_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.assign_ok || !self.remove_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.assign_ok {
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
        let c = GraphLabel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphLabel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphLabel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphLabel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphLabel::new();
        c.assign_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphLabel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
