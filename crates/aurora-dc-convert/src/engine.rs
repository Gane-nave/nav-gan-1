/// dc convert: step, regulate, protect, monitor, log
/// Phase 1360

#[derive(Debug, Clone)]
pub struct DcConvert {
    pub step_ok: bool,
    pub regulate_ok: bool,
    pub protect_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for DcConvert {
    fn default() -> Self {
        Self::new()
    }
}

impl DcConvert {
    pub fn new() -> Self {
        Self {
            step_ok: true,
            regulate_ok: true,
            protect_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.step_ok && self.regulate_ok && self.protect_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.step_ok || !self.regulate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.step_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DcConvert::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DcConvert::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DcConvert::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DcConvert::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DcConvert::new();
        c.step_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DcConvert::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
