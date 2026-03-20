/// fleet cargo: load, track, deliver, confirm, log
/// Phase 1415

#[derive(Debug, Clone)]
pub struct FleetCargo {
    pub load_ok: bool,
    pub track_ok: bool,
    pub deliver_ok: bool,
    pub confirm_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetCargo {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetCargo {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            track_ok: true,
            deliver_ok: true,
            confirm_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.track_ok && self.deliver_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.confirm_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FleetCargo::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetCargo::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetCargo::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetCargo::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetCargo::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetCargo::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
