/// CDC engine: capture, decode, apply, conflict, replay
/// Phase 1035

#[derive(Debug, Clone)]
pub struct CdcEngine {
    pub capture_ok: bool,
    pub decode_ok: bool,
    pub apply_ok: bool,
    pub conflict_ok: bool,
    pub replay_ok: bool,
}

impl Default for CdcEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CdcEngine {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            decode_ok: true,
            apply_ok: true,
            conflict_ok: true,
            replay_ok: true,
        }
    }

    pub fn replication_ok(&self) -> bool {
        self.capture_ok && self.decode_ok && self.apply_ok
    }

    pub fn recovery_ok(&self) -> bool {
        self.conflict_ok && self.replay_ok
    }

    pub fn all_ok(&self) -> bool {
        self.replication_ok() && self.recovery_ok()
    }

    pub fn needs_resync(&self) -> bool {
        !self.capture_ok || !self.apply_ok
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
    fn test_replication() {
        let c = CdcEngine::new();
        assert!(c.replication_ok());
    }

    #[test]
    fn test_recovery() {
        let c = CdcEngine::new();
        assert!(c.recovery_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CdcEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_resync() {
        let c = CdcEngine::new();
        assert!(!c.needs_resync());
    }

    #[test]
    fn test_capture() {
        let mut c = CdcEngine::new();
        c.capture_ok = false;
        assert!(c.needs_resync());
    }

    #[test]
    fn test_health() {
        let c = CdcEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
