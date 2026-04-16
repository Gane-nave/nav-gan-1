/// proto mqtt2: publish, subscribe, unsubscribe, connect, log
/// Phase 2007

#[derive(Debug, Clone)]
pub struct ProtoMqtt2 {
    pub publish_ok: bool,
    pub subscribe_ok: bool,
    pub unsubscribe_ok: bool,
    pub connect_ok: bool,
    pub log_ok: bool,
}

impl Default for ProtoMqtt2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoMqtt2 {
    pub fn new() -> Self {
        Self {
            publish_ok: true,
            subscribe_ok: true,
            unsubscribe_ok: true,
            connect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.publish_ok && self.subscribe_ok && self.unsubscribe_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.connect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.publish_ok || !self.subscribe_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.publish_ok {
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
        let c = ProtoMqtt2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ProtoMqtt2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProtoMqtt2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ProtoMqtt2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ProtoMqtt2::new();
        c.publish_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ProtoMqtt2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
