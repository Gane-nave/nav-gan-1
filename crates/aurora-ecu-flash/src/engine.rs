/// ECU flashing: firmware update, calibration, bootloader management
/// Phase 276

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlashState {
    Idle,
    Erasing,
    Writing,
    Verifying,
    Complete,
    Error,
}

#[derive(Debug, Clone)]
pub struct EcuFlash {
    pub state: FlashState,
    pub progress_pct: f64,
    pub firmware_version: u32,
    pub bootloader_ok: bool,
    pub checksum_valid: bool,
}

impl Default for EcuFlash {
    fn default() -> Self {
        Self::new()
    }
}

impl EcuFlash {
    pub fn new() -> Self {
        Self {
            state: FlashState::Idle,
            progress_pct: 0.0,
            firmware_version: 100,
            bootloader_ok: true,
            checksum_valid: true,
        }
    }

    pub fn is_flashing(&self) -> bool {
        matches!(
            self.state,
            FlashState::Erasing | FlashState::Writing | FlashState::Verifying
        )
    }

    pub fn is_complete(&self) -> bool {
        self.state == FlashState::Complete
    }

    pub fn has_error(&self) -> bool {
        self.state == FlashState::Error || !self.checksum_valid
    }

    pub fn safe_to_flash(&self) -> bool {
        self.bootloader_ok && self.state == FlashState::Idle
    }

    pub fn health_score(&self) -> f64 {
        if self.has_error() {
            return 0.0;
        }
        if !self.bootloader_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_flashing() {
        let e = EcuFlash::new();
        assert!(!e.is_flashing());
    }

    #[test]
    fn test_not_complete() {
        let e = EcuFlash::new();
        assert!(!e.is_complete());
    }

    #[test]
    fn test_no_error() {
        let e = EcuFlash::new();
        assert!(!e.has_error());
    }

    #[test]
    fn test_safe() {
        let e = EcuFlash::new();
        assert!(e.safe_to_flash());
    }

    #[test]
    fn test_flashing() {
        let mut e = EcuFlash::new();
        e.state = FlashState::Writing;
        assert!(e.is_flashing());
    }

    #[test]
    fn test_health() {
        let e = EcuFlash::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
