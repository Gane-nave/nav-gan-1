/// enc html: encode, decode, sanitize, strip, log
/// Phase 2298

#[derive(Debug, Clone)]
pub struct EncHtml {
    pub encode_ok: bool,
    pub decode_ok: bool,
    pub sanitize_ok: bool,
    pub strip_ok: bool,
    pub log_ok: bool,
}

impl Default for EncHtml {
    fn default() -> Self {
        Self::new()
    }
}

impl EncHtml {
    pub fn new() -> Self {
        Self {
            encode_ok: true,
            decode_ok: true,
            sanitize_ok: true,
            strip_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.encode_ok && self.decode_ok && self.sanitize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.strip_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.encode_ok || !self.decode_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.encode_ok {
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
        let c = EncHtml::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EncHtml::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EncHtml::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EncHtml::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EncHtml::new();
        c.encode_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EncHtml::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
