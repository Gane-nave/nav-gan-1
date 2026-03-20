/// bt audio: pair, connect, stream, control, disconnect
/// Phase 1178

#[derive(Debug, Clone)]
pub struct BtAudio {
    pub pair_ok: bool,
    pub connect_ok: bool,
    pub stream_ok: bool,
    pub control_ok: bool,
    pub disconnect_ok: bool,
}

impl Default for BtAudio {
    fn default() -> Self {
        Self::new()
    }
}

impl BtAudio {
    pub fn new() -> Self {
        Self {
            pair_ok: true,
            connect_ok: true,
            stream_ok: true,
            control_ok: true,
            disconnect_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.pair_ok && self.connect_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.control_ok && self.disconnect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.pair_ok || !self.connect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pair_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BtAudio::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BtAudio::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BtAudio::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BtAudio::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BtAudio::new();
        c.pair_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BtAudio::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
