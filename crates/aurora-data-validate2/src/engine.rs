/// data validate2: schema, quality, anomaly, report, log
/// Phase 2205

#[derive(Debug, Clone)]
pub struct DataValidate2 {
    pub schema_ok: bool,
    pub quality_ok: bool,
    pub anomaly_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for DataValidate2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataValidate2 {
    pub fn new() -> Self {
        Self {
            schema_ok: true,
            quality_ok: true,
            anomaly_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.schema_ok && self.quality_ok && self.anomaly_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.schema_ok || !self.quality_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.schema_ok {
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
        let c = DataValidate2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataValidate2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataValidate2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataValidate2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataValidate2::new();
        c.schema_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataValidate2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
