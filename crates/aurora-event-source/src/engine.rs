/// event source: apply, replay, project, snapshot, log
/// Phase 1862

#[derive(Debug, Clone)]
pub struct EventSource {
    pub apply_ok: bool,
    pub replay_ok: bool,
    pub project_ok: bool,
    pub snapshot_ok: bool,
    pub log_ok: bool,
}

impl Default for EventSource {
    fn default() -> Self {
        Self::new()
    }
}

impl EventSource {
    pub fn new() -> Self {
        Self {
            apply_ok: true,
            replay_ok: true,
            project_ok: true,
            snapshot_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.apply_ok && self.replay_ok && self.project_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.snapshot_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.apply_ok || !self.replay_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.apply_ok {
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
        let c = EventSource::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventSource::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventSource::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventSource::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventSource::new();
        c.apply_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventSource::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
