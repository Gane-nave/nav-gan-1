/// hub motor: drive, regen, cool, sense, report
/// Phase 1221

#[derive(Debug, Clone)]
pub struct HubMotor {
    pub drive_ok: bool,
    pub regen_ok: bool,
    pub cool_ok: bool,
    pub sense_ok: bool,
    pub report_ok: bool,
}

impl Default for HubMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl HubMotor {
    pub fn new() -> Self {
        Self {
            drive_ok: true,
            regen_ok: true,
            cool_ok: true,
            sense_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.drive_ok && self.regen_ok && self.cool_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.sense_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.drive_ok || !self.regen_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.drive_ok {
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
        let c = HubMotor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HubMotor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HubMotor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HubMotor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HubMotor::new();
        c.drive_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HubMotor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
