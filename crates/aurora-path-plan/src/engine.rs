/// Path planner: search, smooth, optimize, validate, replan
/// Phase 1111

#[derive(Debug, Clone)]
pub struct PathPlan {
    pub search_ok: bool,
    pub smooth_ok: bool,
    pub optimize_ok: bool,
    pub validate_ok: bool,
    pub replan_ok: bool,
}

impl Default for PathPlan {
    fn default() -> Self {
        Self::new()
    }
}

impl PathPlan {
    pub fn new() -> Self {
        Self {
            search_ok: true,
            smooth_ok: true,
            optimize_ok: true,
            validate_ok: true,
            replan_ok: true,
        }
    }

    pub fn planning_ok(&self) -> bool {
        self.search_ok && self.smooth_ok && self.optimize_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.validate_ok && self.replan_ok
    }

    pub fn all_ok(&self) -> bool {
        self.planning_ok() && self.safety_ok()
    }

    pub fn needs_replan(&self) -> bool {
        !self.search_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.search_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning() {
        let c = PathPlan::new();
        assert!(c.planning_ok());
    }

    #[test]
    fn test_safety() {
        let c = PathPlan::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PathPlan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replan() {
        let c = PathPlan::new();
        assert!(!c.needs_replan());
    }

    #[test]
    fn test_search() {
        let mut c = PathPlan::new();
        c.search_ok = false;
        assert!(c.needs_replan());
    }

    #[test]
    fn test_health() {
        let c = PathPlan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
