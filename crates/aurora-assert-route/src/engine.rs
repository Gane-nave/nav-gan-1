/// aurora-assert-route: assert route
/// Phase 2505

#[derive(Debug, Clone)]
pub struct AssertRoute {
    pub waypoint_ok: bool,
    pub distance_ok: bool,
    pub eta_ok: bool,
    pub detour_ok: bool,
    pub complete_ok: bool,
}

impl Default for AssertRoute {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertRoute {
    pub fn new() -> Self {
        Self {
            waypoint_ok: true,
            distance_ok: true,
            eta_ok: true,
            detour_ok: true,
            complete_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.waypoint_ok && self.distance_ok && self.eta_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.detour_ok && self.complete_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.waypoint_ok || !self.distance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.waypoint_ok {
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
        let c = AssertRoute::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertRoute::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertRoute::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertRoute::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertRoute::new();
        c.waypoint_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertRoute::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertRoute::default();
        assert!(c.all_ok());
    }
}
