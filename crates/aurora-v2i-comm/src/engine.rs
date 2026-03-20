/// v2i comm: connect, query, receive, process, log
/// Phase 1339

#[derive(Debug, Clone)]
pub struct V2iComm {
    pub connect_ok: bool,
    pub query_ok: bool,
    pub receive_ok: bool,
    pub process_ok: bool,
    pub log_ok: bool,
}

impl Default for V2iComm {
    fn default() -> Self {
        Self::new()
    }
}

impl V2iComm {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            query_ok: true,
            receive_ok: true,
            process_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.query_ok && self.receive_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.process_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.query_ok
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
        let c = V2iComm::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = V2iComm::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2iComm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = V2iComm::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = V2iComm::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = V2iComm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
