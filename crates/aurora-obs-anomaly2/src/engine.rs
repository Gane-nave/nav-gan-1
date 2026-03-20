/// obs anomaly2: detect, classify, alert, resolve, log
/// Phase 2157

#[derive(Debug, Clone)]
pub struct ObsAnomaly2 {
    pub detect_ok: bool,
    pub classify_ok: bool,
    pub alert_ok: bool,
    pub resolve_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsAnomaly2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsAnomaly2 {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            classify_ok: true,
            alert_ok: true,
            resolve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.classify_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.resolve_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = ObsAnomaly2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsAnomaly2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsAnomaly2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsAnomaly2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsAnomaly2::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsAnomaly2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
