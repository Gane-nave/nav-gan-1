//! Text-to-Speech engine abstraction.
//!
//! Manages voice profiles, speech rate, pitch, and queued utterances
//! with priority-based interruption.

/// Voice gender preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceGender {
    Male,
    Female,
    Neutral,
}

/// Speech priority — higher priority interrupts lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpeechPriority {
    /// Background info (e.g., "you are on Route 66").
    Low = 0,
    /// Normal navigation instructions.
    Normal = 1,
    /// Important alerts (e.g., "speed camera ahead").
    High = 2,
    /// Critical safety alerts (e.g., "emergency vehicle approaching").
    Critical = 3,
}

/// A voice profile configuration.
#[derive(Debug, Clone)]
pub struct VoiceProfile {
    pub name: String,
    pub language: String,
    pub gender: VoiceGender,
    /// Speech rate multiplier (1.0 = normal).
    pub rate: f64,
    /// Pitch multiplier (1.0 = normal).
    pub pitch: f64,
    /// Volume 0.0 .. 1.0.
    pub volume: f64,
}

impl Default for VoiceProfile {
    fn default() -> Self {
        Self {
            name: "default".into(),
            language: "en-US".into(),
            gender: VoiceGender::Female,
            rate: 1.0,
            pitch: 1.0,
            volume: 0.8,
        }
    }
}

impl VoiceProfile {
    /// Estimated duration in seconds for a given text length.
    pub fn estimated_duration_s(&self, char_count: usize) -> f64 {
        // Average ~15 chars/second at rate 1.0
        let base = char_count as f64 / 15.0;
        if self.rate > 0.0 {
            base / self.rate
        } else {
            base
        }
    }
}

/// An utterance queued for speech.
#[derive(Debug, Clone)]
pub struct Utterance {
    pub text: String,
    pub priority: SpeechPriority,
    pub language: Option<String>,
}

/// TTS engine state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtsState {
    Idle,
    Speaking,
    Paused,
}

/// TTS engine.
#[derive(Debug)]
pub struct TtsEngine {
    profile: VoiceProfile,
    queue: Vec<Utterance>,
    state: TtsState,
    muted: bool,
}

impl TtsEngine {
    pub fn new(profile: VoiceProfile) -> Self {
        Self {
            profile,
            queue: Vec::new(),
            state: TtsState::Idle,
            muted: false,
        }
    }

    /// Enqueue an utterance.
    pub fn enqueue(&mut self, text: impl Into<String>, priority: SpeechPriority) {
        if self.muted {
            return;
        }
        let utterance = Utterance {
            text: text.into(),
            priority,
            language: None,
        };
        // Insert sorted by priority (highest first)
        let pos = self.queue.partition_point(|u| u.priority >= priority);
        self.queue.insert(pos, utterance);
    }

    /// Enqueue with a specific language override.
    pub fn enqueue_with_language(
        &mut self,
        text: impl Into<String>,
        priority: SpeechPriority,
        language: impl Into<String>,
    ) {
        if self.muted {
            return;
        }
        let utterance = Utterance {
            text: text.into(),
            priority,
            language: Some(language.into()),
        };
        let pos = self.queue.partition_point(|u| u.priority >= priority);
        self.queue.insert(pos, utterance);
    }

    /// Pop the next utterance to speak.
    pub fn pop_next(&mut self) -> Option<Utterance> {
        if self.queue.is_empty() {
            self.state = TtsState::Idle;
            return None;
        }
        self.state = TtsState::Speaking;
        Some(self.queue.remove(0))
    }

    /// Clear all queued utterances below a priority threshold.
    pub fn clear_below(&mut self, min_priority: SpeechPriority) {
        self.queue.retain(|u| u.priority >= min_priority);
    }

    /// Clear the entire queue.
    pub fn clear(&mut self) {
        self.queue.clear();
        self.state = TtsState::Idle;
    }

