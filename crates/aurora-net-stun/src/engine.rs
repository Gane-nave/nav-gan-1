/// net stun: bind, request, indicate, close, log
/// Phase 1832

#[derive(Debug, Clone)]
pub struct NetStun {
    pub bind_ok: bool,
    pub request_ok: bool,
    pub indicate_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for NetStun {
    fn default() -> Self {
        Self::new()
    }
}

impl NetStun {
    pub fn new() -> Self {
        Self {
            bind_ok: true,
            request_ok: true,
            indicate_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.bind_ok && self.request_ok && self.indicate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.bind_ok || !self.request_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bind_ok {
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
        let c = NetStun::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetStun::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetStun::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetStun::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetStun::new();
        c.bind_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetStun::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
