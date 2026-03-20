/// Infotainment controller: media, phone, apps, touchscreen management
/// Phase 274

#[derive(Debug, Clone)]
pub struct InfotainmentController {
    pub screen_on: bool,
    pub media_playing: bool,
    pub phone_connected: bool,
    pub bluetooth_ok: bool,
    pub wifi_ok: bool,
    pub cpu_usage_pct: f64,
}

impl Default for InfotainmentController {
    fn default() -> Self {
        Self::new()
    }
}

impl InfotainmentController {
    pub fn new() -> Self {
        Self {
            screen_on: true,
            media_playing: false,
            phone_connected: false,
            bluetooth_ok: true,
            wifi_ok: true,
            cpu_usage_pct: 20.0,
        }
    }

    pub fn connectivity_ok(&self) -> bool {
        self.bluetooth_ok || self.wifi_ok
    }

    pub fn overloaded(&self) -> bool {
        self.cpu_usage_pct > 90.0
    }

    pub fn needs_restart(&self) -> bool {
        self.overloaded() || (!self.screen_on && self.media_playing)
    }

    pub fn active_features(&self) -> u8 {
        let mut count: u8 = 0;
        if self.media_playing {
            count += 1;
        }
        if self.phone_connected {
            count += 1;
        }
        count
    }

    pub fn health_score(&self) -> f64 {
        if self.overloaded() {
            return 30.0;
        }
        if !self.connectivity_ok() {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectivity() {
        let i = InfotainmentController::new();
        assert!(i.connectivity_ok());
    }

    #[test]
    fn test_not_overloaded() {
        let i = InfotainmentController::new();
        assert!(!i.overloaded());
    }

    #[test]
    fn test_no_restart() {
        let i = InfotainmentController::new();
        assert!(!i.needs_restart());
    }

    #[test]
    fn test_no_active() {
        let i = InfotainmentController::new();
        assert_eq!(i.active_features(), 0);
    }

    #[test]
    fn test_overloaded() {
        let mut i = InfotainmentController::new();
        i.cpu_usage_pct = 95.0;
        assert!(i.overloaded());
    }

    #[test]
    fn test_health() {
        let i = InfotainmentController::new();
        assert!((i.health_score() - 100.0).abs() < 0.1);
    }
}
