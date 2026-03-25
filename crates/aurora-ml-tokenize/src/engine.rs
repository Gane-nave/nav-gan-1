/// ml tokenize: split, encode, decode, vocab, log
/// Phase 1962

#[derive(Debug, Clone)]
pub struct MlTokenize {
    pub split_ok: bool,
    pub encode_ok: bool,
    pub decode_ok: bool,
    pub vocab_ok: bool,
    pub log_ok: bool,
}

impl Default for MlTokenize {
    fn default() -> Self {
        Self::new()
    }
}

impl MlTokenize {
    pub fn new() -> Self {
        Self {
            split_ok: true,
            encode_ok: true,
            decode_ok: true,
            vocab_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.split_ok && self.encode_ok && self.decode_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.vocab_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.split_ok || !self.encode_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.split_ok {
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
        let c = MlTokenize::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlTokenize::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlTokenize::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlTokenize::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlTokenize::new();
        c.split_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlTokenize::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
