/// Fleet tracking: locate, route, status, dispatch, report
/// Phase 1094

#[derive(Debug, Clone)]
pub struct FleetTrack {
    pub locate_ok: bool,
    pub route_ok: bool,
    pub status_ok: bool,
    pub dispatch_ok: bool,
    pub report_ok: bool,
}

impl Default for FleetTrack {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetTrack {
    pub fn new() -> Self {
        Self {
            locate_ok: true,
            route_ok: true,
            status_ok: true,
            dispatch_ok: true,
            report_ok: true,
        }
    }

    pub fn tracking_ok(&self) -> bool {
        self.locate_ok && self.route_ok && self.status_ok
    }

    pub fn management_ok(&self) -> bool {
        self.dispatch_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tracking_ok() && self.management_ok()
    }

    pub fn needs_sync(&self) -> bool {
        !self.locate_ok || !self.route_ok
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
    fn test_tracking() {
        let c = FleetTrack::new();
        assert!(c.tracking_ok());
    }

    #[test]
    fn test_management() {
        let c = FleetTrack::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetTrack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_sync() {
        let c = FleetTrack::new();
        assert!(!c.needs_sync());
    }

    #[test]
    fn test_locate() {
        let mut c = FleetTrack::new();
        c.locate_ok = false;
        assert!(c.needs_sync());
    }

    #[test]
    fn test_health() {
        let c = FleetTrack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
