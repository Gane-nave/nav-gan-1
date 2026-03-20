/// graph import: load, validate, transform, commit, log
/// Phase 1905

#[derive(Debug, Clone)]
pub struct GraphImport {
    pub load_ok: bool,
    pub validate_ok: bool,
    pub transform_ok: bool,
    pub commit_ok: bool,
    pub log_ok: bool,
}

impl Default for GraphImport {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphImport {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            validate_ok: true,
            transform_ok: true,
            commit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.validate_ok && self.transform_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.commit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.validate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
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
        let c = GraphImport::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GraphImport::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GraphImport::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GraphImport::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GraphImport::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GraphImport::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
