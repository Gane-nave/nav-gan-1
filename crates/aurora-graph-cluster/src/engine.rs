/// graph cluster: detect, merge, split, label, log
/// Phase 1900

#[derive(Debug, Clone)]
pub struct GraphCluster {
    pub detect_ok: bool,
    pub merge_ok: bool,
    pub split_ok: bool,
    pub label_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphCluster {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphCluster {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            merge_ok: true,
            split_ok: true,
            label_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.merge_ok && self.split_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.label_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.merge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = GraphCluster::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphCluster::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphCluster::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphCluster::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphCluster::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphCluster::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
