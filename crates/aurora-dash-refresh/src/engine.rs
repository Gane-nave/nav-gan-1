/// aurora-dash-refresh: dash refresh
/// Phase 2447

#[derive(Debug, Clone)]
pub struct DashRefresh {
    pub auto_ok: bool,
    pub manual_ok: bool,
    pub schedule_ok: bool,
    pub cancel_ok: bool,
    pub pause_ok: bool,
}

impl Default for DashRefresh {
    fn default() -> Self {
        Self::new()
    }
}

impl DashRefresh {
    pub fn new() -> Self {
        Self {
            auto_ok: true,
            manual_ok: true,
            schedule_ok: true,
            cancel_ok: true,
            pause_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.auto_ok && self.manual_ok && self.schedule_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.pause_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.auto_ok || !self.manual_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.auto_ok {
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
        let c = DashRefresh::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashRefresh::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashRefresh::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashRefresh::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashRefresh::new();
        c.auto_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashRefresh::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashRefresh::default();
        assert!(c.all_ok());
    }
}
