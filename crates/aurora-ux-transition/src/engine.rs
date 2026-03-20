/// ux transition: define, trigger, animate, complete, log
/// Phase 1504

#[derive(Debug, Clone)]
pub struct UxTransition {
    pub define_ok: bool,
    pub trigger_ok: bool,
    pub animate_ok: bool,
    pub complete_ok: bool,
    pub log_ok: bool,
}

impl Default for UxTransition {
    fn default() -> Self {
        Self::new()
    }
}

impl UxTransition {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            trigger_ok: true,
            animate_ok: true,
            complete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.trigger_ok && self.animate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.complete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.trigger_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = UxTransition::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxTransition::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxTransition::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxTransition::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxTransition::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxTransition::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
