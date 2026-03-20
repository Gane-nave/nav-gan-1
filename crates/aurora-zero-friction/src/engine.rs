/// Zero friction interaction: zero input, auto-start, single-action results.
#[derive(Debug, Clone)]
pub struct InteractionMetrics { pub taps_required: u32, pub screens_deep: u32, pub time_to_navigate_ms: u64, pub auto_start_eligible: bool }
impl InteractionMetrics {
    pub fn friction_score(&self) -> f64 {
        let taps = (self.taps_required as f64 / 5.0).min(1.0) * 0.3;
        let screens = (self.screens_deep as f64 / 3.0).min(1.0) * 0.3;
        let time = (self.time_to_navigate_ms as f64 / 5000.0).min(1.0) * 0.2;
        let auto = if self.auto_start_eligible { 0.0 } else { 0.2 };
        (taps + screens + time + auto).clamp(0.0, 1.0)
    }
    pub fn is_zero_friction(&self) -> bool { self.friction_score() < 0.2 }
}
#[derive(Debug, Clone)]
pub struct AutoStartRule { pub destination_confidence: f64, pub time_match: f64, pub habit_strength: f64 }
impl AutoStartRule {
    pub fn should_auto_start(&self) -> bool {
        let score = self.destination_confidence * 0.5 + self.time_match * 0.3 + self.habit_strength * 0.2;
        score > 0.7
    }
    pub fn auto_start_confidence(&self) -> f64 {
        (self.destination_confidence * 0.5 + self.time_match * 0.3 + self.habit_strength * 0.2).clamp(0.0, 1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_zero() { let m = InteractionMetrics { taps_required: 0, screens_deep: 0, time_to_navigate_ms: 0, auto_start_eligible: true }; assert!(m.is_zero_friction()); }
    #[test] fn test_high_friction() { let m = InteractionMetrics { taps_required: 5, screens_deep: 3, time_to_navigate_ms: 5000, auto_start_eligible: false }; assert!(!m.is_zero_friction()); }
    #[test] fn test_auto_start() { let r = AutoStartRule { destination_confidence: 0.9, time_match: 0.8, habit_strength: 0.7 }; assert!(r.should_auto_start()); }
    #[test] fn test_no_auto() { let r = AutoStartRule { destination_confidence: 0.3, time_match: 0.2, habit_strength: 0.1 }; assert!(!r.should_auto_start()); }
}
