/// Voice command processing: speech recognition, NLU, command execution
/// Phase 148

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandCategory {
    Navigation,
    Media,
    Climate,
    Phone,
    Vehicle,
    System,
}

impl CommandCategory {
    pub fn priority(&self) -> u8 {
        match self {
            CommandCategory::Vehicle => 5,
            CommandCategory::Navigation => 4,
            CommandCategory::Phone => 3,
            CommandCategory::Climate => 2,
            CommandCategory::Media => 1,
            CommandCategory::System => 3,
        }
    }

    pub fn requires_confirmation(&self) -> bool {
        matches!(self, CommandCategory::Vehicle | CommandCategory::Phone)
    }
}

#[derive(Debug, Clone)]
pub struct VoiceCommand {
    pub text: String,
    pub category: CommandCategory,
    pub confidence: f64,
    pub language: String,
}

impl VoiceCommand {
    pub fn new(text: &str, category: CommandCategory, confidence: f64) -> Self {
        Self {
            text: text.to_string(),
            category,
            confidence,
            language: "en".to_string(),
        }
    }

    pub fn is_confident(&self) -> bool {
        self.confidence > 0.75
    }

    pub fn should_execute(&self) -> bool {
        self.is_confident() && (!self.category.requires_confirmation() || self.confidence > 0.95)
    }

    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    pub fn is_wake_word(&self) -> bool {
        let lower = self.text.to_lowercase();
        lower.starts_with("hey aurora") || lower.starts_with("ok aurora")
    }
}

#[derive(Debug, Clone)]
pub struct VoiceSystem {
    pub enabled: bool,
    pub listening: bool,
    pub wake_word: String,
    pub noise_level_db: f64,
    pub commands_processed: u64,
}

impl Default for VoiceSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceSystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            listening: false,
            wake_word: "hey aurora".to_string(),
            noise_level_db: 40.0,
            commands_processed: 0,
        }
    }

    pub fn can_hear(&self) -> bool {
        self.noise_level_db < 80.0
    }

    pub fn recognition_quality(&self) -> f64 {
        if self.noise_level_db <= 40.0 {
            1.0
        } else if self.noise_level_db < 60.0 {
            0.8
        } else if self.noise_level_db < 80.0 {
            0.5
        } else {
            0.2
        }
    }

    pub fn process_command(&mut self, cmd: &VoiceCommand) -> bool {
        if !self.enabled || !self.can_hear() {
            return false;
        }
        if cmd.should_execute() {
            self.commands_processed += 1;
            true
        } else {
            false
        }
    }

    pub fn noise_warning(&self) -> bool {
        self.noise_level_db > 70.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_priority() {
        assert!(CommandCategory::Vehicle.priority() > CommandCategory::Media.priority());
    }

    #[test]
    fn test_requires_confirm() {
        assert!(CommandCategory::Vehicle.requires_confirmation());
        assert!(!CommandCategory::Media.requires_confirmation());
    }

    #[test]
    fn test_confident() {
        let c = VoiceCommand::new("navigate home", CommandCategory::Navigation, 0.9);
        assert!(c.is_confident());
    }

    #[test]
    fn test_not_confident() {
        let c = VoiceCommand::new("mumble", CommandCategory::Navigation, 0.3);
        assert!(!c.is_confident());
    }

    #[test]
    fn test_should_execute() {
        let c = VoiceCommand::new("play music", CommandCategory::Media, 0.9);
        assert!(c.should_execute());
    }

    #[test]
    fn test_vehicle_needs_high_conf() {
        let c = VoiceCommand::new("lock doors", CommandCategory::Vehicle, 0.8);
        assert!(!c.should_execute());
    }

    #[test]
    fn test_wake_word() {
        let c = VoiceCommand::new("Hey Aurora navigate home", CommandCategory::Navigation, 0.9);
        assert!(c.is_wake_word());
    }

    #[test]
    fn test_word_count() {
        let c = VoiceCommand::new("navigate to work", CommandCategory::Navigation, 0.9);
        assert_eq!(c.word_count(), 3);
    }

    #[test]
    fn test_system_process() {
        let mut s = VoiceSystem::new();
        let cmd = VoiceCommand::new("play music", CommandCategory::Media, 0.9);
        assert!(s.process_command(&cmd));
        assert_eq!(s.commands_processed, 1);
    }

    #[test]
    fn test_noise_warning() {
        let mut s = VoiceSystem::new();
        s.noise_level_db = 75.0;
        assert!(s.noise_warning());
    }

    #[test]
    fn test_recognition_quality() {
        let s = VoiceSystem::new();
        assert!(s.recognition_quality() > 0.9);
    }
}
