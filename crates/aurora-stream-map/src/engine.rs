/// stream map: apply, filter, chain, collect, log
/// Phase 1923

#[derive(Debug, Clone)]
pub struct StreamMap {
    pub apply_ok: bool,
    pub filter_ok: bool,
    pub chain_ok: bool,
    pub collect_ok: bool,
    pub log_ok: bool,
}

impl Default for StreamMap {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamMap {
    pub fn new() -> Self {
        Self {
            apply_ok: true,
            filter_ok: true,
            chain_ok: true,
            collect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.apply_ok && self.filter_ok && self.chain_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.collect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.apply_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.apply_ok {
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
        let c = StreamMap::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StreamMap::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamMap::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StreamMap::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StreamMap::new();
        c.apply_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StreamMap::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
