/// spat decode: phase, timing, countdown, transition, conflict
/// Phase 1129

#[derive(Debug, Clone)]
pub struct SpatDecode {
    pub phase_ok: bool,
    pub timing_ok: bool,
    pub countdown_ok: bool,
    pub transition_ok: bool,
    pub conflict_ok: bool,
}

impl Default for SpatDecode {
    fn default() -> Self {
        Self::new()
    }
}

impl SpatDecode {
    pub fn new() -> Self {
        Self {
            phase_ok: true,
            timing_ok: true,
            countdown_ok: true,
            transition_ok: true,
            conflict_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.phase_ok && self.timing_ok && self.countdown_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.transition_ok && self.conflict_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.phase_ok || !self.timing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.phase_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SpatDecode::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SpatDecode::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SpatDecode::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SpatDecode::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SpatDecode::new();
        c.phase_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SpatDecode::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
