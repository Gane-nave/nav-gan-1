/// event bridge: connect, forward, transform, buffer, log
/// Phase 1872

#[derive(Debug, Clone)]
pub struct EventBridge {
    pub connect_ok: bool,
    pub forward_ok: bool,
    pub transform_ok: bool,
    pub buffer_ok: bool,
    pub log_ok: bool,
}

impl Default for EventBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBridge {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            forward_ok: true,
            transform_ok: true,
            buffer_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.forward_ok && self.transform_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.buffer_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.forward_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = EventBridge::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventBridge::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventBridge::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventBridge::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventBridge::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventBridge::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
