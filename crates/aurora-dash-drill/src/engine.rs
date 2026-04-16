/// aurora-dash-drill: dash drill
/// Phase 2454

#[derive(Debug, Clone)]
pub struct DashDrill {
    pub down_ok: bool,
    pub up_ok: bool,
    pub filter_ok: bool,
    pub context_ok: bool,
    pub breadcrumb_ok: bool,
}

impl Default for DashDrill {
    fn default() -> Self {
        Self::new()
    }
}

impl DashDrill {
    pub fn new() -> Self {
        Self {
            down_ok: true,
            up_ok: true,
            filter_ok: true,
            context_ok: true,
            breadcrumb_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.down_ok && self.up_ok && self.filter_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.context_ok && self.breadcrumb_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.down_ok || !self.up_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.down_ok {
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
        let c = DashDrill::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashDrill::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashDrill::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashDrill::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashDrill::new();
        c.down_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashDrill::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashDrill::default();
        assert!(c.all_ok());
    }
}
