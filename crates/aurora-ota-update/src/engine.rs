/// OTA update: over-the-air software update, download, install, rollback
/// Phase 283

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OtaState {
    Idle,
    Checking,
    Downloading,
    Installing,
    Complete,
    RollingBack,
}

#[derive(Debug, Clone)]
pub struct OtaUpdate {
    pub state: OtaState,
    pub progress_pct: f64,
    pub update_available: bool,
    pub download_size_mb: f64,
    pub current_version: u32,
    pub target_version: u32,
}

impl Default for OtaUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl OtaUpdate {
    pub fn new() -> Self {
        Self {
            state: OtaState::Idle,
            progress_pct: 0.0,
            update_available: false,
            download_size_mb: 0.0,
            current_version: 100,
            target_version: 100,
        }
    }

    pub fn is_updating(&self) -> bool {
        matches!(self.state, OtaState::Downloading | OtaState::Installing)
    }

    pub fn needs_update(&self) -> bool {
        self.update_available && self.target_version > self.current_version
    }

    pub fn safe_to_drive(&self) -> bool {
        !matches!(self.state, OtaState::Installing | OtaState::RollingBack)
    }

    pub fn health_score(&self) -> f64 {
        if self.state == OtaState::RollingBack {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_updating() {
        let o = OtaUpdate::new();
        assert!(!o.is_updating());
    }

    #[test]
    fn test_no_update() {
        let o = OtaUpdate::new();
        assert!(!o.needs_update());
    }

    #[test]
    fn test_safe() {
        let o = OtaUpdate::new();
        assert!(o.safe_to_drive());
    }

    #[test]
    fn test_update_avail() {
        let mut o = OtaUpdate::new();
        o.update_available = true;
        o.target_version = 101;
        assert!(o.needs_update());
    }

    #[test]
    fn test_installing() {
        let mut o = OtaUpdate::new();
        o.state = OtaState::Installing;
        assert!(!o.safe_to_drive());
    }

    #[test]
    fn test_health() {
        let o = OtaUpdate::new();
        assert!((o.health_score() - 100.0).abs() < 0.1);
    }
}
