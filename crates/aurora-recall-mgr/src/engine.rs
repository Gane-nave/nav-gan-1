/// Recall manager: detect, notify, schedule, track, verify
/// Phase 971

#[derive(Debug, Clone)]
pub struct RecallMgr {
    pub detect_ok: bool,
    pub notify_ok: bool,
    pub schedule_ok: bool,
    pub track_ok: bool,
    pub verify_ok: bool,
}

impl Default for RecallMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl RecallMgr {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            notify_ok: true,
            schedule_ok: true,
            track_ok: true,
            verify_ok: true,
        }
    }

    pub fn identification_ok(&self) -> bool {
        self.detect_ok && self.notify_ok
    }

    pub fn resolution_ok(&self) -> bool {
        self.schedule_ok && self.track_ok && self.verify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.identification_ok() && self.resolution_ok()
    }

    pub fn needs_action(&self) -> bool {
        !self.detect_ok || !self.verify_ok
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
    fn test_identification() {
        let c = RecallMgr::new();
        assert!(c.identification_ok());
    }

    #[test]
    fn test_resolution() {
        let c = RecallMgr::new();
        assert!(c.resolution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RecallMgr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_action() {
        let c = RecallMgr::new();
        assert!(!c.needs_action());
    }

    #[test]
    fn test_detect() {
        let mut c = RecallMgr::new();
        c.detect_ok = false;
        assert!(c.needs_action());
    }

    #[test]
    fn test_health() {
        let c = RecallMgr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
