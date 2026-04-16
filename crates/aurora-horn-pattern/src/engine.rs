/// Horn pattern control: sound patterns, volume adjustment, courtesy beep
/// Phase 141

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HornType {
    Standard,
    Courtesy,
    Emergency,
    Alarm,
    Melody,
}

impl HornType {
    pub fn decibels(&self) -> f64 {
        match self {
            HornType::Standard => 107.0,
            HornType::Courtesy => 85.0,
            HornType::Emergency => 115.0,
            HornType::Alarm => 110.0,
            HornType::Melody => 90.0,
        }
    }
    pub fn frequency_hz(&self) -> u32 {
        match self {
            HornType::Standard => 420,
            HornType::Courtesy => 300,
            HornType::Emergency => 500,
            HornType::Alarm => 600,
            HornType::Melody => 350,
        }
    }
    pub fn max_duration_sec(&self) -> f64 {
        match self {
            HornType::Standard => 3.0,
            HornType::Courtesy => 0.5,
            HornType::Emergency => 10.0,
            HornType::Alarm => 30.0,
            HornType::Melody => 5.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HornEvent {
    pub horn_type: HornType,
    pub duration_sec: f64,
    pub pattern_repeats: u32,
}

impl HornEvent {
    pub fn new(t: HornType, dur: f64) -> Self {
        Self {
            horn_type: t,
            duration_sec: dur.min(t.max_duration_sec()),
            pattern_repeats: 1,
        }
    }
    pub fn effective_range_m(&self) -> f64 {
        self.horn_type.decibels() * 0.5
    }
    pub fn is_emergency(&self) -> bool {
        matches!(self.horn_type, HornType::Emergency | HornType::Alarm)
    }
    pub fn noise_complaint_risk(&self) -> bool {
        self.duration_sec > 2.0 && self.horn_type.decibels() > 100.0
    }
    pub fn total_sound_energy(&self) -> f64 {
        self.horn_type.decibels() * self.duration_sec * self.pattern_repeats as f64
    }
}

#[derive(Debug, Clone)]
pub struct HornSystem {
    pub available_types: Vec<HornType>,
    pub volume_pct: f64,
    pub enabled: bool,
}

impl Default for HornSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HornSystem {
    pub fn new() -> Self {
        Self {
            available_types: vec![HornType::Standard, HornType::Courtesy],
            volume_pct: 100.0,
            enabled: true,
        }
    }
    pub fn effective_db(&self, t: HornType) -> f64 {
        t.decibels() * (self.volume_pct / 100.0)
    }
    pub fn can_use(&self, t: HornType) -> bool {
        self.enabled && self.available_types.contains(&t)
    }
    pub fn courtesy_available(&self) -> bool {
        self.can_use(HornType::Courtesy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decibels() {
        assert!(HornType::Emergency.decibels() > HornType::Courtesy.decibels());
    }
    #[test]
    fn test_frequency() {
        assert_eq!(HornType::Standard.frequency_hz(), 420);
    }
    #[test]
    fn test_event_range() {
        let e = HornEvent::new(HornType::Standard, 1.0);
        assert!(e.effective_range_m() > 40.0);
    }
    #[test]
    fn test_emergency() {
        let e = HornEvent::new(HornType::Emergency, 2.0);
        assert!(e.is_emergency());
    }
    #[test]
    fn test_not_emergency() {
        let e = HornEvent::new(HornType::Courtesy, 0.3);
        assert!(!e.is_emergency());
    }
    #[test]
    fn test_noise_risk() {
        let e = HornEvent::new(HornType::Standard, 3.0);
        assert!(e.noise_complaint_risk());
    }
    #[test]
    fn test_system_db() {
        let s = HornSystem::new();
        assert!(s.effective_db(HornType::Standard) > 100.0);
    }
    #[test]
    fn test_can_use() {
        let s = HornSystem::new();
        assert!(s.can_use(HornType::Standard));
    }
    #[test]
    fn test_courtesy() {
        let s = HornSystem::new();
        assert!(s.courtesy_available());
    }
    #[test]
    fn test_duration_clamp() {
        let e = HornEvent::new(HornType::Courtesy, 5.0);
        assert!((e.duration_sec - 0.5).abs() < 0.01);
    }
}
