//! Finite state machine implementation.

use std::collections::{HashMap, HashSet};

use crate::transition::{Transition, TransitionResult};

/// A record of a state transition that occurred.
#[derive(Debug, Clone)]
pub struct TransitionRecord {
    /// The state before the transition.
    pub from: String,
    /// The event that triggered the transition.
    pub event: String,
    /// The state after the transition.
    pub to: String,
    /// Sequence number of this transition.
    pub sequence: u64,
}

/// A finite state machine with named states and event-driven transitions.
#[derive(Debug)]
pub struct StateMachine {
    /// Current state.
    current: String,
    /// Initial state.
    initial: String,
    /// Set of valid states.
    states: HashSet<String>,
    /// Set of terminal (accepting) states.
    terminal_states: HashSet<String>,
    /// Transitions: (from_state, event) -> Vec<Transition>
    transitions: HashMap<(String, String), Vec<Transition>>,
    /// Named guard conditions that block transitions.
    blocked_guards: HashSet<String>,
    /// History of transitions.
    history: Vec<TransitionRecord>,
    /// Sequence counter.
    sequence: u64,
}

impl StateMachine {
    /// Create a new state machine with the given initial state.
    pub fn new(initial: &str) -> Self {
        let mut states = HashSet::new();
        states.insert(initial.to_string());
        Self {
            current: initial.to_string(),
            initial: initial.to_string(),
            states,
            terminal_states: HashSet::new(),
            transitions: HashMap::new(),
            blocked_guards: HashSet::new(),
            history: Vec::new(),
            sequence: 0,
        }
    }

    /// Add a state to the machine.
    pub fn add_state(&mut self, name: &str) {
        self.states.insert(name.to_string());
    }

    /// Mark a state as terminal (accepting).
    pub fn add_terminal_state(&mut self, name: &str) {
        self.states.insert(name.to_string());
        self.terminal_states.insert(name.to_string());
    }

    /// Add a transition.
    pub fn add_transition(&mut self, transition: Transition) {
        self.states.insert(transition.from().to_string());
        self.states.insert(transition.to().to_string());
        let key = (
            transition.from().to_string(),
            transition.event().to_string(),
        );
        self.transitions.entry(key).or_default().push(transition);
    }

    /// Block a named guard (causes transitions with this guard to be rejected).
    pub fn block_guard(&mut self, guard: &str) {
        self.blocked_guards.insert(guard.to_string());
    }

    /// Unblock a named guard.
    pub fn unblock_guard(&mut self, guard: &str) {
        self.blocked_guards.remove(guard);
    }

    /// Check if a guard is blocked.
    pub fn is_guard_blocked(&self, guard: &str) -> bool {
        self.blocked_guards.contains(guard)
    }

    /// Send an event to the state machine, triggering a transition if defined.
    pub fn send(&mut self, event: &str) -> TransitionResult {
        let key = (self.current.clone(), event.to_string());
        let transitions = match self.transitions.get(&key) {
            Some(ts) => ts.clone(),
            None => {
                return TransitionResult::NoTransition {
                    state: self.current.clone(),
                    event: event.to_string(),
                };
            }
        };

        let mut last_rejected: Option<TransitionResult> = None;
        for t in &transitions {
            if let Some(guard) = t.guard() {
                if self.blocked_guards.contains(guard) {
                    last_rejected = Some(TransitionResult::GuardRejected {
                        from: self.current.clone(),
                        to: t.to().to_string(),
                        reason: format!("guard '{}' is blocked", guard),
                    });
                    continue;
                }
            }
            // Transition succeeds — first passable transition wins.
            let from = self.current.clone();
            self.current = t.to().to_string();
            self.sequence = self.sequence.saturating_add(1);
            self.history.push(TransitionRecord {
                from,
                event: event.to_string(),
                to: self.current.clone(),
                sequence: self.sequence,
            });
            return TransitionResult::Success(self.current.clone());
        }

        // All transitions had blocked guards.
        last_rejected.unwrap_or(TransitionResult::NoTransition {
            state: self.current.clone(),
            event: event.to_string(),
        })
    }

    /// Get the current state.
    pub fn current_state(&self) -> &str {
        &self.current
    }

    /// Get the initial state.
    pub fn initial_state(&self) -> &str {
        &self.initial
    }

