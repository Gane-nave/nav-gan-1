/// Motion as data: animation conveys direction, intent, and urgency.
#[derive(Debug, Clone, PartialEq)]
pub enum MotionType {
    Flow,
    Pulse,
    Expand,
    Contract,
    Rotate,
    Fade,
}
#[derive(Debug, Clone)]
pub struct MotionCue {
    pub motion_type: MotionType,
    pub direction_deg: f64,
    pub speed: f64,
    pub intensity: f64,
    pub duration_ms: u64,
}
impl MotionCue {
    pub fn urgency(&self) -> f64 {
        (self.speed * self.intensity).clamp(0.0, 1.0)
    }
    pub fn is_directional(&self) -> bool {
        matches!(self.motion_type, MotionType::Flow | MotionType::Rotate)
    }
    pub fn perceived_velocity(&self) -> f64 {
        self.speed * (1.0 + self.intensity * 0.5)
    }
}
#[derive(Debug, Clone)]
pub struct MotionSystem {
    pub active_cues: Vec<MotionCue>,
}
impl Default for MotionSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl MotionSystem {
    pub fn new() -> Self {
        Self {
            active_cues: Vec::new(),
        }
    }
    pub fn add_cue(&mut self, c: MotionCue) {
        self.active_cues.push(c);
    }
    pub fn total_motion_energy(&self) -> f64 {
        self.active_cues.iter().map(|c| c.urgency()).sum()
    }
    pub fn dominant_direction(&self) -> Option<f64> {
        let directional: Vec<_> = self
            .active_cues
            .iter()
            .filter(|c| c.is_directional())
            .collect();
        if directional.is_empty() {
            None
        } else {
            let avg =
                directional.iter().map(|c| c.direction_deg).sum::<f64>() / directional.len() as f64;
            Some(avg)
        }
    }
    pub fn is_calm(&self) -> bool {
        self.total_motion_energy() < 0.3
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_urgency() {
        let c = MotionCue {
            motion_type: MotionType::Pulse,
            direction_deg: 0.0,
            speed: 0.8,
            intensity: 0.9,
            duration_ms: 500,
        };
        assert!(c.urgency() > 0.5);
    }
    #[test]
    fn test_directional() {
        let c = MotionCue {
            motion_type: MotionType::Flow,
            direction_deg: 45.0,
            speed: 0.5,
            intensity: 0.5,
            duration_ms: 1000,
        };
        assert!(c.is_directional());
    }
    #[test]
    fn test_system() {
        let mut s = MotionSystem::new();
        s.add_cue(MotionCue {
            motion_type: MotionType::Flow,
            direction_deg: 90.0,
            speed: 0.3,
            intensity: 0.3,
            duration_ms: 1000,
        });
        assert!(s.is_calm());
        assert!(s.dominant_direction().is_some());
    }
    #[test]
    fn test_empty() {
        let s = MotionSystem::new();
        assert!(s.is_calm());
        assert!(s.dominant_direction().is_none());
    }
}
