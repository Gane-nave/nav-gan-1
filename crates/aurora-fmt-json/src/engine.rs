/// fmt json: parse, serialize, validate, pretty, log
/// Phase 1662

#[derive(Debug, Clone)]
pub struct FmtJson {
    pub parse_ok: bool,
    pub serialize_ok: bool,
    pub validate_ok: bool,
    pub pretty_ok: bool,
    pub log_ok: bool,
}

impl Default for FmtJson {
    fn default() -> Self {
        Self::new()
    }
}

impl FmtJson {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            serialize_ok: true,
            validate_ok: true,
            pretty_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.serialize_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.pretty_ok && self.log_ok
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
        let c = FmtJson::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FmtJson::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FmtJson::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FmtJson::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FmtJson::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FmtJson::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
