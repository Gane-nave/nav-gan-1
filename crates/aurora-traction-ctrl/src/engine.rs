/// Traction control: wheel spin detection, torque reduction, surface adaptation
/// Phase 152

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurfaceType {
    Dry,
    Wet,
    Snow,
    Ice,
    Gravel,
    Mud,
}

impl SurfaceType {
    pub fn grip_coefficient(&self) -> f64 {
        match self {
            SurfaceType::Dry => 1.0,
            SurfaceType::Wet => 0.7,
            SurfaceType::Snow => 0.3,
            SurfaceType::Ice => 0.15,
            SurfaceType::Gravel => 0.5,
            SurfaceType::Mud => 0.4,
        }
    }

    pub fn max_safe_speed_kmh(&self) -> f64 {
        match self {
            SurfaceType::Dry => 200.0,
            SurfaceType::Wet => 130.0,
            SurfaceType::Snow => 60.0,
            SurfaceType::Ice => 30.0,
            SurfaceType::Gravel => 80.0,
            SurfaceType::Mud => 40.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TractionSystem {
    pub enabled: bool,
    pub surface: SurfaceType,
    pub wheel_spin_detected: bool,
    pub torque_reduction_pct: f64,
    pub interventions: u64,
}

impl Default for TractionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TractionSystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            surface: SurfaceType::Dry,
            wheel_spin_detected: false,
            torque_reduction_pct: 0.0,
            interventions: 0,
        }
    }

    pub fn needs_intervention(&self) -> bool {
        self.enabled && self.wheel_spin_detected
    }

    pub fn recommended_torque_limit_pct(&self) -> f64 {
        self.surface.grip_coefficient() * 100.0
    }

    pub fn is_low_grip(&self) -> bool {
        self.surface.grip_coefficient() < 0.5
    }

    pub fn apply_intervention(&mut self) {
        if self.needs_intervention() {
            self.torque_reduction_pct = 100.0 - self.recommended_torque_limit_pct();
            self.interventions += 1;
            self.wheel_spin_detected = false;
        }
    }

    pub fn effective_power_pct(&self) -> f64 {
        100.0 - self.torque_reduction_pct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grip_dry() {
        assert!((SurfaceType::Dry.grip_coefficient() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_grip_ice() {
        assert!(SurfaceType::Ice.grip_coefficient() < 0.2);
    }

    #[test]
    fn test_max_speed() {
        assert!(SurfaceType::Ice.max_safe_speed_kmh() < SurfaceType::Dry.max_safe_speed_kmh());
    }

    #[test]
    fn test_needs_intervention() {
        let mut s = TractionSystem::new();
        s.wheel_spin_detected = true;
        assert!(s.needs_intervention());
    }

    #[test]
    fn test_no_intervention() {
        let s = TractionSystem::new();
        assert!(!s.needs_intervention());
    }

    #[test]
    fn test_low_grip() {
        let mut s = TractionSystem::new();
        s.surface = SurfaceType::Ice;
        assert!(s.is_low_grip());
    }

    #[test]
    fn test_apply_intervention() {
        let mut s = TractionSystem::new();
        s.surface = SurfaceType::Ice;
        s.wheel_spin_detected = true;
        s.apply_intervention();
        assert!(s.torque_reduction_pct > 50.0);
        assert_eq!(s.interventions, 1);
    }

    #[test]
    fn test_effective_power() {
        let mut s = TractionSystem::new();
        s.torque_reduction_pct = 30.0;
        assert!((s.effective_power_pct() - 70.0).abs() < 0.1);
    }
}
