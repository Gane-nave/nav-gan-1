/// CAN bus communication: message parsing, filtering, bus monitoring
/// Phase 187

#[derive(Debug, Clone)]
pub struct CanMessage {
    pub id: u32,
    pub data: Vec<u8>,
    pub timestamp_us: u64,
    pub extended: bool,
}

impl CanMessage {
    pub fn new(id: u32, data: Vec<u8>) -> Self {
        Self {
            id,
            data,
            timestamp_us: 0,
            extended: false,
        }
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn is_standard(&self) -> bool {
        !self.extended && self.id <= 0x7FF
    }
}

#[derive(Debug, Clone)]
pub struct CanBus {
    pub bus_speed_kbps: u32,
    pub messages_received: u64,
    pub errors: u64,
    pub bus_load_pct: f64,
}

impl Default for CanBus {
    fn default() -> Self {
        Self::new()
    }
}

impl CanBus {
    pub fn new() -> Self {
        Self {
            bus_speed_kbps: 500,
            messages_received: 0,
            errors: 0,
            bus_load_pct: 0.0,
        }
    }

    pub fn error_rate(&self) -> f64 {
        if self.messages_received == 0 {
            return 0.0;
        }
        self.errors as f64 / self.messages_received as f64
    }

    pub fn bus_healthy(&self) -> bool {
        self.error_rate() < 0.01 && self.bus_load_pct < 80.0
    }

    pub fn is_high_speed(&self) -> bool {
        self.bus_speed_kbps >= 500
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let m = CanMessage::new(0x100, vec![1, 2, 3]);
        assert_eq!(m.data_len(), 3);
    }

    #[test]
    fn test_standard() {
        let m = CanMessage::new(0x100, vec![]);
        assert!(m.is_standard());
    }

    #[test]
    fn test_bus_healthy() {
        let b = CanBus::new();
        assert!(b.bus_healthy());
    }

    #[test]
    fn test_error_rate_zero() {
        let b = CanBus::new();
        assert!((b.error_rate()).abs() < 0.01);
    }

    #[test]
    fn test_high_speed() {
        let b = CanBus::new();
        assert!(b.is_high_speed());
    }

    #[test]
    fn test_bus_overloaded() {
        let mut b = CanBus::new();
        b.bus_load_pct = 90.0;
        assert!(!b.bus_healthy());
    }
}
