/// NLP engine: tokenize, embed, parse, extract, sentiment
/// Phase 1022

#[derive(Debug, Clone)]
pub struct NlpEngine {
    pub tokenize_ok: bool,
    pub embed_ok: bool,
    pub parse_ok: bool,
    pub extract_ok: bool,
    pub sentiment_ok: bool,
}

impl Default for NlpEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl NlpEngine {
    pub fn new() -> Self {
        Self {
            tokenize_ok: true,
            embed_ok: true,
            parse_ok: true,
            extract_ok: true,
            sentiment_ok: true,
        }
    }

    pub fn processing_ok(&self) -> bool {
        self.tokenize_ok && self.embed_ok && self.parse_ok
    }

    pub fn analysis_ok(&self) -> bool {
        self.extract_ok && self.sentiment_ok
    }

    pub fn all_ok(&self) -> bool {
        self.processing_ok() && self.analysis_ok()
    }

    pub fn needs_model(&self) -> bool {
        !self.embed_ok || !self.tokenize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tokenize_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing() {
        let c = NlpEngine::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_analysis() {
        let c = NlpEngine::new();
        assert!(c.analysis_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NlpEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_model() {
        let c = NlpEngine::new();
        assert!(!c.needs_model());
    }

    #[test]
    fn test_embed() {
        let mut c = NlpEngine::new();
        c.embed_ok = false;
        assert!(c.needs_model());
    }

    #[test]
    fn test_health() {
        let c = NlpEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
