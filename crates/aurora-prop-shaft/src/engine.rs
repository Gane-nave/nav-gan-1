/// Propeller shaft: vibration, balance, U-joint condition
/// Phase 320

#[derive(Debug, Clone)]
pub struct PropShaft {
    pub vibration_mm_s: f64,
    pub max_vibration_mm_s: f64,
    pub u_joint_ok: bool,
    pub balance_ok: bool,
    pub bearing_ok: bool,
}

impl Default for PropShaft {
    fn default() -> Self {
        Self::new()
    }
}

impl PropShaft {
    pub fn new() -> Self {
        Self {
            vibration_mm_s: 2.0,
            max_vibration_mm_s: 10.0,
            u_joint_ok: true,
            balance_ok: true,
            bearing_ok: true,
        }
    }

    pub fn vibration_ok(&self) -> bool {
        self.vibration_mm_s < self.max_vibration_mm_s
    }

    pub fn all_ok(&self) -> bool {
        self.u_joint_ok && self.balance_ok && self.bearing_ok && self.vibration_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.u_joint_ok || !self.bearing_ok
    }

    pub fn vibration_pct(&self) -> f64 {
        (self.vibration_mm_s / self.max_vibration_mm_s * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.u_joint_ok {
            return 0.0;
        }
        if !self.bearing_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibration() {
        let p = PropShaft::new();
        assert!(p.vibration_ok());
    }

    #[test]
    fn test_all_ok() {
        let p = PropShaft::new();
        assert!(p.all_ok());
    }

    #[test]
    fn test_no_service() {
        let p = PropShaft::new();
        assert!(!p.needs_service());
    }

    #[test]
    fn test_vib_pct() {
        let p = PropShaft::new();
        assert!(p.vibration_pct() < 30.0);
    }

    #[test]
    fn test_bad_joint() {
        let mut p = PropShaft::new();
        p.u_joint_ok = false;
        assert!(p.needs_service());
    }

    #[test]
    fn test_health() {
        let p = PropShaft::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
