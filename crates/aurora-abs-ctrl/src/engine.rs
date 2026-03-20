/// abs ctrl: sense, pulse, modulate, release, log
/// Phase 1158

#[derive(Debug, Clone)]
pub struct AbsCtrl {
    pub sense_ok: bool,
    pub pulse_ok: bool,
    pub modulate_ok: bool,
    pub release_ok: bool,
    pub log_ok: bool,
}

impl Default for AbsCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl AbsCtrl {
    pub fn new() -> Self {
        Self {
            sense_ok: true,
            pulse_ok: true,
            modulate_ok: true,
            release_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sense_ok && self.pulse_ok && self.modulate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.release_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sense_ok || !self.pulse_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sense_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = AbsCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AbsCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AbsCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AbsCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AbsCtrl::new();
        c.sense_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AbsCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
