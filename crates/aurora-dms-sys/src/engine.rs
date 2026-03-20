/// dms sys: track, analyze, alert, escalate, log
/// Phase 1173

#[derive(Debug, Clone)]
pub struct DmsSys {
    pub track_ok: bool,
    pub analyze_ok: bool,
    pub alert_ok: bool,
    pub escalate_ok: bool,
    pub log_ok: bool,
}

impl Default for DmsSys {
    fn default() -> Self {
        Self::new()
    }
}

impl DmsSys {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            analyze_ok: true,
            alert_ok: true,
            escalate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.track_ok && self.analyze_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.escalate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.track_ok || !self.analyze_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.track_ok {
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
        let c = DmsSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DmsSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DmsSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DmsSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DmsSys::new();
        c.track_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DmsSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
