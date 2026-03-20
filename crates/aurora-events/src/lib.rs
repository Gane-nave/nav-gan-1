//! AURORA NAV — Global Event Bus
//!
//! Event-driven architecture core. All system communication flows through
//! typed events with envelope metadata, idempotency, and signature support.

pub mod bus;
pub mod envelope;
pub mod event_type;
pub mod subscriber;

pub use bus::EventBus;
pub use envelope::EventEnvelope;
pub use event_type::EventType;
pub use subscriber::{EventHandler, Subscription};
