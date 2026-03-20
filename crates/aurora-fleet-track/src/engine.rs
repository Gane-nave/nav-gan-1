/// fleet track: locate, route, assign, report, log
/// Phase 1410

#[derive(Debug, Clone)]
pub struct FleetTrack {
    pub locate_ok: bool,
    pub route_ok: bool,
    pub assign_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
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
            assign_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.locate_ok && self.route_ok && self.assign_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
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
    fn test_primary() {
        let c = FleetTrack::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetTrack::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetTrack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetTrack::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetTrack::new();
        c.locate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetTrack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
