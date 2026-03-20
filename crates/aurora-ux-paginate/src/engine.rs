/// ux paginate: count, slice, navigate, cache, log
/// Phase 1512

#[derive(Debug, Clone)]
pub struct UxPaginate {
    pub count_ok: bool,
    pub slice_ok: bool,
    pub navigate_ok: bool,
    pub cache_ok: bool,
    pub log_ok: bool,
}

impl Default for UxPaginate {
    fn default() -> Self {
        Self::new()
    }
}

impl UxPaginate {
    pub fn new() -> Self {
        Self {
            count_ok: true,
            slice_ok: true,
            navigate_ok: true,
            cache_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.count_ok && self.slice_ok && self.navigate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cache_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.count_ok || !self.slice_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.count_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = UxPaginate::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxPaginate::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxPaginate::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxPaginate::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxPaginate::new();
        c.count_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxPaginate::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
