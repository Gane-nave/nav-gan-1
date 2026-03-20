//! Event subscription and handler traits.

use crate::envelope::EventEnvelope;
use crate::event_type::EventType;

/// Trait for handling events from the bus.
pub trait EventHandler: Send + Sync {
    /// Process a single event. Implementations must be idempotent.
    fn handle(&self, event: &EventEnvelope);

    /// Which event types this handler is interested in.
    /// Return `None` to receive all events.
    fn subscribed_types(&self) -> Option<Vec<EventType>>;
}

/// A registered subscription on the event bus.
#[derive(Debug)]
pub struct Subscription {
    pub id: u64,
    pub filter: Option<Vec<EventType>>,
}
