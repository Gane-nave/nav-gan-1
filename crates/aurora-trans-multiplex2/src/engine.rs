/// trans multiplex2: create, send, receive, close, log
/// Phase 2277

#[derive(Debug, Clone)]
pub struct TransMultiplex2 {
    pub create_ok: bool,
    pub send_ok: bool,
    pub receive_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for TransMultiplex2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransMultiplex2 {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            send_ok: true,
            receive_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.send_ok && self.receive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.send_ok
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
        let c = TransMultiplex2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransMultiplex2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransMultiplex2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransMultiplex2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransMultiplex2::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransMultiplex2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
