/// Remote access: lock, climate, charge, locate, status
/// Phase 886

#[derive(Debug, Clone)]
pub struct RemoteAccess {
    pub lock_ok: bool,
    pub climate_ok: bool,
    pub charge_ok: bool,
    pub locate_ok: bool,
    pub status_ok: bool,
}

impl Default for RemoteAccess {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteAccess {
    pub fn new() -> Self {
        Self {
            lock_ok: true,
            climate_ok: true,
            charge_ok: true,
            locate_ok: true,
            status_ok: true,
        }
    }

    pub fn control_ok(&self) -> bool {
        self.lock_ok && self.climate_ok && self.charge_ok
    }

    pub fn monitoring_ok(&self) -> bool {
        self.locate_ok && self.status_ok
    }

    pub fn all_ok(&self) -> bool {
        self.control_ok() && self.monitoring_ok()
    }

    pub fn needs_auth(&self) -> bool {
        !self.lock_ok || !self.status_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lock_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control() {
        let c = RemoteAccess::new();
        assert!(c.control_ok());
    }

    #[test]
    fn test_monitoring() {
        let c = RemoteAccess::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RemoteAccess::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_auth() {
        let c = RemoteAccess::new();
        assert!(!c.needs_auth());
    }

    #[test]
    fn test_lock() {
        let mut c = RemoteAccess::new();
        c.lock_ok = false;
        assert!(c.needs_auth());
    }

    #[test]
    fn test_health() {
        let c = RemoteAccess::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
