/// sleep mode: detect, prepare, enter, maintain, wake
/// Phase 1373

#[derive(Debug, Clone)]
pub struct SleepMode {
    pub detect_ok: bool,
    pub prepare_ok: bool,
    pub enter_ok: bool,
    pub maintain_ok: bool,
    pub wake_ok: bool,
}

impl Default for SleepMode {
    fn default() -> Self {
        Self::new()
    }
}

impl SleepMode {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            prepare_ok: true,
            enter_ok: true,
            maintain_ok: true,
            wake_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.prepare_ok && self.enter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.maintain_ok && self.wake_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.prepare_ok
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
        let c = SleepMode::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SleepMode::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SleepMode::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SleepMode::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SleepMode::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SleepMode::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
