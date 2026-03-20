/// monitor conn: active, idle, create, timeout, log
/// Phase 1583

#[derive(Debug, Clone)]
pub struct MonitorConn {
    pub active_ok: bool,
    pub idle_ok: bool,
    pub create_ok: bool,
    pub timeout_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorConn {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorConn {
    pub fn new() -> Self {
        Self {
            active_ok: true,
            idle_ok: true,
            create_ok: true,
            timeout_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.active_ok && self.idle_ok && self.create_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.timeout_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.active_ok || !self.idle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.active_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MonitorConn::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorConn::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorConn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorConn::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorConn::new();
        c.active_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorConn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
