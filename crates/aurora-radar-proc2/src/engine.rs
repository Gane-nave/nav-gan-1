/// radar proc: capture, filter, detect, track, log
/// Phase 1481

#[derive(Debug, Clone)]
pub struct RadarProc2 {
    pub capture_ok: bool,
    pub filter_ok: bool,
    pub detect_ok: bool,
    pub track_ok: bool,
    pub log_ok: bool,
}

impl Default for RadarProc2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RadarProc2 {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            filter_ok: true,
            detect_ok: true,
            track_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.filter_ok && self.detect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.track_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = RadarProc2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RadarProc2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RadarProc2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RadarProc2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RadarProc2::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RadarProc2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
