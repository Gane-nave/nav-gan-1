/// obs incident2: create, escalate, resolve, review, log
/// Phase 2164

#[derive(Debug, Clone)]
pub struct ObsIncident2 {
    pub create_ok: bool,
    pub escalate_ok: bool,
    pub resolve_ok: bool,
    pub review_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsIncident2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsIncident2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            escalate_ok: true,
            resolve_ok: true,
            review_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.escalate_ok && self.resolve_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.review_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.escalate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = ObsIncident2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsIncident2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsIncident2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsIncident2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsIncident2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsIncident2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
