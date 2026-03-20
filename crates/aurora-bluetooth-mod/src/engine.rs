/// Bluetooth module: pairing, audio streaming, hands-free, BLE
/// Phase 434

#[derive(Debug, Clone)]
pub struct BluetoothMod {
    pub connected: bool,
    pub signal_rssi: f64,
    pub version: u8,
    pub audio_ok: bool,
    pub handsfree_ok: bool,
}

impl Default for BluetoothMod {
    fn default() -> Self {
        Self::new()
    }
}

impl BluetoothMod {
    pub fn new() -> Self {
        Self {
            connected: true,
            signal_rssi: -55.0,
            version: 5,
            audio_ok: true,
            handsfree_ok: true,
        }
    }

    pub fn connection_ok(&self) -> bool {
        self.connected && self.signal_rssi > -80.0
    }

    pub fn all_ok(&self) -> bool {
        self.connection_ok() && self.audio_ok && self.handsfree_ok
    }

    pub fn needs_reset(&self) -> bool {
        !self.connected && self.signal_rssi > -90.0
    }

    pub fn modern(&self) -> bool {
        self.version >= 5
    }

    pub fn health_score(&self) -> f64 {
        if !self.connected {
            return 20.0;
        }
        if !self.audio_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let b = BluetoothMod::new();
        assert!(b.connection_ok());
    }

    #[test]
    fn test_all_ok() {
        let b = BluetoothMod::new();
        assert!(b.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let b = BluetoothMod::new();
        assert!(!b.needs_reset());
    }

    #[test]
    fn test_modern() {
        let b = BluetoothMod::new();
        assert!(b.modern());
    }

    #[test]
    fn test_disconnected() {
        let mut b = BluetoothMod::new();
        b.connected = false;
        assert!(b.needs_reset());
    }

    #[test]
    fn test_health() {
        let b = BluetoothMod::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
