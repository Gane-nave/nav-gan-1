/// window ctrl: open, close, lock, child, rain
/// Phase 1316

#[derive(Debug, Clone)]
pub struct WindowCtrl2 {
    pub open_ok: bool,
    pub close_ok: bool,
    pub lock_ok: bool,
    pub child_ok: bool,
    pub rain_ok: bool,
}

impl Default for WindowCtrl2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowCtrl2 {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            close_ok: true,
            lock_ok: true,
            child_ok: true,
            rain_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.close_ok && self.lock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.child_ok && self.rain_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.close_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok {
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
        let c = WindowCtrl2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WindowCtrl2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WindowCtrl2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WindowCtrl2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WindowCtrl2::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WindowCtrl2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
