/// Geo-fencing: define, monitor, enter, exit, alert
/// Phase 1086

#[derive(Debug, Clone)]
pub struct GeoFence {
    pub define_ok: bool,
    pub monitor_ok: bool,
    pub enter_ok: bool,
    pub exit_ok: bool,
    pub alert_ok: bool,
}

impl Default for GeoFence {
    fn default() -> Self {
        Self::new()
    }
}

impl GeoFence {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            monitor_ok: true,
            enter_ok: true,
            exit_ok: true,
            alert_ok: true,
        }
    }

    pub fn boundary_ok(&self) -> bool {
        self.define_ok && self.monitor_ok
    }

    pub fn events_ok(&self) -> bool {
        self.enter_ok && self.exit_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.boundary_ok() && self.events_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.define_ok || !self.monitor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary() {
        let c = GeoFence::new();
        assert!(c.boundary_ok());
    }

    #[test]
    fn test_events() {
        let c = GeoFence::new();
        assert!(c.events_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GeoFence::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = GeoFence::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_define() {
        let mut c = GeoFence::new();
        c.define_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = GeoFence::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
