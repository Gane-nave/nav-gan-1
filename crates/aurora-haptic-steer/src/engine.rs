/// Haptic steering feedback: lane departure, navigation cues, collision warnings
/// Phase 149

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HapticPattern {
    Pulse,
    Vibration,
    Rumble,
    Nudge,
    Sustained,
}

impl HapticPattern {
    pub fn intensity(&self) -> f64 {
        match self {
            HapticPattern::Pulse => 0.3,
            HapticPattern::Vibration => 0.5,
            HapticPattern::Rumble => 0.7,
            HapticPattern::Nudge => 0.4,
            HapticPattern::Sustained => 0.6,
        }
    }

    pub fn duration_ms(&self) -> u64 {
        match self {
            HapticPattern::Pulse => 100,
            HapticPattern::Vibration => 200,
            HapticPattern::Rumble => 500,
            HapticPattern::Nudge => 150,
            HapticPattern::Sustained => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedbackReason {
    LaneDeparture,
    NavigationTurn,
    CollisionWarning,
    BlindSpot,
    SpeedLimit,
    RoadTexture,
}

impl FeedbackReason {
    pub fn urgency(&self) -> u8 {
        match self {
            FeedbackReason::CollisionWarning => 5,
            FeedbackReason::LaneDeparture => 4,
            FeedbackReason::BlindSpot => 3,
            FeedbackReason::SpeedLimit => 2,
            FeedbackReason::NavigationTurn => 1,
            FeedbackReason::RoadTexture => 1,
        }
    }

    pub fn recommended_pattern(&self) -> HapticPattern {
        match self {
            FeedbackReason::CollisionWarning => HapticPattern::Rumble,
            FeedbackReason::LaneDeparture => HapticPattern::Vibration,
            FeedbackReason::BlindSpot => HapticPattern::Pulse,
            FeedbackReason::SpeedLimit => HapticPattern::Nudge,
            FeedbackReason::NavigationTurn => HapticPattern::Nudge,
            FeedbackReason::RoadTexture => HapticPattern::Sustained,
        }
    }

    pub fn is_safety_critical(&self) -> bool {
        matches!(
            self,
            FeedbackReason::CollisionWarning | FeedbackReason::LaneDeparture
        )
    }
}

#[derive(Debug, Clone)]
pub struct HapticEvent {
    pub reason: FeedbackReason,
    pub pattern: HapticPattern,
    pub side: &'static str,
}

impl HapticEvent {
    pub fn new(reason: FeedbackReason, side: &'static str) -> Self {
        Self {
            reason,
            pattern: reason.recommended_pattern(),
            side,
        }
    }

    pub fn effective_intensity(&self) -> f64 {
        let base = self.pattern.intensity();
        let urgency_mult = self.reason.urgency() as f64 / 5.0;
        (base * (0.5 + urgency_mult * 0.5)).min(1.0)
    }
}

#[derive(Debug, Clone)]
pub struct HapticSteeringSystem {
    pub enabled: bool,
    pub intensity_scale: f64,
    pub events_delivered: u64,
    pub safety_override: bool,
}

impl Default for HapticSteeringSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl HapticSteeringSystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            intensity_scale: 1.0,
            events_delivered: 0,
            safety_override: true,
        }
    }

    pub fn deliver(&mut self, event: &HapticEvent) -> bool {
        if !self.enabled && !self.should_override(event) {
            return false;
        }
        self.events_delivered += 1;
        true
    }

    pub fn should_override(&self, event: &HapticEvent) -> bool {
        self.safety_override && event.reason.is_safety_critical()
    }

    pub fn scaled_intensity(&self, event: &HapticEvent) -> f64 {
        (event.effective_intensity() * self.intensity_scale).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_intensity() {
        assert!(HapticPattern::Rumble.intensity() > HapticPattern::Pulse.intensity());
    }

    #[test]
    fn test_pattern_duration() {
        assert!(HapticPattern::Sustained.duration_ms() > HapticPattern::Pulse.duration_ms());
    }

    #[test]
    fn test_urgency() {
        assert!(
            FeedbackReason::CollisionWarning.urgency() > FeedbackReason::NavigationTurn.urgency()
        );
    }

    #[test]
    fn test_safety_critical() {
        assert!(FeedbackReason::CollisionWarning.is_safety_critical());
        assert!(!FeedbackReason::NavigationTurn.is_safety_critical());
    }

    #[test]
    fn test_recommended_pattern() {
        assert_eq!(
            FeedbackReason::CollisionWarning.recommended_pattern(),
            HapticPattern::Rumble
        );
    }

    #[test]
    fn test_event_intensity() {
        let e = HapticEvent::new(FeedbackReason::CollisionWarning, "both");
        assert!(e.effective_intensity() > 0.5);
    }

    #[test]
    fn test_deliver() {
        let mut s = HapticSteeringSystem::new();
        let e = HapticEvent::new(FeedbackReason::NavigationTurn, "left");
        assert!(s.deliver(&e));
        assert_eq!(s.events_delivered, 1);
    }

    #[test]
    fn test_safety_override() {
        let mut s = HapticSteeringSystem::new();
        s.enabled = false;
        let e = HapticEvent::new(FeedbackReason::CollisionWarning, "both");
        assert!(s.deliver(&e));
    }

    #[test]
    fn test_no_override_non_critical() {
        let mut s = HapticSteeringSystem::new();
        s.enabled = false;
        let e = HapticEvent::new(FeedbackReason::NavigationTurn, "left");
        assert!(!s.deliver(&e));
    }

    #[test]
    fn test_scaled_intensity() {
        let s = HapticSteeringSystem::new();
        let e = HapticEvent::new(FeedbackReason::LaneDeparture, "right");
        assert!(s.scaled_intensity(&e) > 0.0);
        assert!(s.scaled_intensity(&e) <= 1.0);
    }
}
