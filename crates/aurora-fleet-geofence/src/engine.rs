/// fleet geofence: define, monitor, enter, exit, log
/// Phase 1417

#[derive(Debug, Clone)]
pub struct FleetGeofence {
    pub define_ok: bool,
    pub monitor_ok: bool,
    pub enter_ok: bool,
    pub exit_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetGeofence {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetGeofence {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            monitor_ok: true,
            enter_ok: true,
            exit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.monitor_ok && self.enter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.exit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.monitor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = FleetGeofence::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetGeofence::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetGeofence::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetGeofence::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetGeofence::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetGeofence::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
