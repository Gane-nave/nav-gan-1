/// rear disp: video, climate, seat, light, lock
/// Phase 1305

#[derive(Debug, Clone)]
pub struct RearDisp {
    pub video_ok: bool,
    pub climate_ok: bool,
    pub seat_ok: bool,
    pub light_ok: bool,
    pub lock_ok: bool,
}

impl Default for RearDisp {
    fn default() -> Self {
        Self::new()
    }
}

impl RearDisp {
    pub fn new() -> Self {
        Self {
            video_ok: true,
            climate_ok: true,
            seat_ok: true,
            light_ok: true,
            lock_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.video_ok && self.climate_ok && self.seat_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.light_ok && self.lock_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.video_ok || !self.climate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.video_ok {
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
        let c = RearDisp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RearDisp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RearDisp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RearDisp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RearDisp::new();
        c.video_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RearDisp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
