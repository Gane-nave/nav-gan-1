/// window ctrl: up, down, lock, auto, pinch
/// Phase 1188

#[derive(Debug, Clone)]
pub struct WindowCtrl {
    pub up_ok: bool,
    pub down_ok: bool,
    pub lock_ok: bool,
    pub auto_ok: bool,
    pub pinch_ok: bool,
}

impl Default for WindowCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowCtrl {
    pub fn new() -> Self {
        Self {
            up_ok: true,
            down_ok: true,
            lock_ok: true,
            auto_ok: true,
            pinch_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.up_ok && self.down_ok && self.lock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.auto_ok && self.pinch_ok
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
        let c = WindowCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WindowCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WindowCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WindowCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WindowCtrl::new();
        c.up_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WindowCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
