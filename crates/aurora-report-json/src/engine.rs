/// report json: serialize, schema, validate, stream, log
/// Phase 1566

#[derive(Debug, Clone)]
pub struct ReportJson {
    pub serialize_ok: bool,
    pub schema_ok: bool,
    pub validate_ok: bool,
    pub stream_ok: bool,
    pub log_ok: bool,
}

impl Default for ReportJson {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportJson {
    pub fn new() -> Self {
        Self {
            serialize_ok: true,
            schema_ok: true,
            validate_ok: true,
            stream_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.serialize_ok && self.schema_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stream_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.serialize_ok || !self.schema_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.serialize_ok {
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
        let c = ReportJson::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ReportJson::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ReportJson::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ReportJson::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ReportJson::new();
        c.serialize_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ReportJson::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
