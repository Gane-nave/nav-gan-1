/// Peripheral vision optimization: info visible without direct gaze.
#[derive(Debug, Clone, PartialEq)]
pub enum PeripheralZone {
    Center,
    NearPeriphery,
    FarPeriphery,
    Edge,
}
#[derive(Debug, Clone)]
pub struct PeripheralCue {
    pub zone: PeripheralZone,
    pub color_intensity: f64,
    pub motion_speed: f64,
    pub size_scale: f64,
}
impl PeripheralCue {
    pub fn detectability(&self) -> f64 {
        let zone_factor = match self.zone {
            PeripheralZone::Center => 1.0,
            PeripheralZone::NearPeriphery => 0.7,
            PeripheralZone::FarPeriphery => 0.4,
            PeripheralZone::Edge => 0.2,
        };
        let color = self.color_intensity.clamp(0.0, 1.0) * 0.3;
        let motion = self.motion_speed.clamp(0.0, 1.0) * 0.4;
        let size = self.size_scale.clamp(0.0, 2.0) / 2.0 * 0.3;
        (zone_factor * (color + motion + size)).clamp(0.0, 1.0)
    }
    pub fn is_detectable(&self) -> bool {
        self.detectability() > 0.3
    }
}
#[derive(Debug, Clone)]
pub struct PeripheralSystem {
    pub cues: Vec<PeripheralCue>,
}
impl Default for PeripheralSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl PeripheralSystem {
    pub fn new() -> Self {
        Self { cues: Vec::new() }
    }
    pub fn add_cue(&mut self, c: PeripheralCue) {
        self.cues.push(c);
    }
    pub fn detectable_count(&self) -> usize {
        self.cues.iter().filter(|c| c.is_detectable()).count()
    }
    pub fn optimize_for_peripheral(&mut self) {
        for cue in &mut self.cues {
            if !cue.is_detectable() {
                cue.color_intensity = (cue.color_intensity * 1.5).min(1.0);
                cue.size_scale = (cue.size_scale * 1.3).min(2.0);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_center() {
        let c = PeripheralCue {
            zone: PeripheralZone::Center,
            color_intensity: 0.8,
            motion_speed: 0.5,
            size_scale: 1.0,
        };
        assert!(c.is_detectable());
    }
    #[test]
    fn test_edge() {
        let c = PeripheralCue {
            zone: PeripheralZone::Edge,
            color_intensity: 0.2,
            motion_speed: 0.1,
            size_scale: 0.5,
        };
        assert!(!c.is_detectable());
    }
    #[test]
    fn test_optimize() {
        let mut s = PeripheralSystem::new();
        s.add_cue(PeripheralCue {
            zone: PeripheralZone::FarPeriphery,
            color_intensity: 0.2,
            motion_speed: 0.1,
            size_scale: 0.5,
        });
        let before = s.detectable_count();
        s.optimize_for_peripheral();
        assert!(s.detectable_count() >= before);
    }
    #[test]
    fn test_empty() {
        let s = PeripheralSystem::new();
        assert_eq!(s.detectable_count(), 0);
    }
}
