/// Mirror assistance: auto-dimming, blind spot indicators, parking assist mirrors
/// Phase 134

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MirrorPosition {
    Left,
    Right,
    Center,
}

impl MirrorPosition {
    pub fn blind_spot_angle(&self) -> f64 {
        match self {
            MirrorPosition::Left => 15.0,
            MirrorPosition::Right => 15.0,
            MirrorPosition::Center => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MirrorMode {
    Normal,
    Dimmed,
    Folded,
    Heated,
    ParkingAssist,
}

#[derive(Debug, Clone)]
pub struct SmartMirror {
    pub position: MirrorPosition,
    pub mode: MirrorMode,
    pub glare_level: f64,
    pub blind_spot_detected: bool,
    pub temperature_c: f64,
}

impl SmartMirror {
    pub fn new(pos: MirrorPosition) -> Self {
        Self {
            position: pos,
            mode: MirrorMode::Normal,
            glare_level: 0.0,
            blind_spot_detected: false,
            temperature_c: 20.0,
        }
    }

    pub fn should_auto_dim(&self) -> bool {
        self.glare_level > 60.0
    }
    pub fn should_heat(&self) -> bool {
        self.temperature_c < 3.0
    }
    pub fn should_fold(&self, speed_kmh: f64) -> bool {
        speed_kmh < 1.0 && self.position != MirrorPosition::Center
    }
    pub fn visibility_score(&self) -> f64 {
        let base = 100.0;
        let glare_penalty = self.glare_level * 0.5;
        let mode_bonus = if self.mode == MirrorMode::Dimmed {
            20.0
        } else {
            0.0
        };
        (base - glare_penalty + mode_bonus).clamp(0.0, 100.0)
    }
    pub fn warning_active(&self) -> bool {
        self.blind_spot_detected
    }
    pub fn effective_fov_deg(&self) -> f64 {
        match self.position {
            MirrorPosition::Left | MirrorPosition::Right => 20.0 + self.position.blind_spot_angle(),
            MirrorPosition::Center => 60.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MirrorSystem {
    pub mirrors: Vec<SmartMirror>,
}

impl Default for MirrorSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorSystem {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }
    pub fn add(&mut self, m: SmartMirror) {
        self.mirrors.push(m);
    }
    pub fn any_blind_spot(&self) -> bool {
        self.mirrors.iter().any(|m| m.blind_spot_detected)
    }
    pub fn all_clear(&self) -> bool {
        !self.any_blind_spot()
    }
    pub fn mirrors_needing_heat(&self) -> usize {
        self.mirrors.iter().filter(|m| m.should_heat()).count()
    }
    pub fn total_fov_deg(&self) -> f64 {
        self.mirrors.iter().map(|m| m.effective_fov_deg()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blind_spot_angle() {
        assert_eq!(MirrorPosition::Left.blind_spot_angle(), 15.0);
    }
    #[test]
    fn test_auto_dim() {
        let mut m = SmartMirror::new(MirrorPosition::Left);
        m.glare_level = 80.0;
        assert!(m.should_auto_dim());
    }
    #[test]
    fn test_no_dim() {
        let m = SmartMirror::new(MirrorPosition::Left);
        assert!(!m.should_auto_dim());
    }
    #[test]
    fn test_heat() {
        let mut m = SmartMirror::new(MirrorPosition::Right);
        m.temperature_c = -5.0;
        assert!(m.should_heat());
    }
    #[test]
    fn test_fold() {
        let m = SmartMirror::new(MirrorPosition::Left);
        assert!(m.should_fold(0.0));
    }
    #[test]
    fn test_visibility() {
        let m = SmartMirror::new(MirrorPosition::Center);
        assert!(m.visibility_score() > 90.0);
    }
    #[test]
    fn test_warning() {
        let mut m = SmartMirror::new(MirrorPosition::Left);
        m.blind_spot_detected = true;
        assert!(m.warning_active());
    }
    #[test]
    fn test_fov() {
        let m = SmartMirror::new(MirrorPosition::Left);
        assert!(m.effective_fov_deg() > 30.0);
    }
    #[test]
    fn test_system_blind_spot() {
        let mut sys = MirrorSystem::new();
        let mut m = SmartMirror::new(MirrorPosition::Left);
        m.blind_spot_detected = true;
        sys.add(m);
        assert!(sys.any_blind_spot());
    }
    #[test]
    fn test_system_clear() {
        let mut sys = MirrorSystem::new();
        sys.add(SmartMirror::new(MirrorPosition::Left));
        assert!(sys.all_clear());
    }
    #[test]
    fn test_total_fov() {
        let mut sys = MirrorSystem::new();
        sys.add(SmartMirror::new(MirrorPosition::Left));
        sys.add(SmartMirror::new(MirrorPosition::Right));
        sys.add(SmartMirror::new(MirrorPosition::Center));
        assert!(sys.total_fov_deg() > 90.0);
    }
}
