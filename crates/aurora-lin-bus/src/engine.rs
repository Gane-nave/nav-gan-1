/// LIN bus: low-speed communication, master/slave
/// Phase 525

#[derive(Debug, Clone)]
pub struct LinBus {
    pub baud_rate_kbps: u32,
    pub error_count: u32,
    pub master_ok: bool,
    pub slave_count: u32,
    pub checksum_ok: bool,
}

impl Default for LinBus {
    fn default() -> Self {
        Self::new()
    }
}

impl LinBus {
    pub fn new() -> Self {
        Self {
            baud_rate_kbps: 20,
            error_count: 0,
            master_ok: true,
            slave_count: 8,
            checksum_ok: true,
        }
    }

    pub fn no_errors(&self) -> bool {
        self.error_count == 0
    }

    pub fn master_functional(&self) -> bool {
        self.master_ok && self.checksum_ok
    }

    pub fn all_ok(&self) -> bool {
        self.no_errors() && self.master_functional()
    }

    pub fn needs_service(&self) -> bool {
        !self.master_ok || self.error_count > 5
    }

    pub fn health_score(&self) -> f64 {
        if !self.master_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_errors() {
        let c = LinBus::new();
        assert!(c.no_errors());
    }

    #[test]
    fn test_master() {
        let c = LinBus::new();
        assert!(c.master_functional());
    }

    #[test]
    fn test_all_ok() {
        let c = LinBus::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = LinBus::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_master_fail() {
        let mut c = LinBus::new();
        c.master_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = LinBus::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
