/// Keyless entry: transponder, antenna, rolling code, range
/// Phase 732

#[derive(Debug, Clone)]
pub struct KeylessEntry {
    pub transponder_ok: bool,
    pub antenna_ok: bool,
    pub rolling_code_ok: bool,
    pub range_ok: bool,
    pub battery_ok: bool,
}

impl Default for KeylessEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl KeylessEntry {
    pub fn new() -> Self {
        Self {
            transponder_ok: true,
            antenna_ok: true,
            rolling_code_ok: true,
            range_ok: true,
            battery_ok: true,
        }
    }

    pub fn security_ok(&self) -> bool {
        self.transponder_ok && self.rolling_code_ok
    }

    pub fn signal_ok(&self) -> bool {
        self.antenna_ok && self.range_ok && self.battery_ok
    }

    pub fn all_ok(&self) -> bool {
        self.security_ok() && self.signal_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.transponder_ok || !self.battery_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.transponder_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security() {
        let c = KeylessEntry::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_signal() {
        let c = KeylessEntry::new();
        assert!(c.signal_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = KeylessEntry::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = KeylessEntry::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_transponder() {
        let mut c = KeylessEntry::new();
        c.transponder_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = KeylessEntry::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
