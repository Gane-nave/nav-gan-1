/// freeze frame: capture, store, retrieve, compare, log
/// Phase 1376

#[derive(Debug, Clone)]
pub struct FreezeFrame {
    pub capture_ok: bool,
    pub store_ok: bool,
    pub retrieve_ok: bool,
    pub compare_ok: bool,
    pub log_ok: bool,
}

impl Default for FreezeFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl FreezeFrame {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            store_ok: true,
            retrieve_ok: true,
            compare_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.store_ok && self.retrieve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compare_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.store_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = FreezeFrame::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FreezeFrame::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FreezeFrame::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FreezeFrame::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FreezeFrame::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FreezeFrame::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
