//! Plugin lifecycle management — state machine for plugin initialization, activation, and teardown.

/// Plugin lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    /// Plugin discovered but not yet initialized.
    Discovered,
    /// Plugin is initializing.
    Initializing,
    /// Plugin initialized and ready to activate.
    Initialized,
    /// Plugin is activating.
    Activating,
    /// Plugin is fully active.
    Active,
    /// Plugin is deactivating.
    Deactivating,
    /// Plugin is being destroyed.
    Destroying,
    /// Plugin encountered a fatal error.
    Failed,
}

/// Lifecycle transition result.
#[derive(Debug, Clone)]
pub struct TransitionResult {
    /// Previous state.
    pub from: LifecycleState,
    /// New state.
    pub to: LifecycleState,
    /// Whether the transition was valid.
    pub valid: bool,
    /// Error message if invalid.
    pub error: Option<String>,
}

/// Plugin lifecycle manager — enforces valid state transitions.
pub struct LifecycleManager {
    plugin_id: String,
    state: LifecycleState,
    history: Vec<TransitionResult>,
    max_retries: u32,
    retry_count: u32,
}

impl LifecycleManager {
    /// Create a new lifecycle manager for a plugin.
    pub fn new(plugin_id: &str, max_retries: u32) -> Self {
        Self {
            plugin_id: plugin_id.to_string(),
            state: LifecycleState::Discovered,
            history: Vec::new(),
            max_retries,
            retry_count: 0,
        }
    }

    /// Get current state.
    pub fn state(&self) -> LifecycleState {
        self.state
    }

    /// Get the plugin ID.
    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    /// Check if a transition is valid from the current state.
    fn is_valid_transition(&self, to: LifecycleState) -> bool {
        matches!(
            (self.state, to),
            (LifecycleState::Discovered, LifecycleState::Initializing)
                | (LifecycleState::Initializing, LifecycleState::Initialized)
                | (LifecycleState::Initializing, LifecycleState::Failed)
                | (LifecycleState::Initialized, LifecycleState::Activating)
                | (LifecycleState::Activating, LifecycleState::Active)
                | (LifecycleState::Activating, LifecycleState::Failed)
                | (LifecycleState::Active, LifecycleState::Deactivating)
                | (LifecycleState::Deactivating, LifecycleState::Initialized)
                | (LifecycleState::Deactivating, LifecycleState::Destroying)
                | (LifecycleState::Initialized, LifecycleState::Destroying)
                | (LifecycleState::Failed, LifecycleState::Initializing) // retry
                | (LifecycleState::Failed, LifecycleState::Destroying)
                | (LifecycleState::Destroying, LifecycleState::Discovered) // full reset
        )
    }

    /// Transition to a new state.
    pub fn transition(&mut self, to: LifecycleState) -> TransitionResult {
        let valid = self.is_valid_transition(to);
        let result = if valid {
            let from = self.state;
            // Track retries on Failed → Initializing
            if from == LifecycleState::Failed && to == LifecycleState::Initializing {
                if self.retry_count >= self.max_retries {
                    return TransitionResult {
                        from,
                        to,
                        valid: false,
                        error: Some(format!(
                            "Max retries ({}) exceeded for plugin '{}'",
                            self.max_retries, self.plugin_id
                        )),
                    };
                }
                self.retry_count += 1;
            }
            // Reset retry count on successful activation
            if to == LifecycleState::Active {
                self.retry_count = 0;
            }
            self.state = to;
            TransitionResult {
                from,
                to,
                valid: true,
                error: None,
            }
        } else {
            TransitionResult {
                from: self.state,
                to,
                valid: false,
                error: Some(format!(
                    "Invalid transition {:?} → {:?} for plugin '{}'",
                    self.state, to, self.plugin_id
                )),
            }
        };
        self.history.push(result.clone());
        result
    }

    /// Initialize the plugin (Discovered → Initializing → Initialized).
    pub fn initialize(&mut self) -> TransitionResult {
        let r1 = self.transition(LifecycleState::Initializing);
        if !r1.valid {
            return r1;
        }
        self.transition(LifecycleState::Initialized)
    }

    /// Activate the plugin (Initialized → Activating → Active).
    pub fn activate(&mut self) -> TransitionResult {
        let r1 = self.transition(LifecycleState::Activating);
        if !r1.valid {
            return r1;
        }
        self.transition(LifecycleState::Active)
    }

    /// Deactivate the plugin (Active → Deactivating → Initialized).
    pub fn deactivate(&mut self) -> TransitionResult {
        let r1 = self.transition(LifecycleState::Deactivating);
        if !r1.valid {
            return r1;
        }
        self.transition(LifecycleState::Initialized)
    }

