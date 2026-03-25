/// uwb radio: range, anchor, locate, track, log
/// Phase 1344

#[derive(Debug, Clone)]
pub struct UwbRadio {
    pub range_ok: bool,
    pub anchor_ok: bool,
    pub locate_ok: bool,
    pub track_ok: bool,
    pub log_ok: bool,
}

impl Default for UwbRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl UwbRadio {
    pub fn new() -> Self {
        Self {
            range_ok: true,
            anchor_ok: true,
            locate_ok: true,
            track_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.range_ok && self.anchor_ok && self.locate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.track_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.range_ok || !self.anchor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.range_ok {
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
        let c = UwbRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UwbRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UwbRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UwbRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UwbRadio::new();
        c.range_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UwbRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
