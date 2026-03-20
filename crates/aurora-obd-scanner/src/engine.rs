/// OBD scanner: PID read, DTC clear, freeze frame, monitor
/// Phase 816

#[derive(Debug, Clone)]
pub struct ObdScanner {
    pub pid_ok: bool,
    pub dtc_ok: bool,
    pub freeze_ok: bool,
    pub monitor_ok: bool,
    pub comm_ok: bool,
}

impl Default for ObdScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl ObdScanner {
    pub fn new() -> Self {
        Self {
            pid_ok: true,
            dtc_ok: true,
            freeze_ok: true,
            monitor_ok: true,
            comm_ok: true,
        }
    }

    pub fn reading_ok(&self) -> bool {
        self.pid_ok && self.freeze_ok && self.comm_ok
    }

    pub fn diagnostics_ok(&self) -> bool {
        self.dtc_ok && self.monitor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.reading_ok() && self.diagnostics_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.pid_ok || !self.comm_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.comm_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let c = ObdScanner::new();
        assert!(c.reading_ok());
    }

    #[test]
    fn test_diagnostics() {
        let c = ObdScanner::new();
        assert!(c.diagnostics_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObdScanner::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ObdScanner::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_comm() {
        let mut c = ObdScanner::new();
        c.comm_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ObdScanner::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
