/// graph temporal: snapshot, diff, merge, replay, log
/// Phase 1910

#[derive(Debug, Clone)]
pub struct GraphTemporal {
    pub snapshot_ok: bool,
    pub diff_ok: bool,
    pub merge_ok: bool,
    pub replay_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphTemporal {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphTemporal {
    pub fn new() -> Self {
        Self {
            snapshot_ok: true,
            diff_ok: true,
            merge_ok: true,
            replay_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.snapshot_ok && self.diff_ok && self.merge_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.replay_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.snapshot_ok || !self.diff_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.snapshot_ok {
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
        let c = GraphTemporal::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphTemporal::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphTemporal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphTemporal::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphTemporal::new();
        c.snapshot_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphTemporal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