    /// Destroy the plugin.
    pub fn destroy(&mut self) -> TransitionResult {
        // Try deactivating first if active
        if self.state == LifecycleState::Active {
            let _ = self.transition(LifecycleState::Deactivating);
        }
        self.transition(LifecycleState::Destroying)
    }

    /// Get transition history.
    pub fn history(&self) -> &[TransitionResult] {
        &self.history
    }

    /// Get retry count.
    pub fn retry_count(&self) -> u32 {
        self.retry_count
    }

    /// Check if the plugin is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.state,
            LifecycleState::Failed | LifecycleState::Destroying
        )
    }

    /// Check if the plugin is active.
    pub fn is_active(&self) -> bool {
        self.state == LifecycleState::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path_lifecycle() {
        let mut lm = LifecycleManager::new("test-plugin", 3);
        assert_eq!(lm.state(), LifecycleState::Discovered);

        let r = lm.initialize();
        assert!(r.valid);
        assert_eq!(lm.state(), LifecycleState::Initialized);

        let r = lm.activate();
        assert!(r.valid);
        assert_eq!(lm.state(), LifecycleState::Active);
        assert!(lm.is_active());

        let r = lm.deactivate();
        assert!(r.valid);
        assert_eq!(lm.state(), LifecycleState::Initialized);
        assert!(!lm.is_active());
    }

    #[test]
    fn test_invalid_transition() {
        let mut lm = LifecycleManager::new("test", 3);
        // Cannot go directly from Discovered to Active
        let r = lm.transition(LifecycleState::Active);
        assert!(!r.valid);
        assert!(r.error.is_some());
        assert_eq!(lm.state(), LifecycleState::Discovered); // unchanged
    }

    #[test]
    fn test_failure_and_retry() {
        let mut lm = LifecycleManager::new("test", 2);
        lm.transition(LifecycleState::Initializing);
        lm.transition(LifecycleState::Failed);
        assert_eq!(lm.state(), LifecycleState::Failed);

        // Retry 1
        let r = lm.transition(LifecycleState::Initializing);
        assert!(r.valid);
        assert_eq!(lm.retry_count(), 1);

        // Fail again
        lm.transition(LifecycleState::Failed);

        // Retry 2
        let r = lm.transition(LifecycleState::Initializing);
        assert!(r.valid);
        assert_eq!(lm.retry_count(), 2);

        // Fail again
        lm.transition(LifecycleState::Failed);

        // Retry 3 — should fail (max_retries = 2)
        let r = lm.transition(LifecycleState::Initializing);
        assert!(!r.valid);
        assert!(r.error.unwrap().contains("Max retries"));
    }

    #[test]
    fn test_retry_count_resets_on_activation() {
        let mut lm = LifecycleManager::new("test", 3);
        lm.transition(LifecycleState::Initializing);
        lm.transition(LifecycleState::Failed);
        lm.transition(LifecycleState::Initializing); // retry 1
        lm.transition(LifecycleState::Initialized);
        lm.transition(LifecycleState::Activating);
        lm.transition(LifecycleState::Active);
        assert_eq!(lm.retry_count(), 0); // reset
    }

    #[test]
    fn test_destroy_from_active() {
        let mut lm = LifecycleManager::new("test", 3);
        lm.initialize();
        lm.activate();
        let r = lm.destroy();
        assert!(r.valid);
        assert_eq!(lm.state(), LifecycleState::Destroying);
        assert!(lm.is_terminal());
    }

    #[test]
    fn test_destroy_from_failed() {
        let mut lm = LifecycleManager::new("test", 3);
        lm.transition(LifecycleState::Initializing);
        lm.transition(LifecycleState::Failed);
        let r = lm.destroy();
        assert!(r.valid);
        assert_eq!(lm.state(), LifecycleState::Destroying);
    }

    #[test]
    fn test_history_tracking() {
        let mut lm = LifecycleManager::new("test", 3);
        lm.initialize();
        lm.activate();
        // initialize does 2 transitions, activate does 2 = 4 total
        assert_eq!(lm.history().len(), 4);
    }

    #[test]
    fn test_full_reset_cycle() {
        let mut lm = LifecycleManager::new("test", 3);
        lm.initialize();
        lm.activate();
        lm.deactivate();
        let r = lm.transition(LifecycleState::Destroying);
        assert!(r.valid);
        let r = lm.transition(LifecycleState::Discovered);
        assert!(r.valid);
        assert_eq!(lm.state(), LifecycleState::Discovered);
    }

    #[test]
    fn test_plugin_id() {
        let lm = LifecycleManager::new("my-plugin", 3);
        assert_eq!(lm.plugin_id(), "my-plugin");
    }
}
