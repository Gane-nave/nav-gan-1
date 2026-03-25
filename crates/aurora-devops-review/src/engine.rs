/// devops review: request, comment, approve, merge, log
/// Phase 2172

#[derive(Debug, Clone)]
pub struct DevopsReview {
    pub request_ok: bool,
    pub comment_ok: bool,
    pub approve_ok: bool,
    pub merge_ok: bool,
    pub log_ok: bool,
}

impl Default for DevopsReview {
    fn default() -> Self {
        Self::new()
    }
}

impl DevopsReview {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            comment_ok: true,
            approve_ok: true,
            merge_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.comment_ok && self.approve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.merge_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.comment_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = DevopsReview::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DevopsReview::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DevopsReview::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DevopsReview::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DevopsReview::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DevopsReview::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
