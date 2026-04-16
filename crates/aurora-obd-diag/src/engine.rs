/// OBD-II diagnostics: protocol handling, PID reading, live data
/// Phase 186

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObdProtocol {
    Can11Bit,
    Can29Bit,
    Kwp2000,
    Iso9141,
}

#[derive(Debug, Clone)]
pub struct ObdReading {
    pub pid: u16,
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone)]
pub struct ObdSystem {
    pub protocol: ObdProtocol,
    pub connected: bool,
    pub readings: Vec<ObdReading>,
    pub dtc_count: u32,
}

impl Default for ObdSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ObdSystem {
    pub fn new() -> Self {
        Self {
            protocol: ObdProtocol::Can11Bit,
            connected: false,
            readings: Vec::new(),
            dtc_count: 0,
        }
    }

    pub fn has_faults(&self) -> bool {
        self.dtc_count > 0
    }

    pub fn reading_count(&self) -> usize {
        self.readings.len()
    }

    pub fn get_reading(&self, pid: u16) -> Option<&ObdReading> {
        self.readings.iter().find(|r| r.pid == pid)
    }

    pub fn health_score(&self) -> f64 {
        if !self.connected {
            return 0.0;
        }
        if self.dtc_count == 0 {
            100.0
        } else {
            (100.0 - self.dtc_count as f64 * 15.0).max(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_faults() {
        let s = ObdSystem::new();
        assert!(!s.has_faults());
    }

    #[test]
    fn test_faults() {
        let mut s = ObdSystem::new();
        s.dtc_count = 3;
        assert!(s.has_faults());
    }

    #[test]
    fn test_disconnected_score() {
        let s = ObdSystem::new();
        assert!((s.health_score()).abs() < 0.1);
    }

    #[test]
    fn test_connected_score() {
        let mut s = ObdSystem::new();
        s.connected = true;
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_reading_count() {
        let s = ObdSystem::new();
        assert_eq!(s.reading_count(), 0);
    }

    #[test]
    fn test_get_reading() {
        let s = ObdSystem::new();
        assert!(s.get_reading(0x0C).is_none());
    }
}
