/// rt hook: register, invoke, remove, list, log
/// Phase 2338

#[derive(Debug, Clone)]
pub struct RtHook {
    pub register_ok: bool,
    pub invoke_ok: bool,
    pub remove_ok: bool,
    pub list_ok: bool,
    pub log_ok: bool,
}

impl Default for RtHook {
    fn default() -> Self {
        Self::new()
    }
}

impl RtHook {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            invoke_ok: true,
            remove_ok: true,
            list_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.invoke_ok && self.remove_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.list_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.invoke_ok
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
        let c = RtHook::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtHook::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtHook::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtHook::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtHook::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtHook::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
