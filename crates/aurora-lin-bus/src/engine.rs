/// LIN bus: master, slave, checksum, wakeup
/// Phase 728

#[derive(Debug, Clone)]
pub struct LinBus {
    pub master_ok: bool,
    pub slave_ok: bool,
    pub checksum_ok: bool,
    pub wakeup_ok: bool,
    pub timing_ok: bool,
}

impl Default for LinBus {
    fn default() -> Self {
        Self::new()
    }
}

impl LinBus {
    pub fn new() -> Self {
        Self {
            master_ok: true,
            slave_ok: true,
            checksum_ok: true,
            wakeup_ok: true,
            timing_ok: true,
        }
    }

    pub fn communication_ok(&self) -> bool {
        self.master_ok && self.slave_ok && self.timing_ok
    }

    pub fn integrity_ok(&self) -> bool {
        self.checksum_ok && self.wakeup_ok
    }

    pub fn all_ok(&self) -> bool {
        self.communication_ok() && self.integrity_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.master_ok || !self.checksum_ok
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
    fn test_communication() {
        let c = LinBus::new();
        assert!(c.communication_ok());
    }

    #[test]
    fn test_integrity() {
        let c = LinBus::new();
        assert!(c.integrity_ok());
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
    fn test_master() {
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
