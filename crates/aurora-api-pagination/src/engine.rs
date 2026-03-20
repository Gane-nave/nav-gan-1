/// api pagination: cursor, offset, keyset, total, log
/// Phase 1851

#[derive(Debug, Clone)]
pub struct ApiPagination {
    pub cursor_ok: bool,
    pub offset_ok: bool,
    pub keyset_ok: bool,
    pub total_ok: bool,
    pub log_ok: bool,
}

impl Default for ApiPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiPagination {
    pub fn new() -> Self {
        Self {
            cursor_ok: true,
            offset_ok: true,
            keyset_ok: true,
            total_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.cursor_ok && self.offset_ok && self.keyset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.total_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.cursor_ok || !self.offset_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cursor_ok {
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
        let c = ApiPagination::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ApiPagination::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ApiPagination::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ApiPagination::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ApiPagination::new();
        c.cursor_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ApiPagination::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
