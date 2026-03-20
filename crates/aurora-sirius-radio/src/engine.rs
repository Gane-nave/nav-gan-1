/// sirius radio: subscribe, tune, decode, buffer, log
/// Phase 1350

#[derive(Debug, Clone)]
pub struct SiriusRadio {
    pub subscribe_ok: bool,
    pub tune_ok: bool,
    pub decode_ok: bool,
    pub buffer_ok: bool,
    pub log_ok: bool,
}

impl Default for SiriusRadio {
    fn default() -> Self {
        Self::new()
    }
}

impl SiriusRadio {
    pub fn new() -> Self {
        Self {
            subscribe_ok: true,
            tune_ok: true,
            decode_ok: true,
            buffer_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.subscribe_ok && self.tune_ok && self.decode_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.buffer_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.subscribe_ok || !self.tune_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.subscribe_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SiriusRadio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SiriusRadio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SiriusRadio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SiriusRadio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SiriusRadio::new();
        c.subscribe_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SiriusRadio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
