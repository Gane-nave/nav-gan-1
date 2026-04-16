/// aurora-replay-traffic: replay traffic
/// Phase 2534

#[derive(Debug, Clone)]
pub struct ReplayTraffic {
    pub record_ok: bool,
    pub play_ok: bool,
    pub seek_ok: bool,
    pub speed_ok: bool,
    pub export_ok: bool,
}

impl Default for ReplayTraffic {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplayTraffic {
    pub fn new() -> Self {
        Self {
            record_ok: true,
            play_ok: true,
            seek_ok: true,
            speed_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.record_ok && self.play_ok && self.seek_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.speed_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.record_ok || !self.play_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.record_ok {
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
        let c = ReplayTraffic::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReplayTraffic::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReplayTraffic::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReplayTraffic::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReplayTraffic::new();
        c.record_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReplayTraffic::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ReplayTraffic::default();
        assert!(c.all_ok());
    }
}
