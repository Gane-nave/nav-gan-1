/// rt plugin2: load, init, execute, unload, log
/// Phase 2339

#[derive(Debug, Clone)]
pub struct RtPlugin2 {
    pub load_ok: bool,
    pub init_ok: bool,
    pub execute_ok: bool,
    pub unload_ok: bool,
    pub log_ok: bool,
}

impl Default for RtPlugin2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtPlugin2 {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            init_ok: true,
            execute_ok: true,
            unload_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.init_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.unload_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.init_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
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
        let c = RtPlugin2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtPlugin2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtPlugin2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtPlugin2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtPlugin2::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtPlugin2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
