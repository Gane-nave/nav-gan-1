/// graph embed: train, encode, decode, similar, log
/// Phase 1909

#[derive(Debug, Clone)]
pub struct GraphEmbed {
    pub train_ok: bool,
    pub encode_ok: bool,
    pub decode_ok: bool,
    pub similar_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphEmbed {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphEmbed {
    pub fn new() -> Self {
        Self {
            train_ok: true,
            encode_ok: true,
            decode_ok: true,
            similar_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.train_ok && self.encode_ok && self.decode_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.similar_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.train_ok || !self.encode_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.train_ok {
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
        let c = GraphEmbed::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphEmbed::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphEmbed::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphEmbed::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphEmbed::new();
        c.train_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphEmbed::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
