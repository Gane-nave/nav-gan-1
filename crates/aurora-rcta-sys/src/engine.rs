/// rcta sys: monitor, detect, alert, brake, clear
/// Phase 1164

#[derive(Debug, Clone)]
pub struct RctaSys {
    pub monitor_ok: bool,
    pub detect_ok: bool,
    pub alert_ok: bool,
    pub brake_ok: bool,
    pub clear_ok: bool,
}

impl Default for RctaSys {
    fn default() -> Self {
        Self::new()
    }
}

impl RctaSys {
    pub fn new() -> Self {
        Self {
            monitor_ok: true,
            detect_ok: true,
            alert_ok: true,
            brake_ok: true,
            clear_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.monitor_ok && self.detect_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.brake_ok && self.clear_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.monitor_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.monitor_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = RctaSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RctaSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RctaSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RctaSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RctaSys::new();
        c.monitor_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RctaSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
