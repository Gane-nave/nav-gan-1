/// codec avro2: encode, decode, schema, evolve, log
/// Phase 2029

#[derive(Debug, Clone)]
pub struct CodecAvro2 {
    pub encode_ok: bool,
    pub decode_ok: bool,
    pub schema_ok: bool,
    pub evolve_ok: bool,
    pub log_ok: bool,
}

impl Default for CodecAvro2 {
    fn default() -> Self {
        Self::new()
    }
}

impl CodecAvro2 {
    pub fn new() -> Self {
        Self {
            encode_ok: true,
            decode_ok: true,
            schema_ok: true,
            evolve_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.encode_ok && self.decode_ok && self.schema_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.evolve_ok && self.log_ok
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
        let c = CodecAvro2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CodecAvro2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CodecAvro2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CodecAvro2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CodecAvro2::new();
        c.encode_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CodecAvro2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
