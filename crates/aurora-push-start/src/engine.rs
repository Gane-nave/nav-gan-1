/// push start: detect, authorize, crank, run, stop
/// Phase 1285

#[derive(Debug, Clone)]
pub struct PushStart {
    pub detect_ok: bool,
    pub authorize_ok: bool,
    pub crank_ok: bool,
    pub run_ok: bool,
    pub stop_ok: bool,
}

impl Default for PushStart {
    fn default() -> Self {
        Self::new()
    }
}

impl PushStart {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            authorize_ok: true,
            crank_ok: true,
            run_ok: true,
            stop_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.authorize_ok && self.crank_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.run_ok && self.stop_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.authorize_ok
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
        let c = PushStart::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PushStart::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PushStart::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PushStart::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PushStart::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PushStart::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
