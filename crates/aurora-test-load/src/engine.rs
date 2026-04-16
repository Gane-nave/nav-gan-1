/// test load: ramp, sustain, spike, recover, log
/// Phase 1522

#[derive(Debug, Clone)]
pub struct TestLoad {
    pub ramp_ok: bool,
    pub sustain_ok: bool,
    pub spike_ok: bool,
    pub recover_ok: bool,
    pub log_ok: bool,
}

impl Default for TestLoad {
    fn default() -> Self {
        Self::new()
    }
}

impl TestLoad {
    pub fn new() -> Self {
        Self {
            ramp_ok: true,
            sustain_ok: true,
            spike_ok: true,
            recover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.ramp_ok && self.sustain_ok && self.spike_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recover_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.ramp_ok || !self.sustain_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ramp_ok {
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
        let c = TestLoad::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TestLoad::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TestLoad::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TestLoad::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TestLoad::new();
        c.ramp_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TestLoad::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
