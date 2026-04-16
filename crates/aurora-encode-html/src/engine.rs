/// encode html: escape, unescape, sanitize, validate, log
/// Phase 1702

#[derive(Debug, Clone)]
pub struct EncodeHtml {
    pub escape_ok: bool,
    pub unescape_ok: bool,
    pub sanitize_ok: bool,
    pub validate_ok: bool,
    pub log_ok: bool,
}

impl Default for EncodeHtml {
    fn default() -> Self {
        Self::new()
    }
}

impl EncodeHtml {
    pub fn new() -> Self {
        Self {
            escape_ok: true,
            unescape_ok: true,
            sanitize_ok: true,
            validate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.escape_ok && self.unescape_ok && self.sanitize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.validate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.escape_ok || !self.unescape_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.escape_ok {
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
        let c = EncodeHtml::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EncodeHtml::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EncodeHtml::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EncodeHtml::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EncodeHtml::new();
        c.escape_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EncodeHtml::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
