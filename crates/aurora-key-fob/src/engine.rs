/// Key fob: remote lock/unlock, proximity detection, battery status
/// Phase 253

#[derive(Debug, Clone)]
pub struct KeyFob {
    pub battery_pct: f64,
    pub signal_strength_dbm: f64,
    pub in_range: bool,
    pub buttons_ok: bool,
    pub paired: bool,
}

impl Default for KeyFob {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyFob {
    pub fn new() -> Self {
        Self {
            battery_pct: 80.0,
            signal_strength_dbm: -45.0,
            in_range: true,
            buttons_ok: true,
            paired: true,
        }
    }

    pub fn battery_low(&self) -> bool {
        self.battery_pct < 20.0
    }

    pub fn signal_ok(&self) -> bool {
        self.signal_strength_dbm > -70.0
    }

    pub fn can_unlock(&self) -> bool {
        self.paired && self.in_range && self.buttons_ok
    }

    pub fn needs_battery(&self) -> bool {
        self.battery_pct < 10.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if self.battery_low() {
            score -= 30.0;
        }
        if !self.signal_ok() {
            score -= 20.0;
        }
        if !self.paired {
            score -= 50.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_ok() {
        let k = KeyFob::new();
        assert!(!k.battery_low());
    }

    #[test]
    fn test_signal_ok() {
        let k = KeyFob::new();
        assert!(k.signal_ok());
    }

    #[test]
    fn test_can_unlock() {
        let k = KeyFob::new();
        assert!(k.can_unlock());
    }

    #[test]
    fn test_no_battery_needed() {
        let k = KeyFob::new();
        assert!(!k.needs_battery());
    }

    #[test]
    fn test_low_battery() {
        let mut k = KeyFob::new();
        k.battery_pct = 5.0;
        assert!(k.needs_battery());
    }

    #[test]
    fn test_health() {
        let k = KeyFob::new();
        assert!((k.health_score() - 100.0).abs() < 0.1);
    }
}
