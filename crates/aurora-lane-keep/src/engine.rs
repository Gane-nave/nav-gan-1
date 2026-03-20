/// lane keep: detect, track, correct, alert, disengage
/// Phase 1320

#[derive(Debug, Clone)]
pub struct LaneKeep {
    pub detect_ok: bool,
    pub track_ok: bool,
    pub correct_ok: bool,
    pub alert_ok: bool,
    pub disengage_ok: bool,
}

impl Default for LaneKeep {
    fn default() -> Self {
        Self::new()
    }
}

impl LaneKeep {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            track_ok: true,
            correct_ok: true,
            alert_ok: true,
            disengage_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.track_ok && self.correct_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.disengage_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = LaneKeep::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LaneKeep::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LaneKeep::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LaneKeep::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LaneKeep::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LaneKeep::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
