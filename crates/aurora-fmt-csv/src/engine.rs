/// fmt csv: parse, serialize, validate, transform, log
/// Phase 1666

#[derive(Debug, Clone)]
pub struct FmtCsv {
    pub parse_ok: bool,
    pub serialize_ok: bool,
    pub validate_ok: bool,
    pub transform_ok: bool,
    pub log_ok: bool,
}

impl Default for FmtCsv {
    fn default() -> Self {
        Self::new()
    }
}

impl FmtCsv {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            serialize_ok: true,
            validate_ok: true,
            transform_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.serialize_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.transform_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.serialize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = FmtCsv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FmtCsv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FmtCsv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FmtCsv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FmtCsv::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FmtCsv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
