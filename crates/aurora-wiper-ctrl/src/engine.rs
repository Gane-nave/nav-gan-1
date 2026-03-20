/// wiper ctrl: low, high, interval, wash, park
/// Phase 1192

#[derive(Debug, Clone)]
pub struct WiperCtrl {
    pub low_ok: bool,
    pub high_ok: bool,
    pub interval_ok: bool,
    pub wash_ok: bool,
    pub park_ok: bool,
}

impl Default for WiperCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl WiperCtrl {
    pub fn new() -> Self {
        Self {
            low_ok: true,
            high_ok: true,
            interval_ok: true,
            wash_ok: true,
            park_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.low_ok && self.high_ok && self.interval_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.wash_ok && self.park_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.low_ok || !self.high_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.low_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = WiperCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WiperCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WiperCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WiperCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WiperCtrl::new();
        c.low_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WiperCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
