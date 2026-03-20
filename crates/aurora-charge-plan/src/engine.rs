/// Charge planning: locate, schedule, optimize, price, route
/// Phase 1100

#[derive(Debug, Clone)]
pub struct ChargePlan {
    pub locate_ok: bool,
    pub schedule_ok: bool,
    pub optimize_ok: bool,
    pub price_ok: bool,
    pub route_ok: bool,
}

impl Default for ChargePlan {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargePlan {
    pub fn new() -> Self {
        Self {
            locate_ok: true,
            schedule_ok: true,
            optimize_ok: true,
            price_ok: true,
            route_ok: true,
        }
    }

    pub fn planning_ok(&self) -> bool {
        self.locate_ok && self.schedule_ok && self.optimize_ok
    }

    pub fn economics_ok(&self) -> bool {
        self.price_ok && self.route_ok
    }

    pub fn all_ok(&self) -> bool {
        self.planning_ok() && self.economics_ok()
    }

    pub fn needs_refresh(&self) -> bool {
        !self.locate_ok || !self.schedule_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.locate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning() {
        let c = ChargePlan::new();
        assert!(c.planning_ok());
    }

    #[test]
    fn test_economics() {
        let c = ChargePlan::new();
        assert!(c.economics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargePlan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refresh() {
        let c = ChargePlan::new();
        assert!(!c.needs_refresh());
    }

    #[test]
    fn test_locate() {
        let mut c = ChargePlan::new();
        c.locate_ok = false;
        assert!(c.needs_refresh());
    }

    #[test]
    fn test_health() {
        let c = ChargePlan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