    /// Mute/unmute.
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
        if muted {
            self.clear();
        }
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }

    pub fn state(&self) -> TtsState {
        self.state
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn profile(&self) -> &VoiceProfile {
        &self.profile
    }

    /// Update voice profile.
    pub fn set_profile(&mut self, profile: VoiceProfile) {
        self.profile = profile;
    }

    /// Estimate total remaining speech time.
    pub fn remaining_time_s(&self) -> f64 {
        self.queue
            .iter()
            .map(|u| self.profile.estimated_duration_s(u.text.len()))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_profile_defaults() {
        let p = VoiceProfile::default();
        assert_eq!(p.language, "en-US");
        assert!((p.rate - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_estimated_duration() {
        let p = VoiceProfile::default();
        let dur = p.estimated_duration_s(150);
        assert!((dur - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_estimated_duration_fast() {
        let p = VoiceProfile {
            rate: 2.0,
            ..Default::default()
        };
        let dur = p.estimated_duration_s(150);
        assert!((dur - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_enqueue_and_next() {
        let mut engine = TtsEngine::new(Default::default());
        engine.enqueue("Turn left", SpeechPriority::Normal);
        engine.enqueue("Emergency!", SpeechPriority::Critical);
        let first = engine.pop_next().unwrap();
        assert_eq!(first.text, "Emergency!");
        assert_eq!(first.priority, SpeechPriority::Critical);
    }

    #[test]
    fn test_priority_ordering() {
        let mut engine = TtsEngine::new(Default::default());
        engine.enqueue("low", SpeechPriority::Low);
        engine.enqueue("high", SpeechPriority::High);
        engine.enqueue("normal", SpeechPriority::Normal);
        assert_eq!(engine.pop_next().unwrap().text, "high");
        assert_eq!(engine.pop_next().unwrap().text, "normal");
        assert_eq!(engine.pop_next().unwrap().text, "low");
    }

    #[test]
    fn test_clear_below() {
        let mut engine = TtsEngine::new(Default::default());
        engine.enqueue("low", SpeechPriority::Low);
        engine.enqueue("critical", SpeechPriority::Critical);
        engine.enqueue("normal", SpeechPriority::Normal);
        engine.clear_below(SpeechPriority::High);
        assert_eq!(engine.queue_len(), 1);
        assert_eq!(engine.pop_next().unwrap().text, "critical");
    }

    #[test]
    fn test_mute_blocks_enqueue() {
        let mut engine = TtsEngine::new(Default::default());
        engine.set_muted(true);
        engine.enqueue("hello", SpeechPriority::Normal);
        assert_eq!(engine.queue_len(), 0);
    }

    #[test]
    fn test_mute_clears_queue() {
        let mut engine = TtsEngine::new(Default::default());
        engine.enqueue("hello", SpeechPriority::Normal);
        engine.set_muted(true);
        assert_eq!(engine.queue_len(), 0);
    }

    #[test]
    fn test_state_transitions() {
        let mut engine = TtsEngine::new(Default::default());
        assert_eq!(engine.state(), TtsState::Idle);
        engine.enqueue("hello", SpeechPriority::Normal);
        engine.pop_next();
        assert_eq!(engine.state(), TtsState::Speaking);
        engine.pop_next(); // queue empty
        assert_eq!(engine.state(), TtsState::Idle);
    }

    #[test]
    fn test_language_override() {
        let mut engine = TtsEngine::new(Default::default());
        engine.enqueue_with_language("סע ימינה", SpeechPriority::Normal, "he-IL");
        let u = engine.pop_next().unwrap();
        assert_eq!(u.language, Some("he-IL".to_string()));
    }

    #[test]
    fn test_remaining_time() {
        let mut engine = TtsEngine::new(Default::default());
        engine.enqueue("hello world!!", SpeechPriority::Normal); // 13 chars
                                                                 // 13/15 ≈ 0.867s
        assert!((engine.remaining_time_s() - 13.0 / 15.0).abs() < 0.01);
    }
}
