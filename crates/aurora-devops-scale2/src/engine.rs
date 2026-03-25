/// devops scale2: up, down, auto, predict, log
/// Phase 2182

#[derive(Debug, Clone)]
pub struct DevopsScale2 {
    pub up_ok: bool,
    pub down_ok: bool,
    pub auto_ok: bool,
    pub predict_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsScale2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsScale2 {
    pub fn new() -> Self {
        Self {
            up_ok: true,
            down_ok: true,
            auto_ok: true,
            predict_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.up_ok && self.down_ok && self.auto_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.predict_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.up_ok || !self.down_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.up_ok {
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
        let c = DevopsScale2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsScale2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsScale2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsScale2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsScale2::new();
        c.up_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsScale2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
