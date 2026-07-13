//! Finite state machine for state management.
//!
//! Provides a generic FSM with typed states, transitions, guards,
//! and transition history tracking.

mod machine;
mod transition;

pub use machine::StateMachine;
pub use transition::{Transition, TransitionResult};
