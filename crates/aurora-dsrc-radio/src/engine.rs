/// dsrc radio: tune, transmit, receive, decode, log
/// Phase 1342

#[derive(Debug, Clone)]
pub struct DsrcRadio {
    pub tune_ok: bool,
    pub transmit_ok: bool,
    pub receive_ok: bool,
    pub decode_ok: bool,
    pub log_ok: bool,
}

impl Default for DsrcRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl DsrcRadio {
    pub fn new() -> Self {
        Self {
            tune_ok: true,
            transmit_ok: true,
            receive_ok: true,
            decode_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.tune_ok && self.transmit_ok && self.receive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.decode_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.tune_ok || !self.transmit_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tune_ok {
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
        let c = DsrcRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DsrcRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DsrcRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DsrcRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DsrcRadio::new();
        c.tune_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DsrcRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
