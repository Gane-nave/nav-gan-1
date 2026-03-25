/// wf trigger2: register, evaluate, fire, disable, log
/// Phase 2230

#[derive(Debug, Clone)]
pub struct WfTrigger2 {
    pub register_ok: bool,
    pub evaluate_ok: bool,
    pub fire_ok: bool,
    pub disable_ok: bool,
    pub log_ok: bool,
}

impl Default for WfTrigger2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WfTrigger2 {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            evaluate_ok: true,
            fire_ok: true,
            disable_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.evaluate_ok && self.fire_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.disable_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.evaluate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok {
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
        let c = WfTrigger2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfTrigger2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfTrigger2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfTrigger2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfTrigger2::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfTrigger2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
