/// enc msgpack2: encode, decode, validate, stream, log
/// Phase 2302

#[derive(Debug, Clone)]
pub struct EncMsgpack2 {
    pub encode_ok: bool,
    pub decode_ok: bool,
    pub validate_ok: bool,
    pub stream_ok: bool,
    pub log_ok: bool,
}

impl Default for EncMsgpack2 {
    fn default() -> Self {
        Self::new()
    }
}

impl EncMsgpack2 {
    pub fn new() -> Self {
        Self {
            encode_ok: true,
            decode_ok: true,
            validate_ok: true,
            stream_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.encode_ok && self.decode_ok && self.validate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stream_ok && self.log_ok
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
        let c = EncMsgpack2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EncMsgpack2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EncMsgpack2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EncMsgpack2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EncMsgpack2::new();
        c.encode_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EncMsgpack2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
