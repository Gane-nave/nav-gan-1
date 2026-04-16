/// Defroster system: windshield heating, rear window, mirror defrost
/// Phase 239

#[derive(Debug, Clone)]
pub struct Defroster {
    pub windshield_active: bool,
    pub rear_window_active: bool,
    pub mirror_active: bool,
    pub windshield_temp_c: f64,
    pub frost_detected: bool,
    pub timer_minutes: f64,
}

impl Default for Defroster {
    fn default() -> Self {
        Self::new()
    }
}

impl Defroster {
    pub fn new() -> Self {
        Self {
            windshield_active: false,
            rear_window_active: false,
            mirror_active: false,
            windshield_temp_c: 15.0,
            frost_detected: false,
            timer_minutes: 0.0,
        }
    }

    pub fn any_active(&self) -> bool {
        self.windshield_active || self.rear_window_active || self.mirror_active
    }

    pub fn should_activate(&self) -> bool {
        self.frost_detected || self.windshield_temp_c < 2.0
    }

    pub fn power_draw_w(&self) -> f64 {
        let mut draw: f64 = 0.0;
        if self.windshield_active {
            draw += 500.0;
        }
        if self.rear_window_active {
            draw += 300.0;
        }
        if self.mirror_active {
            draw += 50.0;
        }
        draw
    }

    pub fn all_clear(&self) -> bool {
        !self.frost_detected && self.windshield_temp_c > 5.0
    }

    pub fn health_score(&self) -> f64 {
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_active() {
        let d = Defroster::new();
        assert!(!d.any_active());
    }

    #[test]
    fn test_should_not_activate() {
        let d = Defroster::new();
        assert!(!d.should_activate());
    }

    #[test]
    fn test_no_power() {
        let d = Defroster::new();
        assert!((d.power_draw_w()).abs() < 0.1);
    }

    #[test]
    fn test_all_clear() {
        let d = Defroster::new();
        assert!(d.all_clear());
    }

    #[test]
    fn test_frost() {
        let mut d = Defroster::new();
        d.frost_detected = true;
        assert!(d.should_activate());
    }

    #[test]
    fn test_health() {
        let d = Defroster::new();
        assert!((d.health_score() - 100.0).abs() < 0.1);
    }
}
