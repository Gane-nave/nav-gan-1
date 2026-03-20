/// keyless entry: detect, authenticate, unlock, lock, log
/// Phase 1284

#[derive(Debug, Clone)]
pub struct KeylessEntry {
    pub detect_ok: bool,
    pub authenticate_ok: bool,
    pub unlock_ok: bool,
    pub lock_ok: bool,
    pub log_ok: bool,
}

impl Default for KeylessEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl KeylessEntry {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            authenticate_ok: true,
            unlock_ok: true,
            lock_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.authenticate_ok && self.unlock_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.lock_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.authenticate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = KeylessEntry::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = KeylessEntry::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = KeylessEntry::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = KeylessEntry::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = KeylessEntry::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = KeylessEntry::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
