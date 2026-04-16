/// proto rest2: get, post, put, delete, log
/// Phase 2020

#[derive(Debug, Clone)]
pub struct ProtoRest2 {
    pub get_ok: bool,
    pub post_ok: bool,
    pub put_ok: bool,
    pub delete_ok: bool,
    pub log_ok: bool,
}

impl Default for ProtoRest2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoRest2 {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            post_ok: true,
            put_ok: true,
            delete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.get_ok && self.post_ok && self.put_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.delete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.get_ok || !self.post_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.get_ok {
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
        let c = ProtoRest2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ProtoRest2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProtoRest2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ProtoRest2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ProtoRest2::new();
        c.get_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ProtoRest2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
