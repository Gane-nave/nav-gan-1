/// net avro2: connect, send, recv, close, log
/// Phase 1830

#[derive(Debug, Clone)]
pub struct NetAvro2 {
    pub connect_ok: bool,
    pub send_ok: bool,
    pub recv_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for NetAvro2 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetAvro2 {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            send_ok: true,
            recv_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.send_ok && self.recv_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.send_ok
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
        let c = NetAvro2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetAvro2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetAvro2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetAvro2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetAvro2::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetAvro2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
