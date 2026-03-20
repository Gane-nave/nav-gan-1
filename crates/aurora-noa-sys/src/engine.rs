/// noa sys: route, navigate, overtake, exit, arrive
/// Phase 1170

#[derive(Debug, Clone)]
pub struct NoaSys {
    pub route_ok: bool,
    pub navigate_ok: bool,
    pub overtake_ok: bool,
    pub exit_ok: bool,
    pub arrive_ok: bool,
}

impl Default for NoaSys {
    fn default() -> Self {
        Self::new()
    }
}

impl NoaSys {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            navigate_ok: true,
            overtake_ok: true,
            exit_ok: true,
            arrive_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.route_ok && self.navigate_ok && self.overtake_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.exit_ok && self.arrive_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.route_ok || !self.navigate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.route_ok {
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
        let c = NoaSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NoaSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NoaSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NoaSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NoaSys::new();
        c.route_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NoaSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
