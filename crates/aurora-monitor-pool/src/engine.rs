/// monitor pool: active, idle, create, destroy, log
/// Phase 1582

#[derive(Debug, Clone)]
pub struct MonitorPool {
    pub active_ok: bool,
    pub idle_ok: bool,
    pub create_ok: bool,
    pub destroy_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorPool {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorPool {
    pub fn new() -> Self {
        Self {
            active_ok: true,
            idle_ok: true,
            create_ok: true,
            destroy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.active_ok && self.idle_ok && self.create_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.destroy_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.active_ok || !self.idle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.active_ok {
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
        let c = MonitorPool::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorPool::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorPool::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorPool::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorPool::new();
        c.active_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorPool::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
