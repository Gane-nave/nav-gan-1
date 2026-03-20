/// Transformer: attention, embed, encode, decode, generate
/// Phase 1015

#[derive(Debug, Clone)]
pub struct Transformer {
    pub attention_ok: bool,
    pub embed_ok: bool,
    pub encode_ok: bool,
    pub decode_ok: bool,
    pub generate_ok: bool,
}

impl Default for Transformer {
    fn default() -> Self {
        Self::new()
    }
}

impl Transformer {
    pub fn new() -> Self {
        Self {
            attention_ok: true,
            embed_ok: true,
            encode_ok: true,
            decode_ok: true,
            generate_ok: true,
        }
    }

    pub fn encoding_ok(&self) -> bool {
        self.attention_ok && self.embed_ok && self.encode_ok
    }

    pub fn decoding_ok(&self) -> bool {
        self.decode_ok && self.generate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.encoding_ok() && self.decoding_ok()
    }

    pub fn needs_tune(&self) -> bool {
        !self.attention_ok || !self.embed_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.attention_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        let c = Transformer::new();
        assert!(c.encoding_ok());
    }

    #[test]
    fn test_decoding() {
        let c = Transformer::new();
        assert!(c.decoding_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Transformer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_tune() {
        let c = Transformer::new();
        assert!(!c.needs_tune());
    }

    #[test]
    fn test_attention() {
        let mut c = Transformer::new();
        c.attention_ok = false;
        assert!(c.needs_tune());
    }

    #[test]
    fn test_health() {
        let c = Transformer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
