/// lora radio: configure, send, receive, decode, log
/// Phase 1346

#[derive(Debug, Clone)]
pub struct LoraRadio {
    pub configure_ok: bool,
    pub send_ok: bool,
    pub receive_ok: bool,
    pub decode_ok: bool,
    pub log_ok: bool,
}

impl Default for LoraRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl LoraRadio {
    pub fn new() -> Self {
        Self {
            configure_ok: true,
            send_ok: true,
            receive_ok: true,
            decode_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.configure_ok && self.send_ok && self.receive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.decode_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.configure_ok || !self.send_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.configure_ok {
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
        let c = LoraRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LoraRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LoraRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LoraRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LoraRadio::new();
        c.configure_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LoraRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
