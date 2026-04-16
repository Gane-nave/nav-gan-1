/// graph query2: parse, plan, execute, optimize, log
/// Phase 1904

#[derive(Debug, Clone)]
pub struct GraphQuery2 {
    pub parse_ok: bool,
    pub plan_ok: bool,
    pub execute_ok: bool,
    pub optimize_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphQuery2 {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphQuery2 {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            plan_ok: true,
            execute_ok: true,
            optimize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.plan_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.optimize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.plan_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = GraphQuery2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphQuery2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphQuery2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphQuery2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphQuery2::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphQuery2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
