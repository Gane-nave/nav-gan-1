/// safety train: schedule, deliver, assess, certify, log
/// Phase 1487

#[derive(Debug, Clone)]
pub struct SafetyTrain2 {
    pub schedule_ok: bool,
    pub deliver_ok: bool,
    pub assess_ok: bool,
    pub certify_ok: bool,
    pub log_ok: bool,
}

impl Default for SafetyTrain2 {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyTrain2 {
    pub fn new() -> Self {
        Self {
            schedule_ok: true,
            deliver_ok: true,
            assess_ok: true,
            certify_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schedule_ok && self.deliver_ok && self.assess_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.certify_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schedule_ok || !self.deliver_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schedule_ok {
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
        let c = SafetyTrain2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SafetyTrain2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SafetyTrain2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SafetyTrain2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SafetyTrain2::new();
        c.schedule_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SafetyTrain2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
