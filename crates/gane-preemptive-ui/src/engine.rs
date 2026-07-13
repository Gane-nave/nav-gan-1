/// Preemptive UI: predict what user needs before they ask.
#[derive(Debug, Clone, PartialEq)]
pub enum PreemptiveAction {
    SuggestDestination,
    ShowTrafficAhead,
    OfferAlternateRoute,
    ShowFuelStation,
    ShowRestStop,
    ShowParking,
}
#[derive(Debug, Clone)]
pub struct PreemptiveRule {
    pub action: PreemptiveAction,
    pub trigger_condition: String,
    pub confidence: f64,
    pub priority: u32,
}
impl PreemptiveRule {
    pub fn should_trigger(&self) -> bool {
        self.confidence > 0.6
    }
}
#[derive(Debug, Clone)]
pub struct PreemptiveEngine {
    pub rules: Vec<PreemptiveRule>,
}
impl Default for PreemptiveEngine {
    fn default() -> Self {
        Self::new()
    }
}
impl PreemptiveEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    pub fn add_rule(&mut self, r: PreemptiveRule) {
        self.rules.push(r);
    }
    pub fn active_actions(&self) -> Vec<&PreemptiveRule> {
        let mut triggered: Vec<_> = self.rules.iter().filter(|r| r.should_trigger()).collect();
        triggered.sort_by_key(|a| a.priority);
        triggered
    }
    pub fn top_action(&self) -> Option<&PreemptiveRule> {
        self.active_actions().into_iter().next()
    }
    pub fn action_count(&self) -> usize {
        self.active_actions().len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trigger() {
        let r = PreemptiveRule {
            action: PreemptiveAction::SuggestDestination,
            trigger_condition: "morning_commute".into(),
            confidence: 0.9,
            priority: 1,
        };
        assert!(r.should_trigger());
    }
    #[test]
    fn test_no_trigger() {
        let r = PreemptiveRule {
            action: PreemptiveAction::ShowFuelStation,
            trigger_condition: "low_fuel".into(),
            confidence: 0.3,
            priority: 5,
        };
        assert!(!r.should_trigger());
    }
    #[test]
    fn test_engine() {
        let mut e = PreemptiveEngine::new();
        e.add_rule(PreemptiveRule {
            action: PreemptiveAction::SuggestDestination,
            trigger_condition: "c".into(),
            confidence: 0.9,
            priority: 1,
        });
        e.add_rule(PreemptiveRule {
            action: PreemptiveAction::ShowFuelStation,
            trigger_condition: "c".into(),
            confidence: 0.3,
            priority: 5,
        });
        assert_eq!(e.action_count(), 1);
        assert_eq!(
            e.top_action().unwrap().action,
            PreemptiveAction::SuggestDestination
        );
    }
    #[test]
    fn test_empty() {
        let e = PreemptiveEngine::new();
        assert!(e.top_action().is_none());
    }
}
