/// v2n comm: connect, stream, buffer, route, log
/// Phase 1341

#[derive(Debug, Clone)]
pub struct V2nComm {
    pub connect_ok: bool,
    pub stream_ok: bool,
    pub buffer_ok: bool,
    pub route_ok: bool,
    pub log_ok: bool,
}

impl Default for V2nComm {
    fn default() -> Self {
        Self::new()
    }
}

impl V2nComm {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            stream_ok: true,
            buffer_ok: true,
            route_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.stream_ok && self.buffer_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.route_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.stream_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = V2nComm::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = V2nComm::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2nComm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = V2nComm::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = V2nComm::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = V2nComm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
