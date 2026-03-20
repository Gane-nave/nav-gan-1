use aurora_fsm::{StateMachine, Transition, TransitionResult};

#[test]
fn adversarial_fsm_guards_terminal_reset_history() {
    // Phase 1: Build a door lock FSM
    // States: locked -> unlocked -> open -> locked (cycle)
    // Guards: has_key (for unlock), is_clear (for open)
    let mut fsm = StateMachine::new("locked");
    fsm.add_terminal_state("broken");
    fsm.add_transition(Transition::with_guard("locked", "unlock", "unlocked", "has_key"));
    fsm.add_transition(Transition::with_guard("unlocked", "open", "open", "is_clear"));
    fsm.add_transition(Transition::new("open", "close", "unlocked"));
    fsm.add_transition(Transition::new("unlocked", "lock", "locked"));
    fsm.add_transition(Transition::new("locked", "break", "broken"));
    fsm.add_transition(Transition::new("unlocked", "break", "broken"));
    fsm.add_transition(Transition::new("open", "break", "broken"));

    assert_eq!(fsm.state_count(), 4); // locked, unlocked, open, broken
    assert_eq!(fsm.current_state(), "locked");
    assert!(!fsm.is_terminal());

    // Phase 2: Test guard blocking
    fsm.block_guard("has_key");
    fsm.block_guard("is_clear");

    let result = fsm.send("unlock");
    assert!(matches!(result, TransitionResult::GuardRejected { .. }));
    assert_eq!(fsm.current_state(), "locked"); // didn't move

    // Phase 3: Unblock has_key, try again
    fsm.unblock_guard("has_key");
    assert!(fsm.can_send("unlock"));
    let result = fsm.send("unlock");
    assert_eq!(result, TransitionResult::Success("unlocked".to_string()));
    assert_eq!(fsm.current_state(), "unlocked");

    // Phase 4: Try to open with guard still blocked
    let result = fsm.send("open");
    assert!(matches!(result, TransitionResult::GuardRejected { .. }));
    assert_eq!(fsm.current_state(), "unlocked");

    // Phase 5: Unblock and open
    fsm.unblock_guard("is_clear");
    let result = fsm.send("open");
    assert_eq!(result, TransitionResult::Success("open".to_string()));

    // Phase 6: Close and lock (full cycle)
    assert_eq!(fsm.send("close"), TransitionResult::Success("unlocked".to_string()));
    assert_eq!(fsm.send("lock"), TransitionResult::Success("locked".to_string()));
    assert_eq!(fsm.current_state(), "locked");

    // Phase 7: Verify history
    let history = fsm.history();
    assert_eq!(history.len(), 4); // unlock, open, close, lock
    assert_eq!(history[0].from, "locked");
    assert_eq!(history[0].to, "unlocked");
    assert_eq!(history[0].event, "unlock");
    assert_eq!(history[1].from, "unlocked");
    assert_eq!(history[1].to, "open");
    assert_eq!(history[2].from, "open");
    assert_eq!(history[2].to, "unlocked");
    assert_eq!(history[3].from, "unlocked");
    assert_eq!(history[3].to, "locked");
    assert_eq!(fsm.transition_count(), 4);

    // Phase 8: Invalid event
    let result = fsm.send("fly");
    assert!(matches!(result, TransitionResult::NoTransition { .. }));
    assert_eq!(fsm.current_state(), "locked"); // unchanged

    // Phase 9: Break the door (terminal state)
    let result = fsm.send("break");
    assert_eq!(result, TransitionResult::Success("broken".to_string()));
    assert!(fsm.is_terminal());

    // No events available from terminal state (unless explicitly added)
    let events = fsm.available_events();
    assert!(events.is_empty());

    // Phase 10: Reset and verify clean state
    fsm.reset();
    assert_eq!(fsm.current_state(), "locked");
    assert_eq!(fsm.transition_count(), 0);
    assert!(fsm.history().is_empty());
    assert!(!fsm.is_terminal());

    // Phase 11: Rapid cycling (stress)
    for _ in 0..100 {
        fsm.send("unlock"); // locked -> unlocked
        fsm.send("open");   // unlocked -> open
        fsm.send("close");  // open -> unlocked
        fsm.send("lock");   // unlocked -> locked
    }
    assert_eq!(fsm.current_state(), "locked");
    assert_eq!(fsm.transition_count(), 400);
}
