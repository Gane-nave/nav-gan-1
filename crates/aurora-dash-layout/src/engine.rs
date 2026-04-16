/// aurora-dash-layout: dash layout
/// Phase 2443

#[derive(Debug, Clone)]
pub struct DashLayout {
    pub arrange_ok: bool,
    pub resize_ok: bool,
    pub lock_ok: bool,
    pub save_ok: bool,
    pub restore_ok: bool,
}

impl Default for DashLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl DashLayout {
    pub fn new() -> Self {
        Self {
            arrange_ok: true,
            resize_ok: true,
            lock_ok: true,
            save_ok: true,
            restore_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.arrange_ok && self.resize_ok && self.lock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.save_ok && self.restore_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.arrange_ok || !self.resize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.arrange_ok {
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
        let c = DashLayout::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashLayout::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashLayout::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashLayout::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashLayout::new();
        c.arrange_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashLayout::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashLayout::default();
        assert!(c.all_ok());
    }
}
