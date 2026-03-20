//! Transition definitions for the finite state machine.

/// Result of a transition attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionResult {
    /// Transition succeeded, now in the given state.
    Success(String),
    /// Transition was rejected by a guard.
    GuardRejected {
        from: String,
        to: String,
        reason: String,
    },
    /// No transition defined for this event in the current state.
    NoTransition {
        state: String,
        event: String,
    },
}

/// A transition definition in the state machine.
#[derive(Debug, Clone)]
pub struct Transition {
    /// Source state.
    from: String,
    /// Event that triggers this transition.
    event: String,
    /// Target state.
    to: String,
    /// Optional guard condition name.
    guard: Option<String>,
}

impl Transition {
    /// Create a new transition.
    pub fn new(from: &str, event: &str, to: &str) -> Self {
        Self {
            from: from.to_string(),
            event: event.to_string(),
            to: to.to_string(),
            guard: None,
        }
    }

    /// Create a transition with a named guard.
    pub fn with_guard(from: &str, event: &str, to: &str, guard: &str) -> Self {
        Self {
            from: from.to_string(),
            event: event.to_string(),
            to: to.to_string(),
            guard: Some(guard.to_string()),
        }
    }

    /// Get the source state.
    pub fn from(&self) -> &str {
        &self.from
    }

    /// Get the event name.
    pub fn event(&self) -> &str {
        &self.event
    }

    /// Get the target state.
    pub fn to(&self) -> &str {
        &self.to
    }

    /// Get the guard name, if any.
    pub fn guard(&self) -> Option<&str> {
        self.guard.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_creation() {
        let t = Transition::new("idle", "start", "running");
        assert_eq!(t.from(), "idle");
        assert_eq!(t.event(), "start");
        assert_eq!(t.to(), "running");
        assert!(t.guard().is_none());
    }

    #[test]
    fn test_transition_with_guard() {
        let t = Transition::with_guard("idle", "start", "running", "is_ready");
        assert_eq!(t.guard(), Some("is_ready"));
    }

    #[test]
    fn test_transition_result_success() {
        let r = TransitionResult::Success("running".to_string());
        assert_eq!(r, TransitionResult::Success("running".to_string()));
    }

    #[test]
    fn test_transition_result_guard_rejected() {
        let r = TransitionResult::GuardRejected {
            from: "idle".to_string(),
            to: "running".to_string(),
            reason: "not ready".to_string(),
        };
        if let TransitionResult::GuardRejected { from, to, reason } = &r {
            assert_eq!(from, "idle");
            assert_eq!(to, "running");
            assert_eq!(reason, "not ready");
        } else {
            panic!("expected GuardRejected");
        }
    }

    #[test]
    fn test_transition_result_no_transition() {
        let r = TransitionResult::NoTransition {
            state: "idle".to_string(),
            event: "stop".to_string(),
        };
        if let TransitionResult::NoTransition { state, event } = &r {
            assert_eq!(state, "idle");
            assert_eq!(event, "stop");
        } else {
            panic!("expected NoTransition");
        }
    }

    #[test]
    fn test_transition_clone() {
        let t = Transition::with_guard("a", "b", "c", "g");
        let t2 = t.clone();
        assert_eq!(t.from(), t2.from());
        assert_eq!(t.guard(), t2.guard());
    }
}
