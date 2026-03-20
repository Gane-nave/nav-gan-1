/// gps track: locate, store, transmit, fence, alert
/// Phase 1288

#[derive(Debug, Clone)]
pub struct GpsTrack {
    pub locate_ok: bool,
    pub store_ok: bool,
    pub transmit_ok: bool,
    pub fence_ok: bool,
    pub alert_ok: bool,
}

impl Default for GpsTrack {
    fn default() -> Self {
        Self::new()
    }
}

impl GpsTrack {
    pub fn new() -> Self {
        Self {
            locate_ok: true,
            store_ok: true,
            transmit_ok: true,
            fence_ok: true,
            alert_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.locate_ok && self.store_ok && self.transmit_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.fence_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.locate_ok || !self.store_ok
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
        let c = GpsTrack::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GpsTrack::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GpsTrack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GpsTrack::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GpsTrack::new();
        c.locate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GpsTrack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
