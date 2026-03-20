/// fuse box: monitor, protect, isolate, reset, log
/// Phase 1367

#[derive(Debug, Clone)]
pub struct FuseBox {
    pub monitor_ok: bool,
    pub protect_ok: bool,
    pub isolate_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for FuseBox {
    fn default() -> Self {
        Self::new()
    }
}

impl FuseBox {
    pub fn new() -> Self {
        Self {
            monitor_ok: true,
            protect_ok: true,
            isolate_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.monitor_ok && self.protect_ok && self.isolate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.monitor_ok || !self.protect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.monitor_ok {
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
        let c = FuseBox::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuseBox::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuseBox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuseBox::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuseBox::new();
        c.monitor_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuseBox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
