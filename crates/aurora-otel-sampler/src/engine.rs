/// otel sampler: decide, config, ratio, parent, log
/// Phase 1757

#[derive(Debug, Clone)]
pub struct OtelSampler {
    pub decide_ok: bool,
    pub config_ok: bool,
    pub ratio_ok: bool,
    pub parent_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelSampler {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelSampler {
    pub fn new() -> Self {
        Self {
            decide_ok: true,
            config_ok: true,
            ratio_ok: true,
            parent_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.decide_ok && self.config_ok && self.ratio_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.parent_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.decide_ok || !self.config_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.decide_ok {
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
        let c = OtelSampler::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelSampler::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelSampler::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelSampler::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelSampler::new();
        c.decide_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelSampler::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
