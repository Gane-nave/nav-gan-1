/// dc-dc conv: step_up, step_down, regulate, protect, report
/// Phase 1147

#[derive(Debug, Clone)]
pub struct DcDcConv {
    pub step_up_ok: bool,
    pub step_down_ok: bool,
    pub regulate_ok: bool,
    pub protect_ok: bool,
    pub report_ok: bool,
}

impl Default for DcDcConv {
    fn default() -> Self {
        Self::new()
    }
}

impl DcDcConv {
    pub fn new() -> Self {
        Self {
            step_up_ok: true,
            step_down_ok: true,
            regulate_ok: true,
            protect_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.step_up_ok && self.step_down_ok && self.regulate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.protect_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.step_up_ok || !self.step_down_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.step_up_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DcDcConv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DcDcConv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DcDcConv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DcDcConv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DcDcConv::new();
        c.step_up_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DcDcConv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
