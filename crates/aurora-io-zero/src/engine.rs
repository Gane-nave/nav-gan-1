/// io zero: send, recv, splice, tee, log
/// Phase 1994

#[derive(Debug, Clone)]
pub struct IoZero {
    pub send_ok: bool,
    pub recv_ok: bool,
    pub splice_ok: bool,
    pub tee_ok: bool,
    pub log_ok: bool,
}

impl Default for IoZero {
    fn default() -> Self {
        Self::new()
    }
}

impl IoZero {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            recv_ok: true,
            splice_ok: true,
            tee_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.recv_ok && self.splice_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.tee_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.send_ok || !self.recv_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.send_ok {
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
        let c = IoZero::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoZero::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoZero::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoZero::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoZero::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoZero::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
