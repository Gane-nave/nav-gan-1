/// OBD port: diagnostics, DTC reading, live data, security gateway
/// Phase 436

#[derive(Debug, Clone)]
pub struct ObdPort {
    pub connected: bool,
    pub protocol_ok: bool,
    pub dtc_count: u32,
    pub live_data_ok: bool,
    pub security_ok: bool,
}

impl Default for ObdPort {
    fn default() -> Self {
        Self::new()
    }
}

impl ObdPort {
    pub fn new() -> Self {
        Self {
            connected: true,
            protocol_ok: true,
            dtc_count: 0,
            live_data_ok: true,
            security_ok: true,
        }
    }

    pub fn communication_ok(&self) -> bool {
        self.connected && self.protocol_ok
    }

    pub fn dtc_free(&self) -> bool {
        self.dtc_count == 0
    }

    pub fn all_ok(&self) -> bool {
        self.communication_ok() && self.dtc_free() && self.live_data_ok
    }

    pub fn needs_attention(&self) -> bool {
        self.dtc_count > 0
    }

    pub fn health_score(&self) -> f64 {
        if !self.connected {
            return 0.0;
        }
        if self.dtc_count > 0 {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comm() {
        let o = ObdPort::new();
        assert!(o.communication_ok());
    }

    #[test]
    fn test_dtc_free() {
        let o = ObdPort::new();
        assert!(o.dtc_free());
    }

    #[test]
    fn test_all_ok() {
        let o = ObdPort::new();
        assert!(o.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let o = ObdPort::new();
        assert!(!o.needs_attention());
    }

    #[test]
    fn test_dtc() {
        let mut o = ObdPort::new();
        o.dtc_count = 3;
        assert!(o.needs_attention());
    }

    #[test]
    fn test_health() {
        let o = ObdPort::new();
        assert!((o.health_score() - 100.0).abs() < 0.1);
    }
}
