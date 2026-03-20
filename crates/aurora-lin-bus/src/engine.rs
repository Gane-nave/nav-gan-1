/// LIN bus: low-speed serial network, body electronics, sensor nodes
/// Phase 278

#[derive(Debug, Clone)]
pub struct LinBus {
    pub node_count: u8,
    pub master_ok: bool,
    pub baud_rate: u32,
    pub error_count: u32,
    pub sleep_mode: bool,
}

impl Default for LinBus {
    fn default() -> Self {
        Self::new()
    }
}

impl LinBus {
    pub fn new() -> Self {
        Self {
            node_count: 8,
            master_ok: true,
            baud_rate: 19200,
            error_count: 0,
            sleep_mode: false,
        }
    }

    pub fn is_active(&self) -> bool {
        !self.sleep_mode && self.master_ok
    }

    pub fn error_free(&self) -> bool {
        self.error_count == 0
    }

    pub fn baud_ok(&self) -> bool {
        self.baud_rate == 19200 || self.baud_rate == 9600
    }

    pub fn needs_wakeup(&self) -> bool {
        self.sleep_mode
    }

    pub fn health_score(&self) -> f64 {
        if !self.master_ok {
            return 0.0;
        }
        if !self.error_free() {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active() {
        let l = LinBus::new();
        assert!(l.is_active());
    }

    #[test]
    fn test_error_free() {
        let l = LinBus::new();
        assert!(l.error_free());
    }

    #[test]
    fn test_baud_ok() {
        let l = LinBus::new();
        assert!(l.baud_ok());
    }

    #[test]
    fn test_no_wakeup() {
        let l = LinBus::new();
        assert!(!l.needs_wakeup());
    }

    #[test]
    fn test_sleep() {
        let mut l = LinBus::new();
        l.sleep_mode = true;
        assert!(l.needs_wakeup());
    }

    #[test]
    fn test_health() {
        let l = LinBus::new();
        assert!((l.health_score() - 100.0).abs() < 0.1);
    }
}
