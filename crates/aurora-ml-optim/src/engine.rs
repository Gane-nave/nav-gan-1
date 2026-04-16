/// ml optim: step, zero, schedule, clip, log
/// Phase 1965

#[derive(Debug, Clone)]
pub struct MlOptim {
    pub step_ok: bool,
    pub zero_ok: bool,
    pub schedule_ok: bool,
    pub clip_ok: bool,
    pub log_ok: bool,
}

impl Default for MlOptim {
    fn default() -> Self {
        Self::new()
    }
}

impl MlOptim {
    pub fn new() -> Self {
        Self {
            step_ok: true,
            zero_ok: true,
            schedule_ok: true,
            clip_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.step_ok && self.zero_ok && self.schedule_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clip_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.step_ok || !self.zero_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.step_ok {
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
        let c = MlOptim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlOptim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlOptim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlOptim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlOptim::new();
        c.step_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlOptim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
