/// data quality2: profile, monitor, alert, remediate, log
/// Phase 2210

#[derive(Debug, Clone)]
pub struct DataQuality2 {
    pub profile_ok: bool,
    pub monitor_ok: bool,
    pub alert_ok: bool,
    pub remediate_ok: bool,
    pub log_ok: bool,
}

impl Default for DataQuality2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataQuality2 {
    pub fn new() -> Self {
        Self {
            profile_ok: true,
            monitor_ok: true,
            alert_ok: true,
            remediate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.profile_ok && self.monitor_ok && self.alert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.remediate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.profile_ok || !self.monitor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.profile_ok {
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
        let c = DataQuality2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataQuality2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataQuality2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataQuality2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataQuality2::new();
        c.profile_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataQuality2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
