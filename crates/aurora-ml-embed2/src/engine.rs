/// ml embed2: encode, search, cluster, reduce, log
/// Phase 1961

#[derive(Debug, Clone)]
pub struct MlEmbed2 {
    pub encode_ok: bool,
    pub search_ok: bool,
    pub cluster_ok: bool,
    pub reduce_ok: bool,
    pub log_ok: bool,
}

impl Default for MlEmbed2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MlEmbed2 {
    pub fn new() -> Self {
        Self {
            encode_ok: true,
            search_ok: true,
            cluster_ok: true,
            reduce_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.encode_ok && self.search_ok && self.cluster_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reduce_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.encode_ok || !self.search_ok
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
        let c = MlEmbed2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlEmbed2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlEmbed2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlEmbed2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlEmbed2::new();
        c.encode_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlEmbed2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
