/// aurora-sim-time2: sim time2
/// Phase 2528

#[derive(Debug, Clone)]
pub struct SimTime2 {
    pub advance_ok: bool,
    pub freeze_ok: bool,
    pub rewind_ok: bool,
    pub scale_ok: bool,
    pub sync_ok: bool,
}

impl Default for SimTime2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SimTime2 {
    pub fn new() -> Self {
        Self {
            advance_ok: true,
            freeze_ok: true,
            rewind_ok: true,
            scale_ok: true,
            sync_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.advance_ok && self.freeze_ok && self.rewind_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.scale_ok && self.sync_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.advance_ok || !self.freeze_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.advance_ok {
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
        let c = SimTime2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimTime2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimTime2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimTime2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimTime2::new();
        c.advance_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimTime2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimTime2::default();
        assert!(c.all_ok());
    }
}
