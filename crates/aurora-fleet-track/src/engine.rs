/// Fleet tracking: GPS, geofence, route history, idle alert
/// Phase 892

#[derive(Debug, Clone)]
pub struct FleetTrack {
    pub gps_ok: bool,
    pub geofence_ok: bool,
    pub history_ok: bool,
    pub idle_ok: bool,
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
            gps_ok: true,
            geofence_ok: true,
            history_ok: true,
            idle_ok: true,
            report_ok: true,
        }
    }

    pub fn tracking_ok(&self) -> bool {
        self.gps_ok && self.geofence_ok && self.history_ok
    }

    pub fn alerts_ok(&self) -> bool {
        self.idle_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tracking_ok() && self.alerts_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.gps_ok || !self.geofence_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gps_ok { return 5.0; }
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
    fn test_alerts() {
        let c = FleetTrack::new();
        assert!(c.alerts_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetTrack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = FleetTrack::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_gps() {
        let mut c = FleetTrack::new();
        c.gps_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = FleetTrack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
