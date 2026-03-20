/// monitor proc: list, cpu, memory, status, log
/// Phase 1576

#[derive(Debug, Clone)]
pub struct MonitorProc {
    pub list_ok: bool,
    pub cpu_ok: bool,
    pub memory_ok: bool,
    pub status_ok: bool,
    pub log_ok: bool,
}

impl Default for MonitorProc {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorProc {
    pub fn new() -> Self {
        Self {
            list_ok: true,
            cpu_ok: true,
            memory_ok: true,
            status_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.list_ok && self.cpu_ok && self.memory_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.status_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.list_ok || !self.cpu_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.list_ok {
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
        let c = MonitorProc::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MonitorProc::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MonitorProc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MonitorProc::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MonitorProc::new();
        c.list_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MonitorProc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
