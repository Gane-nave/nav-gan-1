/// runtime actor: spawn, send, recv, stop, log
/// Phase 1796

#[derive(Debug, Clone)]
pub struct RuntimeActor {
    pub spawn_ok: bool,
    pub send_ok: bool,
    pub recv_ok: bool,
    pub stop_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeActor {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeActor {
    pub fn new() -> Self {
        Self {
            spawn_ok: true,
            send_ok: true,
            recv_ok: true,
            stop_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.spawn_ok && self.send_ok && self.recv_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stop_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.spawn_ok || !self.send_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.spawn_ok {
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
        let c = RuntimeActor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeActor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeActor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeActor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeActor::new();
        c.spawn_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeActor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
