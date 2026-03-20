/// summon ctrl: request, route, navigate, arrive, stop
/// Phase 1323

#[derive(Debug, Clone)]
pub struct SummonCtrl {
    pub request_ok: bool,
    pub route_ok: bool,
    pub navigate_ok: bool,
    pub arrive_ok: bool,
    pub stop_ok: bool,
}

impl Default for SummonCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl SummonCtrl {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            route_ok: true,
            navigate_ok: true,
            arrive_ok: true,
            stop_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.route_ok && self.navigate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.arrive_ok && self.stop_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.route_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = SummonCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SummonCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SummonCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SummonCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SummonCtrl::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SummonCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