    /// Check if the machine is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.terminal_states.contains(&self.current)
    }

    /// Reset the machine to its initial state, clearing history.
    pub fn reset(&mut self) {
        self.current = self.initial.clone();
        self.history.clear();
        self.sequence = 0;
    }

    /// Get the number of states.
    pub fn state_count(&self) -> usize {
        self.states.len()
    }

    /// Get all state names.
    pub fn states(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.states.iter().map(|s| s.as_str()).collect();
        names.sort();
        names
    }

    /// Get the transition history.
    pub fn history(&self) -> &[TransitionRecord] {
        &self.history
    }

    /// Get the number of transitions that have occurred.
    pub fn transition_count(&self) -> u64 {
        self.sequence
    }

    /// Get all valid events from the current state.
    pub fn available_events(&self) -> Vec<&str> {
        let mut events: Vec<&str> = self
            .transitions
            .keys()
            .filter(|(from, _)| from == &self.current)
            .map(|(_, event)| event.as_str())
            .collect();
        events.sort();
        events.dedup();
        events
    }

    /// Check if an event can be fired from the current state
    /// (transition exists and guard is not blocked).
    pub fn can_send(&self, event: &str) -> bool {
        let key = (self.current.clone(), event.to_string());
        match self.transitions.get(&key) {
            None => false,
            Some(ts) => ts
                .iter()
                .any(|t| t.guard().is_none_or(|g| !self.blocked_guards.contains(g))),
        }
    }

    /// Check if a state exists.
    pub fn has_state(&self, name: &str) -> bool {
        self.states.contains(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn traffic_light() -> StateMachine {
        let mut fsm = StateMachine::new("red");
        fsm.add_transition(Transition::new("red", "timer", "green"));
        fsm.add_transition(Transition::new("green", "timer", "yellow"));
        fsm.add_transition(Transition::new("yellow", "timer", "red"));
        fsm
    }

    #[test]
    fn test_initial_state() {
        let fsm = traffic_light();
        assert_eq!(fsm.current_state(), "red");
        assert_eq!(fsm.initial_state(), "red");
    }

    #[test]
    fn test_simple_transitions() {
        let mut fsm = traffic_light();
        assert_eq!(
            fsm.send("timer"),
            TransitionResult::Success("green".to_string())
        );
        assert_eq!(fsm.current_state(), "green");
        assert_eq!(
            fsm.send("timer"),
            TransitionResult::Success("yellow".to_string())
        );
        assert_eq!(
            fsm.send("timer"),
            TransitionResult::Success("red".to_string())
        );
    }

    #[test]
    fn test_no_transition() {
        let mut fsm = traffic_light();
        let result = fsm.send("invalid");
        assert!(matches!(result, TransitionResult::NoTransition { .. }));
        assert_eq!(fsm.current_state(), "red"); // unchanged
    }

    #[test]
    fn test_guard_blocks_transition() {
        let mut fsm = StateMachine::new("locked");
        fsm.add_transition(Transition::with_guard(
            "locked", "unlock", "unlocked", "has_key",
        ));
        fsm.block_guard("has_key");
        let result = fsm.send("unlock");
        assert!(matches!(result, TransitionResult::GuardRejected { .. }));
        assert_eq!(fsm.current_state(), "locked");
    }

    #[test]
    fn test_guard_unblock() {
        let mut fsm = StateMachine::new("locked");
        fsm.add_transition(Transition::with_guard(
            "locked", "unlock", "unlocked", "has_key",
        ));
        fsm.block_guard("has_key");
        assert!(!fsm.can_send("unlock"));
        fsm.unblock_guard("has_key");
        assert!(fsm.can_send("unlock"));
        assert_eq!(
            fsm.send("unlock"),
            TransitionResult::Success("unlocked".to_string())
        );
    }

    #[test]
    fn test_terminal_state() {
        let mut fsm = StateMachine::new("start");
        fsm.add_terminal_state("end");
        fsm.add_transition(Transition::new("start", "go", "end"));
        assert!(!fsm.is_terminal());
        fsm.send("go");
        assert!(fsm.is_terminal());
    }

    #[test]
    fn test_reset() {
        let mut fsm = traffic_light();
        fsm.send("timer");
        fsm.send("timer");
        assert_eq!(fsm.current_state(), "yellow");
        assert_eq!(fsm.transition_count(), 2);
        fsm.reset();
        assert_eq!(fsm.current_state(), "red");
        assert_eq!(fsm.transition_count(), 0);
        assert!(fsm.history().is_empty());
    }

    #[test]
    fn test_history() {
        let mut fsm = traffic_light();
        fsm.send("timer");
        fsm.send("timer");
        let hist = fsm.history();
        assert_eq!(hist.len(), 2);
        assert_eq!(hist[0].from, "red");
        assert_eq!(hist[0].to, "green");
        assert_eq!(hist[0].sequence, 1);
        assert_eq!(hist[1].from, "green");
        assert_eq!(hist[1].to, "yellow");
        assert_eq!(hist[1].sequence, 2);
    }

    #[test]
    fn test_state_count() {
        let fsm = traffic_light();
        assert_eq!(fsm.state_count(), 3);
    }

    #[test]
    fn test_available_events() {
        let fsm = traffic_light();
        let events = fsm.available_events();
        assert_eq!(events, vec!["timer"]);
    }

    #[test]
    fn test_can_send() {
        let fsm = traffic_light();
        assert!(fsm.can_send("timer"));
        assert!(!fsm.can_send("stop"));
    }

    #[test]
    fn test_has_state() {
        let fsm = traffic_light();
        assert!(fsm.has_state("red"));
        assert!(fsm.has_state("green"));
        assert!(!fsm.has_state("blue"));
    }

    #[test]
    fn test_states_sorted() {
        let fsm = traffic_light();
        let states = fsm.states();
        assert_eq!(states, vec!["green", "red", "yellow"]);
    }
}
