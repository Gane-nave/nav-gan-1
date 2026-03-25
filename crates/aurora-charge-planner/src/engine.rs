/// Charge planner: route, station, time, cost optimizer
/// Phase 871

#[derive(Debug, Clone)]
pub struct ChargePlanner {
    pub route_ok: bool,
    pub station_ok: bool,
    pub time_ok: bool,
    pub cost_ok: bool,
    pub availability_ok: bool,
}

impl Default for ChargePlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl ChargePlanner {
    pub fn new() -> Self {
        Self {
            route_ok: true,
            station_ok: true,
            time_ok: true,
            cost_ok: true,
            availability_ok: true,
        }
    }

    pub fn planning_ok(&self) -> bool {
        self.route_ok && self.station_ok && self.availability_ok
    }

    pub fn optimization_ok(&self) -> bool {
        self.time_ok && self.cost_ok
    }

    pub fn all_ok(&self) -> bool {
        self.planning_ok() && self.optimization_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.station_ok || !self.availability_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.station_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning() {
        let c = ChargePlanner::new();
        assert!(c.planning_ok());
    }

    #[test]
    fn test_optimization() {
        let c = ChargePlanner::new();
        assert!(c.optimization_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ChargePlanner::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = ChargePlanner::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_station() {
        let mut c = ChargePlanner::new();
        c.station_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = ChargePlanner::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
