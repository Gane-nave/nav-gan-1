/// V2X communication: vehicle-to-everything, DSRC, C-V2X
/// Phase 281

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum V2xMode {
    Dsrc,
    Cv2x,
    Dual,
    Off,
}

#[derive(Debug, Clone)]
pub struct V2xComm {
    pub mode: V2xMode,
    pub connected_vehicles: u16,
    pub infrastructure_msgs: u32,
    pub signal_dbm: f64,
    pub radio_ok: bool,
}

impl Default for V2xComm {
    fn default() -> Self {
        Self::new()
    }
}

impl V2xComm {
    pub fn new() -> Self {
        Self {
            mode: V2xMode::Cv2x,
            connected_vehicles: 0,
            infrastructure_msgs: 0,
            signal_dbm: -60.0,
            radio_ok: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.mode != V2xMode::Off && self.radio_ok
    }

    pub fn has_peers(&self) -> bool {
        self.connected_vehicles > 0
    }

    pub fn signal_ok(&self) -> bool {
        self.signal_dbm > -80.0
    }

    pub fn receiving_infra(&self) -> bool {
        self.infrastructure_msgs > 0
    }

    pub fn health_score(&self) -> f64 {
        if !self.radio_ok {
            return 0.0;
        }
        if !self.signal_ok() {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active() {
        let v = V2xComm::new();
        assert!(v.is_active());
    }

    #[test]
    fn test_no_peers() {
        let v = V2xComm::new();
        assert!(!v.has_peers());
    }

    #[test]
    fn test_signal() {
        let v = V2xComm::new();
        assert!(v.signal_ok());
    }

    #[test]
    fn test_no_infra() {
        let v = V2xComm::new();
        assert!(!v.receiving_infra());
    }

    #[test]
    fn test_off() {
        let mut v = V2xComm::new();
        v.mode = V2xMode::Off;
        assert!(!v.is_active());
    }

    #[test]
    fn test_health() {
        let v = V2xComm::new();
        assert!((v.health_score() - 100.0).abs() < 0.1);
    }
}
