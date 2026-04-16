/// Usage-based insurance: mileage, driving score, incident
/// Phase 893

#[derive(Debug, Clone)]
pub struct UsageInsurance {
    pub mileage_ok: bool,
    pub score_ok: bool,
    pub incident_ok: bool,
    pub upload_ok: bool,
    pub privacy_ok: bool,
}

impl Default for UsageInsurance {
    fn default() -> Self {
        Self::new()
    }
}

impl UsageInsurance {
    pub fn new() -> Self {
        Self {
            mileage_ok: true,
            score_ok: true,
            incident_ok: true,
            upload_ok: true,
            privacy_ok: true,
        }
    }

    pub fn data_ok(&self) -> bool {
        self.mileage_ok && self.score_ok && self.incident_ok
    }

    pub fn compliance_ok(&self) -> bool {
        self.upload_ok && self.privacy_ok
    }

    pub fn all_ok(&self) -> bool {
        self.data_ok() && self.compliance_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.upload_ok || !self.privacy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.upload_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let c = UsageInsurance::new();
        assert!(c.data_ok());
    }

    #[test]
    fn test_compliance() {
        let c = UsageInsurance::new();
        assert!(c.compliance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UsageInsurance::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = UsageInsurance::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_upload() {
        let mut c = UsageInsurance::new();
        c.upload_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = UsageInsurance::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
