//! In-process event bus with typed routing, fan-out, and back-pressure.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;
use tracing::debug;

use crate::envelope::EventEnvelope;
use crate::event_type::EventType;
use crate::subscriber::{EventHandler, Subscription};

/// Thread-safe, in-process event bus.
///
/// Supports multiple subscribers with optional type filters.
/// Events are dispatched synchronously to all matching handlers.
/// For async / cross-process delivery, wrap this bus behind a channel
/// or message broker adapter.
pub struct EventBus {
    subscribers: RwLock<Vec<RegisteredHandler>>,
    next_id: AtomicU64,
    event_count: AtomicU64,
}

struct RegisteredHandler {
    id: u64,
    filter: Option<Vec<EventType>>,
    handler: Arc<dyn EventHandler>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(Vec::new()),
            next_id: AtomicU64::new(1),
            event_count: AtomicU64::new(0),
        }
    }

    /// Register a handler. Returns a `Subscription` that can be used to unsubscribe.
    pub fn subscribe(&self, handler: Arc<dyn EventHandler>) -> Subscription {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let filter = handler.subscribed_types();
        let sub = Subscription {
            id,
            filter: filter.clone(),
        };
        self.subscribers.write().push(RegisteredHandler {
            id,
            filter,
            handler,
        });
        debug!(subscription_id = id, "handler registered on event bus");
        sub
    }

    /// Remove a previously registered subscription.
    pub fn unsubscribe(&self, subscription: &Subscription) {
        self.subscribers.write().retain(|s| s.id != subscription.id);
        debug!(subscription_id = subscription.id, "handler unregistered");
    }

    /// Publish an event to all matching subscribers.
    pub fn publish(&self, event: &EventEnvelope) {
        self.event_count.fetch_add(1, Ordering::Relaxed);
        let readers = self.subscribers.read();
        for sub in readers.iter() {
            let matches = match &sub.filter {
                None => true,
                Some(types) => types.contains(&event.event_type),
            };
            if matches {
                sub.handler.handle(event);
            }
        }
    }

    /// Number of events published since creation.
    pub fn total_events(&self) -> u64 {
        self.event_count.load(Ordering::Relaxed)
    }

    /// Number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.read().len()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_type::EventType;
    use std::sync::atomic::AtomicU32;
    use uuid::Uuid;

    struct CountingHandler {
        count: AtomicU32,
        filter: Option<Vec<EventType>>,
    }

    impl CountingHandler {
        fn new(filter: Option<Vec<EventType>>) -> Self {
            Self {
                count: AtomicU32::new(0),
                filter,
            }
        }

        fn count(&self) -> u32 {
            self.count.load(Ordering::Relaxed)
        }
    }

    impl EventHandler for CountingHandler {
        fn handle(&self, _event: &EventEnvelope) {
            self.count.fetch_add(1, Ordering::Relaxed);
        }

        fn subscribed_types(&self) -> Option<Vec<EventType>> {
            self.filter.clone()
        }
    }

    fn make_event(et: EventType) -> EventEnvelope {
        EventEnvelope::new("test", "Test", Uuid::new_v4(), et, serde_json::json!({}))
    }

    #[test]
    fn publish_reaches_all_subscribers() {
        let bus = EventBus::new();
        let h1 = Arc::new(CountingHandler::new(None));
        let h2 = Arc::new(CountingHandler::new(None));
        bus.subscribe(h1.clone());
        bus.subscribe(h2.clone());

        bus.publish(&make_event(EventType::PositionUpdate));

        assert_eq!(h1.count(), 1);
        assert_eq!(h2.count(), 1);
        assert_eq!(bus.total_events(), 1);
    }

    #[test]
    fn type_filter_works() {
        let bus = EventBus::new();
        let gnss_only = Arc::new(CountingHandler::new(Some(vec![
            EventType::GnssMeasurementReceived,
        ])));
        let all = Arc::new(CountingHandler::new(None));
        bus.subscribe(gnss_only.clone());
        bus.subscribe(all.clone());

        bus.publish(&make_event(EventType::PositionUpdate));
        bus.publish(&make_event(EventType::GnssMeasurementReceived));

        assert_eq!(gnss_only.count(), 1);
        assert_eq!(all.count(), 2);
    }

    #[test]
    fn unsubscribe_works() {
        let bus = EventBus::new();
        let h = Arc::new(CountingHandler::new(None));
        let sub = bus.subscribe(h.clone());

        bus.publish(&make_event(EventType::PositionUpdate));
        assert_eq!(h.count(), 1);

        bus.unsubscribe(&sub);
        bus.publish(&make_event(EventType::PositionUpdate));
        assert_eq!(h.count(), 1);
        assert_eq!(bus.subscriber_count(), 0);
    }
}
