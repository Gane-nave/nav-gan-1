/// Brake light: stop lamp, third brake light, emergency braking flash
/// Phase 246

#[derive(Debug, Clone)]
pub struct BrakeLight {
    pub left_ok: bool,
    pub right_ok: bool,
    pub center_ok: bool,
    pub active: bool,
    pub emergency_flash: bool,
    pub led_type: bool,
}

impl Default for BrakeLight {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeLight {
    pub fn new() -> Self {
        Self {
            left_ok: true,
            right_ok: true,
            center_ok: true,
            active: false,
            emergency_flash: false,
            led_type: true,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.left_ok && self.right_ok && self.center_ok
    }

    pub fn bulb_out(&self) -> bool {
        !self.left_ok || !self.right_ok || !self.center_ok
    }

    pub fn working_count(&self) -> u8 {
        let mut count: u8 = 0;
        if self.left_ok {
            count += 1;
        }
        if self.right_ok {
            count += 1;
        }
        if self.center_ok {
            count += 1;
        }
        count
    }

    pub fn health_score(&self) -> f64 {
        if !self.left_ok && !self.right_ok {
            return 0.0;
        }
        if self.bulb_out() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let b = BrakeLight::new();
        assert!(b.all_ok());
    }

    #[test]
    fn test_no_bulb_out() {
        let b = BrakeLight::new();
        assert!(!b.bulb_out());
    }

    #[test]
    fn test_working_count() {
        let b = BrakeLight::new();
        assert_eq!(b.working_count(), 3);
    }

    #[test]
    fn test_not_active() {
        let b = BrakeLight::new();
        assert!(!b.active);
    }

    #[test]
    fn test_bulb_out() {
        let mut b = BrakeLight::new();
        b.left_ok = false;
        assert!(b.bulb_out());
    }

    #[test]
    fn test_health() {
        let b = BrakeLight::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
