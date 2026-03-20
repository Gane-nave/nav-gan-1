/// Downpipe: flange, cat connection, heat wrap
/// Phase 617

#[derive(Debug, Clone)]
pub struct Downpipe {
    pub flange_ok: bool,
    pub cat_conn_ok: bool,
    pub heat_wrap_ok: bool,
    pub gasket_ok: bool,
    pub leak_free: bool,
}

impl Default for Downpipe {
    fn default() -> Self {
        Self::new()
    }
}

impl Downpipe {
    pub fn new() -> Self {
        Self {
            flange_ok: true,
            cat_conn_ok: true,
            heat_wrap_ok: true,
            gasket_ok: true,
            leak_free: true,
        }
    }

    pub fn connection_ok(&self) -> bool {
        self.flange_ok && self.cat_conn_ok && self.gasket_ok
    }

    pub fn thermal_ok(&self) -> bool {
        self.heat_wrap_ok
    }

    pub fn all_ok(&self) -> bool {
        self.connection_ok() && self.thermal_ok() && self.leak_free
    }

    pub fn needs_service(&self) -> bool {
        !self.flange_ok || !self.gasket_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.flange_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let c = Downpipe::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_thermal() {
        let c = Downpipe::new();
        assert!(c.thermal_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Downpipe::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Downpipe::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_flange() {
        let mut c = Downpipe::new();
        c.flange_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Downpipe::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
