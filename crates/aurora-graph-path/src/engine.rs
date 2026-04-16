/// graph path: find, shortest, all, weight, log
/// Phase 1898

#[derive(Debug, Clone)]
pub struct GraphPath {
    pub find_ok: bool,
    pub shortest_ok: bool,
    pub all_ok: bool,
    pub weight_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphPath {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphPath {
    pub fn new() -> Self {
        Self {
            find_ok: true,
            shortest_ok: true,
            all_ok: true,
            weight_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.find_ok && self.shortest_ok && self.all_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.weight_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.find_ok || !self.shortest_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.find_ok {
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
        let c = GraphPath::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphPath::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphPath::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphPath::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphPath::new();
        c.find_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphPath::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
