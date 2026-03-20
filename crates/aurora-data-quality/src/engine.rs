/// Data quality: profile, validate, cleanse, enrich, monitor
/// Phase 1038

#[derive(Debug, Clone)]
pub struct DataQuality {
    pub profile_ok: bool,
    pub validate_ok: bool,
    pub cleanse_ok: bool,
    pub enrich_ok: bool,
    pub monitor_ok: bool,
}

impl Default for DataQuality {
    fn default() -> Self {
        Self::new()
    }
}

impl DataQuality {
    pub fn new() -> Self {
        Self {
            profile_ok: true,
            validate_ok: true,
            cleanse_ok: true,
            enrich_ok: true,
            monitor_ok: true,
        }
    }

    pub fn assessment_ok(&self) -> bool {
        self.profile_ok && self.validate_ok && self.cleanse_ok
    }

    pub fn improvement_ok(&self) -> bool {
        self.enrich_ok && self.monitor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.assessment_ok() && self.improvement_ok()
    }

    pub fn needs_scan(&self) -> bool {
        !self.profile_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.profile_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assessment() {
        let c = DataQuality::new();
        assert!(c.assessment_ok());
    }

    #[test]
    fn test_improvement() {
        let c = DataQuality::new();
        assert!(c.improvement_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataQuality::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_scan() {
        let c = DataQuality::new();
        assert!(!c.needs_scan());
    }

    #[test]
    fn test_profile() {
        let mut c = DataQuality::new();
        c.profile_ok = false;
        assert!(c.needs_scan());
    }

    #[test]
    fn test_health() {
        let c = DataQuality::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
