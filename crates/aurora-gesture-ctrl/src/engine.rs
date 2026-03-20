/// Gesture control: hand tracking, swipe recognition, air gestures
/// Phase 147

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GestureType {
    SwipeLeft,
    SwipeRight,
    SwipeUp,
    SwipeDown,
    Pinch,
    Spread,
    Tap,
    Wave,
    Point,
}

impl GestureType {
    pub fn is_navigation(&self) -> bool {
        matches!(self, GestureType::SwipeLeft | GestureType::SwipeRight)
    }

    pub fn is_volume(&self) -> bool {
        matches!(self, GestureType::SwipeUp | GestureType::SwipeDown)
    }

    pub fn is_zoom(&self) -> bool {
        matches!(self, GestureType::Pinch | GestureType::Spread)
    }

    pub fn complexity(&self) -> u8 {
        match self {
            GestureType::Tap => 1,
            GestureType::SwipeLeft | GestureType::SwipeRight => 2,
            GestureType::SwipeUp | GestureType::SwipeDown => 2,
            GestureType::Wave => 3,
            GestureType::Point => 3,
            GestureType::Pinch | GestureType::Spread => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GestureEvent {
    pub gesture: GestureType,
    pub confidence: f64,
    pub duration_ms: u64,
}

impl GestureEvent {
    pub fn new(gesture: GestureType, confidence: f64, duration_ms: u64) -> Self {
        Self {
            gesture,
            confidence,
            duration_ms,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.confidence > 0.7
    }

    pub fn is_deliberate(&self) -> bool {
        self.duration_ms > 200 && self.confidence > 0.8
    }

    pub fn action_label(&self) -> &'static str {
        match self.gesture {
            GestureType::SwipeLeft => "previous",
            GestureType::SwipeRight => "next",
            GestureType::SwipeUp => "volume_up",
            GestureType::SwipeDown => "volume_down",
            GestureType::Pinch => "zoom_out",
            GestureType::Spread => "zoom_in",
            GestureType::Tap => "select",
            GestureType::Wave => "dismiss",
            GestureType::Point => "point",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GestureSystem {
    pub enabled: bool,
    pub sensitivity: f64,
    pub min_confidence: f64,
    pub recent_gestures: Vec<GestureEvent>,
}

impl Default for GestureSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl GestureSystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            sensitivity: 0.8,
            min_confidence: 0.7,
            recent_gestures: Vec::new(),
        }
    }

    pub fn process(&mut self, event: GestureEvent) -> bool {
        if !self.enabled || event.confidence < self.min_confidence {
            return false;
        }
        self.recent_gestures.push(event);
        true
    }

    pub fn last_action(&self) -> Option<&'static str> {
        self.recent_gestures.last().map(|g| g.action_label())
    }

    pub fn gesture_count(&self) -> usize {
        self.recent_gestures.len()
    }

    pub fn clear_history(&mut self) {
        self.recent_gestures.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_navigation() {
        assert!(GestureType::SwipeLeft.is_navigation());
        assert!(!GestureType::Tap.is_navigation());
    }

    #[test]
    fn test_is_volume() {
        assert!(GestureType::SwipeUp.is_volume());
    }

    #[test]
    fn test_is_zoom() {
        assert!(GestureType::Pinch.is_zoom());
    }

    #[test]
    fn test_complexity() {
        assert!(GestureType::Pinch.complexity() > GestureType::Tap.complexity());
    }

    #[test]
    fn test_event_valid() {
        let e = GestureEvent::new(GestureType::SwipeRight, 0.9, 300);
        assert!(e.is_valid());
    }

    #[test]
    fn test_event_invalid() {
        let e = GestureEvent::new(GestureType::Tap, 0.3, 100);
        assert!(!e.is_valid());
    }

    #[test]
    fn test_deliberate() {
        let e = GestureEvent::new(GestureType::Wave, 0.95, 500);
        assert!(e.is_deliberate());
    }

    #[test]
    fn test_action_label() {
        let e = GestureEvent::new(GestureType::SwipeLeft, 0.9, 200);
        assert_eq!(e.action_label(), "previous");
    }

    #[test]
    fn test_system_process() {
        let mut s = GestureSystem::new();
        let accepted = s.process(GestureEvent::new(GestureType::Tap, 0.9, 150));
        assert!(accepted);
        assert_eq!(s.gesture_count(), 1);
    }

    #[test]
    fn test_system_reject() {
        let mut s = GestureSystem::new();
        let accepted = s.process(GestureEvent::new(GestureType::Tap, 0.3, 100));
        assert!(!accepted);
    }

    #[test]
    fn test_last_action() {
        let mut s = GestureSystem::new();
        s.process(GestureEvent::new(GestureType::SwipeRight, 0.9, 300));
        assert_eq!(s.last_action(), Some("next"));
    }
}
