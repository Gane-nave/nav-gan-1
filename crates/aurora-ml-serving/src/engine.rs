/// ml serving: load, serve, scale, monitor, log
/// Phase 1957

#[derive(Debug, Clone)]
pub struct MlServing {
    pub load_ok: bool,
    pub serve_ok: bool,
    pub scale_ok: bool,
    pub monitor_ok: bool,
    pub log_ok: bool,
}

impl Default for MlServing {
    fn default() -> Self {
        Self::new()
    }
}

impl MlServing {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            serve_ok: true,
            scale_ok: true,
            monitor_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.serve_ok && self.scale_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.serve_ok
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
        let c = MlServing::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlServing::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlServing::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlServing::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlServing::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlServing::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
