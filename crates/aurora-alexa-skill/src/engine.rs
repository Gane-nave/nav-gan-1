/// Alexa skill: intent, slot, dialog, response, account
/// Phase 989

#[derive(Debug, Clone)]
pub struct AlexaSkill {
    pub intent_ok: bool,
    pub slot_ok: bool,
    pub dialog_ok: bool,
    pub response_ok: bool,
    pub account_ok: bool,
}

impl Default for AlexaSkill {
    fn default() -> Self {
        Self::new()
    }
}

impl AlexaSkill {
    pub fn new() -> Self {
        Self {
            intent_ok: true,
            slot_ok: true,
            dialog_ok: true,
            response_ok: true,
            account_ok: true,
        }
    }

    pub fn understanding_ok(&self) -> bool {
        self.intent_ok && self.slot_ok && self.dialog_ok
    }

    pub fn output_ok(&self) -> bool {
        self.response_ok && self.account_ok
    }

    pub fn all_ok(&self) -> bool {
        self.understanding_ok() && self.output_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.intent_ok || !self.account_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.intent_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_understanding() {
        let c = AlexaSkill::new();
        assert!(c.understanding_ok());
    }

    #[test]
    fn test_output() {
        let c = AlexaSkill::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AlexaSkill::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = AlexaSkill::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_intent() {
        let mut c = AlexaSkill::new();
        c.intent_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = AlexaSkill::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
