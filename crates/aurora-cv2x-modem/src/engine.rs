/// cv2x modem: attach, transmit, receive, handover, idle
/// Phase 1127

#[derive(Debug, Clone)]
pub struct Cv2xModem {
    pub attach_ok: bool,
    pub transmit_ok: bool,
    pub receive_ok: bool,
    pub handover_ok: bool,
    pub idle_ok: bool,
}

impl Default for Cv2xModem {
    fn default() -> Self {
        Self::new()
    }
}

impl Cv2xModem {
    pub fn new() -> Self {
        Self {
            attach_ok: true,
            transmit_ok: true,
            receive_ok: true,
            handover_ok: true,
            idle_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.attach_ok && self.transmit_ok && self.receive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.handover_ok && self.idle_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.attach_ok || !self.transmit_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.attach_ok {
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
        let c = Cv2xModem::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = Cv2xModem::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Cv2xModem::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = Cv2xModem::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = Cv2xModem::new();
        c.attach_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = Cv2xModem::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
