/// Effortless control: voice, gesture, gaze, minimal touch.
#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Voice,
    Gesture,
    Gaze,
    Touch,
    HardwareButton,
    Automatic,
}
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub mode: InputMode,
    pub confidence: f64,
    pub processing_time_ms: u64,
}
impl InputEvent {
    pub fn effort_score(&self) -> f64 {
        let mode_effort = match self.mode {
            InputMode::Automatic => 0.0,
            InputMode::Voice => 0.1,
            InputMode::Gaze => 0.15,
            InputMode::Gesture => 0.2,
            InputMode::HardwareButton => 0.3,
            InputMode::Touch => 0.4,
        };
        let latency = (self.processing_time_ms as f64 / 2000.0).min(1.0) * 0.3;
        let conf_penalty = (1.0 - self.confidence.clamp(0.0, 1.0)) * 0.2;
        (mode_effort + latency + conf_penalty).clamp(0.0, 1.0)
    }
    pub fn is_effortless(&self) -> bool {
        self.effort_score() < 0.3
    }
}
#[derive(Debug, Clone)]
pub struct InputManager {
    pub available_modes: Vec<InputMode>,
    pub preferred_mode: InputMode,
}
impl InputManager {
    pub fn new(preferred: InputMode) -> Self {
        Self {
            available_modes: vec![InputMode::Voice, InputMode::Touch, InputMode::Automatic],
            preferred_mode: preferred,
        }
    }
    pub fn best_mode_for_driving(&self) -> InputMode {
        if self.available_modes.contains(&InputMode::Automatic) {
            InputMode::Automatic
        } else if self.available_modes.contains(&InputMode::Voice) {
            InputMode::Voice
        } else {
            self.preferred_mode.clone()
        }
    }
    pub fn mode_count(&self) -> usize {
        self.available_modes.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_voice() {
        let e = InputEvent {
            mode: InputMode::Voice,
            confidence: 0.95,
            processing_time_ms: 200,
        };
        assert!(e.is_effortless());
    }
    #[test]
    fn test_touch() {
        let e = InputEvent {
            mode: InputMode::Touch,
            confidence: 1.0,
            processing_time_ms: 500,
        };
        assert!(!e.is_effortless());
    }
    #[test]
    fn test_auto() {
        let e = InputEvent {
            mode: InputMode::Automatic,
            confidence: 1.0,
            processing_time_ms: 0,
        };
        assert!(e.is_effortless());
        assert_eq!(e.effort_score(), 0.0);
    }
    #[test]
    fn test_best_mode() {
        let m = InputManager::new(InputMode::Touch);
        assert_eq!(m.best_mode_for_driving(), InputMode::Automatic);
    }
}
