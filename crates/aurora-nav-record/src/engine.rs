/// Navigation recording: trip log, waypoint, replay, export
/// Phase 906

#[derive(Debug, Clone)]
pub struct NavRecord {
    pub trip_ok: bool,
    pub waypoint_ok: bool,
    pub replay_ok: bool,
    pub export_ok: bool,
    pub storage_ok: bool,
}

impl Default for NavRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl NavRecord {
    pub fn new() -> Self {
        Self {
            trip_ok: true,
            waypoint_ok: true,
            replay_ok: true,
            export_ok: true,
            storage_ok: true,
        }
    }

    pub fn recording_ok(&self) -> bool {
        self.trip_ok && self.waypoint_ok && self.storage_ok
    }

    pub fn playback_ok(&self) -> bool {
        self.replay_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.recording_ok() && self.playback_ok()
    }

    pub fn needs_cleanup(&self) -> bool {
        !self.storage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.storage_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording() {
        let c = NavRecord::new();
        assert!(c.recording_ok());
    }

    #[test]
    fn test_playback() {
        let c = NavRecord::new();
        assert!(c.playback_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NavRecord::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cleanup() {
        let c = NavRecord::new();
        assert!(!c.needs_cleanup());
    }

    #[test]
    fn test_storage() {
        let mut c = NavRecord::new();
        c.storage_ok = false;
        assert!(c.needs_cleanup());
    }

    #[test]
    fn test_health() {
        let c = NavRecord::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
