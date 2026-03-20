/// center console: switch, dial, usb, storage, check
/// Phase 1279

#[derive(Debug, Clone)]
pub struct CenterConsole {
    pub switch_ok: bool,
    pub dial_ok: bool,
    pub usb_ok: bool,
    pub storage_ok: bool,
    pub check_ok: bool,
}

impl Default for CenterConsole {
    fn default() -> Self {
        Self::new()
    }
}

impl CenterConsole {
    pub fn new() -> Self {
        Self {
            switch_ok: true,
            dial_ok: true,
            usb_ok: true,
            storage_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.switch_ok && self.dial_ok && self.usb_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.storage_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.switch_ok || !self.dial_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.switch_ok {
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
        let c = CenterConsole::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CenterConsole::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CenterConsole::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CenterConsole::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CenterConsole::new();
        c.switch_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CenterConsole::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
